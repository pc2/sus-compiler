use std::{ops::{Deref, Range}, iter::zip};

use crate::{
    ast::{Span, Module, Expression, SpanExpression, LocalOrGlobal, Operator, AssignableExpression, SpanAssignableExpression, Statement, CodeBlock, IdentifierType, GlobalReference, TypeExpression, DeclIDMarker, SignalDeclaration},
    linker::{Linker, Named, Linkable, get_builtin_uuid, FileUUID, NamedUUID},
    errors::{ErrorCollector, error_info}, arena_alloc::{ListAllocator, UUID, UUIDMarker, FlatAlloc}, tokenizer::kw, typing::{Type, typecheck_unary_operator, get_binary_operator_types, typecheck, typecheck_is_array_indexer}, value::Value
};

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct FlatIDMarker;
impl UUIDMarker for FlatIDMarker {const DISPLAY_NAME : &'static str = "obj_";}
pub type FlatID = UUID<FlatIDMarker>;

pub type FieldID = usize;

#[derive(Debug)]
pub enum ConnectionWritePathElement {
    ArrayIdx{idx : FlatID, idx_span : Span},
    //StructField(FieldID)
}
#[derive(Debug)]
pub enum ConnectionWritePathElementComputed {
    ArrayIdx(usize)
}

// These are assignable connections
#[derive(Debug)]
pub struct ConnectionWrite {
    pub root : FlatID,
    pub path : Vec<ConnectionWritePathElement>,
    pub span : Span
}

impl ConnectionWrite {
    pub fn simple(root : FlatID, span : Span) -> ConnectionWrite {
        ConnectionWrite { root, path: Vec::new(), span }
    }
}

#[derive(Debug)]
pub struct Connection {
    pub num_regs : i64,
    pub from : FlatID,
    pub to : ConnectionWrite,
    pub condition : Option<FlatID>
}

#[derive(Debug)]
pub enum WireSource {
    WireRead{from_wire : FlatID}, // Used to add a span to the reference of a wire. 
    UnaryOp{op : Operator, right : FlatID},
    BinaryOp{op : Operator, left : FlatID, right : FlatID},
    ArrayAccess{arr : FlatID, arr_idx : FlatID},
    Constant{value : Value},
}

impl WireSource {
    pub fn for_each_input_wire<F : FnMut(FlatID)>(&self, func : &mut F) {
        match self {
            &WireSource::WireRead { from_wire } => {func(from_wire)}
            &WireSource::UnaryOp { op:_, right } => {func(right)}
            &WireSource::BinaryOp { op:_, left, right } => {func(left); func(right)}
            &WireSource::ArrayAccess { arr, arr_idx } => {func(arr); func(arr_idx)}
            WireSource::Constant { value:_ } => {}
        }
    }
}

#[derive(Debug)]
pub struct WireInstance {
    pub typ : Type,
    pub is_compiletime : bool,
    pub span : Span,
    pub source : WireSource
}

#[derive(Debug)]
pub struct WireDeclaration {
    pub typ : Type,
    pub typ_span : Span,
    pub read_only : bool,
    pub identifier_type : IdentifierType,
    pub name : Box<str>,
    pub name_token : Option<usize>
}
impl WireDeclaration {
    pub fn get_full_decl_span(&self) -> Span {
        let name_pos = self.name_token.unwrap(); // Temporary, name_token should always be present for proper declarations. 
        Span(self.typ_span.0, name_pos)
    }
}

#[derive(Debug)]
pub struct SubModuleInstance {
    pub module_uuid : NamedUUID,
    pub name : Box<str>,
    pub typ_span : Span,
    pub outputs_start : usize,
    pub local_wires : Box<[FlatID]>
}
impl SubModuleInstance {
    pub fn inputs(&self) -> &[FlatID] {
        &self.local_wires[..self.outputs_start]
    }
    pub fn outputs(&self) -> &[FlatID] {
        &self.local_wires[self.outputs_start..]
    }
}

#[derive(Debug)]
pub enum Instantiation {
    SubModule(SubModuleInstance),
    WireDeclaration(WireDeclaration),
    Wire(WireInstance),
    Connection(Connection)
}

impl Instantiation {
    #[track_caller]
    pub fn extract_wire(&self) -> &WireInstance {
        let Self::Wire(w) = self else {panic!("extract_wire on not a wire! Found {self:?}")};
        w
    }
    #[track_caller]
    pub fn extract_wire_declaration(&self) -> &WireDeclaration {
        let Self::WireDeclaration(w) = self else {panic!("extract_wire on not a WireDeclaration! Found {self:?}")};
        w
    }

    pub fn for_each_embedded_type<F : FnMut(&Type, Span)>(&self, f : &mut F) {
        match self {
            Instantiation::SubModule(_) => {}
            Instantiation::Connection(_) => {}
            Instantiation::WireDeclaration(decl) => {
                f(&decl.typ, decl.typ_span);
            }
            Instantiation::Wire(w) => {
                f(&w.typ, w.span);
            }
        }
    }
}

struct FlatteningContext<'l, 'm, 'fl> {
    decl_to_flat_map : FlatAlloc<Option<FlatID>, DeclIDMarker>,
    instantiations : &'fl ListAllocator<Instantiation, FlatIDMarker>,
    errors : &'fl ErrorCollector,

    linker : &'l Linker,
    module : &'m Module,
}

impl<'l, 'm, 'fl> FlatteningContext<'l, 'm, 'fl> {
    pub fn map_to_type(&self, type_expr : &TypeExpression, global_references : &[GlobalReference]) -> Type {
        match type_expr {
            TypeExpression::Named(n) => {
                if let Some(found_ref) = global_references[*n].1 {
                    Type::Named(found_ref)
                } else {
                    Type::Error
                }
            }
            TypeExpression::Array(b) => {
                let (array_type_expr, array_size_expr) = b.deref();
                let array_element_type = self.map_to_type(&array_type_expr.0, global_references);
                if let Some(array_size_wire_id) = self.flatten_single_expr(array_size_expr, None) {
                    let array_size_wire = self.instantiations[array_size_wire_id].extract_wire();
                    if !array_size_wire.is_compiletime {
                        self.errors.error_basic(array_size_expr.1, "Array size must be compile time");
                    }
                    Type::Array(Box::new((array_element_type, array_size_wire_id)))
                } else {
                    Type::Error
                }
            }
        }
    }
    fn alloc_module_interface(&self, name : Box<str>, module : &Module, module_uuid : NamedUUID, typ_span : Span) -> FlatID {
        let flattened_borrow = module.flattened.borrow();
        let local_wires : Vec<FlatID> = flattened_borrow.interface.interface_wires.iter().enumerate().map(|(port_idx, port)| {
            self.instantiations.alloc(Instantiation::WireDeclaration(WireDeclaration{
                typ: port.typ.clone(),
                typ_span,
                read_only : port_idx >= flattened_borrow.interface.outputs_start,
                identifier_type : IdentifierType::Virtual,
                name : format!("{}_{}", &name, &port.port_name).into_boxed_str(),
                name_token : None
            }))
        }).collect();

        self.instantiations.alloc(Instantiation::SubModule(SubModuleInstance{
            name,
            module_uuid,
            typ_span,
            outputs_start : flattened_borrow.interface.outputs_start,
            local_wires : local_wires.into_boxed_slice()
        }))
    }
    fn desugar_func_call(&self, func_and_args : &[SpanExpression], closing_bracket_pos : usize, condition : Option<FlatID>) -> Option<(&Module, &[FlatID])> {
        let (name_expr, name_expr_span) = &func_and_args[0]; // Function name is always there
        let func_instantiation_id = match name_expr {
            Expression::Named(LocalOrGlobal::Local(l)) => {
                self.decl_to_flat_map[*l].unwrap()
            }
            Expression::Named(LocalOrGlobal::Global(g)) => {
                let module_ref = self.module.link_info.global_references[*g];

                let dependency = self.linker.try_get_module(module_ref, &self.errors)?;
                self.alloc_module_interface(dependency.link_info.name.clone(), dependency, module_ref.1?, *name_expr_span)
            }
            _other => {
                self.errors.error_basic(*name_expr_span, "Function call name cannot be an expression");
                return None;
            }
        };
        let func_instantiation = &self.instantiations[func_instantiation_id];
        let Instantiation::SubModule(SubModuleInstance{module_uuid, name : _, typ_span : _, outputs_start:_, local_wires}) = func_instantiation else {unreachable!("It should be proven {func_instantiation:?} was a Module!");};
        let Named::Module(md) = &self.linker.links.globals[*module_uuid] else {unreachable!("UUID Should be a module!");};
        let (inputs, output_range) = md.flattened.borrow().interface.func_call_syntax_interface();

        let mut args = &func_and_args[1..];

        let arg_count = args.len();
        let expected_arg_count = inputs.len();
        if arg_count != expected_arg_count {
            let module_info = vec![error_info(md.link_info.span, md.link_info.file, "Interface defined here")];
            if arg_count > expected_arg_count {
                // Too many args, complain about excess args at the end
                let excess_args_span = Span(args[expected_arg_count].1.0, closing_bracket_pos - 1);
                self.errors.error_with_info(excess_args_span, format!("Excess argument. Function takes {expected_arg_count} args, but {arg_count} were passed."), module_info);
                // Shorten args to still get proper type checking for smaller arg array
                args = &args[..expected_arg_count];
            } else {
                // Too few args, mention missing argument names
                self.errors.error_with_info(Span::from(closing_bracket_pos), format!("Too few arguments. Function takes {expected_arg_count} args, but {arg_count} were passed."), module_info);
            }
        }

        for (field, arg_expr) in zip(inputs, args) {
            if let Some(arg_read_side) = self.flatten_single_expr(arg_expr, condition) {
                /*if self.typecheck(arg_read_side, &md.interface.interface_wires[field].typ, "submodule output") == None {
                    continue;
                }*/
                let func_input_port = &local_wires[field];
                self.instantiations.alloc(Instantiation::Connection(Connection { num_regs: 0, from: arg_read_side, to: ConnectionWrite::simple(*func_input_port, *name_expr_span), condition }));
            }
        }

        Some((md, &local_wires[output_range]))
    }
    fn flatten_single_expr(&self, (expr, expr_span) : &SpanExpression, condition : Option<FlatID>) -> Option<FlatID> {
        let (is_compiletime, source) = match expr {
            Expression::Named(LocalOrGlobal::Local(l)) => {
                let from_wire = self.decl_to_flat_map[*l].unwrap();
                let WireDeclaration { typ: _, typ_span:_, read_only:_, identifier_type, name:_, name_token:_ } = self.instantiations[from_wire].extract_wire_declaration();
                (*identifier_type == IdentifierType::Generative, WireSource::WireRead{from_wire})
            }
            Expression::Named(LocalOrGlobal::Global(g)) => {
                let r = self.module.link_info.global_references[*g];
                let cst = self.linker.try_get_constant(r, &self.errors)?;
                (true, WireSource::Constant{value : cst})
            }
            Expression::Constant(cst) => {
                (true, WireSource::Constant{value : cst.clone()})
            }
            Expression::UnaryOp(op_box) => {
                let (op, _op_pos, operate_on) = op_box.deref();
                let right = self.flatten_single_expr(operate_on, condition)?;
                let right_wire = self.instantiations[right].extract_wire();
                (right_wire.is_compiletime, WireSource::UnaryOp{op : *op, right})
            }
            Expression::BinOp(binop_box) => {
                let (left_expr, op, _op_pos, right_expr) = binop_box.deref();
                let left = self.flatten_single_expr(left_expr, condition)?;
                let right = self.flatten_single_expr(right_expr, condition)?;
                let left_wire = self.instantiations[left].extract_wire();
                let right_wire = self.instantiations[right].extract_wire();
                let is_compiletime = left_wire.is_compiletime && right_wire.is_compiletime;
                (is_compiletime, WireSource::BinaryOp{op : *op, left, right})
            }
            Expression::Array(arr_box) => {
                let (left, right, _bracket_span) = arr_box.deref();
                let arr = self.flatten_single_expr(left, condition)?;
                let arr_idx = self.flatten_single_expr(right, condition)?;
                let arr_wire = self.instantiations[arr].extract_wire();
                let arr_idx_wire = self.instantiations[arr_idx].extract_wire();
                (arr_wire.is_compiletime && arr_idx_wire.is_compiletime, WireSource::ArrayAccess{arr, arr_idx})
            }
            Expression::FuncCall(func_and_args) => {
                let (md, outputs) = self.desugar_func_call(func_and_args, expr_span.1, condition)?;

                if outputs.len() != 1 {
                    let info = error_info(md.link_info.span, md.link_info.file, "Module Defined here");
                    self.errors.error_with_info(*expr_span, "A function called in this context may only return one result. Split this function call into a separate line instead.", vec![info]);
                    return None;
                }

                return Some(outputs[0])
            }
        };

        Some(self.instantiations.alloc(Instantiation::Wire(WireInstance{typ : Type::Unknown, span : *expr_span, is_compiletime, source})))
    }
    fn flatten_assignable_expr(&self, (expr, span) : &SpanAssignableExpression, condition : Option<FlatID>) -> Option<ConnectionWrite> {
        Some(match expr {
            AssignableExpression::Named{local_idx} => {
                let root = self.decl_to_flat_map[*local_idx].unwrap();
                let decl = self.instantiations[root].extract_wire_declaration();

                if decl.read_only {
                    let decl_info = error_info(self.module.declarations[*local_idx].span, self.errors.file, "Declared here");
                    self.errors.error_with_info(*span, "Cannot Assign to Read-Only value", vec![decl_info]);
                    return None
                }
                ConnectionWrite{root, path : Vec::new(), span : *span}
            }
            AssignableExpression::ArrayIndex(arr_box) => {
                let (arr, idx_expr, _bracket_span) = arr_box.deref();
                let flattened_arr_expr_opt = self.flatten_assignable_expr(arr, condition);
                
                let idx = self.flatten_single_expr(idx_expr, condition)?;

                let mut flattened_arr_expr = flattened_arr_expr_opt?; // only unpack the subexpr after flattening the idx, so we catch all errors

                flattened_arr_expr.path.push(ConnectionWritePathElement::ArrayIdx{idx, idx_span : idx_expr.1});

                flattened_arr_expr
            }
        })
    }
    fn extend_condition(&self, condition : Option<FlatID>, additional_condition : FlatID) -> FlatID {
        if let Some(condition) = condition {
            let bool_typ = Type::Named(get_builtin_uuid("bool"));
            let prev_condition_wire = self.instantiations[condition].extract_wire();
            let additional_condition_wire = self.instantiations[condition].extract_wire();
            assert!(!prev_condition_wire.is_compiletime); // Conditions are only used for runtime conditions. Compile time ifs are handled at instantiation time
            self.instantiations.alloc(Instantiation::Wire(WireInstance{typ : bool_typ, is_compiletime : false, span : additional_condition_wire.span, source : WireSource::BinaryOp{op: Operator{op_typ : kw("&")}, left : condition, right : additional_condition}}))
        } else {
            additional_condition
        }
    }
    fn flatten_declaration(&mut self, decl : &SignalDeclaration) -> FlatID {
        assert!(decl.identifier_type != IdentifierType::Input);
        assert!(decl.identifier_type != IdentifierType::Output);

        let parsed_typ_expr = self.map_to_type(&decl.typ.0, &self.module.link_info.global_references);
        let typ_span = decl.typ.1;

        let typ = if let Some(root_ref) = parsed_typ_expr.get_root() {
            match &self.linker.links.globals[root_ref] {
                Named::Module(md) => {
                    if let Type::Named(name) = parsed_typ_expr {
                        return self.alloc_module_interface(decl.name.clone(), md, name, typ_span)
                    } else {
                        todo!("Implement submodule arrays");
                        //Instantiation::Error
                    }
                }
                Named::Constant(c) => {
                    self.errors.error_basic(typ_span, format!("This should be the type of a declaration, but it refers to the constant '{}'", c.get_full_name()));
                    Type::Error
                }
                Named::Type(_) => {
                    parsed_typ_expr
                }
            }
        } else {
            // Error report handled by linker
            Type::Error
        };
        self.instantiations.alloc(Instantiation::WireDeclaration(WireDeclaration{
            typ,
            typ_span,
            read_only : false,
            identifier_type : decl.identifier_type,
            name : decl.name.clone(),
            name_token : Some(decl.name_token)
        }))
    }
    fn flatten_code(&mut self, code : &CodeBlock, condition : Option<FlatID>) {
        for (stmt, stmt_span) in &code.statements {
            match stmt {
                Statement::Declaration(decl_id) => {
                    let decl = &self.module.declarations[*decl_id];
                    let wire_id = self.flatten_declaration(decl);

                    self.decl_to_flat_map[*decl_id] = Some(wire_id);
                }
                Statement::If{condition : condition_expr, then, els} => {
                    let Some(if_statement_condition) = self.flatten_single_expr(condition_expr, condition) else {continue;};

                    let condition_is_const = self.instantiations[if_statement_condition].extract_wire().is_compiletime;

                    if condition_is_const {
                        println!("TODO generative if statements");
                    }

                    //let bool_typ = Type::Named(get_builtin_uuid("bool"));
                    //if self.typecheck(if_statement_condition, &bool_typ, "if statement condition") == None {continue;}
                    let then_condition = self.extend_condition(condition, if_statement_condition);
                    self.flatten_code(then, Some(then_condition));
                    if let Some(e) = els {
                        let else_condition_bool = self.instantiations.alloc(Instantiation::Wire(WireInstance{typ : Type::Unknown, is_compiletime : false/* Generative If */, span : condition_expr.1, source : WireSource::UnaryOp{op : Operator{op_typ : kw("!")}, right : if_statement_condition}}));
                        let else_condition = self.extend_condition(condition, else_condition_bool);
                        self.flatten_code(e, Some(else_condition));
                    }
                }
                Statement::Assign{to, expr : (Expression::FuncCall(func_and_args), func_span), eq_sign_position} => {
                    let Some((md, outputs)) = self.desugar_func_call(&func_and_args, func_span.1, condition) else {return;};

                    let func_name_span = func_and_args[0].1;
                    let num_func_outputs = outputs.len();
                    let num_targets = to.len();
                    if num_targets != num_func_outputs {
                        let info = vec![error_info(md.link_info.span, md.link_info.file, "Module Defined here")];
                        if num_targets > num_func_outputs {
                            let excess_results_span = Span(to[num_func_outputs].expr.1.0, to.last().unwrap().expr.1.1);
                            self.errors.error_with_info(excess_results_span, format!("Excess output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."), info);
                        } else {
                            let too_few_targets_pos = if let Some(eq) = eq_sign_position {Span::from(*eq)} else {func_name_span};
                            self.errors.error_with_info(too_few_targets_pos, format!("Too few output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."), info);
                        }
                    }

                    for (field, to_i) in zip(outputs, to) {
                        let Some(write_side) = self.flatten_assignable_expr(&to_i.expr, condition) else {return;};

                        // temporary
                        let module_port_wire_decl = self.instantiations[*field].extract_wire_declaration();
                        let module_port_proxy = self.instantiations.alloc(Instantiation::Wire(WireInstance{typ : module_port_wire_decl.typ.clone(), is_compiletime : module_port_wire_decl.identifier_type == IdentifierType::Generative, span : *func_span, source : WireSource::WireRead { from_wire: *field }}));
                        self.instantiations.alloc(Instantiation::Connection(Connection{num_regs : to_i.num_regs, from: module_port_proxy, to: write_side, condition}));
                    }
                },
                Statement::Assign{to, expr : non_func_expr, eq_sign_position : _} => {
                    if to.len() == 1 {
                        let Some(read_side) = self.flatten_single_expr(non_func_expr, condition) else {return;};
                        let t = &to[0];
                        let Some(write_side) = self.flatten_assignable_expr(&t.expr, condition) else {return;};
                        self.instantiations.alloc(Instantiation::Connection(Connection{num_regs : t.num_regs, from: read_side, to: write_side, condition}));
                    } else {
                        self.errors.error_basic(*stmt_span, format!("Non-function assignments must only output exactly 1 instead of {}", to.len()));
                    }
                },
                Statement::Block(inner_code) => {
                    self.flatten_code(inner_code, condition);
                },
                Statement::TimelineStage(_) => {/*TODO */}
            }
        }
    }
}

#[derive(Debug)]
pub struct FlattenedInterfacePort {
    pub wire_id : FlatID,
    pub typ : Type,
    pub port_name : Box<str>,
    pub span : Span
}

#[derive(Debug, Default)]
pub struct FlattenedInterface {
    pub interface_wires : Box<[FlattenedInterfacePort]>, // Ordered such that all inputs come first, then all outputs
    pub outputs_start : usize
}

impl FlattenedInterface {
    pub fn new() -> Self {
        FlattenedInterface { interface_wires: Box::new([]), outputs_start : 0 }
    }
    // Todo, just treat all inputs and outputs as function call interface
    pub fn func_call_syntax_interface(&self) -> (Range<FieldID>, Range<FieldID>) {
        (0..self.outputs_start, self.outputs_start..self.interface_wires.len())
    }
    pub fn inputs(&self) -> &[FlattenedInterfacePort] {
        &self.interface_wires[..self.outputs_start]
    }
    pub fn outputs(&self) -> &[FlattenedInterfacePort] {
        &self.interface_wires[self.outputs_start..]
    }
}

#[derive(Debug)]
pub struct FlattenedModule {
    pub instantiations : ListAllocator<Instantiation, FlatIDMarker>,
    pub errors : ErrorCollector,
    pub interface : FlattenedInterface
}

impl FlattenedModule {
    pub fn empty(file : FileUUID) -> FlattenedModule {
        FlattenedModule {
            instantiations : ListAllocator::new(),
            errors : ErrorCollector::new(file),
            interface : FlattenedInterface::new()
        }
    }
    /* 
    Required to do this first, this only initializes the interfaces, so that proper flattening can typecheck
    Produces an initial FlattenedModule, in which the interface types have already been resolved. 
    Must be further processed by flatten, but this requires all modules to have been Initial Flattened for dependency resolution
    */
    /*
    This method flattens all given code into a simple set of assignments, operators and submodules. 
    It already does basic type checking and assigns a type to every wire. 
    The Generating Structure of the code is not yet executed. 
    It is template-preserving
    */
    pub fn initialize(linker : &Linker, module : &Module, starts_with_errors : bool) -> FlattenedModule {
        let instantiations = ListAllocator::new();
        let errors = ErrorCollector::new(module.link_info.file);
        errors.did_error.set(starts_with_errors);

        let mut context = FlatteningContext{
            decl_to_flat_map: module.declarations.iter().map(|_| None).collect(),
            instantiations: &instantiations,
            errors: &errors,
            linker,
            module,
        };

        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        for (decl_id, decl) in &module.declarations {
            let is_input = match decl.identifier_type {
                IdentifierType::Input => true,
                IdentifierType::Output => false,
                IdentifierType::Local | IdentifierType::State | IdentifierType::Generative => continue,
                IdentifierType::Virtual => unreachable!()
            };
            
            let typ = context.map_to_type(&decl.typ.0, &module.link_info.global_references);
            let wire_id = context.instantiations.alloc(Instantiation::WireDeclaration(WireDeclaration{
                typ : typ.clone(),
                typ_span : decl.typ.1,
                read_only: is_input,
                identifier_type : decl.identifier_type,
                name : decl.name.clone(),
                name_token : Some(decl.name_token)
            }));

            let port = FlattenedInterfacePort { wire_id, typ, port_name: decl.name.clone(), span: decl.span };
            if is_input {
                inputs.push(port);
            } else {
                outputs.push(port);
            }
            
            context.decl_to_flat_map[decl_id] = Some(wire_id);
        }

        let outputs_start = inputs.len();
        inputs.reserve(outputs.len());
        inputs.append(&mut outputs);
        let interface = FlattenedInterface{interface_wires: inputs.into_boxed_slice(), outputs_start};
        
        context.flatten_code(&module.code, None);

        let flat_mod = FlattenedModule {
            instantiations,
            errors,
            interface
        };
        flat_mod
    }

    /* Type Checking */
    fn typecheck_wire_is_of_type(&self, wire : &WireInstance, expected : &Type, context : &str, linker : &Linker) {
        typecheck(&wire.typ, wire.span, expected, context, linker, &self.errors);
    }

    pub fn typecheck(&mut self, linker : &Linker) {
        let look_at_queue : Vec<FlatID> = self.instantiations.iter().map(|(id,_)| id).collect();

        for elem_id in look_at_queue {
            match &self.instantiations[elem_id] {
                Instantiation::SubModule(_) => {}
                Instantiation::WireDeclaration(_) => {},
                Instantiation::Wire(w) => {
                    let result_typ = match &w.source {
                        &WireSource::WireRead{from_wire} => {
                            self.instantiations[from_wire].extract_wire_declaration().typ.clone()
                        }
                        &WireSource::UnaryOp{op, right} => {
                            let right_wire = self.instantiations[right].extract_wire();
                            typecheck_unary_operator(op, &right_wire.typ, right_wire.span, linker, &self.errors)
                        }
                        &WireSource::BinaryOp{op, left, right} => {
                            let left_wire = self.instantiations[left].extract_wire();
                            let right_wire = self.instantiations[right].extract_wire();
                            let ((input_left_type, input_right_type), output_type) = get_binary_operator_types(op);
                            self.typecheck_wire_is_of_type(left_wire, &input_left_type, &format!("{op} left"), linker);
                            self.typecheck_wire_is_of_type(right_wire, &input_right_type, &format!("{op} right"), linker);
                            output_type
                        }
                        &WireSource::ArrayAccess{arr, arr_idx} => {
                            let arr_wire = self.instantiations[arr].extract_wire();
                            let arr_idx_wire = self.instantiations[arr_idx].extract_wire();
                
                            self.typecheck_wire_is_of_type(arr_idx_wire, &Type::Named(get_builtin_uuid("int")), "array index", linker);
                            if let Some(typ) = typecheck_is_array_indexer(&arr_wire.typ, arr_wire.span, linker, &self.errors) {
                                typ.clone()
                            } else {
                                Type::Error
                            }
                        }
                        WireSource::Constant{value} => {
                            value.get_type_of_constant()
                        }
                    };
                    let Instantiation::Wire(w) = &mut self.instantiations[elem_id] else {unreachable!()};
                    w.typ = result_typ;
                }
                Instantiation::Connection(conn) => {

                    // Typecheck digging down into write side
                    let conn_root = self.instantiations[conn.to.root].extract_wire_declaration();
                    let mut write_to_type = Some(&conn_root.typ);
                    for p in &conn.to.path {
                        match p {
                            &ConnectionWritePathElement::ArrayIdx{idx, idx_span} => {
                                let idx_wire = self.instantiations[idx].extract_wire();
                                self.typecheck_wire_is_of_type(idx_wire, &Type::Named(get_builtin_uuid("int")), "array index", linker);
                                if let Some(wr) = write_to_type {
                                    write_to_type = typecheck_is_array_indexer(wr, idx_span, linker, &self.errors);
                                }
                            }
                        }
                    }

                    // Typecheck compile-time ness
                    let from_wire = self.instantiations[conn.from].extract_wire();
                    if conn_root.identifier_type == IdentifierType::Generative && !from_wire.is_compiletime {
                        let decl_info = error_info(conn_root.get_full_decl_span(), self.errors.file, "Declared here");
                        self.errors.error_with_info(from_wire.span, "Assignments to compile-time variables must themselves be known at compile time", vec![decl_info]);
                    }

                    // Typecheck the value with target type
                    if let Some(target_type) = write_to_type {
                        self.typecheck_wire_is_of_type(from_wire, &target_type, "connection", linker);
                    }

                    // Typecheck condition is bool
                    if let Some(condition) = conn.condition {
                        let condition_wire = self.instantiations[condition].extract_wire();
                        self.typecheck_wire_is_of_type(condition_wire, &Type::Named(get_builtin_uuid("bool")), "assignment condition", linker);
                    }
                }
            }
        }

        // Post type application. Flag any remaining Type::Unknown
        for (_id, inst) in &self.instantiations {
            inst.for_each_embedded_type(&mut |typ, span| {
                if typ.contains_error_or_unknown::<false, true>() {
                    self.errors.error_basic(span, format!("Unresolved Type: {}", typ.to_string(linker)))
                }
            });
        }
    }

    /* Additional Warnings */
    pub fn find_unused_variables(&self) {
        // Setup Wire Fanouts List for faster processing
        let mut gathered_connection_fanin : FlatAlloc<Vec<FlatID>, FlatIDMarker> = self.instantiations.iter().map(|_| Vec::new()).collect();

        for (inst_id, inst) in &self.instantiations {
            match inst {
                Instantiation::Connection(conn) => {
                    gathered_connection_fanin[conn.to.root].push(conn.from);
                    if let Some(cond) = conn.condition {
                        gathered_connection_fanin[conn.to.root].push(cond);
                    }
                }
                Instantiation::SubModule(sm) => {
                    for w in sm.outputs() {
                        gathered_connection_fanin[*w].push(inst_id);
                    }
                }
                Instantiation::WireDeclaration(_) => {} // Handle these outside
                Instantiation::Wire(_) => {}
            }
        }

        let mut is_instance_used_map : FlatAlloc<bool, FlatIDMarker> = self.instantiations.iter().map(|_| false).collect();

        let mut wire_to_explore_queue : Vec<FlatID> = Vec::new();

        for port in self.interface.outputs() {
            is_instance_used_map[port.wire_id] = true;
            wire_to_explore_queue.push(port.wire_id);
        }

        while let Some(item) = wire_to_explore_queue.pop() {
            let mut mark_not_unused = |from| {
                if !is_instance_used_map[from] {
                    is_instance_used_map[from] = true;
                    wire_to_explore_queue.push(from);
                }
            };
            match &self.instantiations[item] {
                Instantiation::WireDeclaration(decl) => {
                    decl.typ.for_each_generative_input(&mut mark_not_unused);
                }
                Instantiation::Wire(wire) => {
                    wire.source.for_each_input_wire(&mut mark_not_unused);
                }
                Instantiation::SubModule(submodule) => {
                    for port in submodule.inputs() {
                        mark_not_unused(*port);
                    }
                }
                Instantiation::Connection(_) => {unreachable!()}
            }
            for from in &gathered_connection_fanin[item] {
                mark_not_unused(*from);
            }
        }

        // Now produce warnings from the unused list
        for (id, inst) in &self.instantiations {
            if !is_instance_used_map[id] {
                if let Instantiation::WireDeclaration(WireDeclaration{typ : _, typ_span : _, read_only : _, identifier_type : _, name : _, name_token : Some(name_token)}) = inst {
                    self.errors.warn_basic(Span::from(*name_token), "Unused Variable: This variable does not affect the output ports of this module");
                }
            }
        }
    }
}
