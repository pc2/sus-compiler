#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 228
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 98
#define ALIAS_COUNT 0
#define TOKEN_COUNT 54
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 35
#define MAX_ALIAS_SEQUENCE_LENGTH 7
#define PRODUCTION_ID_COUNT 50

enum ts_symbol_identifiers {
  sym_identifier = 1,
  anon_sym___builtin__ = 2,
  anon_sym_extern = 3,
  anon_sym_module = 4,
  anon_sym_function = 5,
  anon_sym_struct = 6,
  anon_sym_POUND_LPAREN = 7,
  anon_sym_RPAREN = 8,
  anon_sym_LBRACE = 9,
  anon_sym_RBRACE = 10,
  anon_sym_EQ = 11,
  anon_sym_reg = 12,
  anon_sym_initial = 13,
  anon_sym_if = 14,
  anon_sym_else = 15,
  anon_sym_for = 16,
  anon_sym_in = 17,
  anon_sym_DOT_DOT = 18,
  anon_sym_domain = 19,
  anon_sym_interface = 20,
  anon_sym_COLON = 21,
  anon_sym_DASH_GT = 22,
  anon_sym_input = 23,
  anon_sym_output = 24,
  anon_sym_state = 25,
  anon_sym_gen = 26,
  anon_sym_SQUOTE = 27,
  anon_sym_PLUS = 28,
  anon_sym_DASH = 29,
  anon_sym_STAR = 30,
  anon_sym_BANG = 31,
  anon_sym_PIPE = 32,
  anon_sym_AMP = 33,
  anon_sym_CARET = 34,
  anon_sym_EQ_EQ = 35,
  anon_sym_BANG_EQ = 36,
  anon_sym_LT = 37,
  anon_sym_LT_EQ = 38,
  anon_sym_GT = 39,
  anon_sym_GT_EQ = 40,
  anon_sym_SLASH = 41,
  anon_sym_PERCENT = 42,
  anon_sym_DOT = 43,
  anon_sym_LPAREN = 44,
  anon_sym_LBRACK = 45,
  anon_sym_RBRACK = 46,
  anon_sym_COLON_COLON = 47,
  anon_sym_type = 48,
  sym_number = 49,
  anon_sym_COMMA = 50,
  anon_sym_LF = 51,
  sym_single_line_comment = 52,
  sym_multi_line_comment = 53,
  sym_source_file = 54,
  sym_global_object = 55,
  sym_template_declaration_arguments = 56,
  sym_template_declaration_type = 57,
  sym_block = 58,
  sym_decl_assign_statement = 59,
  sym_assign_left_side = 60,
  sym_assign_to = 61,
  sym_write_modifiers = 62,
  sym_if_statement = 63,
  sym_for_statement = 64,
  sym_domain_statement = 65,
  sym_interface_statement = 66,
  sym_interface_ports = 67,
  sym__interface_ports_output = 68,
  sym_declaration_list = 69,
  sym_declaration = 70,
  sym_latency_specifier = 71,
  sym__type = 72,
  sym_array_type = 73,
  sym__expression = 74,
  sym_unary_op = 75,
  sym_binary_op = 76,
  sym_array_op = 77,
  sym_func_call = 78,
  sym_field_access = 79,
  sym_parenthesis_expression_list = 80,
  sym_parenthesis_expression = 81,
  sym_array_bracket_expression = 82,
  sym_namespace_list = 83,
  sym_template_global = 84,
  sym_template_args = 85,
  sym_template_arg = 86,
  sym__comma = 87,
  aux_sym__linebreak = 88,
  aux_sym_source_file_repeat1 = 89,
  aux_sym_template_declaration_arguments_repeat1 = 90,
  aux_sym_block_repeat1 = 91,
  aux_sym_assign_left_side_repeat1 = 92,
  aux_sym_write_modifiers_repeat1 = 93,
  aux_sym_declaration_list_repeat1 = 94,
  aux_sym_parenthesis_expression_list_repeat1 = 95,
  aux_sym_namespace_list_repeat1 = 96,
  aux_sym_template_args_repeat1 = 97,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [anon_sym___builtin__] = "__builtin__",
  [anon_sym_extern] = "extern",
  [anon_sym_module] = "module",
  [anon_sym_function] = "function",
  [anon_sym_struct] = "struct",
  [anon_sym_POUND_LPAREN] = "#(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_EQ] = "=",
  [anon_sym_reg] = "reg",
  [anon_sym_initial] = "initial",
  [anon_sym_if] = "if",
  [anon_sym_else] = "else",
  [anon_sym_for] = "for",
  [anon_sym_in] = "in",
  [anon_sym_DOT_DOT] = "..",
  [anon_sym_domain] = "domain",
  [anon_sym_interface] = "interface",
  [anon_sym_COLON] = ":",
  [anon_sym_DASH_GT] = "->",
  [anon_sym_input] = "input",
  [anon_sym_output] = "output",
  [anon_sym_state] = "state",
  [anon_sym_gen] = "gen",
  [anon_sym_SQUOTE] = "'",
  [anon_sym_PLUS] = "+",
  [anon_sym_DASH] = "-",
  [anon_sym_STAR] = "*",
  [anon_sym_BANG] = "!",
  [anon_sym_PIPE] = "|",
  [anon_sym_AMP] = "&",
  [anon_sym_CARET] = "^",
  [anon_sym_EQ_EQ] = "==",
  [anon_sym_BANG_EQ] = "!=",
  [anon_sym_LT] = "<",
  [anon_sym_LT_EQ] = "<=",
  [anon_sym_GT] = ">",
  [anon_sym_GT_EQ] = ">=",
  [anon_sym_SLASH] = "/",
  [anon_sym_PERCENT] = "%",
  [anon_sym_DOT] = ".",
  [anon_sym_LPAREN] = "(",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [anon_sym_COLON_COLON] = "::",
  [anon_sym_type] = "type",
  [sym_number] = "number",
  [anon_sym_COMMA] = ",",
  [anon_sym_LF] = "\n",
  [sym_single_line_comment] = "single_line_comment",
  [sym_multi_line_comment] = "multi_line_comment",
  [sym_source_file] = "source_file",
  [sym_global_object] = "global_object",
  [sym_template_declaration_arguments] = "template_declaration_arguments",
  [sym_template_declaration_type] = "template_declaration_type",
  [sym_block] = "block",
  [sym_decl_assign_statement] = "decl_assign_statement",
  [sym_assign_left_side] = "assign_left_side",
  [sym_assign_to] = "assign_to",
  [sym_write_modifiers] = "write_modifiers",
  [sym_if_statement] = "if_statement",
  [sym_for_statement] = "for_statement",
  [sym_domain_statement] = "domain_statement",
  [sym_interface_statement] = "interface_statement",
  [sym_interface_ports] = "interface_ports",
  [sym__interface_ports_output] = "_interface_ports_output",
  [sym_declaration_list] = "declaration_list",
  [sym_declaration] = "declaration",
  [sym_latency_specifier] = "latency_specifier",
  [sym__type] = "_type",
  [sym_array_type] = "array_type",
  [sym__expression] = "_expression",
  [sym_unary_op] = "unary_op",
  [sym_binary_op] = "binary_op",
  [sym_array_op] = "array_op",
  [sym_func_call] = "func_call",
  [sym_field_access] = "field_access",
  [sym_parenthesis_expression_list] = "parenthesis_expression_list",
  [sym_parenthesis_expression] = "parenthesis_expression",
  [sym_array_bracket_expression] = "array_bracket_expression",
  [sym_namespace_list] = "namespace_list",
  [sym_template_global] = "template_global",
  [sym_template_args] = "template_args",
  [sym_template_arg] = "template_arg",
  [sym__comma] = "_comma",
  [aux_sym__linebreak] = "_linebreak",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_template_declaration_arguments_repeat1] = "template_declaration_arguments_repeat1",
  [aux_sym_block_repeat1] = "block_repeat1",
  [aux_sym_assign_left_side_repeat1] = "assign_left_side_repeat1",
  [aux_sym_write_modifiers_repeat1] = "write_modifiers_repeat1",
  [aux_sym_declaration_list_repeat1] = "declaration_list_repeat1",
  [aux_sym_parenthesis_expression_list_repeat1] = "parenthesis_expression_list_repeat1",
  [aux_sym_namespace_list_repeat1] = "namespace_list_repeat1",
  [aux_sym_template_args_repeat1] = "template_args_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_identifier] = sym_identifier,
  [anon_sym___builtin__] = anon_sym___builtin__,
  [anon_sym_extern] = anon_sym_extern,
  [anon_sym_module] = anon_sym_module,
  [anon_sym_function] = anon_sym_function,
  [anon_sym_struct] = anon_sym_struct,
  [anon_sym_POUND_LPAREN] = anon_sym_POUND_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_reg] = anon_sym_reg,
  [anon_sym_initial] = anon_sym_initial,
  [anon_sym_if] = anon_sym_if,
  [anon_sym_else] = anon_sym_else,
  [anon_sym_for] = anon_sym_for,
  [anon_sym_in] = anon_sym_in,
  [anon_sym_DOT_DOT] = anon_sym_DOT_DOT,
  [anon_sym_domain] = anon_sym_domain,
  [anon_sym_interface] = anon_sym_interface,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_DASH_GT] = anon_sym_DASH_GT,
  [anon_sym_input] = anon_sym_input,
  [anon_sym_output] = anon_sym_output,
  [anon_sym_state] = anon_sym_state,
  [anon_sym_gen] = anon_sym_gen,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_BANG] = anon_sym_BANG,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_AMP] = anon_sym_AMP,
  [anon_sym_CARET] = anon_sym_CARET,
  [anon_sym_EQ_EQ] = anon_sym_EQ_EQ,
  [anon_sym_BANG_EQ] = anon_sym_BANG_EQ,
  [anon_sym_LT] = anon_sym_LT,
  [anon_sym_LT_EQ] = anon_sym_LT_EQ,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_GT_EQ] = anon_sym_GT_EQ,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [anon_sym_PERCENT] = anon_sym_PERCENT,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_COLON_COLON] = anon_sym_COLON_COLON,
  [anon_sym_type] = anon_sym_type,
  [sym_number] = sym_number,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_LF] = anon_sym_LF,
  [sym_single_line_comment] = sym_single_line_comment,
  [sym_multi_line_comment] = sym_multi_line_comment,
  [sym_source_file] = sym_source_file,
  [sym_global_object] = sym_global_object,
  [sym_template_declaration_arguments] = sym_template_declaration_arguments,
  [sym_template_declaration_type] = sym_template_declaration_type,
  [sym_block] = sym_block,
  [sym_decl_assign_statement] = sym_decl_assign_statement,
  [sym_assign_left_side] = sym_assign_left_side,
  [sym_assign_to] = sym_assign_to,
  [sym_write_modifiers] = sym_write_modifiers,
  [sym_if_statement] = sym_if_statement,
  [sym_for_statement] = sym_for_statement,
  [sym_domain_statement] = sym_domain_statement,
  [sym_interface_statement] = sym_interface_statement,
  [sym_interface_ports] = sym_interface_ports,
  [sym__interface_ports_output] = sym__interface_ports_output,
  [sym_declaration_list] = sym_declaration_list,
  [sym_declaration] = sym_declaration,
  [sym_latency_specifier] = sym_latency_specifier,
  [sym__type] = sym__type,
  [sym_array_type] = sym_array_type,
  [sym__expression] = sym__expression,
  [sym_unary_op] = sym_unary_op,
  [sym_binary_op] = sym_binary_op,
  [sym_array_op] = sym_array_op,
  [sym_func_call] = sym_func_call,
  [sym_field_access] = sym_field_access,
  [sym_parenthesis_expression_list] = sym_parenthesis_expression_list,
  [sym_parenthesis_expression] = sym_parenthesis_expression,
  [sym_array_bracket_expression] = sym_array_bracket_expression,
  [sym_namespace_list] = sym_namespace_list,
  [sym_template_global] = sym_template_global,
  [sym_template_args] = sym_template_args,
  [sym_template_arg] = sym_template_arg,
  [sym__comma] = sym__comma,
  [aux_sym__linebreak] = aux_sym__linebreak,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_template_declaration_arguments_repeat1] = aux_sym_template_declaration_arguments_repeat1,
  [aux_sym_block_repeat1] = aux_sym_block_repeat1,
  [aux_sym_assign_left_side_repeat1] = aux_sym_assign_left_side_repeat1,
  [aux_sym_write_modifiers_repeat1] = aux_sym_write_modifiers_repeat1,
  [aux_sym_declaration_list_repeat1] = aux_sym_declaration_list_repeat1,
  [aux_sym_parenthesis_expression_list_repeat1] = aux_sym_parenthesis_expression_list_repeat1,
  [aux_sym_namespace_list_repeat1] = aux_sym_namespace_list_repeat1,
  [aux_sym_template_args_repeat1] = aux_sym_template_args_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym___builtin__] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_extern] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_module] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_function] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_struct] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_POUND_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_reg] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_initial] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_if] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_else] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_for] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_in] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_domain] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_interface] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_input] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_output] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_state] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_gen] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SQUOTE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AMP] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_CARET] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PERCENT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_type] = {
    .visible = true,
    .named = false,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LF] = {
    .visible = true,
    .named = false,
  },
  [sym_single_line_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_multi_line_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_global_object] = {
    .visible = true,
    .named = true,
  },
  [sym_template_declaration_arguments] = {
    .visible = true,
    .named = true,
  },
  [sym_template_declaration_type] = {
    .visible = true,
    .named = true,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym_decl_assign_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_assign_left_side] = {
    .visible = true,
    .named = true,
  },
  [sym_assign_to] = {
    .visible = true,
    .named = true,
  },
  [sym_write_modifiers] = {
    .visible = true,
    .named = true,
  },
  [sym_if_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_for_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_domain_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_interface_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_interface_ports] = {
    .visible = true,
    .named = true,
  },
  [sym__interface_ports_output] = {
    .visible = false,
    .named = true,
  },
  [sym_declaration_list] = {
    .visible = true,
    .named = true,
  },
  [sym_declaration] = {
    .visible = true,
    .named = true,
  },
  [sym_latency_specifier] = {
    .visible = true,
    .named = true,
  },
  [sym__type] = {
    .visible = false,
    .named = true,
  },
  [sym_array_type] = {
    .visible = true,
    .named = true,
  },
  [sym__expression] = {
    .visible = false,
    .named = true,
  },
  [sym_unary_op] = {
    .visible = true,
    .named = true,
  },
  [sym_binary_op] = {
    .visible = true,
    .named = true,
  },
  [sym_array_op] = {
    .visible = true,
    .named = true,
  },
  [sym_func_call] = {
    .visible = true,
    .named = true,
  },
  [sym_field_access] = {
    .visible = true,
    .named = true,
  },
  [sym_parenthesis_expression_list] = {
    .visible = true,
    .named = true,
  },
  [sym_parenthesis_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_array_bracket_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_namespace_list] = {
    .visible = true,
    .named = true,
  },
  [sym_template_global] = {
    .visible = true,
    .named = true,
  },
  [sym_template_args] = {
    .visible = true,
    .named = true,
  },
  [sym_template_arg] = {
    .visible = true,
    .named = true,
  },
  [sym__comma] = {
    .visible = false,
    .named = true,
  },
  [aux_sym__linebreak] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_template_declaration_arguments_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_block_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_assign_left_side_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_write_modifiers_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_declaration_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_parenthesis_expression_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_namespace_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_template_args_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum ts_field_identifiers {
  field_arguments = 1,
  field_arr = 2,
  field_arr_idx = 3,
  field_assign_left = 4,
  field_assign_value = 5,
  field_block = 6,
  field_condition = 7,
  field_content = 8,
  field_declaration_modifiers = 9,
  field_else_block = 10,
  field_expr_or_decl = 11,
  field_extern_marker = 12,
  field_for_decl = 13,
  field_from = 14,
  field_inputs = 15,
  field_interface_ports = 16,
  field_io_port_modifiers = 17,
  field_is_global_path = 18,
  field_item = 19,
  field_latency_specifier = 20,
  field_left = 21,
  field_name = 22,
  field_namespace_list = 23,
  field_object_type = 24,
  field_operator = 25,
  field_outputs = 26,
  field_right = 27,
  field_template_args = 28,
  field_template_declaration_arguments = 29,
  field_then_block = 30,
  field_to = 31,
  field_type = 32,
  field_type_arg = 33,
  field_val_arg = 34,
  field_write_modifiers = 35,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_arguments] = "arguments",
  [field_arr] = "arr",
  [field_arr_idx] = "arr_idx",
  [field_assign_left] = "assign_left",
  [field_assign_value] = "assign_value",
  [field_block] = "block",
  [field_condition] = "condition",
  [field_content] = "content",
  [field_declaration_modifiers] = "declaration_modifiers",
  [field_else_block] = "else_block",
  [field_expr_or_decl] = "expr_or_decl",
  [field_extern_marker] = "extern_marker",
  [field_for_decl] = "for_decl",
  [field_from] = "from",
  [field_inputs] = "inputs",
  [field_interface_ports] = "interface_ports",
  [field_io_port_modifiers] = "io_port_modifiers",
  [field_is_global_path] = "is_global_path",
  [field_item] = "item",
  [field_latency_specifier] = "latency_specifier",
  [field_left] = "left",
  [field_name] = "name",
  [field_namespace_list] = "namespace_list",
  [field_object_type] = "object_type",
  [field_operator] = "operator",
  [field_outputs] = "outputs",
  [field_right] = "right",
  [field_template_args] = "template_args",
  [field_template_declaration_arguments] = "template_declaration_arguments",
  [field_then_block] = "then_block",
  [field_to] = "to",
  [field_type] = "type",
  [field_type_arg] = "type_arg",
  [field_val_arg] = "val_arg",
  [field_write_modifiers] = "write_modifiers",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 2},
  [3] = {.index = 3, .length = 1},
  [4] = {.index = 4, .length = 3},
  [5] = {.index = 7, .length = 2},
  [6] = {.index = 9, .length = 2},
  [7] = {.index = 11, .length = 4},
  [8] = {.index = 15, .length = 1},
  [9] = {.index = 16, .length = 1},
  [10] = {.index = 17, .length = 1},
  [11] = {.index = 18, .length = 1},
  [12] = {.index = 19, .length = 4},
  [13] = {.index = 23, .length = 5},
  [14] = {.index = 28, .length = 2},
  [15] = {.index = 30, .length = 2},
  [16] = {.index = 32, .length = 2},
  [17] = {.index = 34, .length = 2},
  [18] = {.index = 36, .length = 1},
  [19] = {.index = 37, .length = 2},
  [20] = {.index = 39, .length = 2},
  [21] = {.index = 41, .length = 2},
  [22] = {.index = 43, .length = 3},
  [23] = {.index = 46, .length = 3},
  [24] = {.index = 49, .length = 3},
  [25] = {.index = 52, .length = 3},
  [26] = {.index = 55, .length = 1},
  [27] = {.index = 56, .length = 2},
  [28] = {.index = 58, .length = 2},
  [29] = {.index = 60, .length = 1},
  [30] = {.index = 61, .length = 2},
  [31] = {.index = 63, .length = 3},
  [32] = {.index = 66, .length = 2},
  [33] = {.index = 68, .length = 4},
  [34] = {.index = 72, .length = 4},
  [35] = {.index = 76, .length = 4},
  [36] = {.index = 80, .length = 2},
  [37] = {.index = 82, .length = 1},
  [38] = {.index = 83, .length = 1},
  [39] = {.index = 84, .length = 5},
  [40] = {.index = 89, .length = 2},
  [41] = {.index = 91, .length = 3},
  [42] = {.index = 94, .length = 1},
  [43] = {.index = 95, .length = 2},
  [44] = {.index = 97, .length = 1},
  [45] = {.index = 98, .length = 1},
  [46] = {.index = 99, .length = 2},
  [47] = {.index = 101, .length = 1},
  [48] = {.index = 102, .length = 2},
  [49] = {.index = 104, .length = 4},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_item, 0},
  [1] =
    {field_item, 0},
    {field_item, 1, .inherited = true},
  [3] =
    {field_item, 1},
  [4] =
    {field_block, 2},
    {field_name, 1},
    {field_object_type, 0},
  [7] =
    {field_item, 0, .inherited = true},
    {field_item, 1, .inherited = true},
  [9] =
    {field_item, 1},
    {field_item, 2, .inherited = true},
  [11] =
    {field_block, 3},
    {field_extern_marker, 0},
    {field_name, 2},
    {field_object_type, 1},
  [15] =
    {field_name, 0},
  [16] =
    {field_namespace_list, 0},
  [17] =
    {field_expr_or_decl, 0},
  [18] =
    {field_item, 0, .inherited = true},
  [19] =
    {field_block, 3},
    {field_name, 1},
    {field_object_type, 0},
    {field_template_declaration_arguments, 2},
  [23] =
    {field_block, 4},
    {field_extern_marker, 0},
    {field_name, 2},
    {field_object_type, 1},
    {field_template_declaration_arguments, 3},
  [28] =
    {field_is_global_path, 0},
    {field_namespace_list, 1},
  [30] =
    {field_name, 1},
    {field_type, 0},
  [32] =
    {field_arr, 0},
    {field_arr_idx, 1},
  [34] =
    {field_namespace_list, 0},
    {field_template_args, 1},
  [36] =
    {field_name, 1},
  [37] =
    {field_operator, 0},
    {field_right, 1},
  [39] =
    {field_expr_or_decl, 1},
    {field_write_modifiers, 0},
  [41] =
    {field_arguments, 1},
    {field_name, 0},
  [43] =
    {field_io_port_modifiers, 0},
    {field_name, 2},
    {field_type, 1},
  [46] =
    {field_declaration_modifiers, 0},
    {field_name, 2},
    {field_type, 1},
  [49] =
    {field_is_global_path, 0},
    {field_namespace_list, 1},
    {field_template_args, 2},
  [52] =
    {field_latency_specifier, 2},
    {field_name, 1},
    {field_type, 0},
  [55] =
    {field_item, 2},
  [56] =
    {field_condition, 1},
    {field_then_block, 2},
  [58] =
    {field_interface_ports, 2},
    {field_name, 1},
  [60] =
    {field_content, 1},
  [61] =
    {field_assign_left, 0},
    {field_assign_value, 2},
  [63] =
    {field_left, 0},
    {field_operator, 1},
    {field_right, 2},
  [66] =
    {field_left, 0},
    {field_name, 2},
  [68] =
    {field_declaration_modifiers, 1},
    {field_io_port_modifiers, 0},
    {field_name, 3},
    {field_type, 2},
  [72] =
    {field_io_port_modifiers, 0},
    {field_latency_specifier, 3},
    {field_name, 2},
    {field_type, 1},
  [76] =
    {field_declaration_modifiers, 0},
    {field_latency_specifier, 3},
    {field_name, 2},
    {field_type, 1},
  [80] =
    {field_item, 2},
    {field_item, 3, .inherited = true},
  [82] =
    {field_outputs, 1, .inherited = true},
  [83] =
    {field_inputs, 1},
  [84] =
    {field_declaration_modifiers, 1},
    {field_io_port_modifiers, 0},
    {field_latency_specifier, 4},
    {field_name, 3},
    {field_type, 2},
  [89] =
    {field_name, 0},
    {field_val_arg, 2},
  [91] =
    {field_condition, 1},
    {field_else_block, 4},
    {field_then_block, 2},
  [94] =
    {field_outputs, 1},
  [95] =
    {field_inputs, 1},
    {field_outputs, 2, .inherited = true},
  [97] =
    {field_outputs, 2, .inherited = true},
  [98] =
    {field_inputs, 2},
  [99] =
    {field_name, 0},
    {field_type_arg, 3},
  [101] =
    {field_outputs, 2},
  [102] =
    {field_inputs, 2},
    {field_outputs, 3, .inherited = true},
  [104] =
    {field_block, 6},
    {field_for_decl, 1},
    {field_from, 3},
    {field_to, 5},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 47,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 51,
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 72,
  [73] = 73,
  [74] = 74,
  [75] = 75,
  [76] = 76,
  [77] = 77,
  [78] = 78,
  [79] = 79,
  [80] = 80,
  [81] = 81,
  [82] = 82,
  [83] = 83,
  [84] = 84,
  [85] = 85,
  [86] = 86,
  [87] = 87,
  [88] = 88,
  [89] = 89,
  [90] = 90,
  [91] = 91,
  [92] = 92,
  [93] = 93,
  [94] = 94,
  [95] = 95,
  [96] = 96,
  [97] = 97,
  [98] = 98,
  [99] = 99,
  [100] = 40,
  [101] = 101,
  [102] = 102,
  [103] = 103,
  [104] = 104,
  [105] = 105,
  [106] = 106,
  [107] = 107,
  [108] = 108,
  [109] = 109,
  [110] = 110,
  [111] = 111,
  [112] = 112,
  [113] = 113,
  [114] = 114,
  [115] = 115,
  [116] = 116,
  [117] = 117,
  [118] = 118,
  [119] = 119,
  [120] = 120,
  [121] = 121,
  [122] = 122,
  [123] = 123,
  [124] = 124,
  [125] = 125,
  [126] = 126,
  [127] = 127,
  [128] = 128,
  [129] = 129,
  [130] = 130,
  [131] = 131,
  [132] = 132,
  [133] = 133,
  [134] = 134,
  [135] = 135,
  [136] = 136,
  [137] = 137,
  [138] = 138,
  [139] = 139,
  [140] = 140,
  [141] = 141,
  [142] = 142,
  [143] = 143,
  [144] = 144,
  [145] = 145,
  [146] = 146,
  [147] = 147,
  [148] = 148,
  [149] = 149,
  [150] = 150,
  [151] = 151,
  [152] = 152,
  [153] = 153,
  [154] = 154,
  [155] = 155,
  [156] = 156,
  [157] = 157,
  [158] = 158,
  [159] = 159,
  [160] = 160,
  [161] = 161,
  [162] = 162,
  [163] = 163,
  [164] = 164,
  [165] = 165,
  [166] = 166,
  [167] = 167,
  [168] = 168,
  [169] = 169,
  [170] = 170,
  [171] = 171,
  [172] = 172,
  [173] = 173,
  [174] = 174,
  [175] = 175,
  [176] = 176,
  [177] = 177,
  [178] = 178,
  [179] = 179,
  [180] = 180,
  [181] = 181,
  [182] = 182,
  [183] = 183,
  [184] = 184,
  [185] = 185,
  [186] = 186,
  [187] = 187,
  [188] = 188,
  [189] = 189,
  [190] = 190,
  [191] = 191,
  [192] = 192,
  [193] = 193,
  [194] = 194,
  [195] = 195,
  [196] = 196,
  [197] = 197,
  [198] = 198,
  [199] = 199,
  [200] = 200,
  [201] = 201,
  [202] = 202,
  [203] = 203,
  [204] = 204,
  [205] = 205,
  [206] = 206,
  [207] = 207,
  [208] = 208,
  [209] = 209,
  [210] = 210,
  [211] = 211,
  [212] = 212,
  [213] = 213,
  [214] = 214,
  [215] = 215,
  [216] = 216,
  [217] = 217,
  [218] = 218,
  [219] = 219,
  [220] = 220,
  [221] = 221,
  [222] = 222,
  [223] = 223,
  [224] = 224,
  [225] = 225,
  [226] = 226,
  [227] = 227,
};

static inline bool sym_identifier_character_set_1(int32_t c) {
  return (c < 42994
    ? (c < 3585
      ? (c < 2649
        ? (c < 1869
          ? (c < 931
            ? (c < 748
              ? (c < 186
                ? (c < 'a'
                  ? (c < '_'
                    ? (c >= 'A' && c <= 'Z')
                    : c <= '_')
                  : (c <= 'z' || (c < 181
                    ? c == 170
                    : c <= 181)))
                : (c <= 186 || (c < 248
                  ? (c < 216
                    ? (c >= 192 && c <= 214)
                    : c <= 246)
                  : (c <= 705 || (c < 736
                    ? (c >= 710 && c <= 721)
                    : c <= 740)))))
              : (c <= 748 || (c < 895
                ? (c < 880
                  ? (c < 837
                    ? c == 750
                    : c <= 837)
                  : (c <= 884 || (c < 890
                    ? (c >= 886 && c <= 887)
                    : c <= 893)))
                : (c <= 895 || (c < 908
                  ? (c < 904
                    ? c == 902
                    : c <= 906)
                  : (c <= 908 || (c >= 910 && c <= 929)))))))
            : (c <= 1013 || (c < 1519
              ? (c < 1456
                ? (c < 1329
                  ? (c < 1162
                    ? (c >= 1015 && c <= 1153)
                    : c <= 1327)
                  : (c <= 1366 || (c < 1376
                    ? c == 1369
                    : c <= 1416)))
                : (c <= 1469 || (c < 1476
                  ? (c < 1473
                    ? c == 1471
                    : c <= 1474)
                  : (c <= 1477 || (c < 1488
                    ? c == 1479
                    : c <= 1514)))))
              : (c <= 1522 || (c < 1761
                ? (c < 1625
                  ? (c < 1568
                    ? (c >= 1552 && c <= 1562)
                    : c <= 1623)
                  : (c <= 1631 || (c < 1749
                    ? (c >= 1646 && c <= 1747)
                    : c <= 1756)))
                : (c <= 1768 || (c < 1791
                  ? (c < 1786
                    ? (c >= 1773 && c <= 1775)
                    : c <= 1788)
                  : (c <= 1791 || (c >= 1808 && c <= 1855)))))))))
          : (c <= 1969 || (c < 2486
            ? (c < 2275
              ? (c < 2112
                ? (c < 2042
                  ? (c < 2036
                    ? (c >= 1994 && c <= 2026)
                    : c <= 2037)
                  : (c <= 2042 || (c < 2074
                    ? (c >= 2048 && c <= 2071)
                    : c <= 2092)))
                : (c <= 2136 || (c < 2185
                  ? (c < 2160
                    ? (c >= 2144 && c <= 2154)
                    : c <= 2183)
                  : (c <= 2190 || (c < 2260
                    ? (c >= 2208 && c <= 2249)
                    : c <= 2271)))))
              : (c <= 2281 || (c < 2437
                ? (c < 2382
                  ? (c < 2365
                    ? (c >= 2288 && c <= 2363)
                    : c <= 2380)
                  : (c <= 2384 || (c < 2417
                    ? (c >= 2389 && c <= 2403)
                    : c <= 2435)))
                : (c <= 2444 || (c < 2474
                  ? (c < 2451
                    ? (c >= 2447 && c <= 2448)
                    : c <= 2472)
                  : (c <= 2480 || c == 2482))))))
            : (c <= 2489 || (c < 2565
              ? (c < 2524
                ? (c < 2507
                  ? (c < 2503
                    ? (c >= 2493 && c <= 2500)
                    : c <= 2504)
                  : (c <= 2508 || (c < 2519
                    ? c == 2510
                    : c <= 2519)))
                : (c <= 2525 || (c < 2556
                  ? (c < 2544
                    ? (c >= 2527 && c <= 2531)
                    : c <= 2545)
                  : (c <= 2556 || (c >= 2561 && c <= 2563)))))
              : (c <= 2570 || (c < 2616
                ? (c < 2602
                  ? (c < 2579
                    ? (c >= 2575 && c <= 2576)
                    : c <= 2600)
                  : (c <= 2608 || (c < 2613
                    ? (c >= 2610 && c <= 2611)
                    : c <= 2614)))
                : (c <= 2617 || (c < 2635
                  ? (c < 2631
                    ? (c >= 2622 && c <= 2626)
                    : c <= 2632)
                  : (c <= 2636 || c == 2641))))))))))
        : (c <= 2652 || (c < 3072
          ? (c < 2877
            ? (c < 2763
              ? (c < 2707
                ? (c < 2689
                  ? (c < 2672
                    ? c == 2654
                    : c <= 2677)
                  : (c <= 2691 || (c < 2703
                    ? (c >= 2693 && c <= 2701)
                    : c <= 2705)))
                : (c <= 2728 || (c < 2741
                  ? (c < 2738
                    ? (c >= 2730 && c <= 2736)
                    : c <= 2739)
                  : (c <= 2745 || (c < 2759
                    ? (c >= 2749 && c <= 2757)
                    : c <= 2761)))))
              : (c <= 2764 || (c < 2831
                ? (c < 2809
                  ? (c < 2784
                    ? c == 2768
                    : c <= 2787)
                  : (c <= 2812 || (c < 2821
                    ? (c >= 2817 && c <= 2819)
                    : c <= 2828)))
                : (c <= 2832 || (c < 2866
                  ? (c < 2858
                    ? (c >= 2835 && c <= 2856)
                    : c <= 2864)
                  : (c <= 2867 || (c >= 2869 && c <= 2873)))))))
            : (c <= 2884 || (c < 2969
              ? (c < 2929
                ? (c < 2902
                  ? (c < 2891
                    ? (c >= 2887 && c <= 2888)
                    : c <= 2892)
                  : (c <= 2903 || (c < 2911
                    ? (c >= 2908 && c <= 2909)
                    : c <= 2915)))
                : (c <= 2929 || (c < 2958
                  ? (c < 2949
                    ? (c >= 2946 && c <= 2947)
                    : c <= 2954)
                  : (c <= 2960 || (c >= 2962 && c <= 2965)))))
              : (c <= 2970 || (c < 3006
                ? (c < 2979
                  ? (c < 2974
                    ? c == 2972
                    : c <= 2975)
                  : (c <= 2980 || (c < 2990
                    ? (c >= 2984 && c <= 2986)
                    : c <= 3001)))
                : (c <= 3010 || (c < 3024
                  ? (c < 3018
                    ? (c >= 3014 && c <= 3016)
                    : c <= 3020)
                  : (c <= 3024 || c == 3031))))))))
          : (c <= 3075 || (c < 3296
            ? (c < 3200
              ? (c < 3142
                ? (c < 3090
                  ? (c < 3086
                    ? (c >= 3077 && c <= 3084)
                    : c <= 3088)
                  : (c <= 3112 || (c < 3133
                    ? (c >= 3114 && c <= 3129)
                    : c <= 3140)))
                : (c <= 3144 || (c < 3160
                  ? (c < 3157
                    ? (c >= 3146 && c <= 3148)
                    : c <= 3158)
                  : (c <= 3162 || (c < 3168
                    ? c == 3165
                    : c <= 3171)))))
              : (c <= 3203 || (c < 3261
                ? (c < 3218
                  ? (c < 3214
                    ? (c >= 3205 && c <= 3212)
                    : c <= 3216)
                  : (c <= 3240 || (c < 3253
                    ? (c >= 3242 && c <= 3251)
                    : c <= 3257)))
                : (c <= 3268 || (c < 3285
                  ? (c < 3274
                    ? (c >= 3270 && c <= 3272)
                    : c <= 3276)
                  : (c <= 3286 || (c >= 3293 && c <= 3294)))))))
            : (c <= 3299 || (c < 3450
              ? (c < 3398
                ? (c < 3342
                  ? (c < 3328
                    ? (c >= 3313 && c <= 3314)
                    : c <= 3340)
                  : (c <= 3344 || (c < 3389
                    ? (c >= 3346 && c <= 3386)
                    : c <= 3396)))
                : (c <= 3400 || (c < 3412
                  ? (c < 3406
                    ? (c >= 3402 && c <= 3404)
                    : c <= 3406)
                  : (c <= 3415 || (c >= 3423 && c <= 3427)))))
              : (c <= 3455 || (c < 3520
                ? (c < 3482
                  ? (c < 3461
                    ? (c >= 3457 && c <= 3459)
                    : c <= 3478)
                  : (c <= 3505 || (c < 3517
                    ? (c >= 3507 && c <= 3515)
                    : c <= 3517)))
                : (c <= 3526 || (c < 3544
                  ? (c < 3542
                    ? (c >= 3535 && c <= 3540)
                    : c <= 3542)
                  : (c <= 3551 || (c >= 3570 && c <= 3571)))))))))))))
      : (c <= 3642 || (c < 7357
        ? (c < 5024
          ? (c < 4176
            ? (c < 3789
              ? (c < 3724
                ? (c < 3713
                  ? (c < 3661
                    ? (c >= 3648 && c <= 3654)
                    : c <= 3661)
                  : (c <= 3714 || (c < 3718
                    ? c == 3716
                    : c <= 3722)))
                : (c <= 3747 || (c < 3771
                  ? (c < 3751
                    ? c == 3749
                    : c <= 3769)
                  : (c <= 3773 || (c < 3782
                    ? (c >= 3776 && c <= 3780)
                    : c <= 3782)))))
              : (c <= 3789 || (c < 3976
                ? (c < 3904
                  ? (c < 3840
                    ? (c >= 3804 && c <= 3807)
                    : c <= 3840)
                  : (c <= 3911 || (c < 3953
                    ? (c >= 3913 && c <= 3948)
                    : c <= 3969)))
                : (c <= 3991 || (c < 4152
                  ? (c < 4096
                    ? (c >= 3993 && c <= 4028)
                    : c <= 4150)
                  : (c <= 4152 || (c >= 4155 && c <= 4159)))))))
            : (c <= 4239 || (c < 4746
              ? (c < 4348
                ? (c < 4295
                  ? (c < 4256
                    ? (c >= 4250 && c <= 4253)
                    : c <= 4293)
                  : (c <= 4295 || (c < 4304
                    ? c == 4301
                    : c <= 4346)))
                : (c <= 4680 || (c < 4696
                  ? (c < 4688
                    ? (c >= 4682 && c <= 4685)
                    : c <= 4694)
                  : (c <= 4696 || (c < 4704
                    ? (c >= 4698 && c <= 4701)
                    : c <= 4744)))))
              : (c <= 4749 || (c < 4808
                ? (c < 4792
                  ? (c < 4786
                    ? (c >= 4752 && c <= 4784)
                    : c <= 4789)
                  : (c <= 4798 || (c < 4802
                    ? c == 4800
                    : c <= 4805)))
                : (c <= 4822 || (c < 4888
                  ? (c < 4882
                    ? (c >= 4824 && c <= 4880)
                    : c <= 4885)
                  : (c <= 4954 || (c >= 4992 && c <= 5007)))))))))
          : (c <= 5109 || (c < 6480
            ? (c < 6002
              ? (c < 5870
                ? (c < 5743
                  ? (c < 5121
                    ? (c >= 5112 && c <= 5117)
                    : c <= 5740)
                  : (c <= 5759 || (c < 5792
                    ? (c >= 5761 && c <= 5786)
                    : c <= 5866)))
                : (c <= 5880 || (c < 5952
                  ? (c < 5919
                    ? (c >= 5888 && c <= 5907)
                    : c <= 5939)
                  : (c <= 5971 || (c < 5998
                    ? (c >= 5984 && c <= 5996)
                    : c <= 6000)))))
              : (c <= 6003 || (c < 6272
                ? (c < 6103
                  ? (c < 6070
                    ? (c >= 6016 && c <= 6067)
                    : c <= 6088)
                  : (c <= 6103 || (c < 6176
                    ? c == 6108
                    : c <= 6264)))
                : (c <= 6314 || (c < 6432
                  ? (c < 6400
                    ? (c >= 6320 && c <= 6389)
                    : c <= 6430)
                  : (c <= 6443 || (c >= 6448 && c <= 6456)))))))
            : (c <= 6509 || (c < 6965
              ? (c < 6753
                ? (c < 6576
                  ? (c < 6528
                    ? (c >= 6512 && c <= 6516)
                    : c <= 6571)
                  : (c <= 6601 || (c < 6688
                    ? (c >= 6656 && c <= 6683)
                    : c <= 6750)))
                : (c <= 6772 || (c < 6860
                  ? (c < 6847
                    ? c == 6823
                    : c <= 6848)
                  : (c <= 6862 || (c >= 6912 && c <= 6963)))))
              : (c <= 6979 || (c < 7168
                ? (c < 7084
                  ? (c < 7040
                    ? (c >= 6981 && c <= 6988)
                    : c <= 7081)
                  : (c <= 7087 || (c < 7143
                    ? (c >= 7098 && c <= 7141)
                    : c <= 7153)))
                : (c <= 7222 || (c < 7296
                  ? (c < 7258
                    ? (c >= 7245 && c <= 7247)
                    : c <= 7293)
                  : (c <= 7304 || (c >= 7312 && c <= 7354)))))))))))
        : (c <= 7359 || (c < 11499
          ? (c < 8160
            ? (c < 8025
              ? (c < 7655
                ? (c < 7413
                  ? (c < 7406
                    ? (c >= 7401 && c <= 7404)
                    : c <= 7411)
                  : (c <= 7414 || (c < 7424
                    ? c == 7418
                    : c <= 7615)))
                : (c <= 7668 || (c < 7968
                  ? (c < 7960
                    ? (c >= 7680 && c <= 7957)
                    : c <= 7965)
                  : (c <= 8005 || (c < 8016
                    ? (c >= 8008 && c <= 8013)
                    : c <= 8023)))))
              : (c <= 8025 || (c < 8126
                ? (c < 8031
                  ? (c < 8029
                    ? c == 8027
                    : c <= 8029)
                  : (c <= 8061 || (c < 8118
                    ? (c >= 8064 && c <= 8116)
                    : c <= 8124)))
                : (c <= 8126 || (c < 8144
                  ? (c < 8134
                    ? (c >= 8130 && c <= 8132)
                    : c <= 8140)
                  : (c <= 8147 || (c >= 8150 && c <= 8155)))))))
            : (c <= 8172 || (c < 8484
              ? (c < 8450
                ? (c < 8305
                  ? (c < 8182
                    ? (c >= 8178 && c <= 8180)
                    : c <= 8188)
                  : (c <= 8305 || (c < 8336
                    ? c == 8319
                    : c <= 8348)))
                : (c <= 8450 || (c < 8469
                  ? (c < 8458
                    ? c == 8455
                    : c <= 8467)
                  : (c <= 8469 || (c >= 8473 && c <= 8477)))))
              : (c <= 8484 || (c < 8517
                ? (c < 8490
                  ? (c < 8488
                    ? c == 8486
                    : c <= 8488)
                  : (c <= 8493 || (c < 8508
                    ? (c >= 8495 && c <= 8505)
                    : c <= 8511)))
                : (c <= 8521 || (c < 9398
                  ? (c < 8544
                    ? c == 8526
                    : c <= 8584)
                  : (c <= 9449 || (c >= 11264 && c <= 11492)))))))))
          : (c <= 11502 || (c < 12445
            ? (c < 11712
              ? (c < 11631
                ? (c < 11559
                  ? (c < 11520
                    ? (c >= 11506 && c <= 11507)
                    : c <= 11557)
                  : (c <= 11559 || (c < 11568
                    ? c == 11565
                    : c <= 11623)))
                : (c <= 11631 || (c < 11688
                  ? (c < 11680
                    ? (c >= 11648 && c <= 11670)
                    : c <= 11686)
                  : (c <= 11694 || (c < 11704
                    ? (c >= 11696 && c <= 11702)
                    : c <= 11710)))))
              : (c <= 11718 || (c < 12293
                ? (c < 11736
                  ? (c < 11728
                    ? (c >= 11720 && c <= 11726)
                    : c <= 11734)
                  : (c <= 11742 || (c < 11823
                    ? (c >= 11744 && c <= 11775)
                    : c <= 11823)))
                : (c <= 12295 || (c < 12344
                  ? (c < 12337
                    ? (c >= 12321 && c <= 12329)
                    : c <= 12341)
                  : (c <= 12348 || (c >= 12353 && c <= 12438)))))))
            : (c <= 12447 || (c < 42512
              ? (c < 12784
                ? (c < 12549
                  ? (c < 12540
                    ? (c >= 12449 && c <= 12538)
                    : c <= 12543)
                  : (c <= 12591 || (c < 12704
                    ? (c >= 12593 && c <= 12686)
                    : c <= 12735)))
                : (c <= 12799 || (c < 42192
                  ? (c < 19968
                    ? (c >= 13312 && c <= 19903)
                    : c <= 42124)
                  : (c <= 42237 || (c >= 42240 && c <= 42508)))))
              : (c <= 42527 || (c < 42786
                ? (c < 42612
                  ? (c < 42560
                    ? (c >= 42538 && c <= 42539)
                    : c <= 42606)
                  : (c <= 42619 || (c < 42775
                    ? (c >= 42623 && c <= 42735)
                    : c <= 42783)))
                : (c <= 42888 || (c < 42963
                  ? (c < 42960
                    ? (c >= 42891 && c <= 42954)
                    : c <= 42961)
                  : (c <= 42963 || (c >= 42965 && c <= 42969)))))))))))))))
    : (c <= 43013 || (c < 71096
      ? (c < 67392
        ? (c < 64326
          ? (c < 43744
            ? (c < 43444
              ? (c < 43259
                ? (c < 43136
                  ? (c < 43072
                    ? (c >= 43015 && c <= 43047)
                    : c <= 43123)
                  : (c <= 43203 || (c < 43250
                    ? c == 43205
                    : c <= 43255)))
                : (c <= 43259 || (c < 43312
                  ? (c < 43274
                    ? (c >= 43261 && c <= 43263)
                    : c <= 43306)
                  : (c <= 43346 || (c < 43392
                    ? (c >= 43360 && c <= 43388)
                    : c <= 43442)))))
              : (c <= 43455 || (c < 43616
                ? (c < 43514
                  ? (c < 43488
                    ? c == 43471
                    : c <= 43503)
                  : (c <= 43518 || (c < 43584
                    ? (c >= 43520 && c <= 43574)
                    : c <= 43597)))
                : (c <= 43638 || (c < 43714
                  ? (c < 43712
                    ? (c >= 43642 && c <= 43710)
                    : c <= 43712)
                  : (c <= 43714 || (c >= 43739 && c <= 43741)))))))
            : (c <= 43759 || (c < 55243
              ? (c < 43816
                ? (c < 43785
                  ? (c < 43777
                    ? (c >= 43762 && c <= 43765)
                    : c <= 43782)
                  : (c <= 43790 || (c < 43808
                    ? (c >= 43793 && c <= 43798)
                    : c <= 43814)))
                : (c <= 43822 || (c < 43888
                  ? (c < 43868
                    ? (c >= 43824 && c <= 43866)
                    : c <= 43881)
                  : (c <= 44010 || (c < 55216
                    ? (c >= 44032 && c <= 55203)
                    : c <= 55238)))))
              : (c <= 55291 || (c < 64298
                ? (c < 64256
                  ? (c < 64112
                    ? (c >= 63744 && c <= 64109)
                    : c <= 64217)
                  : (c <= 64262 || (c < 64285
                    ? (c >= 64275 && c <= 64279)
                    : c <= 64296)))
                : (c <= 64310 || (c < 64320
                  ? (c < 64318
                    ? (c >= 64312 && c <= 64316)
                    : c <= 64318)
                  : (c <= 64321 || (c >= 64323 && c <= 64324)))))))))
          : (c <= 64433 || (c < 66208
            ? (c < 65490
              ? (c < 65142
                ? (c < 64914
                  ? (c < 64848
                    ? (c >= 64467 && c <= 64829)
                    : c <= 64911)
                  : (c <= 64967 || (c < 65136
                    ? (c >= 65008 && c <= 65019)
                    : c <= 65140)))
                : (c <= 65276 || (c < 65382
                  ? (c < 65345
                    ? (c >= 65313 && c <= 65338)
                    : c <= 65370)
                  : (c <= 65470 || (c < 65482
                    ? (c >= 65474 && c <= 65479)
                    : c <= 65487)))))
              : (c <= 65495 || (c < 65599
                ? (c < 65549
                  ? (c < 65536
                    ? (c >= 65498 && c <= 65500)
                    : c <= 65547)
                  : (c <= 65574 || (c < 65596
                    ? (c >= 65576 && c <= 65594)
                    : c <= 65597)))
                : (c <= 65613 || (c < 65856
                  ? (c < 65664
                    ? (c >= 65616 && c <= 65629)
                    : c <= 65786)
                  : (c <= 65908 || (c >= 66176 && c <= 66204)))))))
            : (c <= 66256 || (c < 66816
              ? (c < 66504
                ? (c < 66384
                  ? (c < 66349
                    ? (c >= 66304 && c <= 66335)
                    : c <= 66378)
                  : (c <= 66426 || (c < 66464
                    ? (c >= 66432 && c <= 66461)
                    : c <= 66499)))
                : (c <= 66511 || (c < 66736
                  ? (c < 66560
                    ? (c >= 66513 && c <= 66517)
                    : c <= 66717)
                  : (c <= 66771 || (c >= 66776 && c <= 66811)))))
              : (c <= 66855 || (c < 66967
                ? (c < 66940
                  ? (c < 66928
                    ? (c >= 66864 && c <= 66915)
                    : c <= 66938)
                  : (c <= 66954 || (c < 66964
                    ? (c >= 66956 && c <= 66962)
                    : c <= 66965)))
                : (c <= 66977 || (c < 67003
                  ? (c < 66995
                    ? (c >= 66979 && c <= 66993)
                    : c <= 67001)
                  : (c <= 67004 || (c >= 67072 && c <= 67382)))))))))))
        : (c <= 67413 || (c < 69632
          ? (c < 68121
            ? (c < 67712
              ? (c < 67592
                ? (c < 67463
                  ? (c < 67456
                    ? (c >= 67424 && c <= 67431)
                    : c <= 67461)
                  : (c <= 67504 || (c < 67584
                    ? (c >= 67506 && c <= 67514)
                    : c <= 67589)))
                : (c <= 67592 || (c < 67644
                  ? (c < 67639
                    ? (c >= 67594 && c <= 67637)
                    : c <= 67640)
                  : (c <= 67644 || (c < 67680
                    ? (c >= 67647 && c <= 67669)
                    : c <= 67702)))))
              : (c <= 67742 || (c < 68030
                ? (c < 67840
                  ? (c < 67828
                    ? (c >= 67808 && c <= 67826)
                    : c <= 67829)
                  : (c <= 67861 || (c < 67968
                    ? (c >= 67872 && c <= 67897)
                    : c <= 68023)))
                : (c <= 68031 || (c < 68108
                  ? (c < 68101
                    ? (c >= 68096 && c <= 68099)
                    : c <= 68102)
                  : (c <= 68115 || (c >= 68117 && c <= 68119)))))))
            : (c <= 68149 || (c < 68800
              ? (c < 68416
                ? (c < 68288
                  ? (c < 68224
                    ? (c >= 68192 && c <= 68220)
                    : c <= 68252)
                  : (c <= 68295 || (c < 68352
                    ? (c >= 68297 && c <= 68324)
                    : c <= 68405)))
                : (c <= 68437 || (c < 68608
                  ? (c < 68480
                    ? (c >= 68448 && c <= 68466)
                    : c <= 68497)
                  : (c <= 68680 || (c >= 68736 && c <= 68786)))))
              : (c <= 68850 || (c < 69415
                ? (c < 69291
                  ? (c < 69248
                    ? (c >= 68864 && c <= 68903)
                    : c <= 69289)
                  : (c <= 69292 || (c < 69376
                    ? (c >= 69296 && c <= 69297)
                    : c <= 69404)))
                : (c <= 69415 || (c < 69552
                  ? (c < 69488
                    ? (c >= 69424 && c <= 69445)
                    : c <= 69505)
                  : (c <= 69572 || (c >= 69600 && c <= 69622)))))))))
          : (c <= 69701 || (c < 70320
            ? (c < 70106
              ? (c < 69956
                ? (c < 69826
                  ? (c < 69762
                    ? (c >= 69745 && c <= 69749)
                    : c <= 69816)
                  : (c <= 69826 || (c < 69888
                    ? (c >= 69840 && c <= 69864)
                    : c <= 69938)))
                : (c <= 69959 || (c < 70016
                  ? (c < 70006
                    ? (c >= 69968 && c <= 70002)
                    : c <= 70006)
                  : (c <= 70079 || (c < 70094
                    ? (c >= 70081 && c <= 70084)
                    : c <= 70095)))))
              : (c <= 70106 || (c < 70272
                ? (c < 70163
                  ? (c < 70144
                    ? c == 70108
                    : c <= 70161)
                  : (c <= 70196 || (c < 70206
                    ? c == 70199
                    : c <= 70206)))
                : (c <= 70278 || (c < 70287
                  ? (c < 70282
                    ? c == 70280
                    : c <= 70285)
                  : (c <= 70301 || (c >= 70303 && c <= 70312)))))))
            : (c <= 70376 || (c < 70480
              ? (c < 70450
                ? (c < 70415
                  ? (c < 70405
                    ? (c >= 70400 && c <= 70403)
                    : c <= 70412)
                  : (c <= 70416 || (c < 70442
                    ? (c >= 70419 && c <= 70440)
                    : c <= 70448)))
                : (c <= 70451 || (c < 70471
                  ? (c < 70461
                    ? (c >= 70453 && c <= 70457)
                    : c <= 70468)
                  : (c <= 70472 || (c >= 70475 && c <= 70476)))))
              : (c <= 70480 || (c < 70751
                ? (c < 70656
                  ? (c < 70493
                    ? c == 70487
                    : c <= 70499)
                  : (c <= 70721 || (c < 70727
                    ? (c >= 70723 && c <= 70725)
                    : c <= 70730)))
                : (c <= 70753 || (c < 70855
                  ? (c < 70852
                    ? (c >= 70784 && c <= 70849)
                    : c <= 70853)
                  : (c <= 70855 || (c >= 71040 && c <= 71093)))))))))))))
      : (c <= 71102 || (c < 119966
        ? (c < 73063
          ? (c < 72161
            ? (c < 71935
              ? (c < 71352
                ? (c < 71232
                  ? (c < 71168
                    ? (c >= 71128 && c <= 71133)
                    : c <= 71230)
                  : (c <= 71232 || (c < 71296
                    ? c == 71236
                    : c <= 71349)))
                : (c <= 71352 || (c < 71488
                  ? (c < 71453
                    ? (c >= 71424 && c <= 71450)
                    : c <= 71466)
                  : (c <= 71494 || (c < 71840
                    ? (c >= 71680 && c <= 71736)
                    : c <= 71903)))))
              : (c <= 71942 || (c < 71995
                ? (c < 71957
                  ? (c < 71948
                    ? c == 71945
                    : c <= 71955)
                  : (c <= 71958 || (c < 71991
                    ? (c >= 71960 && c <= 71989)
                    : c <= 71992)))
                : (c <= 71996 || (c < 72106
                  ? (c < 72096
                    ? (c >= 71999 && c <= 72002)
                    : c <= 72103)
                  : (c <= 72151 || (c >= 72154 && c <= 72159)))))))
            : (c <= 72161 || (c < 72850
              ? (c < 72368
                ? (c < 72245
                  ? (c < 72192
                    ? (c >= 72163 && c <= 72164)
                    : c <= 72242)
                  : (c <= 72254 || (c < 72349
                    ? (c >= 72272 && c <= 72343)
                    : c <= 72349)))
                : (c <= 72440 || (c < 72760
                  ? (c < 72714
                    ? (c >= 72704 && c <= 72712)
                    : c <= 72758)
                  : (c <= 72766 || (c < 72818
                    ? c == 72768
                    : c <= 72847)))))
              : (c <= 72871 || (c < 73020
                ? (c < 72968
                  ? (c < 72960
                    ? (c >= 72873 && c <= 72886)
                    : c <= 72966)
                  : (c <= 72969 || (c < 73018
                    ? (c >= 72971 && c <= 73014)
                    : c <= 73018)))
                : (c <= 73021 || (c < 73030
                  ? (c < 73027
                    ? (c >= 73023 && c <= 73025)
                    : c <= 73027)
                  : (c <= 73031 || (c >= 73056 && c <= 73061)))))))))
          : (c <= 73064 || (c < 94031
            ? (c < 82944
              ? (c < 73648
                ? (c < 73107
                  ? (c < 73104
                    ? (c >= 73066 && c <= 73102)
                    : c <= 73105)
                  : (c <= 73110 || (c < 73440
                    ? c == 73112
                    : c <= 73462)))
                : (c <= 73648 || (c < 74880
                  ? (c < 74752
                    ? (c >= 73728 && c <= 74649)
                    : c <= 74862)
                  : (c <= 75075 || (c < 77824
                    ? (c >= 77712 && c <= 77808)
                    : c <= 78894)))))
              : (c <= 83526 || (c < 92992
                ? (c < 92784
                  ? (c < 92736
                    ? (c >= 92160 && c <= 92728)
                    : c <= 92766)
                  : (c <= 92862 || (c < 92928
                    ? (c >= 92880 && c <= 92909)
                    : c <= 92975)))
                : (c <= 92995 || (c < 93760
                  ? (c < 93053
                    ? (c >= 93027 && c <= 93047)
                    : c <= 93071)
                  : (c <= 93823 || (c >= 93952 && c <= 94026)))))))
            : (c <= 94087 || (c < 110592
              ? (c < 100352
                ? (c < 94179
                  ? (c < 94176
                    ? (c >= 94095 && c <= 94111)
                    : c <= 94177)
                  : (c <= 94179 || (c < 94208
                    ? (c >= 94192 && c <= 94193)
                    : c <= 100343)))
                : (c <= 101589 || (c < 110581
                  ? (c < 110576
                    ? (c >= 101632 && c <= 101640)
                    : c <= 110579)
                  : (c <= 110587 || (c >= 110589 && c <= 110590)))))
              : (c <= 110882 || (c < 113792
                ? (c < 110960
                  ? (c < 110948
                    ? (c >= 110928 && c <= 110930)
                    : c <= 110951)
                  : (c <= 111355 || (c < 113776
                    ? (c >= 113664 && c <= 113770)
                    : c <= 113788)))
                : (c <= 113800 || (c < 119808
                  ? (c < 113822
                    ? (c >= 113808 && c <= 113817)
                    : c <= 113822)
                  : (c <= 119892 || (c >= 119894 && c <= 119964)))))))))))
        : (c <= 119967 || (c < 125255
          ? (c < 120656
            ? (c < 120123
              ? (c < 119997
                ? (c < 119977
                  ? (c < 119973
                    ? c == 119970
                    : c <= 119974)
                  : (c <= 119980 || (c < 119995
                    ? (c >= 119982 && c <= 119993)
                    : c <= 119995)))
                : (c <= 120003 || (c < 120077
                  ? (c < 120071
                    ? (c >= 120005 && c <= 120069)
                    : c <= 120074)
                  : (c <= 120084 || (c < 120094
                    ? (c >= 120086 && c <= 120092)
                    : c <= 120121)))))
              : (c <= 120126 || (c < 120514
                ? (c < 120138
                  ? (c < 120134
                    ? (c >= 120128 && c <= 120132)
                    : c <= 120134)
                  : (c <= 120144 || (c < 120488
                    ? (c >= 120146 && c <= 120485)
                    : c <= 120512)))
                : (c <= 120538 || (c < 120598
                  ? (c < 120572
                    ? (c >= 120540 && c <= 120570)
                    : c <= 120596)
                  : (c <= 120628 || (c >= 120630 && c <= 120654)))))))
            : (c <= 120686 || (c < 123136
              ? (c < 122880
                ? (c < 120746
                  ? (c < 120714
                    ? (c >= 120688 && c <= 120712)
                    : c <= 120744)
                  : (c <= 120770 || (c < 122624
                    ? (c >= 120772 && c <= 120779)
                    : c <= 122654)))
                : (c <= 122886 || (c < 122915
                  ? (c < 122907
                    ? (c >= 122888 && c <= 122904)
                    : c <= 122913)
                  : (c <= 122916 || (c >= 122918 && c <= 122922)))))
              : (c <= 123180 || (c < 124904
                ? (c < 123536
                  ? (c < 123214
                    ? (c >= 123191 && c <= 123197)
                    : c <= 123214)
                  : (c <= 123565 || (c < 124896
                    ? (c >= 123584 && c <= 123627)
                    : c <= 124902)))
                : (c <= 124907 || (c < 124928
                  ? (c < 124912
                    ? (c >= 124909 && c <= 124910)
                    : c <= 124926)
                  : (c <= 125124 || (c >= 125184 && c <= 125251)))))))))
          : (c <= 125255 || (c < 126561
            ? (c < 126535
              ? (c < 126503
                ? (c < 126469
                  ? (c < 126464
                    ? c == 125259
                    : c <= 126467)
                  : (c <= 126495 || (c < 126500
                    ? (c >= 126497 && c <= 126498)
                    : c <= 126500)))
                : (c <= 126503 || (c < 126521
                  ? (c < 126516
                    ? (c >= 126505 && c <= 126514)
                    : c <= 126519)
                  : (c <= 126521 || (c < 126530
                    ? c == 126523
                    : c <= 126530)))))
              : (c <= 126535 || (c < 126551
                ? (c < 126541
                  ? (c < 126539
                    ? c == 126537
                    : c <= 126539)
                  : (c <= 126543 || (c < 126548
                    ? (c >= 126545 && c <= 126546)
                    : c <= 126548)))
                : (c <= 126551 || (c < 126557
                  ? (c < 126555
                    ? c == 126553
                    : c <= 126555)
                  : (c <= 126557 || c == 126559))))))
            : (c <= 126562 || (c < 126635
              ? (c < 126590
                ? (c < 126572
                  ? (c < 126567
                    ? c == 126564
                    : c <= 126570)
                  : (c <= 126578 || (c < 126585
                    ? (c >= 126580 && c <= 126583)
                    : c <= 126588)))
                : (c <= 126590 || (c < 126625
                  ? (c < 126603
                    ? (c >= 126592 && c <= 126601)
                    : c <= 126619)
                  : (c <= 126627 || (c >= 126629 && c <= 126633)))))
              : (c <= 126651 || (c < 177984
                ? (c < 127344
                  ? (c < 127312
                    ? (c >= 127280 && c <= 127305)
                    : c <= 127337)
                  : (c <= 127369 || (c < 173824
                    ? (c >= 131072 && c <= 173791)
                    : c <= 177976)))
                : (c <= 178205 || (c < 194560
                  ? (c < 183984
                    ? (c >= 178208 && c <= 183969)
                    : c <= 191456)
                  : (c <= 195101 || (c >= 196608 && c <= 201546)))))))))))))))));
}

static inline bool sym_identifier_character_set_2(int32_t c) {
  return (c < 42786
    ? (c < 3544
      ? (c < 2654
        ? (c < 1984
          ? (c < 931
            ? (c < 736
              ? (c < 181
                ? (c < '_'
                  ? (c < 'A'
                    ? (c >= '0' && c <= '9')
                    : c <= 'Z')
                  : (c <= '_' || (c < 170
                    ? (c >= 'a' && c <= 'z')
                    : c <= 170)))
                : (c <= 181 || (c < 216
                  ? (c < 192
                    ? c == 186
                    : c <= 214)
                  : (c <= 246 || (c < 710
                    ? (c >= 248 && c <= 705)
                    : c <= 721)))))
              : (c <= 740 || (c < 890
                ? (c < 837
                  ? (c < 750
                    ? c == 748
                    : c <= 750)
                  : (c <= 837 || (c < 886
                    ? (c >= 880 && c <= 884)
                    : c <= 887)))
                : (c <= 893 || (c < 904
                  ? (c < 902
                    ? c == 895
                    : c <= 902)
                  : (c <= 906 || (c < 910
                    ? c == 908
                    : c <= 929)))))))
            : (c <= 1013 || (c < 1519
              ? (c < 1456
                ? (c < 1329
                  ? (c < 1162
                    ? (c >= 1015 && c <= 1153)
                    : c <= 1327)
                  : (c <= 1366 || (c < 1376
                    ? c == 1369
                    : c <= 1416)))
                : (c <= 1469 || (c < 1476
                  ? (c < 1473
                    ? c == 1471
                    : c <= 1474)
                  : (c <= 1477 || (c < 1488
                    ? c == 1479
                    : c <= 1514)))))
              : (c <= 1522 || (c < 1761
                ? (c < 1625
                  ? (c < 1568
                    ? (c >= 1552 && c <= 1562)
                    : c <= 1623)
                  : (c <= 1641 || (c < 1749
                    ? (c >= 1646 && c <= 1747)
                    : c <= 1756)))
                : (c <= 1768 || (c < 1808
                  ? (c < 1791
                    ? (c >= 1773 && c <= 1788)
                    : c <= 1791)
                  : (c <= 1855 || (c >= 1869 && c <= 1969)))))))))
          : (c <= 2026 || (c < 2486
            ? (c < 2288
              ? (c < 2144
                ? (c < 2048
                  ? (c < 2042
                    ? (c >= 2036 && c <= 2037)
                    : c <= 2042)
                  : (c <= 2071 || (c < 2112
                    ? (c >= 2074 && c <= 2092)
                    : c <= 2136)))
                : (c <= 2154 || (c < 2208
                  ? (c < 2185
                    ? (c >= 2160 && c <= 2183)
                    : c <= 2190)
                  : (c <= 2249 || (c < 2275
                    ? (c >= 2260 && c <= 2271)
                    : c <= 2281)))))
              : (c <= 2363 || (c < 2437
                ? (c < 2389
                  ? (c < 2382
                    ? (c >= 2365 && c <= 2380)
                    : c <= 2384)
                  : (c <= 2403 || (c < 2417
                    ? (c >= 2406 && c <= 2415)
                    : c <= 2435)))
                : (c <= 2444 || (c < 2474
                  ? (c < 2451
                    ? (c >= 2447 && c <= 2448)
                    : c <= 2472)
                  : (c <= 2480 || c == 2482))))))
            : (c <= 2489 || (c < 2575
              ? (c < 2524
                ? (c < 2507
                  ? (c < 2503
                    ? (c >= 2493 && c <= 2500)
                    : c <= 2504)
                  : (c <= 2508 || (c < 2519
                    ? c == 2510
                    : c <= 2519)))
                : (c <= 2525 || (c < 2556
                  ? (c < 2534
                    ? (c >= 2527 && c <= 2531)
                    : c <= 2545)
                  : (c <= 2556 || (c < 2565
                    ? (c >= 2561 && c <= 2563)
                    : c <= 2570)))))
              : (c <= 2576 || (c < 2622
                ? (c < 2610
                  ? (c < 2602
                    ? (c >= 2579 && c <= 2600)
                    : c <= 2608)
                  : (c <= 2611 || (c < 2616
                    ? (c >= 2613 && c <= 2614)
                    : c <= 2617)))
                : (c <= 2626 || (c < 2641
                  ? (c < 2635
                    ? (c >= 2631 && c <= 2632)
                    : c <= 2636)
                  : (c <= 2641 || (c >= 2649 && c <= 2652)))))))))))
        : (c <= 2654 || (c < 3072
          ? (c < 2887
            ? (c < 2768
              ? (c < 2730
                ? (c < 2693
                  ? (c < 2689
                    ? (c >= 2662 && c <= 2677)
                    : c <= 2691)
                  : (c <= 2701 || (c < 2707
                    ? (c >= 2703 && c <= 2705)
                    : c <= 2728)))
                : (c <= 2736 || (c < 2749
                  ? (c < 2741
                    ? (c >= 2738 && c <= 2739)
                    : c <= 2745)
                  : (c <= 2757 || (c < 2763
                    ? (c >= 2759 && c <= 2761)
                    : c <= 2764)))))
              : (c <= 2768 || (c < 2831
                ? (c < 2809
                  ? (c < 2790
                    ? (c >= 2784 && c <= 2787)
                    : c <= 2799)
                  : (c <= 2812 || (c < 2821
                    ? (c >= 2817 && c <= 2819)
                    : c <= 2828)))
                : (c <= 2832 || (c < 2866
                  ? (c < 2858
                    ? (c >= 2835 && c <= 2856)
                    : c <= 2864)
                  : (c <= 2867 || (c < 2877
                    ? (c >= 2869 && c <= 2873)
                    : c <= 2884)))))))
            : (c <= 2888 || (c < 2972
              ? (c < 2929
                ? (c < 2908
                  ? (c < 2902
                    ? (c >= 2891 && c <= 2892)
                    : c <= 2903)
                  : (c <= 2909 || (c < 2918
                    ? (c >= 2911 && c <= 2915)
                    : c <= 2927)))
                : (c <= 2929 || (c < 2958
                  ? (c < 2949
                    ? (c >= 2946 && c <= 2947)
                    : c <= 2954)
                  : (c <= 2960 || (c < 2969
                    ? (c >= 2962 && c <= 2965)
                    : c <= 2970)))))
              : (c <= 2972 || (c < 3014
                ? (c < 2984
                  ? (c < 2979
                    ? (c >= 2974 && c <= 2975)
                    : c <= 2980)
                  : (c <= 2986 || (c < 3006
                    ? (c >= 2990 && c <= 3001)
                    : c <= 3010)))
                : (c <= 3016 || (c < 3031
                  ? (c < 3024
                    ? (c >= 3018 && c <= 3020)
                    : c <= 3024)
                  : (c <= 3031 || (c >= 3046 && c <= 3055)))))))))
          : (c <= 3075 || (c < 3293
            ? (c < 3174
              ? (c < 3142
                ? (c < 3090
                  ? (c < 3086
                    ? (c >= 3077 && c <= 3084)
                    : c <= 3088)
                  : (c <= 3112 || (c < 3133
                    ? (c >= 3114 && c <= 3129)
                    : c <= 3140)))
                : (c <= 3144 || (c < 3160
                  ? (c < 3157
                    ? (c >= 3146 && c <= 3148)
                    : c <= 3158)
                  : (c <= 3162 || (c < 3168
                    ? c == 3165
                    : c <= 3171)))))
              : (c <= 3183 || (c < 3253
                ? (c < 3214
                  ? (c < 3205
                    ? (c >= 3200 && c <= 3203)
                    : c <= 3212)
                  : (c <= 3216 || (c < 3242
                    ? (c >= 3218 && c <= 3240)
                    : c <= 3251)))
                : (c <= 3257 || (c < 3274
                  ? (c < 3270
                    ? (c >= 3261 && c <= 3268)
                    : c <= 3272)
                  : (c <= 3276 || (c >= 3285 && c <= 3286)))))))
            : (c <= 3294 || (c < 3423
              ? (c < 3346
                ? (c < 3313
                  ? (c < 3302
                    ? (c >= 3296 && c <= 3299)
                    : c <= 3311)
                  : (c <= 3314 || (c < 3342
                    ? (c >= 3328 && c <= 3340)
                    : c <= 3344)))
                : (c <= 3386 || (c < 3402
                  ? (c < 3398
                    ? (c >= 3389 && c <= 3396)
                    : c <= 3400)
                  : (c <= 3404 || (c < 3412
                    ? c == 3406
                    : c <= 3415)))))
              : (c <= 3427 || (c < 3507
                ? (c < 3457
                  ? (c < 3450
                    ? (c >= 3430 && c <= 3439)
                    : c <= 3455)
                  : (c <= 3459 || (c < 3482
                    ? (c >= 3461 && c <= 3478)
                    : c <= 3505)))
                : (c <= 3515 || (c < 3535
                  ? (c < 3520
                    ? c == 3517
                    : c <= 3526)
                  : (c <= 3540 || c == 3542))))))))))))
      : (c <= 3551 || (c < 7040
        ? (c < 4824
          ? (c < 3976
            ? (c < 3751
              ? (c < 3664
                ? (c < 3585
                  ? (c < 3570
                    ? (c >= 3558 && c <= 3567)
                    : c <= 3571)
                  : (c <= 3642 || (c < 3661
                    ? (c >= 3648 && c <= 3654)
                    : c <= 3661)))
                : (c <= 3673 || (c < 3718
                  ? (c < 3716
                    ? (c >= 3713 && c <= 3714)
                    : c <= 3716)
                  : (c <= 3722 || (c < 3749
                    ? (c >= 3724 && c <= 3747)
                    : c <= 3749)))))
              : (c <= 3769 || (c < 3804
                ? (c < 3782
                  ? (c < 3776
                    ? (c >= 3771 && c <= 3773)
                    : c <= 3780)
                  : (c <= 3782 || (c < 3792
                    ? c == 3789
                    : c <= 3801)))
                : (c <= 3807 || (c < 3904
                  ? (c < 3872
                    ? c == 3840
                    : c <= 3881)
                  : (c <= 3911 || (c < 3953
                    ? (c >= 3913 && c <= 3948)
                    : c <= 3969)))))))
            : (c <= 3991 || (c < 4688
              ? (c < 4256
                ? (c < 4152
                  ? (c < 4096
                    ? (c >= 3993 && c <= 4028)
                    : c <= 4150)
                  : (c <= 4152 || (c < 4176
                    ? (c >= 4155 && c <= 4169)
                    : c <= 4253)))
                : (c <= 4293 || (c < 4304
                  ? (c < 4301
                    ? c == 4295
                    : c <= 4301)
                  : (c <= 4346 || (c < 4682
                    ? (c >= 4348 && c <= 4680)
                    : c <= 4685)))))
              : (c <= 4694 || (c < 4786
                ? (c < 4704
                  ? (c < 4698
                    ? c == 4696
                    : c <= 4701)
                  : (c <= 4744 || (c < 4752
                    ? (c >= 4746 && c <= 4749)
                    : c <= 4784)))
                : (c <= 4789 || (c < 4802
                  ? (c < 4800
                    ? (c >= 4792 && c <= 4798)
                    : c <= 4800)
                  : (c <= 4805 || (c >= 4808 && c <= 4822)))))))))
          : (c <= 4880 || (c < 6176
            ? (c < 5919
              ? (c < 5121
                ? (c < 4992
                  ? (c < 4888
                    ? (c >= 4882 && c <= 4885)
                    : c <= 4954)
                  : (c <= 5007 || (c < 5112
                    ? (c >= 5024 && c <= 5109)
                    : c <= 5117)))
                : (c <= 5740 || (c < 5792
                  ? (c < 5761
                    ? (c >= 5743 && c <= 5759)
                    : c <= 5786)
                  : (c <= 5866 || (c < 5888
                    ? (c >= 5870 && c <= 5880)
                    : c <= 5907)))))
              : (c <= 5939 || (c < 6070
                ? (c < 5998
                  ? (c < 5984
                    ? (c >= 5952 && c <= 5971)
                    : c <= 5996)
                  : (c <= 6000 || (c < 6016
                    ? (c >= 6002 && c <= 6003)
                    : c <= 6067)))
                : (c <= 6088 || (c < 6112
                  ? (c < 6108
                    ? c == 6103
                    : c <= 6108)
                  : (c <= 6121 || (c >= 6160 && c <= 6169)))))))
            : (c <= 6264 || (c < 6688
              ? (c < 6470
                ? (c < 6400
                  ? (c < 6320
                    ? (c >= 6272 && c <= 6314)
                    : c <= 6389)
                  : (c <= 6430 || (c < 6448
                    ? (c >= 6432 && c <= 6443)
                    : c <= 6456)))
                : (c <= 6509 || (c < 6576
                  ? (c < 6528
                    ? (c >= 6512 && c <= 6516)
                    : c <= 6571)
                  : (c <= 6601 || (c < 6656
                    ? (c >= 6608 && c <= 6617)
                    : c <= 6683)))))
              : (c <= 6750 || (c < 6860
                ? (c < 6800
                  ? (c < 6784
                    ? (c >= 6753 && c <= 6772)
                    : c <= 6793)
                  : (c <= 6809 || (c < 6847
                    ? c == 6823
                    : c <= 6848)))
                : (c <= 6862 || (c < 6981
                  ? (c < 6965
                    ? (c >= 6912 && c <= 6963)
                    : c <= 6979)
                  : (c <= 6988 || (c >= 6992 && c <= 7001)))))))))))
        : (c <= 7081 || (c < 8495
          ? (c < 8031
            ? (c < 7418
              ? (c < 7296
                ? (c < 7168
                  ? (c < 7143
                    ? (c >= 7084 && c <= 7141)
                    : c <= 7153)
                  : (c <= 7222 || (c < 7245
                    ? (c >= 7232 && c <= 7241)
                    : c <= 7293)))
                : (c <= 7304 || (c < 7401
                  ? (c < 7357
                    ? (c >= 7312 && c <= 7354)
                    : c <= 7359)
                  : (c <= 7404 || (c < 7413
                    ? (c >= 7406 && c <= 7411)
                    : c <= 7414)))))
              : (c <= 7418 || (c < 8008
                ? (c < 7680
                  ? (c < 7655
                    ? (c >= 7424 && c <= 7615)
                    : c <= 7668)
                  : (c <= 7957 || (c < 7968
                    ? (c >= 7960 && c <= 7965)
                    : c <= 8005)))
                : (c <= 8013 || (c < 8027
                  ? (c < 8025
                    ? (c >= 8016 && c <= 8023)
                    : c <= 8025)
                  : (c <= 8027 || c == 8029))))))
            : (c <= 8061 || (c < 8319
              ? (c < 8144
                ? (c < 8126
                  ? (c < 8118
                    ? (c >= 8064 && c <= 8116)
                    : c <= 8124)
                  : (c <= 8126 || (c < 8134
                    ? (c >= 8130 && c <= 8132)
                    : c <= 8140)))
                : (c <= 8147 || (c < 8178
                  ? (c < 8160
                    ? (c >= 8150 && c <= 8155)
                    : c <= 8172)
                  : (c <= 8180 || (c < 8305
                    ? (c >= 8182 && c <= 8188)
                    : c <= 8305)))))
              : (c <= 8319 || (c < 8473
                ? (c < 8455
                  ? (c < 8450
                    ? (c >= 8336 && c <= 8348)
                    : c <= 8450)
                  : (c <= 8455 || (c < 8469
                    ? (c >= 8458 && c <= 8467)
                    : c <= 8469)))
                : (c <= 8477 || (c < 8488
                  ? (c < 8486
                    ? c == 8484
                    : c <= 8486)
                  : (c <= 8488 || (c >= 8490 && c <= 8493)))))))))
          : (c <= 8505 || (c < 11744
            ? (c < 11568
              ? (c < 11264
                ? (c < 8526
                  ? (c < 8517
                    ? (c >= 8508 && c <= 8511)
                    : c <= 8521)
                  : (c <= 8526 || (c < 9398
                    ? (c >= 8544 && c <= 8584)
                    : c <= 9449)))
                : (c <= 11492 || (c < 11520
                  ? (c < 11506
                    ? (c >= 11499 && c <= 11502)
                    : c <= 11507)
                  : (c <= 11557 || (c < 11565
                    ? c == 11559
                    : c <= 11565)))))
              : (c <= 11623 || (c < 11704
                ? (c < 11680
                  ? (c < 11648
                    ? c == 11631
                    : c <= 11670)
                  : (c <= 11686 || (c < 11696
                    ? (c >= 11688 && c <= 11694)
                    : c <= 11702)))
                : (c <= 11710 || (c < 11728
                  ? (c < 11720
                    ? (c >= 11712 && c <= 11718)
                    : c <= 11726)
                  : (c <= 11734 || (c >= 11736 && c <= 11742)))))))
            : (c <= 11775 || (c < 12704
              ? (c < 12353
                ? (c < 12321
                  ? (c < 12293
                    ? c == 11823
                    : c <= 12295)
                  : (c <= 12329 || (c < 12344
                    ? (c >= 12337 && c <= 12341)
                    : c <= 12348)))
                : (c <= 12438 || (c < 12540
                  ? (c < 12449
                    ? (c >= 12445 && c <= 12447)
                    : c <= 12538)
                  : (c <= 12543 || (c < 12593
                    ? (c >= 12549 && c <= 12591)
                    : c <= 12686)))))
              : (c <= 12735 || (c < 42512
                ? (c < 19968
                  ? (c < 13312
                    ? (c >= 12784 && c <= 12799)
                    : c <= 19903)
                  : (c <= 42124 || (c < 42240
                    ? (c >= 42192 && c <= 42237)
                    : c <= 42508)))
                : (c <= 42539 || (c < 42623
                  ? (c < 42612
                    ? (c >= 42560 && c <= 42606)
                    : c <= 42619)
                  : (c <= 42735 || (c >= 42775 && c <= 42783)))))))))))))))
    : (c <= 42888 || (c < 70784
      ? (c < 66964
        ? (c < 64298
          ? (c < 43642
            ? (c < 43259
              ? (c < 43015
                ? (c < 42963
                  ? (c < 42960
                    ? (c >= 42891 && c <= 42954)
                    : c <= 42961)
                  : (c <= 42963 || (c < 42994
                    ? (c >= 42965 && c <= 42969)
                    : c <= 43013)))
                : (c <= 43047 || (c < 43205
                  ? (c < 43136
                    ? (c >= 43072 && c <= 43123)
                    : c <= 43203)
                  : (c <= 43205 || (c < 43250
                    ? (c >= 43216 && c <= 43225)
                    : c <= 43255)))))
              : (c <= 43259 || (c < 43471
                ? (c < 43360
                  ? (c < 43312
                    ? (c >= 43261 && c <= 43306)
                    : c <= 43346)
                  : (c <= 43388 || (c < 43444
                    ? (c >= 43392 && c <= 43442)
                    : c <= 43455)))
                : (c <= 43481 || (c < 43584
                  ? (c < 43520
                    ? (c >= 43488 && c <= 43518)
                    : c <= 43574)
                  : (c <= 43597 || (c < 43616
                    ? (c >= 43600 && c <= 43609)
                    : c <= 43638)))))))
            : (c <= 43710 || (c < 43868
              ? (c < 43777
                ? (c < 43739
                  ? (c < 43714
                    ? c == 43712
                    : c <= 43714)
                  : (c <= 43741 || (c < 43762
                    ? (c >= 43744 && c <= 43759)
                    : c <= 43765)))
                : (c <= 43782 || (c < 43808
                  ? (c < 43793
                    ? (c >= 43785 && c <= 43790)
                    : c <= 43798)
                  : (c <= 43814 || (c < 43824
                    ? (c >= 43816 && c <= 43822)
                    : c <= 43866)))))
              : (c <= 43881 || (c < 63744
                ? (c < 44032
                  ? (c < 44016
                    ? (c >= 43888 && c <= 44010)
                    : c <= 44025)
                  : (c <= 55203 || (c < 55243
                    ? (c >= 55216 && c <= 55238)
                    : c <= 55291)))
                : (c <= 64109 || (c < 64275
                  ? (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)
                  : (c <= 64279 || (c >= 64285 && c <= 64296)))))))))
          : (c <= 64310 || (c < 65596
            ? (c < 65296
              ? (c < 64467
                ? (c < 64320
                  ? (c < 64318
                    ? (c >= 64312 && c <= 64316)
                    : c <= 64318)
                  : (c <= 64321 || (c < 64326
                    ? (c >= 64323 && c <= 64324)
                    : c <= 64433)))
                : (c <= 64829 || (c < 65008
                  ? (c < 64914
                    ? (c >= 64848 && c <= 64911)
                    : c <= 64967)
                  : (c <= 65019 || (c < 65142
                    ? (c >= 65136 && c <= 65140)
                    : c <= 65276)))))
              : (c <= 65305 || (c < 65490
                ? (c < 65382
                  ? (c < 65345
                    ? (c >= 65313 && c <= 65338)
                    : c <= 65370)
                  : (c <= 65470 || (c < 65482
                    ? (c >= 65474 && c <= 65479)
                    : c <= 65487)))
                : (c <= 65495 || (c < 65549
                  ? (c < 65536
                    ? (c >= 65498 && c <= 65500)
                    : c <= 65547)
                  : (c <= 65574 || (c >= 65576 && c <= 65594)))))))
            : (c <= 65597 || (c < 66504
              ? (c < 66208
                ? (c < 65664
                  ? (c < 65616
                    ? (c >= 65599 && c <= 65613)
                    : c <= 65629)
                  : (c <= 65786 || (c < 66176
                    ? (c >= 65856 && c <= 65908)
                    : c <= 66204)))
                : (c <= 66256 || (c < 66384
                  ? (c < 66349
                    ? (c >= 66304 && c <= 66335)
                    : c <= 66378)
                  : (c <= 66426 || (c < 66464
                    ? (c >= 66432 && c <= 66461)
                    : c <= 66499)))))
              : (c <= 66511 || (c < 66816
                ? (c < 66720
                  ? (c < 66560
                    ? (c >= 66513 && c <= 66517)
                    : c <= 66717)
                  : (c <= 66729 || (c < 66776
                    ? (c >= 66736 && c <= 66771)
                    : c <= 66811)))
                : (c <= 66855 || (c < 66940
                  ? (c < 66928
                    ? (c >= 66864 && c <= 66915)
                    : c <= 66938)
                  : (c <= 66954 || (c >= 66956 && c <= 66962)))))))))))
        : (c <= 66965 || (c < 69415
          ? (c < 67968
            ? (c < 67592
              ? (c < 67392
                ? (c < 66995
                  ? (c < 66979
                    ? (c >= 66967 && c <= 66977)
                    : c <= 66993)
                  : (c <= 67001 || (c < 67072
                    ? (c >= 67003 && c <= 67004)
                    : c <= 67382)))
                : (c <= 67413 || (c < 67463
                  ? (c < 67456
                    ? (c >= 67424 && c <= 67431)
                    : c <= 67461)
                  : (c <= 67504 || (c < 67584
                    ? (c >= 67506 && c <= 67514)
                    : c <= 67589)))))
              : (c <= 67592 || (c < 67712
                ? (c < 67644
                  ? (c < 67639
                    ? (c >= 67594 && c <= 67637)
                    : c <= 67640)
                  : (c <= 67644 || (c < 67680
                    ? (c >= 67647 && c <= 67669)
                    : c <= 67702)))
                : (c <= 67742 || (c < 67840
                  ? (c < 67828
                    ? (c >= 67808 && c <= 67826)
                    : c <= 67829)
                  : (c <= 67861 || (c >= 67872 && c <= 67897)))))))
            : (c <= 68023 || (c < 68416
              ? (c < 68121
                ? (c < 68101
                  ? (c < 68096
                    ? (c >= 68030 && c <= 68031)
                    : c <= 68099)
                  : (c <= 68102 || (c < 68117
                    ? (c >= 68108 && c <= 68115)
                    : c <= 68119)))
                : (c <= 68149 || (c < 68288
                  ? (c < 68224
                    ? (c >= 68192 && c <= 68220)
                    : c <= 68252)
                  : (c <= 68295 || (c < 68352
                    ? (c >= 68297 && c <= 68324)
                    : c <= 68405)))))
              : (c <= 68437 || (c < 68864
                ? (c < 68608
                  ? (c < 68480
                    ? (c >= 68448 && c <= 68466)
                    : c <= 68497)
                  : (c <= 68680 || (c < 68800
                    ? (c >= 68736 && c <= 68786)
                    : c <= 68850)))
                : (c <= 68903 || (c < 69296
                  ? (c < 69291
                    ? (c >= 69248 && c <= 69289)
                    : c <= 69292)
                  : (c <= 69297 || (c >= 69376 && c <= 69404)))))))))
          : (c <= 69415 || (c < 70272
            ? (c < 69968
              ? (c < 69745
                ? (c < 69552
                  ? (c < 69488
                    ? (c >= 69424 && c <= 69445)
                    : c <= 69505)
                  : (c <= 69572 || (c < 69632
                    ? (c >= 69600 && c <= 69622)
                    : c <= 69701)))
                : (c <= 69749 || (c < 69840
                  ? (c < 69826
                    ? (c >= 69762 && c <= 69816)
                    : c <= 69826)
                  : (c <= 69864 || (c < 69956
                    ? (c >= 69888 && c <= 69938)
                    : c <= 69959)))))
              : (c <= 70002 || (c < 70108
                ? (c < 70081
                  ? (c < 70016
                    ? c == 70006
                    : c <= 70079)
                  : (c <= 70084 || (c < 70106
                    ? (c >= 70094 && c <= 70095)
                    : c <= 70106)))
                : (c <= 70108 || (c < 70199
                  ? (c < 70163
                    ? (c >= 70144 && c <= 70161)
                    : c <= 70196)
                  : (c <= 70199 || c == 70206))))))
            : (c <= 70278 || (c < 70453
              ? (c < 70400
                ? (c < 70287
                  ? (c < 70282
                    ? c == 70280
                    : c <= 70285)
                  : (c <= 70301 || (c < 70320
                    ? (c >= 70303 && c <= 70312)
                    : c <= 70376)))
                : (c <= 70403 || (c < 70419
                  ? (c < 70415
                    ? (c >= 70405 && c <= 70412)
                    : c <= 70416)
                  : (c <= 70440 || (c < 70450
                    ? (c >= 70442 && c <= 70448)
                    : c <= 70451)))))
              : (c <= 70457 || (c < 70493
                ? (c < 70475
                  ? (c < 70471
                    ? (c >= 70461 && c <= 70468)
                    : c <= 70472)
                  : (c <= 70476 || (c < 70487
                    ? c == 70480
                    : c <= 70487)))
                : (c <= 70499 || (c < 70727
                  ? (c < 70723
                    ? (c >= 70656 && c <= 70721)
                    : c <= 70725)
                  : (c <= 70730 || (c >= 70751 && c <= 70753)))))))))))))
      : (c <= 70849 || (c < 119808
        ? (c < 73027
          ? (c < 72096
            ? (c < 71453
              ? (c < 71168
                ? (c < 71040
                  ? (c < 70855
                    ? (c >= 70852 && c <= 70853)
                    : c <= 70855)
                  : (c <= 71093 || (c < 71128
                    ? (c >= 71096 && c <= 71102)
                    : c <= 71133)))
                : (c <= 71230 || (c < 71296
                  ? (c < 71236
                    ? c == 71232
                    : c <= 71236)
                  : (c <= 71349 || (c < 71424
                    ? c == 71352
                    : c <= 71450)))))
              : (c <= 71466 || (c < 71948
                ? (c < 71840
                  ? (c < 71680
                    ? (c >= 71488 && c <= 71494)
                    : c <= 71736)
                  : (c <= 71903 || (c < 71945
                    ? (c >= 71935 && c <= 71942)
                    : c <= 71945)))
                : (c <= 71955 || (c < 71991
                  ? (c < 71960
                    ? (c >= 71957 && c <= 71958)
                    : c <= 71989)
                  : (c <= 71992 || (c < 71999
                    ? (c >= 71995 && c <= 71996)
                    : c <= 72002)))))))
            : (c <= 72103 || (c < 72760
              ? (c < 72245
                ? (c < 72161
                  ? (c < 72154
                    ? (c >= 72106 && c <= 72151)
                    : c <= 72159)
                  : (c <= 72161 || (c < 72192
                    ? (c >= 72163 && c <= 72164)
                    : c <= 72242)))
                : (c <= 72254 || (c < 72368
                  ? (c < 72349
                    ? (c >= 72272 && c <= 72343)
                    : c <= 72349)
                  : (c <= 72440 || (c < 72714
                    ? (c >= 72704 && c <= 72712)
                    : c <= 72758)))))
              : (c <= 72766 || (c < 72968
                ? (c < 72850
                  ? (c < 72818
                    ? c == 72768
                    : c <= 72847)
                  : (c <= 72871 || (c < 72960
                    ? (c >= 72873 && c <= 72886)
                    : c <= 72966)))
                : (c <= 72969 || (c < 73020
                  ? (c < 73018
                    ? (c >= 72971 && c <= 73014)
                    : c <= 73018)
                  : (c <= 73021 || (c >= 73023 && c <= 73025)))))))))
          : (c <= 73027 || (c < 93053
            ? (c < 74880
              ? (c < 73107
                ? (c < 73063
                  ? (c < 73056
                    ? (c >= 73030 && c <= 73031)
                    : c <= 73061)
                  : (c <= 73064 || (c < 73104
                    ? (c >= 73066 && c <= 73102)
                    : c <= 73105)))
                : (c <= 73110 || (c < 73648
                  ? (c < 73440
                    ? c == 73112
                    : c <= 73462)
                  : (c <= 73648 || (c < 74752
                    ? (c >= 73728 && c <= 74649)
                    : c <= 74862)))))
              : (c <= 75075 || (c < 92784
                ? (c < 82944
                  ? (c < 77824
                    ? (c >= 77712 && c <= 77808)
                    : c <= 78894)
                  : (c <= 83526 || (c < 92736
                    ? (c >= 92160 && c <= 92728)
                    : c <= 92766)))
                : (c <= 92862 || (c < 92992
                  ? (c < 92928
                    ? (c >= 92880 && c <= 92909)
                    : c <= 92975)
                  : (c <= 92995 || (c >= 93027 && c <= 93047)))))))
            : (c <= 93071 || (c < 110581
              ? (c < 94179
                ? (c < 94031
                  ? (c < 93952
                    ? (c >= 93760 && c <= 93823)
                    : c <= 94026)
                  : (c <= 94087 || (c < 94176
                    ? (c >= 94095 && c <= 94111)
                    : c <= 94177)))
                : (c <= 94179 || (c < 100352
                  ? (c < 94208
                    ? (c >= 94192 && c <= 94193)
                    : c <= 100343)
                  : (c <= 101589 || (c < 110576
                    ? (c >= 101632 && c <= 101640)
                    : c <= 110579)))))
              : (c <= 110587 || (c < 113664
                ? (c < 110928
                  ? (c < 110592
                    ? (c >= 110589 && c <= 110590)
                    : c <= 110882)
                  : (c <= 110930 || (c < 110960
                    ? (c >= 110948 && c <= 110951)
                    : c <= 111355)))
                : (c <= 113770 || (c < 113808
                  ? (c < 113792
                    ? (c >= 113776 && c <= 113788)
                    : c <= 113800)
                  : (c <= 113817 || c == 113822))))))))))
        : (c <= 119892 || (c < 125184
          ? (c < 120598
            ? (c < 120086
              ? (c < 119982
                ? (c < 119970
                  ? (c < 119966
                    ? (c >= 119894 && c <= 119964)
                    : c <= 119967)
                  : (c <= 119970 || (c < 119977
                    ? (c >= 119973 && c <= 119974)
                    : c <= 119980)))
                : (c <= 119993 || (c < 120005
                  ? (c < 119997
                    ? c == 119995
                    : c <= 120003)
                  : (c <= 120069 || (c < 120077
                    ? (c >= 120071 && c <= 120074)
                    : c <= 120084)))))
              : (c <= 120092 || (c < 120146
                ? (c < 120128
                  ? (c < 120123
                    ? (c >= 120094 && c <= 120121)
                    : c <= 120126)
                  : (c <= 120132 || (c < 120138
                    ? c == 120134
                    : c <= 120144)))
                : (c <= 120485 || (c < 120540
                  ? (c < 120514
                    ? (c >= 120488 && c <= 120512)
                    : c <= 120538)
                  : (c <= 120570 || (c >= 120572 && c <= 120596)))))))
            : (c <= 120628 || (c < 122918
              ? (c < 120772
                ? (c < 120688
                  ? (c < 120656
                    ? (c >= 120630 && c <= 120654)
                    : c <= 120686)
                  : (c <= 120712 || (c < 120746
                    ? (c >= 120714 && c <= 120744)
                    : c <= 120770)))
                : (c <= 120779 || (c < 122888
                  ? (c < 122880
                    ? (c >= 122624 && c <= 122654)
                    : c <= 122886)
                  : (c <= 122904 || (c < 122915
                    ? (c >= 122907 && c <= 122913)
                    : c <= 122916)))))
              : (c <= 122922 || (c < 124896
                ? (c < 123214
                  ? (c < 123191
                    ? (c >= 123136 && c <= 123180)
                    : c <= 123197)
                  : (c <= 123214 || (c < 123584
                    ? (c >= 123536 && c <= 123565)
                    : c <= 123627)))
                : (c <= 124902 || (c < 124912
                  ? (c < 124909
                    ? (c >= 124904 && c <= 124907)
                    : c <= 124910)
                  : (c <= 124926 || (c >= 124928 && c <= 125124)))))))))
          : (c <= 125251 || (c < 126559
            ? (c < 126530
              ? (c < 126500
                ? (c < 126464
                  ? (c < 125259
                    ? c == 125255
                    : c <= 125259)
                  : (c <= 126467 || (c < 126497
                    ? (c >= 126469 && c <= 126495)
                    : c <= 126498)))
                : (c <= 126500 || (c < 126516
                  ? (c < 126505
                    ? c == 126503
                    : c <= 126514)
                  : (c <= 126519 || (c < 126523
                    ? c == 126521
                    : c <= 126523)))))
              : (c <= 126530 || (c < 126548
                ? (c < 126539
                  ? (c < 126537
                    ? c == 126535
                    : c <= 126537)
                  : (c <= 126539 || (c < 126545
                    ? (c >= 126541 && c <= 126543)
                    : c <= 126546)))
                : (c <= 126548 || (c < 126555
                  ? (c < 126553
                    ? c == 126551
                    : c <= 126553)
                  : (c <= 126555 || c == 126557))))))
            : (c <= 126559 || (c < 126635
              ? (c < 126585
                ? (c < 126567
                  ? (c < 126564
                    ? (c >= 126561 && c <= 126562)
                    : c <= 126564)
                  : (c <= 126570 || (c < 126580
                    ? (c >= 126572 && c <= 126578)
                    : c <= 126583)))
                : (c <= 126588 || (c < 126603
                  ? (c < 126592
                    ? c == 126590
                    : c <= 126601)
                  : (c <= 126619 || (c < 126629
                    ? (c >= 126625 && c <= 126627)
                    : c <= 126633)))))
              : (c <= 126651 || (c < 177984
                ? (c < 127344
                  ? (c < 127312
                    ? (c >= 127280 && c <= 127305)
                    : c <= 127337)
                  : (c <= 127369 || (c < 173824
                    ? (c >= 131072 && c <= 173791)
                    : c <= 177976)))
                : (c <= 178205 || (c < 194560
                  ? (c < 183984
                    ? (c >= 178208 && c <= 183969)
                    : c <= 191456)
                  : (c <= 195101 || (c >= 196608 && c <= 201546)))))))))))))))));
}

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(9);
      if (lookahead == '\n') ADVANCE(43);
      if (lookahead == '!') ADVANCE(23);
      if (lookahead == '#') ADVANCE(3);
      if (lookahead == '%') ADVANCE(34);
      if (lookahead == '&') ADVANCE(25);
      if (lookahead == '\'') ADVANCE(18);
      if (lookahead == '(') ADVANCE(36);
      if (lookahead == ')') ADVANCE(11);
      if (lookahead == '*') ADVANCE(21);
      if (lookahead == '+') ADVANCE(19);
      if (lookahead == ',') ADVANCE(42);
      if (lookahead == '-') ADVANCE(20);
      if (lookahead == '.') ADVANCE(35);
      if (lookahead == '/') ADVANCE(33);
      if (lookahead == ':') ADVANCE(16);
      if (lookahead == '<') ADVANCE(29);
      if (lookahead == '=') ADVANCE(14);
      if (lookahead == '>') ADVANCE(31);
      if (lookahead == '[') ADVANCE(37);
      if (lookahead == ']') ADVANCE(38);
      if (lookahead == '^') ADVANCE(26);
      if (lookahead == '{') ADVANCE(12);
      if (lookahead == '|') ADVANCE(24);
      if (lookahead == '}') ADVANCE(13);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(41);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(40);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(43);
      if (lookahead == '!') ADVANCE(22);
      if (lookahead == '#') ADVANCE(3);
      if (lookahead == '&') ADVANCE(25);
      if (lookahead == '(') ADVANCE(36);
      if (lookahead == ')') ADVANCE(11);
      if (lookahead == '*') ADVANCE(21);
      if (lookahead == '+') ADVANCE(19);
      if (lookahead == ',') ADVANCE(42);
      if (lookahead == '-') ADVANCE(20);
      if (lookahead == '/') ADVANCE(4);
      if (lookahead == ':') ADVANCE(7);
      if (lookahead == '[') ADVANCE(37);
      if (lookahead == '^') ADVANCE(26);
      if (lookahead == '{') ADVANCE(12);
      if (lookahead == '|') ADVANCE(24);
      if (lookahead == '}') ADVANCE(13);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(1)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(41);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(40);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(43);
      if (lookahead == '!') ADVANCE(8);
      if (lookahead == '#') ADVANCE(3);
      if (lookahead == '%') ADVANCE(34);
      if (lookahead == '&') ADVANCE(25);
      if (lookahead == '(') ADVANCE(36);
      if (lookahead == ')') ADVANCE(11);
      if (lookahead == '*') ADVANCE(21);
      if (lookahead == '+') ADVANCE(19);
      if (lookahead == ',') ADVANCE(42);
      if (lookahead == '-') ADVANCE(20);
      if (lookahead == '.') ADVANCE(35);
      if (lookahead == '/') ADVANCE(33);
      if (lookahead == ':') ADVANCE(7);
      if (lookahead == '<') ADVANCE(29);
      if (lookahead == '=') ADVANCE(14);
      if (lookahead == '>') ADVANCE(31);
      if (lookahead == '[') ADVANCE(37);
      if (lookahead == ']') ADVANCE(38);
      if (lookahead == '^') ADVANCE(26);
      if (lookahead == '{') ADVANCE(12);
      if (lookahead == '|') ADVANCE(24);
      if (lookahead == '}') ADVANCE(13);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(2)
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(40);
      END_STATE();
    case 3:
      if (lookahead == '(') ADVANCE(10);
      END_STATE();
    case 4:
      if (lookahead == '*') ADVANCE(6);
      if (lookahead == '/') ADVANCE(44);
      END_STATE();
    case 5:
      if (lookahead == '*') ADVANCE(5);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead != 0) ADVANCE(6);
      END_STATE();
    case 6:
      if (lookahead == '*') ADVANCE(5);
      if (lookahead != 0) ADVANCE(6);
      END_STATE();
    case 7:
      if (lookahead == ':') ADVANCE(39);
      END_STATE();
    case 8:
      if (lookahead == '=') ADVANCE(28);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_POUND_LPAREN);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead == '=') ADVANCE(27);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_DOT_DOT);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '>') ADVANCE(17);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_BANG);
      if (lookahead == '=') ADVANCE(28);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_AMP);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(30);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(32);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_GT_EQ);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '*') ADVANCE(6);
      if (lookahead == '/') ADVANCE(44);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_DOT);
      if (lookahead == '.') ADVANCE(15);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(anon_sym_COLON_COLON);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(40);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(41);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(anon_sym_LF);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(sym_single_line_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(44);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(sym_multi_line_comment);
      END_STATE();
    default:
      return false;
  }
}

static bool ts_lex_keywords(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (lookahead == '_') ADVANCE(1);
      if (lookahead == 'd') ADVANCE(2);
      if (lookahead == 'e') ADVANCE(3);
      if (lookahead == 'f') ADVANCE(4);
      if (lookahead == 'g') ADVANCE(5);
      if (lookahead == 'i') ADVANCE(6);
      if (lookahead == 'm') ADVANCE(7);
      if (lookahead == 'o') ADVANCE(8);
      if (lookahead == 'r') ADVANCE(9);
      if (lookahead == 's') ADVANCE(10);
      if (lookahead == 't') ADVANCE(11);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == '_') ADVANCE(12);
      END_STATE();
    case 2:
      if (lookahead == 'o') ADVANCE(13);
      END_STATE();
    case 3:
      if (lookahead == 'l') ADVANCE(14);
      if (lookahead == 'x') ADVANCE(15);
      END_STATE();
    case 4:
      if (lookahead == 'o') ADVANCE(16);
      if (lookahead == 'u') ADVANCE(17);
      END_STATE();
    case 5:
      if (lookahead == 'e') ADVANCE(18);
      END_STATE();
    case 6:
      if (lookahead == 'f') ADVANCE(19);
      if (lookahead == 'n') ADVANCE(20);
      END_STATE();
    case 7:
      if (lookahead == 'o') ADVANCE(21);
      END_STATE();
    case 8:
      if (lookahead == 'u') ADVANCE(22);
      END_STATE();
    case 9:
      if (lookahead == 'e') ADVANCE(23);
      END_STATE();
    case 10:
      if (lookahead == 't') ADVANCE(24);
      END_STATE();
    case 11:
      if (lookahead == 'y') ADVANCE(25);
      END_STATE();
    case 12:
      if (lookahead == 'b') ADVANCE(26);
      END_STATE();
    case 13:
      if (lookahead == 'm') ADVANCE(27);
      END_STATE();
    case 14:
      if (lookahead == 's') ADVANCE(28);
      END_STATE();
    case 15:
      if (lookahead == 't') ADVANCE(29);
      END_STATE();
    case 16:
      if (lookahead == 'r') ADVANCE(30);
      END_STATE();
    case 17:
      if (lookahead == 'n') ADVANCE(31);
      END_STATE();
    case 18:
      if (lookahead == 'n') ADVANCE(32);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_in);
      if (lookahead == 'i') ADVANCE(33);
      if (lookahead == 'p') ADVANCE(34);
      if (lookahead == 't') ADVANCE(35);
      END_STATE();
    case 21:
      if (lookahead == 'd') ADVANCE(36);
      END_STATE();
    case 22:
      if (lookahead == 't') ADVANCE(37);
      END_STATE();
    case 23:
      if (lookahead == 'g') ADVANCE(38);
      END_STATE();
    case 24:
      if (lookahead == 'a') ADVANCE(39);
      if (lookahead == 'r') ADVANCE(40);
      END_STATE();
    case 25:
      if (lookahead == 'p') ADVANCE(41);
      END_STATE();
    case 26:
      if (lookahead == 'u') ADVANCE(42);
      END_STATE();
    case 27:
      if (lookahead == 'a') ADVANCE(43);
      END_STATE();
    case 28:
      if (lookahead == 'e') ADVANCE(44);
      END_STATE();
    case 29:
      if (lookahead == 'e') ADVANCE(45);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(anon_sym_for);
      END_STATE();
    case 31:
      if (lookahead == 'c') ADVANCE(46);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_gen);
      END_STATE();
    case 33:
      if (lookahead == 't') ADVANCE(47);
      END_STATE();
    case 34:
      if (lookahead == 'u') ADVANCE(48);
      END_STATE();
    case 35:
      if (lookahead == 'e') ADVANCE(49);
      END_STATE();
    case 36:
      if (lookahead == 'u') ADVANCE(50);
      END_STATE();
    case 37:
      if (lookahead == 'p') ADVANCE(51);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_reg);
      END_STATE();
    case 39:
      if (lookahead == 't') ADVANCE(52);
      END_STATE();
    case 40:
      if (lookahead == 'u') ADVANCE(53);
      END_STATE();
    case 41:
      if (lookahead == 'e') ADVANCE(54);
      END_STATE();
    case 42:
      if (lookahead == 'i') ADVANCE(55);
      END_STATE();
    case 43:
      if (lookahead == 'i') ADVANCE(56);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 45:
      if (lookahead == 'r') ADVANCE(57);
      END_STATE();
    case 46:
      if (lookahead == 't') ADVANCE(58);
      END_STATE();
    case 47:
      if (lookahead == 'i') ADVANCE(59);
      END_STATE();
    case 48:
      if (lookahead == 't') ADVANCE(60);
      END_STATE();
    case 49:
      if (lookahead == 'r') ADVANCE(61);
      END_STATE();
    case 50:
      if (lookahead == 'l') ADVANCE(62);
      END_STATE();
    case 51:
      if (lookahead == 'u') ADVANCE(63);
      END_STATE();
    case 52:
      if (lookahead == 'e') ADVANCE(64);
      END_STATE();
    case 53:
      if (lookahead == 'c') ADVANCE(65);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(anon_sym_type);
      END_STATE();
    case 55:
      if (lookahead == 'l') ADVANCE(66);
      END_STATE();
    case 56:
      if (lookahead == 'n') ADVANCE(67);
      END_STATE();
    case 57:
      if (lookahead == 'n') ADVANCE(68);
      END_STATE();
    case 58:
      if (lookahead == 'i') ADVANCE(69);
      END_STATE();
    case 59:
      if (lookahead == 'a') ADVANCE(70);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(anon_sym_input);
      END_STATE();
    case 61:
      if (lookahead == 'f') ADVANCE(71);
      END_STATE();
    case 62:
      if (lookahead == 'e') ADVANCE(72);
      END_STATE();
    case 63:
      if (lookahead == 't') ADVANCE(73);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(anon_sym_state);
      END_STATE();
    case 65:
      if (lookahead == 't') ADVANCE(74);
      END_STATE();
    case 66:
      if (lookahead == 't') ADVANCE(75);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(anon_sym_domain);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(anon_sym_extern);
      END_STATE();
    case 69:
      if (lookahead == 'o') ADVANCE(76);
      END_STATE();
    case 70:
      if (lookahead == 'l') ADVANCE(77);
      END_STATE();
    case 71:
      if (lookahead == 'a') ADVANCE(78);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(anon_sym_module);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(anon_sym_output);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(anon_sym_struct);
      END_STATE();
    case 75:
      if (lookahead == 'i') ADVANCE(79);
      END_STATE();
    case 76:
      if (lookahead == 'n') ADVANCE(80);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(anon_sym_initial);
      END_STATE();
    case 78:
      if (lookahead == 'c') ADVANCE(81);
      END_STATE();
    case 79:
      if (lookahead == 'n') ADVANCE(82);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(anon_sym_function);
      END_STATE();
    case 81:
      if (lookahead == 'e') ADVANCE(83);
      END_STATE();
    case 82:
      if (lookahead == '_') ADVANCE(84);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(anon_sym_interface);
      END_STATE();
    case 84:
      if (lookahead == '_') ADVANCE(85);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(anon_sym___builtin__);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 1},
  [3] = {.lex_state = 1},
  [4] = {.lex_state = 1},
  [5] = {.lex_state = 1},
  [6] = {.lex_state = 1},
  [7] = {.lex_state = 1},
  [8] = {.lex_state = 1},
  [9] = {.lex_state = 1},
  [10] = {.lex_state = 1},
  [11] = {.lex_state = 1},
  [12] = {.lex_state = 2},
  [13] = {.lex_state = 2},
  [14] = {.lex_state = 2},
  [15] = {.lex_state = 2},
  [16] = {.lex_state = 2},
  [17] = {.lex_state = 2},
  [18] = {.lex_state = 2},
  [19] = {.lex_state = 2},
  [20] = {.lex_state = 2},
  [21] = {.lex_state = 2},
  [22] = {.lex_state = 2},
  [23] = {.lex_state = 2},
  [24] = {.lex_state = 2},
  [25] = {.lex_state = 2},
  [26] = {.lex_state = 2},
  [27] = {.lex_state = 2},
  [28] = {.lex_state = 2},
  [29] = {.lex_state = 2},
  [30] = {.lex_state = 2},
  [31] = {.lex_state = 2},
  [32] = {.lex_state = 2},
  [33] = {.lex_state = 2},
  [34] = {.lex_state = 2},
  [35] = {.lex_state = 2},
  [36] = {.lex_state = 2},
  [37] = {.lex_state = 2},
  [38] = {.lex_state = 2},
  [39] = {.lex_state = 2},
  [40] = {.lex_state = 1},
  [41] = {.lex_state = 2},
  [42] = {.lex_state = 1},
  [43] = {.lex_state = 2},
  [44] = {.lex_state = 2},
  [45] = {.lex_state = 2},
  [46] = {.lex_state = 2},
  [47] = {.lex_state = 2},
  [48] = {.lex_state = 2},
  [49] = {.lex_state = 2},
  [50] = {.lex_state = 2},
  [51] = {.lex_state = 2},
  [52] = {.lex_state = 2},
  [53] = {.lex_state = 2},
  [54] = {.lex_state = 2},
  [55] = {.lex_state = 1},
  [56] = {.lex_state = 2},
  [57] = {.lex_state = 2},
  [58] = {.lex_state = 1},
  [59] = {.lex_state = 1},
  [60] = {.lex_state = 1},
  [61] = {.lex_state = 1},
  [62] = {.lex_state = 1},
  [63] = {.lex_state = 1},
  [64] = {.lex_state = 2},
  [65] = {.lex_state = 1},
  [66] = {.lex_state = 1},
  [67] = {.lex_state = 1},
  [68] = {.lex_state = 1},
  [69] = {.lex_state = 1},
  [70] = {.lex_state = 2},
  [71] = {.lex_state = 1},
  [72] = {.lex_state = 1},
  [73] = {.lex_state = 2},
  [74] = {.lex_state = 1},
  [75] = {.lex_state = 1},
  [76] = {.lex_state = 1},
  [77] = {.lex_state = 1},
  [78] = {.lex_state = 1},
  [79] = {.lex_state = 1},
  [80] = {.lex_state = 1},
  [81] = {.lex_state = 1},
  [82] = {.lex_state = 1},
  [83] = {.lex_state = 1},
  [84] = {.lex_state = 1},
  [85] = {.lex_state = 1},
  [86] = {.lex_state = 1},
  [87] = {.lex_state = 1},
  [88] = {.lex_state = 1},
  [89] = {.lex_state = 1},
  [90] = {.lex_state = 1},
  [91] = {.lex_state = 1},
  [92] = {.lex_state = 0},
  [93] = {.lex_state = 0},
  [94] = {.lex_state = 0},
  [95] = {.lex_state = 0},
  [96] = {.lex_state = 0},
  [97] = {.lex_state = 0},
  [98] = {.lex_state = 0},
  [99] = {.lex_state = 0},
  [100] = {.lex_state = 0},
  [101] = {.lex_state = 0},
  [102] = {.lex_state = 0},
  [103] = {.lex_state = 1},
  [104] = {.lex_state = 1},
  [105] = {.lex_state = 0},
  [106] = {.lex_state = 0},
  [107] = {.lex_state = 0},
  [108] = {.lex_state = 0},
  [109] = {.lex_state = 0},
  [110] = {.lex_state = 1},
  [111] = {.lex_state = 1},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 0},
  [115] = {.lex_state = 0},
  [116] = {.lex_state = 0},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 1},
  [119] = {.lex_state = 0},
  [120] = {.lex_state = 0},
  [121] = {.lex_state = 0},
  [122] = {.lex_state = 0},
  [123] = {.lex_state = 0},
  [124] = {.lex_state = 0},
  [125] = {.lex_state = 0},
  [126] = {.lex_state = 0},
  [127] = {.lex_state = 0},
  [128] = {.lex_state = 0},
  [129] = {.lex_state = 0},
  [130] = {.lex_state = 0},
  [131] = {.lex_state = 0},
  [132] = {.lex_state = 0},
  [133] = {.lex_state = 0},
  [134] = {.lex_state = 0},
  [135] = {.lex_state = 0},
  [136] = {.lex_state = 0},
  [137] = {.lex_state = 0},
  [138] = {.lex_state = 0},
  [139] = {.lex_state = 0},
  [140] = {.lex_state = 0},
  [141] = {.lex_state = 0},
  [142] = {.lex_state = 0},
  [143] = {.lex_state = 0},
  [144] = {.lex_state = 0},
  [145] = {.lex_state = 0},
  [146] = {.lex_state = 0},
  [147] = {.lex_state = 0},
  [148] = {.lex_state = 0},
  [149] = {.lex_state = 0},
  [150] = {.lex_state = 0},
  [151] = {.lex_state = 0},
  [152] = {.lex_state = 0},
  [153] = {.lex_state = 0},
  [154] = {.lex_state = 0},
  [155] = {.lex_state = 0},
  [156] = {.lex_state = 0},
  [157] = {.lex_state = 0},
  [158] = {.lex_state = 0},
  [159] = {.lex_state = 0},
  [160] = {.lex_state = 0},
  [161] = {.lex_state = 0},
  [162] = {.lex_state = 0},
  [163] = {.lex_state = 0},
  [164] = {.lex_state = 0},
  [165] = {.lex_state = 0},
  [166] = {.lex_state = 0},
  [167] = {.lex_state = 0},
  [168] = {.lex_state = 0},
  [169] = {.lex_state = 0},
  [170] = {.lex_state = 0},
  [171] = {.lex_state = 0},
  [172] = {.lex_state = 0},
  [173] = {.lex_state = 0},
  [174] = {.lex_state = 0},
  [175] = {.lex_state = 0},
  [176] = {.lex_state = 0},
  [177] = {.lex_state = 0},
  [178] = {.lex_state = 0},
  [179] = {.lex_state = 0},
  [180] = {.lex_state = 0},
  [181] = {.lex_state = 0},
  [182] = {.lex_state = 0},
  [183] = {.lex_state = 0},
  [184] = {.lex_state = 0},
  [185] = {.lex_state = 0},
  [186] = {.lex_state = 0},
  [187] = {.lex_state = 0},
  [188] = {.lex_state = 0},
  [189] = {.lex_state = 0},
  [190] = {.lex_state = 0},
  [191] = {.lex_state = 0},
  [192] = {.lex_state = 0},
  [193] = {.lex_state = 0},
  [194] = {.lex_state = 0},
  [195] = {.lex_state = 0},
  [196] = {.lex_state = 0},
  [197] = {.lex_state = 0},
  [198] = {.lex_state = 0},
  [199] = {.lex_state = 0},
  [200] = {.lex_state = 0},
  [201] = {.lex_state = 0},
  [202] = {.lex_state = 0},
  [203] = {.lex_state = 0},
  [204] = {.lex_state = 0},
  [205] = {.lex_state = 0},
  [206] = {.lex_state = 0},
  [207] = {.lex_state = 0},
  [208] = {.lex_state = 0},
  [209] = {.lex_state = 0},
  [210] = {.lex_state = 0},
  [211] = {.lex_state = 0},
  [212] = {.lex_state = 0},
  [213] = {.lex_state = 0},
  [214] = {.lex_state = 0},
  [215] = {.lex_state = 0},
  [216] = {.lex_state = 0},
  [217] = {.lex_state = 0},
  [218] = {.lex_state = 0},
  [219] = {.lex_state = 0},
  [220] = {.lex_state = 0},
  [221] = {.lex_state = 0},
  [222] = {.lex_state = 0},
  [223] = {.lex_state = 0},
  [224] = {.lex_state = 0},
  [225] = {.lex_state = 0},
  [226] = {.lex_state = 0},
  [227] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym___builtin__] = ACTIONS(1),
    [anon_sym_extern] = ACTIONS(1),
    [anon_sym_module] = ACTIONS(1),
    [anon_sym_function] = ACTIONS(1),
    [anon_sym_struct] = ACTIONS(1),
    [anon_sym_POUND_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_reg] = ACTIONS(1),
    [anon_sym_initial] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [anon_sym_for] = ACTIONS(1),
    [anon_sym_in] = ACTIONS(1),
    [anon_sym_DOT_DOT] = ACTIONS(1),
    [anon_sym_domain] = ACTIONS(1),
    [anon_sym_interface] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_DASH_GT] = ACTIONS(1),
    [anon_sym_input] = ACTIONS(1),
    [anon_sym_output] = ACTIONS(1),
    [anon_sym_state] = ACTIONS(1),
    [anon_sym_gen] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_BANG] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_AMP] = ACTIONS(1),
    [anon_sym_CARET] = ACTIONS(1),
    [anon_sym_EQ_EQ] = ACTIONS(1),
    [anon_sym_BANG_EQ] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_LT_EQ] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_GT_EQ] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_PERCENT] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_type] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_LF] = ACTIONS(1),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(225),
    [sym_global_object] = STATE(169),
    [aux_sym__linebreak] = STATE(94),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym___builtin__] = ACTIONS(7),
    [anon_sym_extern] = ACTIONS(7),
    [anon_sym_module] = ACTIONS(9),
    [anon_sym_function] = ACTIONS(9),
    [anon_sym_struct] = ACTIONS(9),
    [anon_sym_LF] = ACTIONS(11),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 28,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(17), 1,
      anon_sym_RBRACE,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(27), 1,
      anon_sym_domain,
    ACTIONS(29), 1,
      anon_sym_interface,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(41), 1,
      sym_number,
    ACTIONS(43), 1,
      anon_sym_LF,
    STATE(17), 1,
      sym_namespace_list,
    STATE(40), 1,
      aux_sym__linebreak,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(79), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(127), 1,
      sym_assign_left_side,
    STATE(141), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 2,
      sym__type,
      sym_array_type,
    STATE(156), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(49), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [106] = 28,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(27), 1,
      anon_sym_domain,
    ACTIONS(29), 1,
      anon_sym_interface,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(41), 1,
      sym_number,
    ACTIONS(43), 1,
      anon_sym_LF,
    ACTIONS(45), 1,
      anon_sym_RBRACE,
    STATE(17), 1,
      sym_namespace_list,
    STATE(40), 1,
      aux_sym__linebreak,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(79), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(141), 1,
      sym_declaration,
    STATE(179), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 2,
      sym__type,
      sym_array_type,
    STATE(200), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(49), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [212] = 28,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(27), 1,
      anon_sym_domain,
    ACTIONS(29), 1,
      anon_sym_interface,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(41), 1,
      sym_number,
    ACTIONS(43), 1,
      anon_sym_LF,
    ACTIONS(47), 1,
      anon_sym_RBRACE,
    STATE(17), 1,
      sym_namespace_list,
    STATE(40), 1,
      aux_sym__linebreak,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(79), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(141), 1,
      sym_declaration,
    STATE(179), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 2,
      sym__type,
      sym_array_type,
    STATE(200), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(49), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [318] = 28,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(27), 1,
      anon_sym_domain,
    ACTIONS(29), 1,
      anon_sym_interface,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(41), 1,
      sym_number,
    ACTIONS(43), 1,
      anon_sym_LF,
    ACTIONS(49), 1,
      anon_sym_RBRACE,
    STATE(17), 1,
      sym_namespace_list,
    STATE(40), 1,
      aux_sym__linebreak,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(79), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(141), 1,
      sym_declaration,
    STATE(179), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 2,
      sym__type,
      sym_array_type,
    STATE(200), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(49), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [424] = 28,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(27), 1,
      anon_sym_domain,
    ACTIONS(29), 1,
      anon_sym_interface,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(41), 1,
      sym_number,
    ACTIONS(43), 1,
      anon_sym_LF,
    ACTIONS(51), 1,
      anon_sym_RBRACE,
    STATE(17), 1,
      sym_namespace_list,
    STATE(40), 1,
      aux_sym__linebreak,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(79), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(141), 1,
      sym_declaration,
    STATE(179), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 2,
      sym__type,
      sym_array_type,
    STATE(200), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(49), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [530] = 28,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(27), 1,
      anon_sym_domain,
    ACTIONS(29), 1,
      anon_sym_interface,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(41), 1,
      sym_number,
    ACTIONS(53), 1,
      anon_sym_RBRACE,
    ACTIONS(55), 1,
      anon_sym_LF,
    STATE(2), 1,
      aux_sym__linebreak,
    STATE(17), 1,
      sym_namespace_list,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(79), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(132), 1,
      sym_assign_left_side,
    STATE(141), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 2,
      sym__type,
      sym_array_type,
    STATE(144), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(49), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [636] = 28,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(27), 1,
      anon_sym_domain,
    ACTIONS(29), 1,
      anon_sym_interface,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(41), 1,
      sym_number,
    ACTIONS(43), 1,
      anon_sym_LF,
    ACTIONS(57), 1,
      anon_sym_RBRACE,
    STATE(17), 1,
      sym_namespace_list,
    STATE(40), 1,
      aux_sym__linebreak,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(79), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(141), 1,
      sym_declaration,
    STATE(179), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 2,
      sym__type,
      sym_array_type,
    STATE(200), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(49), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [742] = 28,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(27), 1,
      anon_sym_domain,
    ACTIONS(29), 1,
      anon_sym_interface,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(41), 1,
      sym_number,
    ACTIONS(43), 1,
      anon_sym_LF,
    ACTIONS(59), 1,
      anon_sym_RBRACE,
    STATE(17), 1,
      sym_namespace_list,
    STATE(40), 1,
      aux_sym__linebreak,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(79), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(141), 1,
      sym_declaration,
    STATE(179), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 2,
      sym__type,
      sym_array_type,
    STATE(200), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(49), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [848] = 27,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(27), 1,
      anon_sym_domain,
    ACTIONS(29), 1,
      anon_sym_interface,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(41), 1,
      sym_number,
    ACTIONS(43), 1,
      anon_sym_LF,
    STATE(17), 1,
      sym_namespace_list,
    STATE(40), 1,
      aux_sym__linebreak,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(79), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(141), 1,
      sym_declaration,
    STATE(179), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 2,
      sym__type,
      sym_array_type,
    STATE(200), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(49), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [951] = 18,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(41), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(79), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(137), 1,
      sym_assign_to,
    STATE(141), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 2,
      sym__type,
      sym_array_type,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(49), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [1022] = 5,
    ACTIONS(65), 1,
      anon_sym_COLON_COLON,
    STATE(13), 1,
      aux_sym_namespace_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(61), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(63), 21,
      anon_sym_POUND_LPAREN,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1066] = 5,
    ACTIONS(65), 1,
      anon_sym_COLON_COLON,
    STATE(14), 1,
      aux_sym_namespace_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(67), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(69), 21,
      anon_sym_POUND_LPAREN,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1110] = 5,
    ACTIONS(75), 1,
      anon_sym_COLON_COLON,
    STATE(14), 1,
      aux_sym_namespace_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(71), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(73), 21,
      anon_sym_POUND_LPAREN,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1154] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(78), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(80), 22,
      anon_sym_POUND_LPAREN,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
      anon_sym_LF,
  [1193] = 5,
    ACTIONS(84), 1,
      anon_sym_POUND_LPAREN,
    STATE(27), 1,
      sym_template_args,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(82), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(86), 20,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1236] = 5,
    ACTIONS(84), 1,
      anon_sym_POUND_LPAREN,
    STATE(37), 1,
      sym_template_args,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(88), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(90), 20,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1279] = 8,
    ACTIONS(96), 1,
      anon_sym_DOT,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(94), 5,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(92), 19,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1327] = 10,
    ACTIONS(96), 1,
      anon_sym_DOT,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(104), 4,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(102), 17,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1379] = 15,
    ACTIONS(96), 1,
      anon_sym_DOT,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    ACTIONS(110), 1,
      anon_sym_PLUS,
    ACTIONS(112), 1,
      anon_sym_DASH,
    ACTIONS(114), 1,
      anon_sym_PIPE,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_CARET,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(104), 3,
      anon_sym_EQ,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(102), 13,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1441] = 12,
    ACTIONS(96), 1,
      anon_sym_DOT,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    ACTIONS(110), 1,
      anon_sym_PLUS,
    ACTIONS(112), 1,
      anon_sym_DASH,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(104), 3,
      anon_sym_EQ,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(102), 16,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1497] = 14,
    ACTIONS(96), 1,
      anon_sym_DOT,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    ACTIONS(110), 1,
      anon_sym_PLUS,
    ACTIONS(112), 1,
      anon_sym_DASH,
    ACTIONS(114), 1,
      anon_sym_PIPE,
    ACTIONS(118), 1,
      anon_sym_CARET,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(104), 3,
      anon_sym_EQ,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(102), 14,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1557] = 13,
    ACTIONS(96), 1,
      anon_sym_DOT,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    ACTIONS(110), 1,
      anon_sym_PLUS,
    ACTIONS(112), 1,
      anon_sym_DASH,
    ACTIONS(118), 1,
      anon_sym_CARET,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(104), 3,
      anon_sym_EQ,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(102), 15,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1615] = 8,
    ACTIONS(96), 1,
      anon_sym_DOT,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(104), 5,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(102), 19,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1663] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(120), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(122), 20,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1700] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(124), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(126), 20,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1737] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(128), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(130), 20,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1774] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(132), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(134), 20,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1811] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(136), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(138), 20,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1848] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(140), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(142), 20,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1885] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(144), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(146), 20,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1922] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(148), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(150), 20,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1959] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(152), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(154), 20,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1996] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(156), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(158), 20,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [2033] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(160), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(162), 20,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [2070] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(164), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(166), 20,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [2107] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(168), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(170), 20,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [2144] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(174), 6,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(172), 21,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [2180] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(178), 6,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(176), 21,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [2216] = 5,
    ACTIONS(184), 1,
      anon_sym_LF,
    STATE(40), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(180), 12,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
      anon_sym_domain,
      anon_sym_interface,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_DASH,
      sym_identifier,
    ACTIONS(182), 13,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_COLON_COLON,
      sym_number,
  [2256] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(189), 6,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(187), 21,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [2292] = 13,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(191), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    STATE(51), 1,
      sym_template_global,
    STATE(138), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 2,
      sym__type,
      sym_array_type,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(50), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [2348] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(195), 6,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(193), 21,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [2384] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(199), 6,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(197), 21,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [2420] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(203), 6,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(201), 21,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [2456] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(207), 6,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(205), 21,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [2492] = 17,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    ACTIONS(110), 1,
      anon_sym_PLUS,
    ACTIONS(112), 1,
      anon_sym_DASH,
    ACTIONS(114), 1,
      anon_sym_PIPE,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_CARET,
    ACTIONS(211), 1,
      anon_sym_EQ,
    ACTIONS(217), 1,
      anon_sym_DOT,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(215), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(213), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
    ACTIONS(209), 6,
      anon_sym_RPAREN,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [2555] = 18,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    ACTIONS(114), 1,
      anon_sym_PIPE,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_CARET,
    ACTIONS(217), 1,
      anon_sym_DOT,
    ACTIONS(219), 1,
      anon_sym_RPAREN,
    ACTIONS(221), 1,
      anon_sym_COMMA,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    STATE(61), 1,
      sym__comma,
    STATE(170), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(110), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(215), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(213), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2617] = 16,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    ACTIONS(114), 1,
      anon_sym_PIPE,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_CARET,
    ACTIONS(217), 1,
      anon_sym_DOT,
    ACTIONS(225), 1,
      anon_sym_EQ,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(110), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(215), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(223), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_LF,
    ACTIONS(213), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2675] = 16,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    ACTIONS(114), 1,
      anon_sym_PIPE,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_CARET,
    ACTIONS(217), 1,
      anon_sym_DOT,
    ACTIONS(229), 1,
      anon_sym_EQ,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(110), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(215), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(227), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_LF,
    ACTIONS(213), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2733] = 5,
    ACTIONS(231), 1,
      sym_identifier,
    ACTIONS(237), 1,
      anon_sym_LBRACK,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(235), 4,
      anon_sym_EQ,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(233), 16,
      anon_sym_RBRACE,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_DOT,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_LF,
  [2768] = 15,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    ACTIONS(114), 1,
      anon_sym_PIPE,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_CARET,
    ACTIONS(217), 1,
      anon_sym_DOT,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(110), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(215), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(240), 3,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LF,
    ACTIONS(213), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2823] = 16,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    ACTIONS(114), 1,
      anon_sym_PIPE,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_CARET,
    ACTIONS(217), 1,
      anon_sym_DOT,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    STATE(193), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(110), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(215), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(213), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2879] = 16,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    ACTIONS(114), 1,
      anon_sym_PIPE,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_CARET,
    ACTIONS(217), 1,
      anon_sym_DOT,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    STATE(186), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(110), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(215), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(213), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2935] = 9,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(242), 1,
      anon_sym_type,
    ACTIONS(244), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(52), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [2977] = 15,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    ACTIONS(114), 1,
      anon_sym_PIPE,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_CARET,
    ACTIONS(217), 1,
      anon_sym_DOT,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(110), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(215), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(246), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(213), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3031] = 15,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    ACTIONS(114), 1,
      anon_sym_PIPE,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_CARET,
    ACTIONS(217), 1,
      anon_sym_DOT,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(110), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(215), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(248), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
    ACTIONS(213), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3085] = 9,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(252), 1,
      anon_sym_RPAREN,
    ACTIONS(254), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(48), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3127] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(256), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(20), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3166] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(258), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(19), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3205] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(260), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(56), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3244] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(262), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(23), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3283] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(264), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(53), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3322] = 15,
    ACTIONS(96), 1,
      anon_sym_DOT,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    ACTIONS(114), 1,
      anon_sym_PIPE,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_CARET,
    ACTIONS(266), 1,
      anon_sym_DOT_DOT,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(110), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(215), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(213), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3375] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(268), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(24), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3414] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(270), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(47), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3453] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(272), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(70), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3492] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(274), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(73), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3531] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(276), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(54), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3570] = 15,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    ACTIONS(114), 1,
      anon_sym_PIPE,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_CARET,
    ACTIONS(217), 1,
      anon_sym_DOT,
    ACTIONS(278), 1,
      anon_sym_RPAREN,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(110), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(215), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(213), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3623] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(280), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(21), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3662] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(282), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(57), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3701] = 15,
    ACTIONS(98), 1,
      anon_sym_LPAREN,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_SLASH,
    ACTIONS(114), 1,
      anon_sym_PIPE,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_CARET,
    ACTIONS(217), 1,
      anon_sym_DOT,
    ACTIONS(284), 1,
      anon_sym_RBRACK,
    STATE(43), 1,
      sym_parenthesis_expression_list,
    STATE(46), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(110), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(215), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(213), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3754] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(286), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(18), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3793] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(288), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(22), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3832] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(290), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(35), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(64), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3871] = 5,
    ACTIONS(296), 1,
      anon_sym_LF,
    STATE(78), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(292), 7,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(294), 10,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_COLON_COLON,
      sym_number,
  [3903] = 5,
    ACTIONS(43), 1,
      anon_sym_LF,
    STATE(40), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(298), 7,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(300), 10,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_COLON_COLON,
      sym_number,
  [3935] = 5,
    ACTIONS(19), 1,
      anon_sym_reg,
    STATE(80), 1,
      aux_sym_write_modifiers_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(302), 5,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(304), 10,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_COLON_COLON,
      sym_number,
  [3965] = 5,
    ACTIONS(308), 1,
      anon_sym_reg,
    STATE(80), 1,
      aux_sym_write_modifiers_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(306), 5,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(311), 10,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_COLON_COLON,
      sym_number,
  [3995] = 13,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(313), 1,
      anon_sym_DASH_GT,
    ACTIONS(315), 1,
      anon_sym_LF,
    STATE(17), 1,
      sym_namespace_list,
    STATE(82), 1,
      aux_sym__linebreak,
    STATE(112), 1,
      sym_declaration,
    STATE(162), 1,
      sym_declaration_list,
    STATE(206), 1,
      sym__interface_ports_output,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4040] = 13,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(43), 1,
      anon_sym_LF,
    ACTIONS(313), 1,
      anon_sym_DASH_GT,
    STATE(17), 1,
      sym_namespace_list,
    STATE(40), 1,
      aux_sym__linebreak,
    STATE(112), 1,
      sym_declaration,
    STATE(140), 1,
      sym_declaration_list,
    STATE(208), 1,
      sym__interface_ports_output,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4085] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(317), 6,
      anon_sym_reg,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(319), 10,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_COLON_COLON,
      sym_number,
  [4110] = 11,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(321), 1,
      sym_identifier,
    ACTIONS(323), 1,
      anon_sym_RPAREN,
    ACTIONS(325), 1,
      anon_sym_LF,
    STATE(17), 1,
      sym_namespace_list,
    STATE(86), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(119), 2,
      sym_template_declaration_type,
      sym_declaration,
    STATE(178), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4150] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(327), 5,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(329), 10,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_COLON_COLON,
      sym_number,
  [4174] = 11,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(43), 1,
      anon_sym_LF,
    ACTIONS(321), 1,
      sym_identifier,
    ACTIONS(331), 1,
      anon_sym_RPAREN,
    STATE(17), 1,
      sym_namespace_list,
    STATE(40), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(122), 2,
      sym_template_declaration_type,
      sym_declaration,
    STATE(178), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4214] = 11,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(333), 1,
      anon_sym_LF,
    STATE(17), 1,
      sym_namespace_list,
    STATE(88), 1,
      aux_sym__linebreak,
    STATE(112), 1,
      sym_declaration,
    STATE(203), 1,
      sym_declaration_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4253] = 11,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(43), 1,
      anon_sym_LF,
    STATE(17), 1,
      sym_namespace_list,
    STATE(40), 1,
      aux_sym__linebreak,
    STATE(112), 1,
      sym_declaration,
    STATE(197), 1,
      sym_declaration_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4292] = 8,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(321), 1,
      sym_identifier,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(184), 2,
      sym_template_declaration_type,
      sym_declaration,
    STATE(178), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4323] = 8,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    STATE(17), 1,
      sym_namespace_list,
    STATE(221), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4353] = 8,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    STATE(17), 1,
      sym_namespace_list,
    STATE(168), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(33), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(178), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4383] = 7,
    ACTIONS(335), 1,
      ts_builtin_sym_end,
    ACTIONS(337), 1,
      anon_sym_LF,
    STATE(100), 1,
      aux_sym__linebreak,
    STATE(191), 1,
      sym_global_object,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
    ACTIONS(9), 3,
      anon_sym_module,
      anon_sym_function,
      anon_sym_struct,
  [4409] = 4,
    ACTIONS(341), 1,
      anon_sym_SQUOTE,
    STATE(105), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(339), 7,
      anon_sym_RPAREN,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4429] = 7,
    ACTIONS(337), 1,
      anon_sym_LF,
    ACTIONS(343), 1,
      ts_builtin_sym_end,
    STATE(100), 1,
      aux_sym__linebreak,
    STATE(161), 1,
      sym_global_object,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
    ACTIONS(9), 3,
      anon_sym_module,
      anon_sym_function,
      anon_sym_struct,
  [4455] = 4,
    ACTIONS(341), 1,
      anon_sym_SQUOTE,
    STATE(108), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(345), 7,
      anon_sym_RPAREN,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4475] = 4,
    ACTIONS(341), 1,
      anon_sym_SQUOTE,
    STATE(106), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(347), 7,
      anon_sym_RPAREN,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4495] = 4,
    ACTIONS(341), 1,
      anon_sym_SQUOTE,
    STATE(107), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(349), 7,
      anon_sym_RPAREN,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4515] = 7,
    ACTIONS(337), 1,
      anon_sym_LF,
    ACTIONS(351), 1,
      ts_builtin_sym_end,
    STATE(100), 1,
      aux_sym__linebreak,
    STATE(191), 1,
      sym_global_object,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
    ACTIONS(9), 3,
      anon_sym_module,
      anon_sym_function,
      anon_sym_struct,
  [4541] = 7,
    ACTIONS(337), 1,
      anon_sym_LF,
    ACTIONS(353), 1,
      ts_builtin_sym_end,
    STATE(100), 1,
      aux_sym__linebreak,
    STATE(191), 1,
      sym_global_object,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
    ACTIONS(9), 3,
      anon_sym_module,
      anon_sym_function,
      anon_sym_struct,
  [4567] = 4,
    ACTIONS(355), 1,
      anon_sym_LF,
    STATE(100), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(182), 7,
      ts_builtin_sym_end,
      anon_sym___builtin__,
      anon_sym_extern,
      anon_sym_module,
      anon_sym_function,
      anon_sym_struct,
      anon_sym_RPAREN,
  [4587] = 7,
    ACTIONS(337), 1,
      anon_sym_LF,
    ACTIONS(358), 1,
      ts_builtin_sym_end,
    STATE(100), 1,
      aux_sym__linebreak,
    STATE(191), 1,
      sym_global_object,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
    ACTIONS(9), 3,
      anon_sym_module,
      anon_sym_function,
      anon_sym_struct,
  [4613] = 6,
    ACTIONS(337), 1,
      anon_sym_LF,
    STATE(100), 1,
      aux_sym__linebreak,
    STATE(191), 1,
      sym_global_object,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
    ACTIONS(9), 3,
      anon_sym_module,
      anon_sym_function,
      anon_sym_struct,
  [4636] = 5,
    ACTIONS(65), 1,
      anon_sym_COLON_COLON,
    STATE(13), 1,
      aux_sym_namespace_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(63), 3,
      anon_sym_POUND_LPAREN,
      anon_sym_LBRACK,
      sym_identifier,
    ACTIONS(360), 3,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LF,
  [4657] = 6,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(362), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(173), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4680] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(364), 7,
      anon_sym_RPAREN,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4694] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(366), 7,
      anon_sym_RPAREN,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4708] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(368), 7,
      anon_sym_RPAREN,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4722] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(370), 7,
      anon_sym_RPAREN,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4736] = 5,
    ACTIONS(221), 1,
      anon_sym_COMMA,
    STATE(11), 1,
      sym__comma,
    STATE(120), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(372), 3,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_LF,
  [4755] = 5,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(189), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4774] = 5,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(126), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4793] = 5,
    ACTIONS(221), 1,
      anon_sym_COMMA,
    STATE(91), 1,
      sym__comma,
    STATE(125), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(374), 3,
      anon_sym_RBRACE,
      anon_sym_DASH_GT,
      anon_sym_LF,
  [4812] = 7,
    ACTIONS(221), 1,
      anon_sym_COMMA,
    ACTIONS(376), 1,
      anon_sym_RPAREN,
    ACTIONS(378), 1,
      anon_sym_LF,
    STATE(116), 1,
      aux_sym_template_args_repeat1,
    STATE(182), 1,
      aux_sym__linebreak,
    STATE(198), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4835] = 7,
    ACTIONS(221), 1,
      anon_sym_COMMA,
    ACTIONS(380), 1,
      anon_sym_RPAREN,
    ACTIONS(382), 1,
      anon_sym_LF,
    STATE(128), 1,
      aux_sym_template_args_repeat1,
    STATE(174), 1,
      aux_sym__linebreak,
    STATE(198), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4858] = 7,
    ACTIONS(221), 1,
      anon_sym_COMMA,
    ACTIONS(384), 1,
      anon_sym_RPAREN,
    ACTIONS(386), 1,
      anon_sym_LF,
    STATE(89), 1,
      sym__comma,
    STATE(129), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(188), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4881] = 7,
    ACTIONS(221), 1,
      anon_sym_COMMA,
    ACTIONS(388), 1,
      anon_sym_RPAREN,
    ACTIONS(390), 1,
      anon_sym_LF,
    STATE(128), 1,
      aux_sym_template_args_repeat1,
    STATE(176), 1,
      aux_sym__linebreak,
    STATE(198), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4904] = 5,
    ACTIONS(394), 1,
      anon_sym_COMMA,
    STATE(91), 1,
      sym__comma,
    STATE(117), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(392), 3,
      anon_sym_RBRACE,
      anon_sym_DASH_GT,
      anon_sym_LF,
  [4923] = 5,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      sym_identifier,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(180), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4942] = 7,
    ACTIONS(221), 1,
      anon_sym_COMMA,
    ACTIONS(397), 1,
      anon_sym_RPAREN,
    ACTIONS(399), 1,
      anon_sym_LF,
    STATE(89), 1,
      sym__comma,
    STATE(115), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(181), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4965] = 5,
    ACTIONS(403), 1,
      anon_sym_COMMA,
    STATE(11), 1,
      sym__comma,
    STATE(120), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(401), 3,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_LF,
  [4984] = 7,
    ACTIONS(221), 1,
      anon_sym_COMMA,
    ACTIONS(406), 1,
      anon_sym_RPAREN,
    ACTIONS(408), 1,
      anon_sym_LF,
    STATE(114), 1,
      aux_sym_template_args_repeat1,
    STATE(177), 1,
      aux_sym__linebreak,
    STATE(198), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5007] = 7,
    ACTIONS(221), 1,
      anon_sym_COMMA,
    ACTIONS(410), 1,
      anon_sym_RPAREN,
    ACTIONS(412), 1,
      anon_sym_LF,
    STATE(89), 1,
      sym__comma,
    STATE(123), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(175), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5030] = 7,
    ACTIONS(221), 1,
      anon_sym_COMMA,
    ACTIONS(414), 1,
      anon_sym_RPAREN,
    ACTIONS(416), 1,
      anon_sym_LF,
    STATE(89), 1,
      sym__comma,
    STATE(129), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(183), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5053] = 5,
    ACTIONS(221), 1,
      anon_sym_COMMA,
    STATE(11), 1,
      sym__comma,
    STATE(109), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(418), 3,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_LF,
  [5072] = 5,
    ACTIONS(221), 1,
      anon_sym_COMMA,
    STATE(91), 1,
      sym__comma,
    STATE(117), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(420), 3,
      anon_sym_RBRACE,
      anon_sym_DASH_GT,
      anon_sym_LF,
  [5091] = 4,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    STATE(133), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(422), 3,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LF,
  [5107] = 6,
    ACTIONS(424), 1,
      anon_sym_RBRACE,
    ACTIONS(426), 1,
      anon_sym_EQ,
    ACTIONS(428), 1,
      anon_sym_LF,
    STATE(6), 1,
      aux_sym__linebreak,
    STATE(148), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5127] = 5,
    ACTIONS(432), 1,
      anon_sym_COMMA,
    STATE(128), 1,
      aux_sym_template_args_repeat1,
    STATE(198), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(430), 2,
      anon_sym_RPAREN,
      anon_sym_LF,
  [5145] = 5,
    ACTIONS(437), 1,
      anon_sym_COMMA,
    STATE(89), 1,
      sym__comma,
    STATE(129), 1,
      aux_sym_template_declaration_arguments_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(435), 2,
      anon_sym_RPAREN,
      anon_sym_LF,
  [5163] = 6,
    ACTIONS(43), 1,
      anon_sym_LF,
    ACTIONS(440), 1,
      sym_identifier,
    ACTIONS(442), 1,
      anon_sym_RPAREN,
    STATE(40), 1,
      aux_sym__linebreak,
    STATE(113), 1,
      sym_template_arg,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5183] = 6,
    ACTIONS(440), 1,
      sym_identifier,
    ACTIONS(444), 1,
      anon_sym_RPAREN,
    ACTIONS(446), 1,
      anon_sym_LF,
    STATE(121), 1,
      sym_template_arg,
    STATE(130), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5203] = 6,
    ACTIONS(426), 1,
      anon_sym_EQ,
    ACTIONS(448), 1,
      anon_sym_RBRACE,
    ACTIONS(450), 1,
      anon_sym_LF,
    STATE(5), 1,
      aux_sym__linebreak,
    STATE(154), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5223] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(452), 5,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      sym_identifier,
      anon_sym_COMMA,
      anon_sym_LF,
  [5235] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(454), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5246] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(456), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5257] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(458), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5268] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(460), 4,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_COMMA,
      anon_sym_LF,
  [5279] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(227), 4,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_COMMA,
      anon_sym_LF,
  [5290] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(458), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5301] = 4,
    ACTIONS(313), 1,
      anon_sym_DASH_GT,
    STATE(194), 1,
      sym__interface_ports_output,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(462), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5316] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(223), 4,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_COMMA,
      anon_sym_LF,
  [5327] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(464), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5338] = 4,
    ACTIONS(468), 1,
      anon_sym_COLON,
    STATE(192), 1,
      sym_interface_ports,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(466), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5353] = 5,
    ACTIONS(448), 1,
      anon_sym_RBRACE,
    ACTIONS(450), 1,
      anon_sym_LF,
    STATE(5), 1,
      aux_sym__linebreak,
    STATE(152), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5370] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(470), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5381] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(472), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5392] = 5,
    ACTIONS(474), 1,
      anon_sym_RBRACE,
    ACTIONS(476), 1,
      anon_sym_LF,
    STATE(3), 1,
      aux_sym__linebreak,
    STATE(153), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5409] = 5,
    ACTIONS(478), 1,
      anon_sym_RBRACE,
    ACTIONS(480), 1,
      anon_sym_LF,
    STATE(4), 1,
      aux_sym__linebreak,
    STATE(153), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5426] = 5,
    ACTIONS(482), 1,
      anon_sym_RPAREN,
    ACTIONS(484), 1,
      anon_sym_COMMA,
    STATE(61), 1,
      sym__comma,
    STATE(149), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5443] = 3,
    ACTIONS(489), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(487), 3,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LF,
  [5456] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(456), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5467] = 5,
    ACTIONS(491), 1,
      anon_sym_RBRACE,
    ACTIONS(493), 1,
      anon_sym_LF,
    STATE(9), 1,
      aux_sym__linebreak,
    STATE(153), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5484] = 5,
    ACTIONS(495), 1,
      anon_sym_RBRACE,
    ACTIONS(497), 1,
      anon_sym_LF,
    STATE(10), 1,
      aux_sym__linebreak,
    STATE(153), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5501] = 5,
    ACTIONS(500), 1,
      anon_sym_RBRACE,
    ACTIONS(502), 1,
      anon_sym_LF,
    STATE(8), 1,
      aux_sym__linebreak,
    STATE(153), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5518] = 5,
    ACTIONS(504), 1,
      ts_builtin_sym_end,
    ACTIONS(506), 1,
      anon_sym_LF,
    STATE(92), 1,
      aux_sym__linebreak,
    STATE(157), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5535] = 5,
    ACTIONS(424), 1,
      anon_sym_RBRACE,
    ACTIONS(428), 1,
      anon_sym_LF,
    STATE(6), 1,
      aux_sym__linebreak,
    STATE(147), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5552] = 5,
    ACTIONS(508), 1,
      ts_builtin_sym_end,
    ACTIONS(510), 1,
      anon_sym_LF,
    STATE(102), 1,
      aux_sym__linebreak,
    STATE(157), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5569] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(513), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5580] = 4,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(515), 1,
      anon_sym_if,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(201), 2,
      sym_block,
      sym_if_statement,
  [5595] = 5,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(517), 1,
      anon_sym_POUND_LPAREN,
    STATE(202), 1,
      sym_template_declaration_arguments,
    STATE(204), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5612] = 5,
    ACTIONS(519), 1,
      ts_builtin_sym_end,
    ACTIONS(521), 1,
      anon_sym_LF,
    STATE(98), 1,
      aux_sym__linebreak,
    STATE(155), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5629] = 4,
    ACTIONS(313), 1,
      anon_sym_DASH_GT,
    STATE(205), 1,
      sym__interface_ports_output,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(523), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5644] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(525), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5655] = 5,
    ACTIONS(527), 1,
      ts_builtin_sym_end,
    ACTIONS(529), 1,
      anon_sym_LF,
    STATE(99), 1,
      aux_sym__linebreak,
    STATE(157), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5672] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(531), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5683] = 5,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(517), 1,
      anon_sym_POUND_LPAREN,
    STATE(195), 1,
      sym_template_declaration_arguments,
    STATE(199), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5700] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(531), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5711] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(533), 4,
      anon_sym_RBRACE,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [5722] = 5,
    ACTIONS(535), 1,
      ts_builtin_sym_end,
    ACTIONS(537), 1,
      anon_sym_LF,
    STATE(101), 1,
      aux_sym__linebreak,
    STATE(164), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5739] = 5,
    ACTIONS(221), 1,
      anon_sym_COMMA,
    ACTIONS(539), 1,
      anon_sym_RPAREN,
    STATE(61), 1,
      sym__comma,
    STATE(149), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5756] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(541), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5767] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(513), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5778] = 4,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(543), 1,
      sym_identifier,
    STATE(133), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5792] = 4,
    ACTIONS(337), 1,
      anon_sym_LF,
    ACTIONS(545), 1,
      anon_sym_RPAREN,
    STATE(100), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5806] = 4,
    ACTIONS(337), 1,
      anon_sym_LF,
    ACTIONS(547), 1,
      anon_sym_RPAREN,
    STATE(100), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5820] = 4,
    ACTIONS(337), 1,
      anon_sym_LF,
    ACTIONS(549), 1,
      anon_sym_RPAREN,
    STATE(100), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5834] = 4,
    ACTIONS(337), 1,
      anon_sym_LF,
    ACTIONS(551), 1,
      anon_sym_RPAREN,
    STATE(100), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5848] = 4,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(553), 1,
      sym_identifier,
    STATE(133), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5862] = 3,
    ACTIONS(426), 1,
      anon_sym_EQ,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(555), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5874] = 4,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(557), 1,
      sym_identifier,
    STATE(133), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5888] = 4,
    ACTIONS(337), 1,
      anon_sym_LF,
    ACTIONS(559), 1,
      anon_sym_RPAREN,
    STATE(100), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5902] = 4,
    ACTIONS(337), 1,
      anon_sym_LF,
    ACTIONS(561), 1,
      anon_sym_RPAREN,
    STATE(100), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5916] = 4,
    ACTIONS(337), 1,
      anon_sym_LF,
    ACTIONS(563), 1,
      anon_sym_RPAREN,
    STATE(100), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5930] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(565), 3,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LF,
  [5940] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(567), 3,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LF,
  [5950] = 3,
    ACTIONS(571), 1,
      anon_sym_else,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(569), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5962] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(573), 3,
      anon_sym_module,
      anon_sym_function,
      anon_sym_struct,
  [5972] = 4,
    ACTIONS(337), 1,
      anon_sym_LF,
    ACTIONS(575), 1,
      anon_sym_RPAREN,
    STATE(100), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5986] = 4,
    ACTIONS(100), 1,
      anon_sym_LBRACK,
    ACTIONS(577), 1,
      sym_identifier,
    STATE(133), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6000] = 3,
    ACTIONS(250), 1,
      sym_identifier,
    STATE(16), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6011] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(579), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6020] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(581), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6029] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(583), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6038] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(585), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6047] = 3,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    STATE(207), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6058] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(587), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6067] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(589), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6076] = 3,
    ACTIONS(440), 1,
      sym_identifier,
    STATE(185), 1,
      sym_template_arg,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6087] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(591), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6096] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(555), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6105] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(593), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6114] = 3,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    STATE(209), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6125] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(595), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6134] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(597), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6143] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(599), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6152] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(601), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6161] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(603), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6170] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(605), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6179] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(607), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6188] = 2,
    ACTIONS(609), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6196] = 2,
    ACTIONS(611), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6204] = 2,
    ACTIONS(613), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6212] = 2,
    ACTIONS(615), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6220] = 2,
    ACTIONS(617), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6228] = 2,
    ACTIONS(619), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6236] = 2,
    ACTIONS(621), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6244] = 2,
    ACTIONS(623), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6252] = 2,
    ACTIONS(625), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6260] = 2,
    ACTIONS(627), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6268] = 2,
    ACTIONS(629), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6276] = 2,
    ACTIONS(631), 1,
      anon_sym_in,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6284] = 2,
    ACTIONS(633), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6292] = 2,
    ACTIONS(635), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6300] = 2,
    ACTIONS(637), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6308] = 2,
    ACTIONS(639), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6316] = 2,
    ACTIONS(641), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6324] = 2,
    ACTIONS(643), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 106,
  [SMALL_STATE(4)] = 212,
  [SMALL_STATE(5)] = 318,
  [SMALL_STATE(6)] = 424,
  [SMALL_STATE(7)] = 530,
  [SMALL_STATE(8)] = 636,
  [SMALL_STATE(9)] = 742,
  [SMALL_STATE(10)] = 848,
  [SMALL_STATE(11)] = 951,
  [SMALL_STATE(12)] = 1022,
  [SMALL_STATE(13)] = 1066,
  [SMALL_STATE(14)] = 1110,
  [SMALL_STATE(15)] = 1154,
  [SMALL_STATE(16)] = 1193,
  [SMALL_STATE(17)] = 1236,
  [SMALL_STATE(18)] = 1279,
  [SMALL_STATE(19)] = 1327,
  [SMALL_STATE(20)] = 1379,
  [SMALL_STATE(21)] = 1441,
  [SMALL_STATE(22)] = 1497,
  [SMALL_STATE(23)] = 1557,
  [SMALL_STATE(24)] = 1615,
  [SMALL_STATE(25)] = 1663,
  [SMALL_STATE(26)] = 1700,
  [SMALL_STATE(27)] = 1737,
  [SMALL_STATE(28)] = 1774,
  [SMALL_STATE(29)] = 1811,
  [SMALL_STATE(30)] = 1848,
  [SMALL_STATE(31)] = 1885,
  [SMALL_STATE(32)] = 1922,
  [SMALL_STATE(33)] = 1959,
  [SMALL_STATE(34)] = 1996,
  [SMALL_STATE(35)] = 2033,
  [SMALL_STATE(36)] = 2070,
  [SMALL_STATE(37)] = 2107,
  [SMALL_STATE(38)] = 2144,
  [SMALL_STATE(39)] = 2180,
  [SMALL_STATE(40)] = 2216,
  [SMALL_STATE(41)] = 2256,
  [SMALL_STATE(42)] = 2292,
  [SMALL_STATE(43)] = 2348,
  [SMALL_STATE(44)] = 2384,
  [SMALL_STATE(45)] = 2420,
  [SMALL_STATE(46)] = 2456,
  [SMALL_STATE(47)] = 2492,
  [SMALL_STATE(48)] = 2555,
  [SMALL_STATE(49)] = 2617,
  [SMALL_STATE(50)] = 2675,
  [SMALL_STATE(51)] = 2733,
  [SMALL_STATE(52)] = 2768,
  [SMALL_STATE(53)] = 2823,
  [SMALL_STATE(54)] = 2879,
  [SMALL_STATE(55)] = 2935,
  [SMALL_STATE(56)] = 2977,
  [SMALL_STATE(57)] = 3031,
  [SMALL_STATE(58)] = 3085,
  [SMALL_STATE(59)] = 3127,
  [SMALL_STATE(60)] = 3166,
  [SMALL_STATE(61)] = 3205,
  [SMALL_STATE(62)] = 3244,
  [SMALL_STATE(63)] = 3283,
  [SMALL_STATE(64)] = 3322,
  [SMALL_STATE(65)] = 3375,
  [SMALL_STATE(66)] = 3414,
  [SMALL_STATE(67)] = 3453,
  [SMALL_STATE(68)] = 3492,
  [SMALL_STATE(69)] = 3531,
  [SMALL_STATE(70)] = 3570,
  [SMALL_STATE(71)] = 3623,
  [SMALL_STATE(72)] = 3662,
  [SMALL_STATE(73)] = 3701,
  [SMALL_STATE(74)] = 3754,
  [SMALL_STATE(75)] = 3793,
  [SMALL_STATE(76)] = 3832,
  [SMALL_STATE(77)] = 3871,
  [SMALL_STATE(78)] = 3903,
  [SMALL_STATE(79)] = 3935,
  [SMALL_STATE(80)] = 3965,
  [SMALL_STATE(81)] = 3995,
  [SMALL_STATE(82)] = 4040,
  [SMALL_STATE(83)] = 4085,
  [SMALL_STATE(84)] = 4110,
  [SMALL_STATE(85)] = 4150,
  [SMALL_STATE(86)] = 4174,
  [SMALL_STATE(87)] = 4214,
  [SMALL_STATE(88)] = 4253,
  [SMALL_STATE(89)] = 4292,
  [SMALL_STATE(90)] = 4323,
  [SMALL_STATE(91)] = 4353,
  [SMALL_STATE(92)] = 4383,
  [SMALL_STATE(93)] = 4409,
  [SMALL_STATE(94)] = 4429,
  [SMALL_STATE(95)] = 4455,
  [SMALL_STATE(96)] = 4475,
  [SMALL_STATE(97)] = 4495,
  [SMALL_STATE(98)] = 4515,
  [SMALL_STATE(99)] = 4541,
  [SMALL_STATE(100)] = 4567,
  [SMALL_STATE(101)] = 4587,
  [SMALL_STATE(102)] = 4613,
  [SMALL_STATE(103)] = 4636,
  [SMALL_STATE(104)] = 4657,
  [SMALL_STATE(105)] = 4680,
  [SMALL_STATE(106)] = 4694,
  [SMALL_STATE(107)] = 4708,
  [SMALL_STATE(108)] = 4722,
  [SMALL_STATE(109)] = 4736,
  [SMALL_STATE(110)] = 4755,
  [SMALL_STATE(111)] = 4774,
  [SMALL_STATE(112)] = 4793,
  [SMALL_STATE(113)] = 4812,
  [SMALL_STATE(114)] = 4835,
  [SMALL_STATE(115)] = 4858,
  [SMALL_STATE(116)] = 4881,
  [SMALL_STATE(117)] = 4904,
  [SMALL_STATE(118)] = 4923,
  [SMALL_STATE(119)] = 4942,
  [SMALL_STATE(120)] = 4965,
  [SMALL_STATE(121)] = 4984,
  [SMALL_STATE(122)] = 5007,
  [SMALL_STATE(123)] = 5030,
  [SMALL_STATE(124)] = 5053,
  [SMALL_STATE(125)] = 5072,
  [SMALL_STATE(126)] = 5091,
  [SMALL_STATE(127)] = 5107,
  [SMALL_STATE(128)] = 5127,
  [SMALL_STATE(129)] = 5145,
  [SMALL_STATE(130)] = 5163,
  [SMALL_STATE(131)] = 5183,
  [SMALL_STATE(132)] = 5203,
  [SMALL_STATE(133)] = 5223,
  [SMALL_STATE(134)] = 5235,
  [SMALL_STATE(135)] = 5246,
  [SMALL_STATE(136)] = 5257,
  [SMALL_STATE(137)] = 5268,
  [SMALL_STATE(138)] = 5279,
  [SMALL_STATE(139)] = 5290,
  [SMALL_STATE(140)] = 5301,
  [SMALL_STATE(141)] = 5316,
  [SMALL_STATE(142)] = 5327,
  [SMALL_STATE(143)] = 5338,
  [SMALL_STATE(144)] = 5353,
  [SMALL_STATE(145)] = 5370,
  [SMALL_STATE(146)] = 5381,
  [SMALL_STATE(147)] = 5392,
  [SMALL_STATE(148)] = 5409,
  [SMALL_STATE(149)] = 5426,
  [SMALL_STATE(150)] = 5443,
  [SMALL_STATE(151)] = 5456,
  [SMALL_STATE(152)] = 5467,
  [SMALL_STATE(153)] = 5484,
  [SMALL_STATE(154)] = 5501,
  [SMALL_STATE(155)] = 5518,
  [SMALL_STATE(156)] = 5535,
  [SMALL_STATE(157)] = 5552,
  [SMALL_STATE(158)] = 5569,
  [SMALL_STATE(159)] = 5580,
  [SMALL_STATE(160)] = 5595,
  [SMALL_STATE(161)] = 5612,
  [SMALL_STATE(162)] = 5629,
  [SMALL_STATE(163)] = 5644,
  [SMALL_STATE(164)] = 5655,
  [SMALL_STATE(165)] = 5672,
  [SMALL_STATE(166)] = 5683,
  [SMALL_STATE(167)] = 5700,
  [SMALL_STATE(168)] = 5711,
  [SMALL_STATE(169)] = 5722,
  [SMALL_STATE(170)] = 5739,
  [SMALL_STATE(171)] = 5756,
  [SMALL_STATE(172)] = 5767,
  [SMALL_STATE(173)] = 5778,
  [SMALL_STATE(174)] = 5792,
  [SMALL_STATE(175)] = 5806,
  [SMALL_STATE(176)] = 5820,
  [SMALL_STATE(177)] = 5834,
  [SMALL_STATE(178)] = 5848,
  [SMALL_STATE(179)] = 5862,
  [SMALL_STATE(180)] = 5874,
  [SMALL_STATE(181)] = 5888,
  [SMALL_STATE(182)] = 5902,
  [SMALL_STATE(183)] = 5916,
  [SMALL_STATE(184)] = 5930,
  [SMALL_STATE(185)] = 5940,
  [SMALL_STATE(186)] = 5950,
  [SMALL_STATE(187)] = 5962,
  [SMALL_STATE(188)] = 5972,
  [SMALL_STATE(189)] = 5986,
  [SMALL_STATE(190)] = 6000,
  [SMALL_STATE(191)] = 6011,
  [SMALL_STATE(192)] = 6020,
  [SMALL_STATE(193)] = 6029,
  [SMALL_STATE(194)] = 6038,
  [SMALL_STATE(195)] = 6047,
  [SMALL_STATE(196)] = 6058,
  [SMALL_STATE(197)] = 6067,
  [SMALL_STATE(198)] = 6076,
  [SMALL_STATE(199)] = 6087,
  [SMALL_STATE(200)] = 6096,
  [SMALL_STATE(201)] = 6105,
  [SMALL_STATE(202)] = 6114,
  [SMALL_STATE(203)] = 6125,
  [SMALL_STATE(204)] = 6134,
  [SMALL_STATE(205)] = 6143,
  [SMALL_STATE(206)] = 6152,
  [SMALL_STATE(207)] = 6161,
  [SMALL_STATE(208)] = 6170,
  [SMALL_STATE(209)] = 6179,
  [SMALL_STATE(210)] = 6188,
  [SMALL_STATE(211)] = 6196,
  [SMALL_STATE(212)] = 6204,
  [SMALL_STATE(213)] = 6212,
  [SMALL_STATE(214)] = 6220,
  [SMALL_STATE(215)] = 6228,
  [SMALL_STATE(216)] = 6236,
  [SMALL_STATE(217)] = 6244,
  [SMALL_STATE(218)] = 6252,
  [SMALL_STATE(219)] = 6260,
  [SMALL_STATE(220)] = 6268,
  [SMALL_STATE(221)] = 6276,
  [SMALL_STATE(222)] = 6284,
  [SMALL_STATE(223)] = 6292,
  [SMALL_STATE(224)] = 6300,
  [SMALL_STATE(225)] = 6308,
  [SMALL_STATE(226)] = 6316,
  [SMALL_STATE(227)] = 6324,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(187),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(227),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(163),
  [19] = {.entry = {.count = 1, .reusable = false}}, SHIFT(83),
  [21] = {.entry = {.count = 1, .reusable = false}}, SHIFT(85),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(69),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(90),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(212),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(211),
  [31] = {.entry = {.count = 1, .reusable = false}}, SHIFT(104),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(118),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(190),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(151),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(135),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(142),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(165),
  [61] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_namespace_list, 1, .production_id = 1),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_namespace_list, 1, .production_id = 1),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(214),
  [67] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_namespace_list, 2, .production_id = 2),
  [69] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_namespace_list, 2, .production_id = 2),
  [71] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_namespace_list_repeat1, 2, .production_id = 5),
  [73] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_namespace_list_repeat1, 2, .production_id = 5),
  [75] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_namespace_list_repeat1, 2, .production_id = 5), SHIFT_REPEAT(214),
  [78] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_namespace_list_repeat1, 2, .production_id = 3),
  [80] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_namespace_list_repeat1, 2, .production_id = 3),
  [82] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 2, .production_id = 14),
  [84] = {.entry = {.count = 1, .reusable = true}}, SHIFT(131),
  [86] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 2, .production_id = 14),
  [88] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 1, .production_id = 9),
  [90] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 1, .production_id = 9),
  [92] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_op, 2, .production_id = 19),
  [94] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_op, 2, .production_id = 19),
  [96] = {.entry = {.count = 1, .reusable = false}}, SHIFT(222),
  [98] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [100] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [102] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_op, 3, .production_id = 31),
  [104] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_op, 3, .production_id = 31),
  [106] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [108] = {.entry = {.count = 1, .reusable = false}}, SHIFT(65),
  [110] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [112] = {.entry = {.count = 1, .reusable = false}}, SHIFT(60),
  [114] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [116] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [118] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [120] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 3),
  [122] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 3),
  [124] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 4, .production_id = 3),
  [126] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 4, .production_id = 3),
  [128] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 3, .production_id = 24),
  [130] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 3, .production_id = 24),
  [132] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 5, .production_id = 36),
  [134] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 5, .production_id = 36),
  [136] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 6, .production_id = 36),
  [138] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 6, .production_id = 36),
  [140] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 4, .production_id = 26),
  [142] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 4, .production_id = 26),
  [144] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 29),
  [146] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 29),
  [148] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 4, .production_id = 6),
  [150] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 4, .production_id = 6),
  [152] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 5, .production_id = 26),
  [154] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 5, .production_id = 26),
  [156] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 5, .production_id = 6),
  [158] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 5, .production_id = 6),
  [160] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 2),
  [162] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 2),
  [164] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 3, .production_id = 3),
  [166] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 3, .production_id = 3),
  [168] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 2, .production_id = 17),
  [170] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 2, .production_id = 17),
  [172] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 29),
  [174] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 29),
  [176] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 4, .production_id = 6),
  [178] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 4, .production_id = 6),
  [180] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__linebreak, 2),
  [182] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__linebreak, 2),
  [184] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__linebreak, 2), SHIFT_REPEAT(40),
  [187] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 3, .production_id = 3),
  [189] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 3, .production_id = 3),
  [191] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [193] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 2, .production_id = 21),
  [195] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 2, .production_id = 21),
  [197] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 2),
  [199] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 2),
  [201] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_access, 3, .production_id = 32),
  [203] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_access, 3, .production_id = 32),
  [205] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_op, 2, .production_id = 16),
  [207] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_op, 2, .production_id = 16),
  [209] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_latency_specifier, 2, .production_id = 29),
  [211] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_latency_specifier, 2, .production_id = 29),
  [213] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [215] = {.entry = {.count = 1, .reusable = false}}, SHIFT(59),
  [217] = {.entry = {.count = 1, .reusable = true}}, SHIFT(222),
  [219] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [221] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [223] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_to, 1, .production_id = 10),
  [225] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_to, 1, .production_id = 10),
  [227] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_to, 2, .production_id = 20),
  [229] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_to, 2, .production_id = 20),
  [231] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__type, 1),
  [233] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expression, 1),
  [235] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expression, 1),
  [237] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym__type, 1), REDUCE(sym__expression, 1),
  [240] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_arg, 3, .production_id = 40),
  [242] = {.entry = {.count = 1, .reusable = false}}, SHIFT(111),
  [244] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [246] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 3),
  [248] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_decl_assign_statement, 3, .production_id = 30),
  [250] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [252] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [254] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [256] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [258] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [260] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [262] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [264] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [266] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [268] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [270] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [272] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [274] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [276] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [278] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [280] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [282] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [284] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [286] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [288] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [290] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [292] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__comma, 1),
  [294] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__comma, 1),
  [296] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [298] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__comma, 2),
  [300] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__comma, 2),
  [302] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_write_modifiers, 1, .production_id = 11),
  [304] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_write_modifiers, 1, .production_id = 11),
  [306] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 5),
  [308] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 5), SHIFT_REPEAT(83),
  [311] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 5),
  [313] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [315] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [317] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 1, .production_id = 1),
  [319] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_write_modifiers_repeat1, 1, .production_id = 1),
  [321] = {.entry = {.count = 1, .reusable = false}}, SHIFT(103),
  [323] = {.entry = {.count = 1, .reusable = true}}, SHIFT(215),
  [325] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [327] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_write_modifiers, 1, .production_id = 1),
  [329] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_write_modifiers, 1, .production_id = 1),
  [331] = {.entry = {.count = 1, .reusable = true}}, SHIFT(220),
  [333] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [335] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 4, .production_id = 6),
  [337] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [339] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 2, .production_id = 15),
  [341] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [343] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [345] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 33),
  [347] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 22),
  [349] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 23),
  [351] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 3),
  [353] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 2),
  [355] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__linebreak, 2), SHIFT_REPEAT(100),
  [358] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 1),
  [360] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_type, 1, .production_id = 8),
  [362] = {.entry = {.count = 1, .reusable = false}}, SHIFT(110),
  [364] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 25),
  [366] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 34),
  [368] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 35),
  [370] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 5, .production_id = 39),
  [372] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 2, .production_id = 2),
  [374] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration_list, 1, .production_id = 1),
  [376] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [378] = {.entry = {.count = 1, .reusable = true}}, SHIFT(182),
  [380] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [382] = {.entry = {.count = 1, .reusable = true}}, SHIFT(174),
  [384] = {.entry = {.count = 1, .reusable = true}}, SHIFT(226),
  [386] = {.entry = {.count = 1, .reusable = true}}, SHIFT(188),
  [388] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [390] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [392] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 5),
  [394] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 5), SHIFT_REPEAT(77),
  [397] = {.entry = {.count = 1, .reusable = true}}, SHIFT(216),
  [399] = {.entry = {.count = 1, .reusable = true}}, SHIFT(181),
  [401] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 5),
  [403] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 5), SHIFT_REPEAT(77),
  [406] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [408] = {.entry = {.count = 1, .reusable = true}}, SHIFT(177),
  [410] = {.entry = {.count = 1, .reusable = true}}, SHIFT(218),
  [412] = {.entry = {.count = 1, .reusable = true}}, SHIFT(175),
  [414] = {.entry = {.count = 1, .reusable = true}}, SHIFT(217),
  [416] = {.entry = {.count = 1, .reusable = true}}, SHIFT(183),
  [418] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 1, .production_id = 1),
  [420] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration_list, 2, .production_id = 2),
  [422] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_arg, 4, .production_id = 46),
  [424] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [426] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [428] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [430] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_args_repeat1, 2, .production_id = 5),
  [432] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_args_repeat1, 2, .production_id = 5), SHIFT_REPEAT(77),
  [435] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_declaration_arguments_repeat1, 2, .production_id = 5),
  [437] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_declaration_arguments_repeat1, 2, .production_id = 5), SHIFT_REPEAT(77),
  [440] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [442] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [444] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [446] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [448] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
  [450] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [452] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type, 2, .production_id = 16),
  [454] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, .production_id = 3),
  [456] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 6, .production_id = 36),
  [458] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 6),
  [460] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 3),
  [462] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 45),
  [464] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 3),
  [466] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_statement, 2, .production_id = 18),
  [468] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [470] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 26),
  [472] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [474] = {.entry = {.count = 1, .reusable = true}}, SHIFT(172),
  [476] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [478] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [480] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [482] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 5),
  [484] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 5), SHIFT_REPEAT(77),
  [487] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_arg, 1, .production_id = 8),
  [489] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [491] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [493] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [495] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 5),
  [497] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 5), SHIFT_REPEAT(10),
  [500] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [502] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [504] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 6),
  [506] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [508] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 5),
  [510] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 5), SHIFT_REPEAT(102),
  [513] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 36),
  [515] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [517] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [519] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 3),
  [521] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [523] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2, .production_id = 38),
  [525] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [527] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 2),
  [529] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [531] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 6),
  [533] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 3),
  [535] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, .production_id = 1),
  [537] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [539] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [541] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 26),
  [543] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [545] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [547] = {.entry = {.count = 1, .reusable = true}}, SHIFT(210),
  [549] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [551] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [553] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [555] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 3),
  [557] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [559] = {.entry = {.count = 1, .reusable = true}}, SHIFT(224),
  [561] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [563] = {.entry = {.count = 1, .reusable = true}}, SHIFT(219),
  [565] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_declaration_arguments_repeat1, 2, .production_id = 3),
  [567] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_args_repeat1, 2, .production_id = 3),
  [569] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 3, .production_id = 27),
  [571] = {.entry = {.count = 1, .reusable = true}}, SHIFT(159),
  [573] = {.entry = {.count = 1, .reusable = true}}, SHIFT(223),
  [575] = {.entry = {.count = 1, .reusable = true}}, SHIFT(213),
  [577] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [579] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 3),
  [581] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_statement, 3, .production_id = 28),
  [583] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_statement, 7, .production_id = 49),
  [585] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 4, .production_id = 48),
  [587] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_domain_statement, 2, .production_id = 18),
  [589] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__interface_ports_output, 3, .production_id = 47),
  [591] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_object, 3, .production_id = 4),
  [593] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 5, .production_id = 41),
  [595] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__interface_ports_output, 2, .production_id = 42),
  [597] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_object, 4, .production_id = 7),
  [599] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 43),
  [601] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2, .production_id = 37),
  [603] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_object, 4, .production_id = 12),
  [605] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 44),
  [607] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_object, 5, .production_id = 13),
  [609] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 5, .production_id = 26),
  [611] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [613] = {.entry = {.count = 1, .reusable = true}}, SHIFT(196),
  [615] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 5, .production_id = 6),
  [617] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [619] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 2),
  [621] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 3, .production_id = 3),
  [623] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 5, .production_id = 36),
  [625] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 4, .production_id = 26),
  [627] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 6, .production_id = 36),
  [629] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 3),
  [631] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [633] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [635] = {.entry = {.count = 1, .reusable = true}}, SHIFT(160),
  [637] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 4, .production_id = 3),
  [639] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [641] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 4, .production_id = 6),
  [643] = {.entry = {.count = 1, .reusable = true}}, SHIFT(166),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_sus() {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .keyword_lex_fn = ts_lex_keywords,
    .keyword_capture_token = sym_identifier,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
