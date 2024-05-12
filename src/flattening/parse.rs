

use std::{iter::zip, ops::{Deref, DerefMut}, str::FromStr};

use num::BigInt;
use sus_proc_macro::{field, kind, kw};
use crate::{
    arena_alloc::{UUIDRange, UUID}, debug::SpanDebugger, errors::ErrorCollector, file_position::{BracketSpan, Span}, linker::{with_module_editing_context, ConstantUUIDMarker, Linker, ModuleUUID, ModuleUUIDMarker, NameElem, NameResolver, NamedConstant, NamedType, ResolvedName, Resolver, TypeUUIDMarker, WorkingOnResolver}, parser::Cursor, typing::{AbstractType, WrittenType}, value::Value
};

use super::{initialization::ModulePorts, name_context::LocalVariableContext};
use super::*;

enum LocalOrGlobal<'l> {
    Local(FlatID),
    Global(ResolvedName<'l>)
}

impl<'l> LocalOrGlobal<'l> {
    fn expect_local(self, context : &str) -> Option<FlatID> {
        match self {
            LocalOrGlobal::Local(local) => Some(local),
            LocalOrGlobal::Global(global) => {
                global.errors.error(global.span, format!("Can only use local variables in {context}!"));
                None
            }
        }
    }
}



struct FlatteningContext<'l, 'errs> {
    modules : WorkingOnResolver<'l, 'errs, ModuleUUIDMarker, Module>,
    #[allow(dead_code)]
    types : Resolver<'l, 'errs, TypeUUIDMarker, NamedType>,
    #[allow(dead_code)]
    constants : Resolver<'l, 'errs, ConstantUUIDMarker, NamedConstant>,
    name_resolver : NameResolver<'l, 'errs>,
    errors : &'errs ErrorCollector<'l>,

    ports_to_visit : UUIDRange<PortIDMarker>,

    local_variable_context : LocalVariableContext<'l, FlatID>
}

impl<'l, 'errs> Deref for FlatteningContext<'l, 'errs> {
    type Target = WorkingOnResolver<'l, 'errs, ModuleUUIDMarker, Module>;

    fn deref(&self) -> &Self::Target {
        &self.modules
    }
}
impl<'l, 'errs> DerefMut for FlatteningContext<'l, 'errs> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.modules
    }
}

impl<'l, 'errs> FlatteningContext<'l, 'errs> {
    fn resolve_identifier(&self, identifier_span : Span) -> LocalOrGlobal {
        // Possibly local
        let name_text = &self.name_resolver.file_text[identifier_span];
        if let Some(decl_id) = self.local_variable_context.get_declaration_for(name_text) {
            return LocalOrGlobal::Local(decl_id);
        }
        // Global identifier
        LocalOrGlobal::Global(self.name_resolver.resolve_global(identifier_span))
    }

    fn flatten_array_type(&mut self, span : Span, cursor : &mut Cursor) -> WrittenType {
        cursor.go_down(kind!("array_type"), |cursor| {
            cursor.field(field!("arr"));
            let array_element_type = self.flatten_type(cursor);

            cursor.field(field!("arr_idx"));
            let (array_size_wire_id, bracket_span) = self.flatten_array_bracket(cursor);
            
            WrittenType::Array(span, Box::new((array_element_type, array_size_wire_id, bracket_span)))
        })
    }
    
    fn flatten_type(&mut self, cursor : &mut Cursor) -> WrittenType {
        let (kind, span) = cursor.kind_span();
        if kind == kind!("global_identifier") {
            if let Some(typ_id) = &self.name_resolver.resolve_global(span).expect_type() {
                WrittenType::Named(span, *typ_id)
            } else {
                WrittenType::Error(span)
            }
        } else if kind == kind!("array_type") {
            self.flatten_array_type(span, cursor)
        } else {cursor.could_not_match()}
    }

    fn flatten_module_or_type<const ALLOW_MODULES : bool>(&mut self, cursor : &mut Cursor) -> ModuleOrWrittenType {
        let (kind, span) = cursor.kind_span();
        // Only difference is that 
        if kind == kind!("global_identifier") {
            let found_global = self.name_resolver.resolve_global(span);
            match &found_global.name_elem {
                Some(NameElem::Type(typ_id)) => ModuleOrWrittenType::WrittenType(WrittenType::Named(span, *typ_id)),
                Some(NameElem::Module(md)) if ALLOW_MODULES => ModuleOrWrittenType::Module(span, *md),
                Some(_) => {
                    let accepted_text = if ALLOW_MODULES {"Type or Module"} else {"Type"};
                    found_global.not_expected_global_error(accepted_text);
                    ModuleOrWrittenType::WrittenType(WrittenType::Error(span))
                }
                None => ModuleOrWrittenType::WrittenType(WrittenType::Error(span)) // Non existent global already covered by Linker
            }
        } else if kind == kind!("array_type") {
            ModuleOrWrittenType::WrittenType(self.flatten_array_type(span, cursor))
        } else {cursor.could_not_match()}
    }

    fn flatten_declaration<const ALLOW_MODULES : bool, const ALLOW_MODIFIERS : bool>(&mut self, fallback_identifier_type : IdentifierType, read_only : bool, declaration_itself_is_not_written_to : bool, cursor : &mut Cursor) -> FlatID {
        cursor.go_down(kind!("declaration"), |cursor| {
            let identifier_type = if cursor.optional_field(field!("declaration_modifiers")) {
                let (modifier_kind, modifier_span) = cursor.kind_span();

                if !ALLOW_MODIFIERS {
                    self.errors.error(modifier_span, "Inputs and outputs of a module cannot be decorated with 'state' or 'gen'");
                    fallback_identifier_type
                } else {
                    if modifier_kind == kw!("state") {
                        IdentifierType::State
                    } else if modifier_kind == kw!("gen") {
                        IdentifierType::Generative
                    } else {
                        cursor.could_not_match()
                    }
                }
            } else {fallback_identifier_type};
            
            cursor.field(field!("type"));
            let typ_or_module_expr = self.flatten_module_or_type::<ALLOW_MODULES>(cursor);
            
            let name_span = cursor.field_span(field!("name"), kind!("identifier"));
    
            let span_latency_specifier = if cursor.optional_field(field!("latency_specifier")) {
                cursor.go_down_content(kind!("latency_specifier"), 
                    |cursor| Some((self.flatten_expr(cursor), cursor.span()))
            )} else {None};
            // Parsing components done

            let typ_expr = match typ_or_module_expr {
                ModuleOrWrittenType::WrittenType(typ) => {
                    typ
                }
                ModuleOrWrittenType::Module(span, module_uuid) => {
                    assert!(ALLOW_MODULES);
                    if let Some((_, span)) = span_latency_specifier {
                        self.errors.error(span, "Cannot add latency specifier to module instances");
                    }
                    let name = self.name_resolver.file_text[name_span].to_owned();
                    return self.working_on.instructions.alloc(Instruction::SubModule(SubModuleInstance{
                        name,
                        module_uuid,
                        module_name_span: span
                    }))
                }
            };

            let typ_expr_span = typ_expr.get_span();
            let name = &self.name_resolver.file_text[name_span];
            let documentation = cursor.extract_gathered_comments();

            let inst_id = self.working_on.instructions.alloc(Instruction::Declaration(Declaration{
                typ : typ_expr.to_type(),
                typ_expr,
                read_only,
                declaration_itself_is_not_written_to,
                identifier_type,
                name : name.to_owned(),
                name_span,
                latency_specifier : span_latency_specifier.map(|(ls, _)| ls),
                documentation
            }));

            if let Err(conflict) = self.local_variable_context.add_declaration(name, inst_id) {
                self.errors
                    .error(Span::new_overarching(typ_expr_span, name_span), "This declaration conflicts with a previous declaration in the same scope")
                    .info_obj_same_file(self.working_on.instructions[conflict].unwrap_wire_declaration());
            }

            inst_id
        })
    }

    fn flatten_array_bracket(&mut self, cursor : &mut Cursor) -> (FlatID, BracketSpan) {
        cursor.go_down_content(kind!("array_bracket_expression"), 
            |cursor| (self.flatten_expr(cursor), BracketSpan::from_outer(cursor.span()))
        )
    }

    fn desugar_func_call(&mut self, cursor : &mut Cursor) -> Option<(ModuleUUID, FlatID, PortIDRange)> {
        let whole_function_span = cursor.span();
        cursor.go_down(kind!("func_call"), |cursor| {
            cursor.field(field!("name"));
            let instantiation_flat_id = self.get_or_alloc_module_by_global_identifier(cursor);

            cursor.field(field!("arguments"));
            let arguments_span = BracketSpan::from_outer(cursor.span());
            let arguments = cursor.collect_list(kind!("parenthesis_expression_list"), |cursor| {
                self.flatten_expr(cursor)
            });

            let instantiation_flat_id = instantiation_flat_id?;
            let func_instantiation = self.working_on.instructions[instantiation_flat_id].unwrap_submodule();

            
            let module_uuid = func_instantiation.module_uuid;
            let md = &self.modules[module_uuid];

            let inputs = md.module_ports.interfaces[ModulePorts::MAIN_INTERFACE_ID].func_call_inputs;
            let outputs = md.module_ports.interfaces[ModulePorts::MAIN_INTERFACE_ID].func_call_outputs;

            let arg_count = arguments.len();
            let expected_arg_count = inputs.len();

            let mut args = arguments.as_slice();
            
            if arg_count != expected_arg_count {
                if arg_count > expected_arg_count {
                    // Too many args, complain about excess args at the end
                    let excess_args_span = Span::new_overarching(self.working_on.instructions[args[expected_arg_count]].unwrap_wire().span, self.working_on.instructions[*args.last().unwrap()].unwrap_wire().span);
                    
                    self.errors
                        .error(excess_args_span, format!("Excess argument. Function takes {expected_arg_count} args, but {arg_count} were passed."))
                        .info_obj(&md.link_info);
                    // Shorten args to still get proper type checking for smaller arg array
                    args = &args[..expected_arg_count];
                } else {
                    // Too few args, mention missing argument names
                    self.errors
                        .error(arguments_span.close_bracket(), format!("Too few arguments. Function takes {expected_arg_count} args, but {arg_count} were passed."))
                        .info_obj(&md.link_info);
                }
            }

            for (port_id, arg_read_side) in zip(inputs, args) {
                let arg_wire = self.working_on.instructions[*arg_read_side].unwrap_wire();
                let arg_wire_span = arg_wire.span;
                let root = WireReferenceRoot::SubModulePort(PortInfo{
                    submodule_flat : instantiation_flat_id,
                    port : port_id,
                    port_name_span : None, // Not present in function call notation
                    port_identifier_typ : IdentifierType::Input
                });
                self.working_on.instructions.alloc(Instruction::Write(Write{
                    from: *arg_read_side,
                    to: WireReference{
                        root,
                        root_span : whole_function_span,
                        path : Vec::new(),
                        span : arg_wire_span,
                    },
                    write_modifiers : WriteModifiers::Connection{num_regs : 0, regs_span : arg_wire_span.empty_span_at_front()}
                }));
            }

            Some((module_uuid, instantiation_flat_id, outputs))
        })
    }

    /// Produces a new [SubModuleInstance] if a global was passed, or a reference to the existing instance if it's referenced by name
    fn get_or_alloc_module_by_global_identifier(&mut self, cursor : &mut Cursor) -> Option<FlatID> {
        let (kind, span) = cursor.kind_span();
        if kind == kind!("global_identifier") {
            cursor.go_down(kind!("global_identifier"), |_cursor| {
                match self.resolve_identifier(span) {
                    LocalOrGlobal::Local(id) => {
                        if let Instruction::SubModule(_) = &self.working_on.instructions[id] {
                            Some(id)
                        } else {
                            let decl = self.working_on.instructions[id].unwrap_wire_declaration();
                            self.errors
                                .error(span, "Function call syntax is only possible on modules")
                                .info_obj_same_file(decl);
                            None
                        }
                    }
                    LocalOrGlobal::Global(global) => {
                        if let Some(module_uuid) = global.expect_module() {
                            let md = &self.modules[module_uuid];
                            let name = md.link_info.name.clone();
                            Some(self.working_on.instructions.alloc(Instruction::SubModule(SubModuleInstance{
                                name,
                                module_uuid,
                                module_name_span: span
                            })))
                        } else {
                            None
                        }
                    }
                }
            })
        } else {
            self.errors.error(span, "Module name may not be an expression");
            None
        }
    }

    fn get_port(&self, submodule_decl : FlatID, cursor : &mut Cursor) -> Option<PortInfo> {
        let submodule = self.working_on.instructions[submodule_decl].unwrap_submodule();

        let port_name_span = cursor.field_span(field!("name"), kind!("identifier"));
        let name = &self.name_resolver.file_text[port_name_span];
        let submod = &self.modules[submodule.module_uuid];

        let port = submod.module_ports.get_port_by_name(name)?;
        Some(PortInfo { submodule_flat: submodule_decl, port, port_name_span : Some(port_name_span), port_identifier_typ: submod.module_ports.ports[port].id_typ })
    }

    fn flatten_expr(&mut self, cursor : &mut Cursor) -> FlatID {
        let (kind, expr_span) = cursor.kind_span();
        
        let source = if kind == kind!("number") {
            let text = &self.name_resolver.file_text[expr_span];
            WireSource::Constant(Value::Integer(BigInt::from_str(text).unwrap()))
        } else if kind == kind!("unary_op") {
            cursor.go_down_no_check(|cursor| {
                cursor.field(field!("operator"));
                let op = UnaryOperator::from_kind_id(cursor.kind());
                
                cursor.field(field!("right"));
                let right = self.flatten_expr(cursor);

                WireSource::UnaryOp{op, right}
            })
        } else if kind == kind!("binary_op") {
            cursor.go_down_no_check(|cursor| {
                cursor.field(field!("left"));
                let left = self.flatten_expr(cursor);

                cursor.field(field!("operator"));
                let op = BinaryOperator::from_kind_id(cursor.kind());

                cursor.field(field!("right"));
                let right = self.flatten_expr(cursor);

                WireSource::BinaryOp{op, left, right}
            })
        } else if kind == kind!("func_call") {
            if let Some((md_id, submodule, outputs)) = self.desugar_func_call(cursor) {
                if outputs.len() != 1 {
                    let md = &self.modules[md_id];
                    self.errors
                        .error(expr_span, "A function called in this context may only return one result. Split this function call into a separate line instead.")
                        .info_obj(&md.link_info);
                }

                if outputs.len() >= 1 {
                    WireSource::WireRead(WireReference::simple_port(expr_span, PortInfo{
                        submodule_flat: submodule,
                        port: outputs.0,
                        port_name_span: None,
                        port_identifier_typ: IdentifierType::Output,
                    }))
                } else {
                    // Function desugaring or using threw an error
                    WireSource::Constant(Value::Error)
                }
            } else {
                // Function desugaring or using threw an error
                WireSource::Constant(Value::Error)
            }
        } else if kind == kind!("parenthesis_expression") {
            return cursor.go_down_content(kind!("parenthesis_expression"), |cursor| self.flatten_expr(cursor));
        } else {
            if let Some(wr) = self.flatten_wire_reference(cursor) {
                WireSource::WireRead(wr)
            } else {
                WireSource::Constant(Value::Error)
            }
        };

        let wire_instance = WireInstance{
            typ : AbstractType::Unknown,
            is_compiletime : IS_GEN_UNINIT,
            span: expr_span,
            source
        };
        self.working_on.instructions.alloc(Instruction::Wire(wire_instance))
    }

    fn flatten_wire_reference(&mut self, cursor : &mut Cursor) -> Option<WireReference> {
        let (kind, expr_span) = cursor.kind_span();
        if kind == kind!("global_identifier") {
            // TODO add namespacing
            let root = match self.resolve_identifier(expr_span) {
                LocalOrGlobal::Local(id) => {
                    WireReferenceRoot::LocalDecl(id)
                }
                LocalOrGlobal::Global(global) => {
                    let cst = global.expect_constant()?;

                    WireReferenceRoot::NamedConstant(cst)
                }
            };

            Some(WireReference{root, root_span : expr_span, path : Vec::new(), span: expr_span})
        } else if kind == kind!("array_op") {
            cursor.go_down_no_check(|cursor| {
                cursor.field(field!("arr"));
                let flattened_arr_expr_opt = self.flatten_wire_reference(cursor);
                
                cursor.field(field!("arr_idx"));
                let (idx, bracket_span) = self.flatten_array_bracket(cursor);
                
                let mut flattened_arr_expr = flattened_arr_expr_opt?; // only unpack the subexpr after flattening the idx, so we catch all errors
                
                flattened_arr_expr.path.push(WireReferencePathElement::ArrayIdx{idx, bracket_span});
                flattened_arr_expr.span = Span::new_overarching(flattened_arr_expr.span, bracket_span.outer_span());
                
                Some(flattened_arr_expr)
            })
        } else if kind == kind!("field_access") {
            cursor.go_down_no_check(|cursor| {
                cursor.field(field!("left"));
                let flattened_arr_expr_opt = self.flatten_wire_reference(cursor);
                
                cursor.field(field!("name"));
                //let (idx, bracket_span) = self.flatten_array_bracket(cursor);
                
                //let mut flattened_arr_expr = flattened_arr_expr_opt?; // only unpack the subexpr after flattening the idx, so we catch all errors
                
                println!("TODO: Field access in assign");

                /*else if kind == kind!("field_access") {
                    cursor.go_down_no_check(|cursor| {
                        /*cursor.field(field!("left"));
                        
                        let (left_kind, left_span) = cursor.kind_span();
                        if left_kind == kind!("global_identifier") {


                            let isntr_id = self.resolve_identifier(identifier_span).expect_local("submodule")?;
                            let instr_id = self.get_or_alloc_module_by_global_identifier(cursor)?;
            
                    
                            if let Some(port) = self.get_port(cursor) {
                                WireSource::PortRead(port)
                            } else {
                            }
                        } else {
                            WireSource::Constant(Value::Error)
                        }*/

                        println!("TODO: Field access");
                        WireSource::Constant(Value::Error)
                    })
                } */

                return None
            })
        } else if kind == kind!("number") {self.errors.error(expr_span, "A constant is not a wire reference"); None
        } else if kind == kind!("unary_op") {self.errors.error(expr_span, "The result of an operator is not a wire reference"); None
        } else if kind == kind!("binary_op") {self.errors.error(expr_span, "The result of an operator is not a wire reference"); None
        } else if kind == kind!("func_call") {self.errors.error(expr_span, "A submodule call is not a wire reference"); None
        } else if kind == kind!("parenthesis_expression") {self.errors.error(expr_span, "Remove these parentheses"); None
        } else {cursor.could_not_match()}
    }

    fn flatten_if_statement(&mut self, cursor : &mut Cursor) {
        cursor.go_down(kind!("if_statement"), |cursor| {
            cursor.field(field!("condition"));
            let condition = self.flatten_expr(cursor);
            
            let if_id = self.working_on.instructions.alloc(Instruction::IfStatement(IfStatement{condition, then_start : UUID::PLACEHOLDER, then_end_else_start : UUID::PLACEHOLDER, else_end : UUID::PLACEHOLDER}));
            let then_start = self.working_on.instructions.get_next_alloc_id();
            
            cursor.field(field!("then_block"));
            self.flatten_code(cursor);

            let then_end_else_start = self.working_on.instructions.get_next_alloc_id();
            if cursor.optional_field(field!("else_block")) {
                if cursor.kind() == kind!("if_statement") {
                    self.flatten_if_statement(cursor); // Chained if statements
                } else {
                    self.flatten_code(cursor)
                }
            };
            let else_end = self.working_on.instructions.get_next_alloc_id();
            
            let Instruction::IfStatement(if_stmt) = &mut self.working_on.instructions[if_id] else {unreachable!()};
            if_stmt.then_start = then_start;
            if_stmt.then_end_else_start = then_end_else_start;
            if_stmt.else_end = else_end;
        })
    }

    fn flatten_assign_function_call(&mut self, to : Vec<Result<(WireReference, WriteModifiers), Span>>, cursor : &mut Cursor) {
        let func_call_span = cursor.span();
        let to_iter = if let Some((md_id, submodule, outputs)) = self.desugar_func_call(cursor) {

            fn get_span(v : &Result<(WireReference, WriteModifiers), Span>) -> Span {
                match v {
                    Ok(wr) => wr.0.span,
                    Err(span) => *span,
                }.debug()
            }

            let num_func_outputs = outputs.len();
            let num_targets = to.len();
            if num_targets != num_func_outputs {
                let md = &self.modules[md_id];
                if num_targets > num_func_outputs {
                    let excess_results_span = Span::new_overarching(get_span(&to[num_func_outputs]), get_span(to.last().unwrap()));
                    self.errors
                        .error(excess_results_span, format!("Excess output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."))
                        .info_obj(&md.link_info);
                } else {
                    self.errors
                        .error(func_call_span, format!("Too few output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."))
                        .info_obj(&md.link_info);
                }
            }

            let mut to_iter = to.into_iter();
            for port in outputs {
                if let Some(Ok((to, write_modifiers))) = to_iter.next() {
                    let from = self.working_on.instructions.alloc(Instruction::Wire(WireInstance{typ: AbstractType::Unknown, is_compiletime: false, span: func_call_span, source: WireSource::WireRead(WireReference::simple_port(to.span, PortInfo{
                        submodule_flat: submodule,
                        port,
                        port_name_span: None,
                        port_identifier_typ: IdentifierType::Output,
                    }))}));
                    self.working_on.instructions.alloc(Instruction::Write(Write{from, to, write_modifiers}));
                }
            }
            to_iter
        } else {
            to.into_iter()
        };
        for leftover_to in to_iter {
            if let Ok((to, write_modifiers)) = leftover_to {
                let err_id = self.working_on.instructions.alloc(Instruction::Wire(WireInstance{typ : AbstractType::Error, is_compiletime : true, span : func_call_span, source : WireSource::Constant(Value::Error)}));
                self.working_on.instructions.alloc(Instruction::Write(Write{from: err_id, to, write_modifiers}));
            }
        }
    }

    fn flatten_code(&mut self, cursor : &mut Cursor) {
        let old_frame = self.local_variable_context.new_frame();
        
        self.flatten_code_keep_context(cursor);

        self.local_variable_context.pop_frame(old_frame);
    }
    fn flatten_code_keep_context(&mut self, cursor : &mut Cursor) {
        cursor.clear_gathered_comments(); // Clear comments at the start of a block
        cursor.list(kind!("block"), |cursor| {
            let kind = cursor.kind();
            if kind == kind!("assign_left_side") {
                self.flatten_standalone_decls(cursor);
            } else if kind == kind!("decl_assign_statement") {
                cursor.go_down_no_check(|cursor| {
                    cursor.field(field!("assign_left"));
                    let to = self.flatten_assignment_left_side(cursor);
                    
                    cursor.field(field!("assign_value"));

                    let (node_kind, span) = cursor.kind_span();
                    
                    if node_kind == kind!("func_call") {
                        self.flatten_assign_function_call(to, cursor);
                    } else {
                        let read_side = self.flatten_expr(cursor);
                        
                        if to.len() != 1 {
                            self.errors.error(span, format!("Non-function assignments must output exactly 1 output instead of {}", to.len()));
                        }
                        if let Some(Ok((to, write_modifiers))) = to.into_iter().next() {
                            self.working_on.instructions.alloc(Instruction::Write(Write{from: read_side, to, write_modifiers}));
                        }
                    }
                });
            } else if kind == kind!("block") {
                self.flatten_code(cursor);
            } else if kind == kind!("if_statement") {
                self.flatten_if_statement(cursor);
            } else if kind == kind!("for_statement") {
                cursor.go_down_no_check(|cursor| {
                    cursor.field(field!("for_decl"));
                    let loop_var_decl = self.flatten_declaration::<false, false>(IdentifierType::Generative, true, true, cursor);

                    cursor.field(field!("from"));
                    let start = self.flatten_expr(cursor);

                    cursor.field(field!("to"));
                    let end = self.flatten_expr(cursor);
                    
                    let for_id = self.working_on.instructions.alloc(Instruction::ForStatement(ForStatement{loop_var_decl, start, end, loop_body: UUIDRange(UUID::PLACEHOLDER, UUID::PLACEHOLDER)}));

                    let code_start = self.working_on.instructions.get_next_alloc_id();

                    cursor.field(field!("block"));
                    self.flatten_code(cursor);
                    
                    let code_end = self.working_on.instructions.get_next_alloc_id();

                    let Instruction::ForStatement(for_stmt) = &mut self.working_on.instructions[for_id] else {unreachable!()};

                    for_stmt.loop_body = UUIDRange(code_start, code_end);
                })
            } else if kind == kind!("interface_statement") {
                println!("TODO: Interface Statement");
            } else if kind == kind!("cross_statement") {
                println!("TODO: Cross Statement");
            } else {
                cursor.could_not_match()
            }
            cursor.clear_gathered_comments(); // Clear comments after every statement, so comments don't bleed over
        });
    }

    fn flatten_write_modifiers(&self, cursor : &mut Cursor) -> WriteModifiers {
        if cursor.optional_field(field!("write_modifiers")) {
            let modifiers_span = cursor.span();
            let mut initial_count = 0;
            let mut reg_count = 0;
            cursor.list(kind!("write_modifiers"), |cursor| {
                let kw_kind = cursor.kind();
                if kw_kind == kw!("reg") {
                    reg_count += 1;
                } else if kw_kind == kw!("initial") {
                    initial_count += 1;
                } else {
                    unreachable!()
                }
            });
            match (initial_count, reg_count) {
                (0, num_regs) => WriteModifiers::Connection{num_regs, regs_span : modifiers_span},
                (1, 0) => WriteModifiers::Initial{initial_kw_span : modifiers_span},
                _other => unreachable!()
            }
        } else {
            WriteModifiers::Connection { num_regs: 0, regs_span: cursor.span().empty_span_at_front()}
        }
    }

    /// See [Self::flatten_standalone_decls][]
    /// Two cases:
    /// - Left side of assignment:
    ///     No modules, Yes write modifiers, Only assignable expressions
    fn flatten_assignment_left_side(&mut self, cursor : &mut Cursor) -> Vec<Result<(WireReference, WriteModifiers), Span>> {
        cursor.collect_list(kind!("assign_left_side"), |cursor| {
            cursor.go_down(kind!("assign_to"), |cursor| {
                let write_modifiers = self.flatten_write_modifiers(cursor);
                
                cursor.field(field!("expr_or_decl"));
                let (kind, span) = cursor.kind_span();

                if kind == kind!("declaration") {
                    let root = self.flatten_declaration::<false, true>(IdentifierType::Local, false, true, cursor);
                    let flat_root_decl = self.working_on.instructions[root].unwrap_wire_declaration();
                    Ok((WireReference{root : WireReferenceRoot::LocalDecl(root), root_span : flat_root_decl.name_span, path: Vec::new(), span}, write_modifiers))
                } else { // It's _expression
                    if let Some(wire_ref) = self.flatten_wire_reference(cursor) {
                        Ok((wire_ref, write_modifiers))
                    } else {
                        Err(span)
                    }
                }
            })
        })
    }

    /// See [Self::flatten_assignment_left_side][]
    /// - Standalone declarations:
    ///     Yes modules, No write modifiers, Yes expressions (-> single expressions)
    fn flatten_standalone_decls(&mut self, cursor : &mut Cursor) {
        let mut is_first_item = true;
        cursor.list(kind!("assign_left_side"), |cursor| {
            cursor.go_down(kind!("assign_to"), |cursor| {
                if !is_first_item {
                    self.errors.warn(cursor.span(), "Standalone declarations and expressions should be on their own line.");
                }
                is_first_item = false;

                if let Some(span) = cursor.optional_field_span(field!("write_modifiers"), kind!("write_modifiers")) {
                    self.errors.error(span, "No write modifiers are allowed on non-assigned to declarations or expressions");
                }
                
                cursor.field(field!("expr_or_decl"));
                let (kind, span) = cursor.kind_span();

                if kind == kind!("declaration") {
                    let _ = self.flatten_declaration::<true, true>(IdentifierType::Local, false, true, cursor);
                } else { // It's _expression
                    if kind == kind!("func_call") {
                        self.flatten_assign_function_call(Vec::new(), cursor);
                    } else {
                        self.errors.warn(span, "The result of this operation is not used");
                        let _ = self.flatten_expr(cursor);
                    }
                }
            });
        })
    }

    fn flatten_declaration_list(&mut self, identifier_type : IdentifierType, read_only : bool, cursor : &mut Cursor) {
        cursor.list(kind!("declaration_list"), |cursor| {
            let found_decl_span = cursor.span();

            let id = self.flatten_declaration::<false, false>(identifier_type, read_only, true, cursor);
            let this_port_id = self.ports_to_visit.next().unwrap();
            let port = &mut self.working_on.module_ports.ports[this_port_id];
            assert_eq!(port.decl_span, found_decl_span);
            port.declaration_instruction = id;
        });
    }

    fn flatten_interface_ports(&mut self, cursor : &mut Cursor) {
        cursor.go_down(kind!("interface_ports"), |cursor| {
            if cursor.optional_field(field!("inputs")) {
                // Read only on inputs and outputs is obviously flipped for submodules, since we're looking at the other side of the in/outputs. 
                self.flatten_declaration_list(IdentifierType::Input, true, cursor)
            }
            if cursor.optional_field(field!("outputs")) {
                // Read only on inputs and outputs is obviously flipped for submodules, since we're looking at the other side of the in/outputs. 
                self.flatten_declaration_list(IdentifierType::Output, false, cursor)
            }
        })
    }

    fn flatten_module(&mut self, cursor : &mut Cursor) {
        cursor.go_down(kind!("module"), |cursor| {
            let name_span = cursor.field_span(field!("name"), kind!("identifier"));
            let module_name = &self.name_resolver.file_text[name_span];
            println!("TREE SITTER module! {module_name}");
            // Interface is allocated in self
            if cursor.optional_field(field!("interface_ports")) {
                self.flatten_interface_ports(cursor);
            }
            
            cursor.field(field!("block"));
            self.flatten_code(cursor);
        })
    }
}

#[derive(Debug)]
pub struct FlattenedInterfacePort {
    pub wire_id : FlatID,
    pub port_name : String,
    pub span : Span
}


/// This method flattens all given code into a simple set of assignments, operators and submodules. 
/// It already does basic type checking and assigns a type to every wire. 
/// The Generating Structure of the code is not yet executed. 
/// It is template-preserving
/// 
/// Separate 'md lifetime for the module. 
/// For some reason if it has the same lifetime as the linker ('l), 
/// then the compiler thinks we could store cursor elements in the module, which would be bad? 
/// Don't fully understand this, but separating the lifetimes makes it work. 
pub fn flatten<'cursor_linker, 'errs>(linker : *mut Linker, module_uuid : ModuleUUID, cursor : &mut Cursor) {
    with_module_editing_context(linker, module_uuid, |modules, types, constants, name_resolver| {
        println!("Flattening {}", modules.working_on.link_info.name);

        let mut context = FlatteningContext {
            ports_to_visit : modules.working_on.module_ports.ports.id_range(),
            errors : name_resolver.errors,
            modules, 
            types, 
            constants,
            name_resolver,
            local_variable_context : LocalVariableContext::new_initial()
        };
        
        context.flatten_module(cursor);
        
        // Make sure all ports have been visited
        assert!(context.ports_to_visit.is_empty());
    });
}

/// Flattens all modules in the project. 
/// 
/// Requires that first, all modules have been initialized. 
pub fn flatten_all_modules(linker : &mut Linker) {
    let linker_ptr : *mut Linker = linker;
    for (_file_id, file) in &linker.files {
        let mut span_debugger = SpanDebugger::new("flatten_all_modules", &file.file_text);
        let mut associated_value_iter = file.associated_values.iter();

        let mut cursor = Cursor::new_at_root(&file.tree, &file.file_text);

        cursor.list(kind!("source_file"), |cursor| {
            match cursor.kind() {
                kind!("module") => {
                    let Some(NameElem::Module(module_uuid)) = associated_value_iter.next() else {unreachable!()};

                    flatten(linker_ptr, *module_uuid, cursor);
                }
                other => todo!("{}", tree_sitter_sus::language().node_kind_for_id(other).unwrap())
            }
        });
        span_debugger.defuse();
    }
}
