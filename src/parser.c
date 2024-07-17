#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 252
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 97
#define ALIAS_COUNT 0
#define TOKEN_COUNT 51
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 33
#define MAX_ALIAS_SEQUENCE_LENGTH 7
#define PRODUCTION_ID_COUNT 54

enum ts_symbol_identifiers {
  sym_identifier = 1,
  anon_sym___builtin__ = 2,
  anon_sym_extern = 3,
  anon_sym_module = 4,
  anon_sym_LT = 5,
  anon_sym_GT = 6,
  anon_sym_EQ = 7,
  anon_sym_LBRACE = 8,
  anon_sym_RBRACE = 9,
  anon_sym_reg = 10,
  anon_sym_initial = 11,
  anon_sym_if = 12,
  anon_sym_else = 13,
  anon_sym_for = 14,
  anon_sym_in = 15,
  anon_sym_DOT_DOT = 16,
  anon_sym_domain = 17,
  anon_sym_interface = 18,
  anon_sym_COLON = 19,
  anon_sym_DASH_GT = 20,
  anon_sym_input = 21,
  anon_sym_output = 22,
  anon_sym_state = 23,
  anon_sym_gen = 24,
  anon_sym_SQUOTE = 25,
  anon_sym_PLUS = 26,
  anon_sym_DASH = 27,
  anon_sym_STAR = 28,
  anon_sym_BANG = 29,
  anon_sym_PIPE = 30,
  anon_sym_AMP = 31,
  anon_sym_CARET = 32,
  anon_sym_EQ_EQ = 33,
  anon_sym_BANG_EQ = 34,
  anon_sym_LT_EQ = 35,
  anon_sym_GT_EQ = 36,
  anon_sym_SLASH = 37,
  anon_sym_PERCENT = 38,
  anon_sym_DOT = 39,
  anon_sym_LPAREN = 40,
  anon_sym_RPAREN = 41,
  anon_sym_LBRACK = 42,
  anon_sym_RBRACK = 43,
  anon_sym_COLON_COLON = 44,
  anon_sym_SEMI = 45,
  sym_number = 46,
  anon_sym_COMMA = 47,
  anon_sym_LF = 48,
  sym_single_line_comment = 49,
  sym_multi_line_comment = 50,
  sym_source_file = 51,
  sym_source_obj = 52,
  sym_module = 53,
  sym_template_declaration_arguments = 54,
  sym_template_declaration_type = 55,
  sym_block = 56,
  sym_decl_assign_statement = 57,
  sym_assign_left_side = 58,
  sym_assign_to = 59,
  sym_write_modifiers = 60,
  sym_if_statement = 61,
  sym_for_statement = 62,
  sym_domain_statement = 63,
  sym_interface_statement = 64,
  sym_interface_ports = 65,
  sym__interface_ports_output = 66,
  sym_declaration_list = 67,
  sym_declaration = 68,
  sym_latency_specifier = 69,
  sym__type = 70,
  sym_array_type = 71,
  sym__expression = 72,
  sym_unary_op = 73,
  sym_binary_op = 74,
  sym_array_op = 75,
  sym_func_call = 76,
  sym_field_access = 77,
  sym_parenthesis_expression_list = 78,
  sym_parenthesis_expression = 79,
  sym_array_bracket_expression = 80,
  sym_template_global = 81,
  sym_template_type_param = 82,
  sym_template_value_param = 83,
  sym_template_params = 84,
  sym__comma = 85,
  aux_sym__linebreak = 86,
  aux_sym_source_file_repeat1 = 87,
  aux_sym_template_declaration_arguments_repeat1 = 88,
  aux_sym_block_repeat1 = 89,
  aux_sym_assign_left_side_repeat1 = 90,
  aux_sym_write_modifiers_repeat1 = 91,
  aux_sym_declaration_list_repeat1 = 92,
  aux_sym_parenthesis_expression_list_repeat1 = 93,
  aux_sym_template_global_repeat1 = 94,
  aux_sym_template_params_repeat1 = 95,
  aux_sym_template_params_repeat2 = 96,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [anon_sym___builtin__] = "__builtin__",
  [anon_sym_extern] = "extern",
  [anon_sym_module] = "module",
  [anon_sym_LT] = "<",
  [anon_sym_GT] = ">",
  [anon_sym_EQ] = "=",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
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
  [anon_sym_LT_EQ] = "<=",
  [anon_sym_GT_EQ] = ">=",
  [anon_sym_SLASH] = "/",
  [anon_sym_PERCENT] = "%",
  [anon_sym_DOT] = ".",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [anon_sym_COLON_COLON] = "::",
  [anon_sym_SEMI] = ";",
  [sym_number] = "number",
  [anon_sym_COMMA] = ",",
  [anon_sym_LF] = "\n",
  [sym_single_line_comment] = "single_line_comment",
  [sym_multi_line_comment] = "multi_line_comment",
  [sym_source_file] = "source_file",
  [sym_source_obj] = "source_obj",
  [sym_module] = "module",
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
  [sym_template_global] = "template_global",
  [sym_template_type_param] = "template_type_param",
  [sym_template_value_param] = "template_value_param",
  [sym_template_params] = "template_params",
  [sym__comma] = "_comma",
  [aux_sym__linebreak] = "_linebreak",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_template_declaration_arguments_repeat1] = "template_declaration_arguments_repeat1",
  [aux_sym_block_repeat1] = "block_repeat1",
  [aux_sym_assign_left_side_repeat1] = "assign_left_side_repeat1",
  [aux_sym_write_modifiers_repeat1] = "write_modifiers_repeat1",
  [aux_sym_declaration_list_repeat1] = "declaration_list_repeat1",
  [aux_sym_parenthesis_expression_list_repeat1] = "parenthesis_expression_list_repeat1",
  [aux_sym_template_global_repeat1] = "template_global_repeat1",
  [aux_sym_template_params_repeat1] = "template_params_repeat1",
  [aux_sym_template_params_repeat2] = "template_params_repeat2",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_identifier] = sym_identifier,
  [anon_sym___builtin__] = anon_sym___builtin__,
  [anon_sym_extern] = anon_sym_extern,
  [anon_sym_module] = anon_sym_module,
  [anon_sym_LT] = anon_sym_LT,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
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
  [anon_sym_LT_EQ] = anon_sym_LT_EQ,
  [anon_sym_GT_EQ] = anon_sym_GT_EQ,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [anon_sym_PERCENT] = anon_sym_PERCENT,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_COLON_COLON] = anon_sym_COLON_COLON,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [sym_number] = sym_number,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_LF] = anon_sym_LF,
  [sym_single_line_comment] = sym_single_line_comment,
  [sym_multi_line_comment] = sym_multi_line_comment,
  [sym_source_file] = sym_source_file,
  [sym_source_obj] = sym_source_obj,
  [sym_module] = sym_module,
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
  [sym_template_global] = sym_template_global,
  [sym_template_type_param] = sym_template_type_param,
  [sym_template_value_param] = sym_template_value_param,
  [sym_template_params] = sym_template_params,
  [sym__comma] = sym__comma,
  [aux_sym__linebreak] = aux_sym__linebreak,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_template_declaration_arguments_repeat1] = aux_sym_template_declaration_arguments_repeat1,
  [aux_sym_block_repeat1] = aux_sym_block_repeat1,
  [aux_sym_assign_left_side_repeat1] = aux_sym_assign_left_side_repeat1,
  [aux_sym_write_modifiers_repeat1] = aux_sym_write_modifiers_repeat1,
  [aux_sym_declaration_list_repeat1] = aux_sym_declaration_list_repeat1,
  [aux_sym_parenthesis_expression_list_repeat1] = aux_sym_parenthesis_expression_list_repeat1,
  [aux_sym_template_global_repeat1] = aux_sym_template_global_repeat1,
  [aux_sym_template_params_repeat1] = aux_sym_template_params_repeat1,
  [aux_sym_template_params_repeat2] = aux_sym_template_params_repeat2,
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
  [anon_sym_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
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
  [anon_sym_LT_EQ] = {
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
  [anon_sym_RPAREN] = {
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
  [anon_sym_SEMI] = {
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
  [sym_source_obj] = {
    .visible = true,
    .named = true,
  },
  [sym_module] = {
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
  [sym_template_global] = {
    .visible = true,
    .named = true,
  },
  [sym_template_type_param] = {
    .visible = true,
    .named = true,
  },
  [sym_template_value_param] = {
    .visible = true,
    .named = true,
  },
  [sym_template_params] = {
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
  [aux_sym_template_global_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_template_params_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_template_params_repeat2] = {
    .visible = false,
    .named = false,
  },
};

enum ts_field_identifiers {
  field_arg = 1,
  field_arguments = 2,
  field_arr = 3,
  field_arr_idx = 4,
  field_assign_left = 5,
  field_assign_value = 6,
  field_block = 7,
  field_condition = 8,
  field_content = 9,
  field_declaration_modifiers = 10,
  field_default_value = 11,
  field_else_block = 12,
  field_expr_or_decl = 13,
  field_extern_marker = 14,
  field_for_decl = 15,
  field_from = 16,
  field_inputs = 17,
  field_interface_ports = 18,
  field_io_port_modifiers = 19,
  field_is_global_path = 20,
  field_item = 21,
  field_latency_specifier = 22,
  field_left = 23,
  field_name = 24,
  field_object = 25,
  field_operator = 26,
  field_outputs = 27,
  field_right = 28,
  field_template_declaration_arguments = 29,
  field_then_block = 30,
  field_to = 31,
  field_type = 32,
  field_write_modifiers = 33,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_arg] = "arg",
  [field_arguments] = "arguments",
  [field_arr] = "arr",
  [field_arr_idx] = "arr_idx",
  [field_assign_left] = "assign_left",
  [field_assign_value] = "assign_value",
  [field_block] = "block",
  [field_condition] = "condition",
  [field_content] = "content",
  [field_declaration_modifiers] = "declaration_modifiers",
  [field_default_value] = "default_value",
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
  [field_object] = "object",
  [field_operator] = "operator",
  [field_outputs] = "outputs",
  [field_right] = "right",
  [field_template_declaration_arguments] = "template_declaration_arguments",
  [field_then_block] = "then_block",
  [field_to] = "to",
  [field_type] = "type",
  [field_write_modifiers] = "write_modifiers",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 2},
  [4] = {.index = 4, .length = 2},
  [5] = {.index = 6, .length = 1},
  [6] = {.index = 7, .length = 2},
  [7] = {.index = 9, .length = 2},
  [8] = {.index = 11, .length = 2},
  [9] = {.index = 13, .length = 1},
  [10] = {.index = 14, .length = 1},
  [11] = {.index = 15, .length = 1},
  [12] = {.index = 16, .length = 3},
  [13] = {.index = 19, .length = 1},
  [14] = {.index = 20, .length = 2},
  [15] = {.index = 22, .length = 2},
  [16] = {.index = 24, .length = 2},
  [17] = {.index = 26, .length = 2},
  [18] = {.index = 28, .length = 2},
  [19] = {.index = 30, .length = 2},
  [20] = {.index = 32, .length = 2},
  [21] = {.index = 34, .length = 2},
  [22] = {.index = 36, .length = 2},
  [23] = {.index = 38, .length = 3},
  [24] = {.index = 41, .length = 3},
  [25] = {.index = 44, .length = 1},
  [26] = {.index = 45, .length = 3},
  [27] = {.index = 48, .length = 2},
  [28] = {.index = 50, .length = 3},
  [29] = {.index = 53, .length = 3},
  [30] = {.index = 56, .length = 2},
  [31] = {.index = 58, .length = 1},
  [32] = {.index = 59, .length = 1},
  [33] = {.index = 60, .length = 1},
  [34] = {.index = 61, .length = 3},
  [35] = {.index = 64, .length = 4},
  [36] = {.index = 68, .length = 4},
  [37] = {.index = 72, .length = 4},
  [38] = {.index = 76, .length = 1},
  [39] = {.index = 77, .length = 2},
  [40] = {.index = 79, .length = 3},
  [41] = {.index = 82, .length = 1},
  [42] = {.index = 83, .length = 2},
  [43] = {.index = 85, .length = 1},
  [44] = {.index = 86, .length = 1},
  [45] = {.index = 87, .length = 5},
  [46] = {.index = 92, .length = 1},
  [47] = {.index = 93, .length = 2},
  [48] = {.index = 95, .length = 2},
  [49] = {.index = 97, .length = 4},
  [50] = {.index = 101, .length = 2},
  [51] = {.index = 103, .length = 3},
  [52] = {.index = 106, .length = 3},
  [53] = {.index = 109, .length = 4},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_item, 0},
  [1] =
    {field_object, 0},
  [2] =
    {field_extern_marker, 0},
    {field_object, 1},
  [4] =
    {field_item, 0},
    {field_item, 1, .inherited = true},
  [6] =
    {field_item, 1},
  [7] =
    {field_block, 2},
    {field_name, 1},
  [9] =
    {field_item, 0, .inherited = true},
    {field_item, 1, .inherited = true},
  [11] =
    {field_item, 1},
    {field_item, 2, .inherited = true},
  [13] =
    {field_name, 0},
  [14] =
    {field_expr_or_decl, 0},
  [15] =
    {field_item, 0, .inherited = true},
  [16] =
    {field_block, 3},
    {field_name, 1},
    {field_template_declaration_arguments, 2},
  [19] =
    {field_name, 1},
  [20] =
    {field_operator, 0},
    {field_right, 1},
  [22] =
    {field_is_global_path, 0},
    {field_item, 1},
  [24] =
    {field_expr_or_decl, 1},
    {field_write_modifiers, 0},
  [26] =
    {field_name, 1},
    {field_type, 0},
  [28] =
    {field_arr, 0},
    {field_arr_idx, 1},
  [30] =
    {field_arguments, 1},
    {field_name, 0},
  [32] =
    {field_default_value, 2},
    {field_name, 0},
  [34] =
    {field_condition, 1},
    {field_then_block, 2},
  [36] =
    {field_interface_ports, 2},
    {field_name, 1},
  [38] =
    {field_io_port_modifiers, 0},
    {field_name, 2},
    {field_type, 1},
  [41] =
    {field_declaration_modifiers, 0},
    {field_name, 2},
    {field_type, 1},
  [44] =
    {field_content, 1},
  [45] =
    {field_is_global_path, 0},
    {field_item, 1},
    {field_item, 2, .inherited = true},
  [48] =
    {field_assign_left, 0},
    {field_assign_value, 2},
  [50] =
    {field_latency_specifier, 2},
    {field_name, 1},
    {field_type, 0},
  [53] =
    {field_left, 0},
    {field_operator, 1},
    {field_right, 2},
  [56] =
    {field_left, 0},
    {field_name, 2},
  [58] =
    {field_item, 2},
  [59] =
    {field_outputs, 1, .inherited = true},
  [60] =
    {field_inputs, 1},
  [61] =
    {field_block, 3},
    {field_interface_ports, 2},
    {field_name, 1},
  [64] =
    {field_declaration_modifiers, 1},
    {field_io_port_modifiers, 0},
    {field_name, 3},
    {field_type, 2},
  [68] =
    {field_io_port_modifiers, 0},
    {field_latency_specifier, 3},
    {field_name, 2},
    {field_type, 1},
  [72] =
    {field_declaration_modifiers, 0},
    {field_latency_specifier, 3},
    {field_name, 2},
    {field_type, 1},
  [76] =
    {field_arg, 0},
  [77] =
    {field_item, 2},
    {field_item, 3, .inherited = true},
  [79] =
    {field_condition, 1},
    {field_else_block, 4},
    {field_then_block, 2},
  [82] =
    {field_outputs, 1},
  [83] =
    {field_inputs, 1},
    {field_outputs, 2, .inherited = true},
  [85] =
    {field_outputs, 2, .inherited = true},
  [86] =
    {field_inputs, 2},
  [87] =
    {field_declaration_modifiers, 1},
    {field_io_port_modifiers, 0},
    {field_latency_specifier, 4},
    {field_name, 3},
    {field_type, 2},
  [92] =
    {field_outputs, 2},
  [93] =
    {field_inputs, 2},
    {field_outputs, 3, .inherited = true},
  [95] =
    {field_arg, 2},
    {field_name, 0},
  [97] =
    {field_block, 6},
    {field_for_decl, 1},
    {field_from, 3},
    {field_to, 5},
  [101] =
    {field_item, 1},
    {field_item, 3},
  [103] =
    {field_item, 1},
    {field_item, 3},
    {field_item, 4, .inherited = true},
  [106] =
    {field_item, 1},
    {field_item, 2, .inherited = true},
    {field_item, 4},
  [109] =
    {field_item, 1},
    {field_item, 2, .inherited = true},
    {field_item, 4},
    {field_item, 5, .inherited = true},
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
  [57] = 54,
  [58] = 58,
  [59] = 59,
  [60] = 59,
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
  [78] = 70,
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
  [100] = 100,
  [101] = 101,
  [102] = 102,
  [103] = 103,
  [104] = 104,
  [105] = 105,
  [106] = 106,
  [107] = 107,
  [108] = 108,
  [109] = 105,
  [110] = 110,
  [111] = 111,
  [112] = 112,
  [113] = 106,
  [114] = 114,
  [115] = 115,
  [116] = 104,
  [117] = 117,
  [118] = 118,
  [119] = 119,
  [120] = 120,
  [121] = 44,
  [122] = 122,
  [123] = 123,
  [124] = 124,
  [125] = 125,
  [126] = 126,
  [127] = 127,
  [128] = 128,
  [129] = 129,
  [130] = 16,
  [131] = 12,
  [132] = 11,
  [133] = 14,
  [134] = 134,
  [135] = 15,
  [136] = 136,
  [137] = 137,
  [138] = 138,
  [139] = 28,
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
  [154] = 144,
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
  [168] = 18,
  [169] = 169,
  [170] = 20,
  [171] = 171,
  [172] = 22,
  [173] = 23,
  [174] = 174,
  [175] = 24,
  [176] = 176,
  [177] = 177,
  [178] = 178,
  [179] = 179,
  [180] = 180,
  [181] = 161,
  [182] = 25,
  [183] = 31,
  [184] = 32,
  [185] = 143,
  [186] = 19,
  [187] = 187,
  [188] = 188,
  [189] = 189,
  [190] = 190,
  [191] = 191,
  [192] = 192,
  [193] = 193,
  [194] = 147,
  [195] = 195,
  [196] = 150,
  [197] = 197,
  [198] = 198,
  [199] = 159,
  [200] = 151,
  [201] = 201,
  [202] = 158,
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
  [219] = 211,
  [220] = 220,
  [221] = 221,
  [222] = 222,
  [223] = 34,
  [224] = 224,
  [225] = 225,
  [226] = 226,
  [227] = 227,
  [228] = 228,
  [229] = 229,
  [230] = 230,
  [231] = 231,
  [232] = 232,
  [233] = 233,
  [234] = 234,
  [235] = 235,
  [236] = 236,
  [237] = 237,
  [238] = 238,
  [239] = 239,
  [240] = 240,
  [241] = 241,
  [242] = 242,
  [243] = 243,
  [244] = 244,
  [245] = 245,
  [246] = 246,
  [247] = 247,
  [248] = 248,
  [249] = 249,
  [250] = 244,
  [251] = 251,
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
      if (eof) ADVANCE(8);
      if (lookahead == '\n') ADVANCE(44);
      if (lookahead == '!') ADVANCE(24);
      if (lookahead == '%') ADVANCE(33);
      if (lookahead == '&') ADVANCE(26);
      if (lookahead == '\'') ADVANCE(19);
      if (lookahead == '(') ADVANCE(35);
      if (lookahead == ')') ADVANCE(36);
      if (lookahead == '*') ADVANCE(22);
      if (lookahead == '+') ADVANCE(20);
      if (lookahead == ',') ADVANCE(43);
      if (lookahead == '-') ADVANCE(21);
      if (lookahead == '.') ADVANCE(34);
      if (lookahead == '/') ADVANCE(32);
      if (lookahead == ':') ADVANCE(17);
      if (lookahead == ';') ADVANCE(40);
      if (lookahead == '<') ADVANCE(9);
      if (lookahead == '=') ADVANCE(13);
      if (lookahead == '>') ADVANCE(11);
      if (lookahead == '[') ADVANCE(37);
      if (lookahead == ']') ADVANCE(38);
      if (lookahead == '^') ADVANCE(27);
      if (lookahead == '{') ADVANCE(14);
      if (lookahead == '|') ADVANCE(25);
      if (lookahead == '}') ADVANCE(15);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(42);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(41);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(44);
      if (lookahead == '!') ADVANCE(23);
      if (lookahead == '&') ADVANCE(26);
      if (lookahead == '(') ADVANCE(35);
      if (lookahead == ')') ADVANCE(36);
      if (lookahead == '*') ADVANCE(22);
      if (lookahead == '+') ADVANCE(20);
      if (lookahead == ',') ADVANCE(43);
      if (lookahead == '-') ADVANCE(21);
      if (lookahead == '/') ADVANCE(3);
      if (lookahead == ':') ADVANCE(6);
      if (lookahead == ';') ADVANCE(40);
      if (lookahead == '=') ADVANCE(12);
      if (lookahead == '>') ADVANCE(10);
      if (lookahead == '[') ADVANCE(37);
      if (lookahead == '^') ADVANCE(27);
      if (lookahead == '{') ADVANCE(14);
      if (lookahead == '|') ADVANCE(25);
      if (lookahead == '}') ADVANCE(15);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(1)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(42);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(41);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(44);
      if (lookahead == '!') ADVANCE(7);
      if (lookahead == '%') ADVANCE(33);
      if (lookahead == '&') ADVANCE(26);
      if (lookahead == '(') ADVANCE(35);
      if (lookahead == ')') ADVANCE(36);
      if (lookahead == '*') ADVANCE(22);
      if (lookahead == '+') ADVANCE(20);
      if (lookahead == ',') ADVANCE(43);
      if (lookahead == '-') ADVANCE(21);
      if (lookahead == '.') ADVANCE(34);
      if (lookahead == '/') ADVANCE(32);
      if (lookahead == ':') ADVANCE(6);
      if (lookahead == ';') ADVANCE(40);
      if (lookahead == '<') ADVANCE(9);
      if (lookahead == '=') ADVANCE(13);
      if (lookahead == '>') ADVANCE(11);
      if (lookahead == '[') ADVANCE(37);
      if (lookahead == ']') ADVANCE(38);
      if (lookahead == '^') ADVANCE(27);
      if (lookahead == '{') ADVANCE(14);
      if (lookahead == '|') ADVANCE(25);
      if (lookahead == '}') ADVANCE(15);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(2)
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(41);
      END_STATE();
    case 3:
      if (lookahead == '*') ADVANCE(5);
      if (lookahead == '/') ADVANCE(45);
      END_STATE();
    case 4:
      if (lookahead == '*') ADVANCE(4);
      if (lookahead == '/') ADVANCE(46);
      if (lookahead != 0) ADVANCE(5);
      END_STATE();
    case 5:
      if (lookahead == '*') ADVANCE(4);
      if (lookahead != 0) ADVANCE(5);
      END_STATE();
    case 6:
      if (lookahead == ':') ADVANCE(39);
      END_STATE();
    case 7:
      if (lookahead == '=') ADVANCE(29);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(30);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(31);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead == '=') ADVANCE(28);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_DOT_DOT);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '>') ADVANCE(18);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_BANG);
      if (lookahead == '=') ADVANCE(29);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_AMP);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(anon_sym_GT_EQ);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '*') ADVANCE(5);
      if (lookahead == '/') ADVANCE(45);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_DOT);
      if (lookahead == '.') ADVANCE(16);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_RPAREN);
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
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(41);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(42);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(anon_sym_LF);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(sym_single_line_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(45);
      END_STATE();
    case 46:
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
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == '_') ADVANCE(11);
      END_STATE();
    case 2:
      if (lookahead == 'o') ADVANCE(12);
      END_STATE();
    case 3:
      if (lookahead == 'l') ADVANCE(13);
      if (lookahead == 'x') ADVANCE(14);
      END_STATE();
    case 4:
      if (lookahead == 'o') ADVANCE(15);
      END_STATE();
    case 5:
      if (lookahead == 'e') ADVANCE(16);
      END_STATE();
    case 6:
      if (lookahead == 'f') ADVANCE(17);
      if (lookahead == 'n') ADVANCE(18);
      END_STATE();
    case 7:
      if (lookahead == 'o') ADVANCE(19);
      END_STATE();
    case 8:
      if (lookahead == 'u') ADVANCE(20);
      END_STATE();
    case 9:
      if (lookahead == 'e') ADVANCE(21);
      END_STATE();
    case 10:
      if (lookahead == 't') ADVANCE(22);
      END_STATE();
    case 11:
      if (lookahead == 'b') ADVANCE(23);
      END_STATE();
    case 12:
      if (lookahead == 'm') ADVANCE(24);
      END_STATE();
    case 13:
      if (lookahead == 's') ADVANCE(25);
      END_STATE();
    case 14:
      if (lookahead == 't') ADVANCE(26);
      END_STATE();
    case 15:
      if (lookahead == 'r') ADVANCE(27);
      END_STATE();
    case 16:
      if (lookahead == 'n') ADVANCE(28);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_in);
      if (lookahead == 'i') ADVANCE(29);
      if (lookahead == 'p') ADVANCE(30);
      if (lookahead == 't') ADVANCE(31);
      END_STATE();
    case 19:
      if (lookahead == 'd') ADVANCE(32);
      END_STATE();
    case 20:
      if (lookahead == 't') ADVANCE(33);
      END_STATE();
    case 21:
      if (lookahead == 'g') ADVANCE(34);
      END_STATE();
    case 22:
      if (lookahead == 'a') ADVANCE(35);
      END_STATE();
    case 23:
      if (lookahead == 'u') ADVANCE(36);
      END_STATE();
    case 24:
      if (lookahead == 'a') ADVANCE(37);
      END_STATE();
    case 25:
      if (lookahead == 'e') ADVANCE(38);
      END_STATE();
    case 26:
      if (lookahead == 'e') ADVANCE(39);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(anon_sym_for);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_gen);
      END_STATE();
    case 29:
      if (lookahead == 't') ADVANCE(40);
      END_STATE();
    case 30:
      if (lookahead == 'u') ADVANCE(41);
      END_STATE();
    case 31:
      if (lookahead == 'e') ADVANCE(42);
      END_STATE();
    case 32:
      if (lookahead == 'u') ADVANCE(43);
      END_STATE();
    case 33:
      if (lookahead == 'p') ADVANCE(44);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_reg);
      END_STATE();
    case 35:
      if (lookahead == 't') ADVANCE(45);
      END_STATE();
    case 36:
      if (lookahead == 'i') ADVANCE(46);
      END_STATE();
    case 37:
      if (lookahead == 'i') ADVANCE(47);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 39:
      if (lookahead == 'r') ADVANCE(48);
      END_STATE();
    case 40:
      if (lookahead == 'i') ADVANCE(49);
      END_STATE();
    case 41:
      if (lookahead == 't') ADVANCE(50);
      END_STATE();
    case 42:
      if (lookahead == 'r') ADVANCE(51);
      END_STATE();
    case 43:
      if (lookahead == 'l') ADVANCE(52);
      END_STATE();
    case 44:
      if (lookahead == 'u') ADVANCE(53);
      END_STATE();
    case 45:
      if (lookahead == 'e') ADVANCE(54);
      END_STATE();
    case 46:
      if (lookahead == 'l') ADVANCE(55);
      END_STATE();
    case 47:
      if (lookahead == 'n') ADVANCE(56);
      END_STATE();
    case 48:
      if (lookahead == 'n') ADVANCE(57);
      END_STATE();
    case 49:
      if (lookahead == 'a') ADVANCE(58);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(anon_sym_input);
      END_STATE();
    case 51:
      if (lookahead == 'f') ADVANCE(59);
      END_STATE();
    case 52:
      if (lookahead == 'e') ADVANCE(60);
      END_STATE();
    case 53:
      if (lookahead == 't') ADVANCE(61);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(anon_sym_state);
      END_STATE();
    case 55:
      if (lookahead == 't') ADVANCE(62);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(anon_sym_domain);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(anon_sym_extern);
      END_STATE();
    case 58:
      if (lookahead == 'l') ADVANCE(63);
      END_STATE();
    case 59:
      if (lookahead == 'a') ADVANCE(64);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(anon_sym_module);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(anon_sym_output);
      END_STATE();
    case 62:
      if (lookahead == 'i') ADVANCE(65);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(anon_sym_initial);
      END_STATE();
    case 64:
      if (lookahead == 'c') ADVANCE(66);
      END_STATE();
    case 65:
      if (lookahead == 'n') ADVANCE(67);
      END_STATE();
    case 66:
      if (lookahead == 'e') ADVANCE(68);
      END_STATE();
    case 67:
      if (lookahead == '_') ADVANCE(69);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(anon_sym_interface);
      END_STATE();
    case 69:
      if (lookahead == '_') ADVANCE(70);
      END_STATE();
    case 70:
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
  [11] = {.lex_state = 2},
  [12] = {.lex_state = 2},
  [13] = {.lex_state = 1},
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
  [40] = {.lex_state = 2},
  [41] = {.lex_state = 2},
  [42] = {.lex_state = 2},
  [43] = {.lex_state = 1},
  [44] = {.lex_state = 1},
  [45] = {.lex_state = 2},
  [46] = {.lex_state = 2},
  [47] = {.lex_state = 2},
  [48] = {.lex_state = 2},
  [49] = {.lex_state = 2},
  [50] = {.lex_state = 2},
  [51] = {.lex_state = 2},
  [52] = {.lex_state = 2},
  [53] = {.lex_state = 2},
  [54] = {.lex_state = 1},
  [55] = {.lex_state = 2},
  [56] = {.lex_state = 2},
  [57] = {.lex_state = 1},
  [58] = {.lex_state = 1},
  [59] = {.lex_state = 2},
  [60] = {.lex_state = 2},
  [61] = {.lex_state = 1},
  [62] = {.lex_state = 2},
  [63] = {.lex_state = 2},
  [64] = {.lex_state = 1},
  [65] = {.lex_state = 1},
  [66] = {.lex_state = 1},
  [67] = {.lex_state = 1},
  [68] = {.lex_state = 1},
  [69] = {.lex_state = 1},
  [70] = {.lex_state = 1},
  [71] = {.lex_state = 1},
  [72] = {.lex_state = 1},
  [73] = {.lex_state = 1},
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
  [92] = {.lex_state = 1},
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
  [103] = {.lex_state = 0},
  [104] = {.lex_state = 1},
  [105] = {.lex_state = 1},
  [106] = {.lex_state = 1},
  [107] = {.lex_state = 0},
  [108] = {.lex_state = 0},
  [109] = {.lex_state = 1},
  [110] = {.lex_state = 1},
  [111] = {.lex_state = 0},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 1},
  [114] = {.lex_state = 0},
  [115] = {.lex_state = 0},
  [116] = {.lex_state = 1},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 1},
  [119] = {.lex_state = 0},
  [120] = {.lex_state = 0},
  [121] = {.lex_state = 0},
  [122] = {.lex_state = 1},
  [123] = {.lex_state = 0},
  [124] = {.lex_state = 0},
  [125] = {.lex_state = 1},
  [126] = {.lex_state = 1},
  [127] = {.lex_state = 0},
  [128] = {.lex_state = 0},
  [129] = {.lex_state = 1},
  [130] = {.lex_state = 1},
  [131] = {.lex_state = 1},
  [132] = {.lex_state = 1},
  [133] = {.lex_state = 1},
  [134] = {.lex_state = 0},
  [135] = {.lex_state = 1},
  [136] = {.lex_state = 0},
  [137] = {.lex_state = 1},
  [138] = {.lex_state = 0},
  [139] = {.lex_state = 1},
  [140] = {.lex_state = 0},
  [141] = {.lex_state = 0},
  [142] = {.lex_state = 0},
  [143] = {.lex_state = 0},
  [144] = {.lex_state = 0},
  [145] = {.lex_state = 0},
  [146] = {.lex_state = 0},
  [147] = {.lex_state = 1},
  [148] = {.lex_state = 0},
  [149] = {.lex_state = 0},
  [150] = {.lex_state = 1},
  [151] = {.lex_state = 1},
  [152] = {.lex_state = 0},
  [153] = {.lex_state = 0},
  [154] = {.lex_state = 0},
  [155] = {.lex_state = 0},
  [156] = {.lex_state = 0},
  [157] = {.lex_state = 0},
  [158] = {.lex_state = 1},
  [159] = {.lex_state = 1},
  [160] = {.lex_state = 1},
  [161] = {.lex_state = 1},
  [162] = {.lex_state = 0},
  [163] = {.lex_state = 0},
  [164] = {.lex_state = 0},
  [165] = {.lex_state = 0},
  [166] = {.lex_state = 0},
  [167] = {.lex_state = 1},
  [168] = {.lex_state = 1},
  [169] = {.lex_state = 0},
  [170] = {.lex_state = 1},
  [171] = {.lex_state = 0},
  [172] = {.lex_state = 1},
  [173] = {.lex_state = 1},
  [174] = {.lex_state = 1},
  [175] = {.lex_state = 1},
  [176] = {.lex_state = 0},
  [177] = {.lex_state = 0},
  [178] = {.lex_state = 0},
  [179] = {.lex_state = 0},
  [180] = {.lex_state = 1},
  [181] = {.lex_state = 1},
  [182] = {.lex_state = 1},
  [183] = {.lex_state = 1},
  [184] = {.lex_state = 1},
  [185] = {.lex_state = 0},
  [186] = {.lex_state = 1},
  [187] = {.lex_state = 0},
  [188] = {.lex_state = 0},
  [189] = {.lex_state = 1},
  [190] = {.lex_state = 0},
  [191] = {.lex_state = 0},
  [192] = {.lex_state = 0},
  [193] = {.lex_state = 0},
  [194] = {.lex_state = 1},
  [195] = {.lex_state = 0},
  [196] = {.lex_state = 1},
  [197] = {.lex_state = 0},
  [198] = {.lex_state = 0},
  [199] = {.lex_state = 1},
  [200] = {.lex_state = 1},
  [201] = {.lex_state = 1},
  [202] = {.lex_state = 1},
  [203] = {.lex_state = 1},
  [204] = {.lex_state = 0},
  [205] = {.lex_state = 0},
  [206] = {.lex_state = 1},
  [207] = {.lex_state = 0},
  [208] = {.lex_state = 0},
  [209] = {.lex_state = 0},
  [210] = {.lex_state = 0},
  [211] = {.lex_state = 0},
  [212] = {.lex_state = 0},
  [213] = {.lex_state = 1},
  [214] = {.lex_state = 0},
  [215] = {.lex_state = 0},
  [216] = {.lex_state = 0},
  [217] = {.lex_state = 0},
  [218] = {.lex_state = 0},
  [219] = {.lex_state = 0},
  [220] = {.lex_state = 0},
  [221] = {.lex_state = 0},
  [222] = {.lex_state = 1},
  [223] = {.lex_state = 1},
  [224] = {.lex_state = 0},
  [225] = {.lex_state = 0},
  [226] = {.lex_state = 1},
  [227] = {.lex_state = 0},
  [228] = {.lex_state = 0},
  [229] = {.lex_state = 0},
  [230] = {.lex_state = 0},
  [231] = {.lex_state = 0},
  [232] = {.lex_state = 0},
  [233] = {.lex_state = 0},
  [234] = {.lex_state = 0},
  [235] = {.lex_state = 0},
  [236] = {.lex_state = 1},
  [237] = {.lex_state = 0},
  [238] = {.lex_state = 0},
  [239] = {.lex_state = 0},
  [240] = {.lex_state = 0},
  [241] = {.lex_state = 0},
  [242] = {.lex_state = 0},
  [243] = {.lex_state = 0},
  [244] = {.lex_state = 0},
  [245] = {.lex_state = 0},
  [246] = {.lex_state = 0},
  [247] = {.lex_state = 0},
  [248] = {.lex_state = 0},
  [249] = {.lex_state = 0},
  [250] = {.lex_state = 0},
  [251] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym___builtin__] = ACTIONS(1),
    [anon_sym_extern] = ACTIONS(1),
    [anon_sym_module] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
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
    [anon_sym_LT_EQ] = ACTIONS(1),
    [anon_sym_GT_EQ] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_PERCENT] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_LF] = ACTIONS(1),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(248),
    [sym_source_obj] = STATE(153),
    [sym_module] = STATE(238),
    [aux_sym__linebreak] = STATE(98),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym___builtin__] = ACTIONS(7),
    [anon_sym_extern] = ACTIONS(7),
    [anon_sym_module] = ACTIONS(9),
    [anon_sym_LF] = ACTIONS(11),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 27,
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
    STATE(43), 1,
      sym_write_modifiers,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(49), 1,
      sym_template_global,
    STATE(84), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(120), 1,
      sym_assign_to,
    STATE(134), 1,
      sym_assign_left_side,
    STATE(197), 1,
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
    STATE(214), 2,
      sym__type,
      sym_array_type,
    STATE(169), 6,
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
    STATE(47), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [103] = 27,
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
    STATE(43), 1,
      sym_write_modifiers,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(49), 1,
      sym_template_global,
    STATE(84), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(120), 1,
      sym_assign_to,
    STATE(197), 1,
      sym_declaration,
    STATE(215), 1,
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
    STATE(214), 2,
      sym__type,
      sym_array_type,
    STATE(232), 6,
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
    STATE(47), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [206] = 27,
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
    STATE(43), 1,
      sym_write_modifiers,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(49), 1,
      sym_template_global,
    STATE(84), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(120), 1,
      sym_assign_to,
    STATE(197), 1,
      sym_declaration,
    STATE(215), 1,
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
    STATE(214), 2,
      sym__type,
      sym_array_type,
    STATE(232), 6,
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
    STATE(47), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [309] = 27,
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
    STATE(43), 1,
      sym_write_modifiers,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(49), 1,
      sym_template_global,
    STATE(84), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(120), 1,
      sym_assign_to,
    STATE(197), 1,
      sym_declaration,
    STATE(215), 1,
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
    STATE(214), 2,
      sym__type,
      sym_array_type,
    STATE(232), 6,
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
    STATE(47), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [412] = 27,
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
    STATE(43), 1,
      sym_write_modifiers,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(49), 1,
      sym_template_global,
    STATE(84), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(120), 1,
      sym_assign_to,
    STATE(197), 1,
      sym_declaration,
    STATE(215), 1,
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
    STATE(214), 2,
      sym__type,
      sym_array_type,
    STATE(232), 6,
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
    STATE(47), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [515] = 27,
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
    ACTIONS(53), 1,
      anon_sym_RBRACE,
    STATE(43), 1,
      sym_write_modifiers,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(49), 1,
      sym_template_global,
    STATE(84), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(120), 1,
      sym_assign_to,
    STATE(197), 1,
      sym_declaration,
    STATE(215), 1,
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
    STATE(214), 2,
      sym__type,
      sym_array_type,
    STATE(232), 6,
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
    STATE(47), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [618] = 27,
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
    ACTIONS(55), 1,
      anon_sym_RBRACE,
    STATE(43), 1,
      sym_write_modifiers,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(49), 1,
      sym_template_global,
    STATE(84), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(120), 1,
      sym_assign_to,
    STATE(197), 1,
      sym_declaration,
    STATE(215), 1,
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
    STATE(214), 2,
      sym__type,
      sym_array_type,
    STATE(232), 6,
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
    STATE(47), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [721] = 27,
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
    ACTIONS(57), 1,
      anon_sym_RBRACE,
    ACTIONS(59), 1,
      anon_sym_LF,
    STATE(2), 1,
      aux_sym__linebreak,
    STATE(43), 1,
      sym_write_modifiers,
    STATE(49), 1,
      sym_template_global,
    STATE(84), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(120), 1,
      sym_assign_to,
    STATE(128), 1,
      sym_assign_left_side,
    STATE(197), 1,
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
    STATE(214), 2,
      sym__type,
      sym_array_type,
    STATE(198), 6,
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
    STATE(47), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [824] = 26,
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
    STATE(43), 1,
      sym_write_modifiers,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(49), 1,
      sym_template_global,
    STATE(84), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(120), 1,
      sym_assign_to,
    STATE(197), 1,
      sym_declaration,
    STATE(215), 1,
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
    STATE(214), 2,
      sym__type,
      sym_array_type,
    STATE(232), 6,
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
    STATE(47), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [924] = 5,
    ACTIONS(65), 1,
      anon_sym_COLON_COLON,
    STATE(15), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(61), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(63), 21,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [968] = 5,
    ACTIONS(65), 1,
      anon_sym_COLON_COLON,
    STATE(14), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(67), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(69), 21,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1012] = 17,
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
    STATE(43), 1,
      sym_write_modifiers,
    STATE(49), 1,
      sym_template_global,
    STATE(84), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(145), 1,
      sym_assign_to,
    STATE(197), 1,
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
    STATE(214), 2,
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
    STATE(47), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [1080] = 5,
    ACTIONS(65), 1,
      anon_sym_COLON_COLON,
    STATE(15), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(71), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(73), 21,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1124] = 5,
    ACTIONS(79), 1,
      anon_sym_COLON_COLON,
    STATE(15), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(75), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(77), 21,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1168] = 5,
    ACTIONS(65), 1,
      anon_sym_COLON_COLON,
    STATE(11), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(82), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(84), 21,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1212] = 8,
    ACTIONS(90), 1,
      anon_sym_DOT,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
    ACTIONS(88), 20,
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
      anon_sym_RPAREN,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1261] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(96), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(98), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1300] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(100), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(102), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1339] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(104), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(106), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1378] = 15,
    ACTIONS(90), 1,
      anon_sym_DOT,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(112), 1,
      anon_sym_PLUS,
    ACTIONS(114), 1,
      anon_sym_DASH,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_AMP,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(108), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(110), 14,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1441] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(126), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(128), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1480] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(130), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(132), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1519] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(134), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(136), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1558] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(138), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(140), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1597] = 10,
    ACTIONS(90), 1,
      anon_sym_DOT,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(108), 4,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
    ACTIONS(110), 18,
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
      anon_sym_RPAREN,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1650] = 8,
    ACTIONS(90), 1,
      anon_sym_DOT,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(108), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
    ACTIONS(110), 20,
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
      anon_sym_RPAREN,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1699] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(142), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(144), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1738] = 13,
    ACTIONS(90), 1,
      anon_sym_DOT,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(112), 1,
      anon_sym_PLUS,
    ACTIONS(114), 1,
      anon_sym_DASH,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(108), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(110), 16,
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
      anon_sym_RPAREN,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1797] = 14,
    ACTIONS(90), 1,
      anon_sym_DOT,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(112), 1,
      anon_sym_PLUS,
    ACTIONS(114), 1,
      anon_sym_DASH,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(108), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(110), 15,
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
      anon_sym_RPAREN,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1858] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(146), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(148), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1897] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(150), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(152), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1936] = 12,
    ACTIONS(90), 1,
      anon_sym_DOT,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(112), 1,
      anon_sym_PLUS,
    ACTIONS(114), 1,
      anon_sym_DASH,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(108), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(110), 17,
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
      anon_sym_RPAREN,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1993] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(154), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(156), 21,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [2031] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(158), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(160), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [2068] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(162), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(164), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [2105] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(166), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(168), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [2142] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(170), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(172), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [2179] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(174), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(176), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [2216] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(178), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(180), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [2253] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(182), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(184), 22,
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
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [2290] = 17,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(112), 1,
      anon_sym_PLUS,
    ACTIONS(114), 1,
      anon_sym_DASH,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_AMP,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    ACTIONS(188), 1,
      anon_sym_EQ,
    ACTIONS(194), 1,
      anon_sym_DOT,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(192), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
    ACTIONS(190), 6,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [2353] = 12,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(196), 1,
      sym_number,
    STATE(49), 1,
      sym_template_global,
    STATE(176), 1,
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
    STATE(214), 2,
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
    STATE(45), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [2406] = 5,
    ACTIONS(202), 1,
      anon_sym_LF,
    STATE(44), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(198), 12,
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
    ACTIONS(200), 12,
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
  [2445] = 16,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_AMP,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    ACTIONS(194), 1,
      anon_sym_DOT,
    ACTIONS(205), 1,
      anon_sym_EQ,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(207), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_LF,
    ACTIONS(192), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2503] = 18,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_AMP,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    ACTIONS(194), 1,
      anon_sym_DOT,
    ACTIONS(209), 1,
      anon_sym_RPAREN,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    STATE(79), 1,
      sym__comma,
    STATE(162), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(192), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2565] = 16,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_AMP,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    ACTIONS(194), 1,
      anon_sym_DOT,
    ACTIONS(213), 1,
      anon_sym_EQ,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_LF,
    ACTIONS(192), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2623] = 6,
    ACTIONS(65), 1,
      anon_sym_COLON_COLON,
    ACTIONS(217), 1,
      anon_sym_EQ,
    STATE(11), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(82), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(84), 16,
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
      anon_sym_LBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [2660] = 5,
    ACTIONS(219), 1,
      sym_identifier,
    ACTIONS(225), 1,
      anon_sym_LBRACK,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(221), 4,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_SLASH,
    ACTIONS(223), 16,
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
  [2695] = 16,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_AMP,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    ACTIONS(194), 1,
      anon_sym_DOT,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    STATE(210), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(192), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2751] = 16,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_AMP,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    ACTIONS(194), 1,
      anon_sym_DOT,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    STATE(231), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(192), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2807] = 15,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_AMP,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    ACTIONS(194), 1,
      anon_sym_DOT,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(228), 2,
      anon_sym_SEMI,
      anon_sym_COMMA,
    ACTIONS(192), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2861] = 15,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_AMP,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    ACTIONS(194), 1,
      anon_sym_DOT,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(230), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
    ACTIONS(192), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2915] = 9,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(232), 1,
      sym_identifier,
    ACTIONS(234), 1,
      anon_sym_SEMI,
    ACTIONS(236), 1,
      sym_number,
    STATE(154), 1,
      sym_template_value_param,
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
  [2957] = 15,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_AMP,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    ACTIONS(194), 1,
      anon_sym_DOT,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(238), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(192), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3011] = 15,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_AMP,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    ACTIONS(194), 1,
      anon_sym_DOT,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(240), 2,
      anon_sym_SEMI,
      anon_sym_COMMA,
    ACTIONS(192), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3065] = 9,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(232), 1,
      sym_identifier,
    ACTIONS(236), 1,
      sym_number,
    ACTIONS(242), 1,
      anon_sym_SEMI,
    STATE(144), 1,
      sym_template_value_param,
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
  [3107] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(246), 1,
      anon_sym_RPAREN,
    ACTIONS(248), 1,
      sym_number,
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
    STATE(46), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3146] = 15,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_AMP,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    ACTIONS(194), 1,
      anon_sym_DOT,
    ACTIONS(250), 1,
      anon_sym_RBRACK,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(192), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3199] = 15,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_AMP,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    ACTIONS(194), 1,
      anon_sym_DOT,
    ACTIONS(252), 1,
      anon_sym_RBRACK,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(192), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3252] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(232), 1,
      sym_identifier,
    ACTIONS(236), 1,
      sym_number,
    STATE(239), 1,
      sym_template_value_param,
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
  [3291] = 15,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_AMP,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    ACTIONS(194), 1,
      anon_sym_DOT,
    ACTIONS(254), 1,
      anon_sym_RPAREN,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(192), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3344] = 15,
    ACTIONS(90), 1,
      anon_sym_DOT,
    ACTIONS(92), 1,
      anon_sym_LPAREN,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_AMP,
    ACTIONS(122), 1,
      anon_sym_CARET,
    ACTIONS(124), 1,
      anon_sym_SLASH,
    ACTIONS(256), 1,
      anon_sym_DOT_DOT,
    STATE(35), 1,
      sym_array_bracket_expression,
    STATE(41), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(116), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(192), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3397] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(258), 1,
      sym_number,
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
    STATE(63), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3433] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(260), 1,
      sym_number,
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
    STATE(17), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3469] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(262), 1,
      sym_number,
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
    STATE(42), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3505] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(264), 1,
      sym_number,
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
    STATE(51), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3541] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(266), 1,
      sym_number,
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
  [3577] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(268), 1,
      sym_number,
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
    STATE(30), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3613] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(270), 1,
      sym_number,
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
    STATE(60), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3649] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(272), 1,
      sym_number,
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
    STATE(29), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3685] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(274), 1,
      sym_number,
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
  [3721] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(276), 1,
      sym_number,
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
    STATE(50), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3757] = 5,
    ACTIONS(282), 1,
      anon_sym_LF,
    STATE(80), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(278), 7,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(280), 10,
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
  [3789] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(284), 1,
      sym_number,
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
    STATE(33), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3825] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(286), 1,
      sym_number,
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
    STATE(62), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3861] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(288), 1,
      sym_number,
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
    STATE(27), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3897] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(290), 1,
      sym_number,
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
    STATE(59), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3933] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(292), 1,
      sym_number,
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
    STATE(55), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3969] = 5,
    ACTIONS(43), 1,
      anon_sym_LF,
    STATE(44), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(294), 7,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(296), 10,
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
  [4001] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(298), 1,
      sym_number,
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
  [4037] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(300), 1,
      sym_number,
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
    STATE(26), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [4073] = 5,
    ACTIONS(304), 1,
      anon_sym_reg,
    STATE(83), 1,
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
    ACTIONS(307), 10,
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
  [4103] = 5,
    ACTIONS(19), 1,
      anon_sym_reg,
    STATE(83), 1,
      aux_sym_write_modifiers_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(309), 5,
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
  [4133] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(313), 6,
      anon_sym_reg,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(315), 10,
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
  [4158] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(317), 5,
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
  [4182] = 12,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(43), 1,
      anon_sym_LF,
    ACTIONS(321), 1,
      anon_sym_DASH_GT,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(112), 1,
      sym_declaration,
    STATE(124), 1,
      sym_declaration_list,
    STATE(218), 1,
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
    STATE(214), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4224] = 12,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(321), 1,
      anon_sym_DASH_GT,
    ACTIONS(323), 1,
      anon_sym_LF,
    STATE(87), 1,
      aux_sym__linebreak,
    STATE(112), 1,
      sym_declaration,
    STATE(127), 1,
      sym_declaration_list,
    STATE(209), 1,
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
    STATE(214), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4266] = 10,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(325), 1,
      anon_sym_LF,
    STATE(90), 1,
      aux_sym__linebreak,
    STATE(112), 1,
      sym_declaration,
    STATE(212), 1,
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
    STATE(214), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4302] = 10,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(43), 1,
      anon_sym_LF,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(112), 1,
      sym_declaration,
    STATE(207), 1,
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
    STATE(214), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4338] = 7,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    STATE(136), 1,
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
    STATE(214), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4365] = 7,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    STATE(246), 1,
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
    STATE(214), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4392] = 4,
    ACTIONS(329), 1,
      anon_sym_SQUOTE,
    STATE(107), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(327), 7,
      anon_sym_EQ,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4412] = 4,
    ACTIONS(329), 1,
      anon_sym_SQUOTE,
    STATE(103), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(331), 7,
      anon_sym_EQ,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4432] = 4,
    ACTIONS(329), 1,
      anon_sym_SQUOTE,
    STATE(115), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(333), 7,
      anon_sym_EQ,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4452] = 4,
    ACTIONS(329), 1,
      anon_sym_SQUOTE,
    STATE(102), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(335), 7,
      anon_sym_EQ,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4472] = 8,
    ACTIONS(9), 1,
      anon_sym_module,
    ACTIONS(337), 1,
      ts_builtin_sym_end,
    ACTIONS(339), 1,
      anon_sym_LF,
    STATE(121), 1,
      aux_sym__linebreak,
    STATE(230), 1,
      sym_source_obj,
    STATE(238), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
  [4499] = 8,
    ACTIONS(9), 1,
      anon_sym_module,
    ACTIONS(339), 1,
      anon_sym_LF,
    ACTIONS(341), 1,
      ts_builtin_sym_end,
    STATE(121), 1,
      aux_sym__linebreak,
    STATE(192), 1,
      sym_source_obj,
    STATE(238), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
  [4526] = 8,
    ACTIONS(9), 1,
      anon_sym_module,
    ACTIONS(339), 1,
      anon_sym_LF,
    ACTIONS(343), 1,
      ts_builtin_sym_end,
    STATE(121), 1,
      aux_sym__linebreak,
    STATE(230), 1,
      sym_source_obj,
    STATE(238), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
  [4553] = 8,
    ACTIONS(9), 1,
      anon_sym_module,
    ACTIONS(339), 1,
      anon_sym_LF,
    ACTIONS(345), 1,
      ts_builtin_sym_end,
    STATE(121), 1,
      aux_sym__linebreak,
    STATE(230), 1,
      sym_source_obj,
    STATE(238), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
  [4580] = 8,
    ACTIONS(9), 1,
      anon_sym_module,
    ACTIONS(339), 1,
      anon_sym_LF,
    ACTIONS(347), 1,
      ts_builtin_sym_end,
    STATE(121), 1,
      aux_sym__linebreak,
    STATE(230), 1,
      sym_source_obj,
    STATE(238), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
  [4607] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(349), 7,
      anon_sym_EQ,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4621] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(351), 7,
      anon_sym_EQ,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4635] = 6,
    ACTIONS(353), 1,
      sym_identifier,
    ACTIONS(355), 1,
      anon_sym_GT,
    ACTIONS(357), 1,
      anon_sym_COLON_COLON,
    STATE(150), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(180), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4657] = 6,
    ACTIONS(353), 1,
      sym_identifier,
    ACTIONS(357), 1,
      anon_sym_COLON_COLON,
    ACTIONS(359), 1,
      anon_sym_GT,
    STATE(158), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(180), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4679] = 6,
    ACTIONS(353), 1,
      sym_identifier,
    ACTIONS(357), 1,
      anon_sym_COLON_COLON,
    ACTIONS(361), 1,
      anon_sym_GT,
    STATE(161), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(180), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4701] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(363), 7,
      anon_sym_EQ,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4715] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    STATE(91), 1,
      sym__comma,
    STATE(111), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(365), 4,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DASH_GT,
      anon_sym_LF,
  [4735] = 6,
    ACTIONS(353), 1,
      sym_identifier,
    ACTIONS(357), 1,
      anon_sym_COLON_COLON,
    ACTIONS(367), 1,
      anon_sym_GT,
    STATE(202), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(180), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4757] = 5,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(369), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(221), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4777] = 5,
    ACTIONS(373), 1,
      anon_sym_COMMA,
    STATE(91), 1,
      sym__comma,
    STATE(111), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(371), 4,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DASH_GT,
      anon_sym_LF,
  [4797] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    STATE(91), 1,
      sym__comma,
    STATE(108), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(376), 4,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DASH_GT,
      anon_sym_LF,
  [4817] = 6,
    ACTIONS(353), 1,
      sym_identifier,
    ACTIONS(357), 1,
      anon_sym_COLON_COLON,
    ACTIONS(378), 1,
      anon_sym_GT,
    STATE(181), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(180), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4839] = 7,
    ACTIONS(9), 1,
      anon_sym_module,
    ACTIONS(339), 1,
      anon_sym_LF,
    STATE(121), 1,
      aux_sym__linebreak,
    STATE(230), 1,
      sym_source_obj,
    STATE(238), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
  [4863] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(380), 7,
      anon_sym_EQ,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4877] = 6,
    ACTIONS(353), 1,
      sym_identifier,
    ACTIONS(357), 1,
      anon_sym_COLON_COLON,
    ACTIONS(382), 1,
      anon_sym_GT,
    STATE(196), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(180), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4899] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    STATE(13), 1,
      sym__comma,
    STATE(119), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(384), 3,
      anon_sym_EQ,
      anon_sym_RBRACE,
      anon_sym_LF,
  [4918] = 5,
    ACTIONS(386), 1,
      anon_sym_EQ,
    ACTIONS(388), 1,
      anon_sym_COLON_COLON,
    STATE(132), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(84), 3,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COMMA,
  [4937] = 5,
    ACTIONS(392), 1,
      anon_sym_COMMA,
    STATE(13), 1,
      sym__comma,
    STATE(119), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(390), 3,
      anon_sym_EQ,
      anon_sym_RBRACE,
      anon_sym_LF,
  [4956] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    STATE(13), 1,
      sym__comma,
    STATE(117), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(395), 3,
      anon_sym_EQ,
      anon_sym_RBRACE,
      anon_sym_LF,
  [4975] = 4,
    ACTIONS(397), 1,
      anon_sym_LF,
    STATE(121), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(200), 4,
      ts_builtin_sym_end,
      anon_sym___builtin__,
      anon_sym_extern,
      anon_sym_module,
  [4992] = 5,
    ACTIONS(353), 1,
      sym_identifier,
    ACTIONS(357), 1,
      anon_sym_COLON_COLON,
    STATE(226), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(180), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [5011] = 6,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(402), 1,
      anon_sym_COLON,
    STATE(156), 1,
      sym_interface_ports,
    STATE(240), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(400), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5032] = 4,
    ACTIONS(321), 1,
      anon_sym_DASH_GT,
    STATE(216), 1,
      sym__interface_ports_output,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(404), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5048] = 4,
    ACTIONS(357), 1,
      anon_sym_COLON_COLON,
    ACTIONS(406), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(206), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [5064] = 4,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(217), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [5080] = 4,
    ACTIONS(321), 1,
      anon_sym_DASH_GT,
    STATE(208), 1,
      sym__interface_ports_output,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(408), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5096] = 6,
    ACTIONS(410), 1,
      anon_sym_EQ,
    ACTIONS(412), 1,
      anon_sym_RBRACE,
    ACTIONS(414), 1,
      anon_sym_LF,
    STATE(4), 1,
      aux_sym__linebreak,
    STATE(177), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5116] = 4,
    ACTIONS(39), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(220), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [5132] = 4,
    ACTIONS(388), 1,
      anon_sym_COLON_COLON,
    STATE(132), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(84), 3,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COMMA,
  [5148] = 4,
    ACTIONS(388), 1,
      anon_sym_COLON_COLON,
    STATE(133), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(69), 3,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COMMA,
  [5164] = 4,
    ACTIONS(388), 1,
      anon_sym_COLON_COLON,
    STATE(135), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(63), 3,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COMMA,
  [5180] = 4,
    ACTIONS(388), 1,
      anon_sym_COLON_COLON,
    STATE(135), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(73), 3,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COMMA,
  [5196] = 6,
    ACTIONS(410), 1,
      anon_sym_EQ,
    ACTIONS(416), 1,
      anon_sym_RBRACE,
    ACTIONS(418), 1,
      anon_sym_LF,
    STATE(3), 1,
      aux_sym__linebreak,
    STATE(191), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5216] = 4,
    ACTIONS(420), 1,
      anon_sym_COLON_COLON,
    STATE(135), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(77), 3,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COMMA,
  [5232] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(423), 5,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [5244] = 4,
    ACTIONS(357), 1,
      anon_sym_COLON_COLON,
    ACTIONS(406), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(167), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [5260] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(425), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5271] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(144), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5282] = 4,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(427), 1,
      anon_sym_if,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(234), 2,
      sym_block,
      sym_if_statement,
  [5297] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(429), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5308] = 5,
    ACTIONS(431), 1,
      anon_sym_RBRACE,
    ACTIONS(433), 1,
      anon_sym_LF,
    STATE(5), 1,
      aux_sym__linebreak,
    STATE(148), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5325] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(435), 1,
      anon_sym_SEMI,
    STATE(61), 1,
      sym__comma,
    STATE(205), 1,
      aux_sym_template_params_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5342] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(437), 1,
      anon_sym_SEMI,
    STATE(61), 1,
      sym__comma,
    STATE(143), 1,
      aux_sym_template_params_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5359] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(439), 4,
      anon_sym_EQ,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_LF,
  [5370] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(441), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5381] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(443), 1,
      anon_sym_GT,
    STATE(122), 1,
      sym__comma,
    STATE(203), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5398] = 5,
    ACTIONS(445), 1,
      anon_sym_RBRACE,
    ACTIONS(447), 1,
      anon_sym_LF,
    STATE(10), 1,
      aux_sym__linebreak,
    STATE(148), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5415] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(441), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5426] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(450), 1,
      anon_sym_GT,
    STATE(122), 1,
      sym__comma,
    STATE(147), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5443] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(452), 1,
      anon_sym_GT,
    STATE(122), 1,
      sym__comma,
    STATE(203), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5460] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(454), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5471] = 5,
    ACTIONS(456), 1,
      ts_builtin_sym_end,
    ACTIONS(458), 1,
      anon_sym_LF,
    STATE(99), 1,
      aux_sym__linebreak,
    STATE(187), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5488] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(460), 1,
      anon_sym_SEMI,
    STATE(61), 1,
      sym__comma,
    STATE(185), 1,
      aux_sym_template_params_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5505] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(462), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5516] = 4,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    STATE(233), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(464), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5531] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(462), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5542] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(466), 1,
      anon_sym_GT,
    STATE(122), 1,
      sym__comma,
    STATE(151), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5559] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(468), 1,
      anon_sym_GT,
    STATE(122), 1,
      sym__comma,
    STATE(203), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5576] = 5,
    ACTIONS(470), 1,
      anon_sym_GT,
    ACTIONS(472), 1,
      anon_sym_COMMA,
    STATE(160), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(224), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5593] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(475), 1,
      anon_sym_GT,
    STATE(122), 1,
      sym__comma,
    STATE(159), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5610] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(477), 1,
      anon_sym_RPAREN,
    STATE(79), 1,
      sym__comma,
    STATE(188), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5627] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(479), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5638] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(481), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5649] = 5,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(483), 1,
      anon_sym_LT,
    STATE(228), 1,
      sym_template_declaration_arguments,
    STATE(229), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5666] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(481), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5677] = 4,
    ACTIONS(487), 1,
      anon_sym_LBRACK,
    STATE(174), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(485), 2,
      anon_sym_GT,
      anon_sym_COMMA,
  [5692] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(98), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5703] = 5,
    ACTIONS(416), 1,
      anon_sym_RBRACE,
    ACTIONS(418), 1,
      anon_sym_LF,
    STATE(3), 1,
      aux_sym__linebreak,
    STATE(142), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5720] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5731] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(489), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5742] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(128), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5753] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(132), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5764] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(491), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      sym_identifier,
      anon_sym_COMMA,
  [5775] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(136), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5786] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(207), 4,
      anon_sym_EQ,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_LF,
  [5797] = 5,
    ACTIONS(493), 1,
      anon_sym_RBRACE,
    ACTIONS(495), 1,
      anon_sym_LF,
    STATE(8), 1,
      aux_sym__linebreak,
    STATE(148), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5814] = 5,
    ACTIONS(497), 1,
      anon_sym_RBRACE,
    ACTIONS(499), 1,
      anon_sym_LF,
    STATE(6), 1,
      aux_sym__linebreak,
    STATE(148), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5831] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(501), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5842] = 4,
    ACTIONS(487), 1,
      anon_sym_LBRACK,
    STATE(174), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(503), 2,
      anon_sym_GT,
      anon_sym_COMMA,
  [5857] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(505), 1,
      anon_sym_GT,
    STATE(122), 1,
      sym__comma,
    STATE(199), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5874] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(140), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5885] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(148), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5896] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(152), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5907] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(507), 1,
      anon_sym_SEMI,
    STATE(61), 1,
      sym__comma,
    STATE(205), 1,
      aux_sym_template_params_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5924] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(102), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5935] = 5,
    ACTIONS(509), 1,
      ts_builtin_sym_end,
    ACTIONS(511), 1,
      anon_sym_LF,
    STATE(100), 1,
      aux_sym__linebreak,
    STATE(193), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5952] = 5,
    ACTIONS(513), 1,
      anon_sym_RPAREN,
    ACTIONS(515), 1,
      anon_sym_COMMA,
    STATE(79), 1,
      sym__comma,
    STATE(188), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5969] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(518), 1,
      anon_sym_GT,
    STATE(160), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(224), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5986] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(425), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5997] = 5,
    ACTIONS(520), 1,
      anon_sym_RBRACE,
    ACTIONS(522), 1,
      anon_sym_LF,
    STATE(7), 1,
      aux_sym__linebreak,
    STATE(148), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6014] = 5,
    ACTIONS(524), 1,
      ts_builtin_sym_end,
    ACTIONS(526), 1,
      anon_sym_LF,
    STATE(101), 1,
      aux_sym__linebreak,
    STATE(195), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6031] = 5,
    ACTIONS(528), 1,
      ts_builtin_sym_end,
    ACTIONS(530), 1,
      anon_sym_LF,
    STATE(114), 1,
      aux_sym__linebreak,
    STATE(193), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6048] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(533), 1,
      anon_sym_GT,
    STATE(122), 1,
      sym__comma,
    STATE(203), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6065] = 5,
    ACTIONS(535), 1,
      ts_builtin_sym_end,
    ACTIONS(537), 1,
      anon_sym_LF,
    STATE(97), 1,
      aux_sym__linebreak,
    STATE(193), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6082] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(539), 1,
      anon_sym_GT,
    STATE(122), 1,
      sym__comma,
    STATE(194), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6099] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(215), 4,
      anon_sym_EQ,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_LF,
  [6110] = 5,
    ACTIONS(412), 1,
      anon_sym_RBRACE,
    ACTIONS(414), 1,
      anon_sym_LF,
    STATE(4), 1,
      aux_sym__linebreak,
    STATE(178), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6127] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(541), 1,
      anon_sym_GT,
    STATE(122), 1,
      sym__comma,
    STATE(203), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6144] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(543), 1,
      anon_sym_GT,
    STATE(122), 1,
      sym__comma,
    STATE(203), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6161] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(545), 1,
      anon_sym_GT,
    STATE(189), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(224), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6178] = 5,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(547), 1,
      anon_sym_GT,
    STATE(122), 1,
      sym__comma,
    STATE(200), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6195] = 5,
    ACTIONS(549), 1,
      anon_sym_GT,
    ACTIONS(551), 1,
      anon_sym_COMMA,
    STATE(122), 1,
      sym__comma,
    STATE(203), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6212] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(554), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [6223] = 5,
    ACTIONS(556), 1,
      anon_sym_SEMI,
    ACTIONS(558), 1,
      anon_sym_COMMA,
    STATE(61), 1,
      sym__comma,
    STATE(205), 1,
      aux_sym_template_params_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6240] = 4,
    ACTIONS(487), 1,
      anon_sym_LBRACK,
    STATE(174), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(561), 2,
      anon_sym_GT,
      anon_sym_COMMA,
  [6255] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(563), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6265] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(565), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6275] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(567), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6285] = 3,
    ACTIONS(571), 1,
      anon_sym_else,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(569), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6297] = 4,
    ACTIONS(573), 1,
      sym_identifier,
    ACTIONS(575), 1,
      anon_sym_LT,
    STATE(139), 1,
      sym_template_params,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6311] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(577), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6321] = 3,
    ACTIONS(581), 1,
      anon_sym_EQ,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(579), 2,
      anon_sym_GT,
      anon_sym_COMMA,
  [6333] = 4,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(583), 1,
      sym_identifier,
    STATE(174), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6347] = 3,
    ACTIONS(410), 1,
      anon_sym_EQ,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(585), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6359] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(587), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6369] = 4,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(589), 1,
      sym_identifier,
    STATE(174), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6383] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(591), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6393] = 4,
    ACTIONS(593), 1,
      sym_identifier,
    ACTIONS(595), 1,
      anon_sym_LT,
    STATE(28), 1,
      sym_template_params,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6407] = 4,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(597), 1,
      sym_identifier,
    STATE(174), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6421] = 4,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(599), 1,
      sym_identifier,
    STATE(174), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6435] = 4,
    ACTIONS(601), 1,
      sym_identifier,
    ACTIONS(603), 1,
      anon_sym_GT,
    STATE(201), 1,
      sym_template_declaration_type,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6449] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(156), 3,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COMMA,
  [6459] = 3,
    ACTIONS(601), 1,
      sym_identifier,
    STATE(236), 1,
      sym_template_declaration_type,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6470] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(605), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6479] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(607), 2,
      anon_sym_GT,
      anon_sym_COMMA,
  [6488] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(609), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6497] = 3,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    STATE(225), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6508] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(611), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6517] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(613), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6526] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(615), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6535] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(585), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6544] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(617), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6553] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(619), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6562] = 3,
    ACTIONS(9), 1,
      anon_sym_module,
    STATE(237), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6573] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(621), 2,
      anon_sym_GT,
      anon_sym_COMMA,
  [6582] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(623), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6591] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(625), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6600] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(627), 2,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [6609] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(629), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6618] = 2,
    ACTIONS(631), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6626] = 2,
    ACTIONS(633), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6634] = 2,
    ACTIONS(635), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6642] = 2,
    ACTIONS(637), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6650] = 2,
    ACTIONS(639), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6658] = 2,
    ACTIONS(641), 1,
      anon_sym_in,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6666] = 2,
    ACTIONS(643), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6674] = 2,
    ACTIONS(645), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6682] = 2,
    ACTIONS(647), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6690] = 2,
    ACTIONS(649), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6698] = 2,
    ACTIONS(651), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 103,
  [SMALL_STATE(4)] = 206,
  [SMALL_STATE(5)] = 309,
  [SMALL_STATE(6)] = 412,
  [SMALL_STATE(7)] = 515,
  [SMALL_STATE(8)] = 618,
  [SMALL_STATE(9)] = 721,
  [SMALL_STATE(10)] = 824,
  [SMALL_STATE(11)] = 924,
  [SMALL_STATE(12)] = 968,
  [SMALL_STATE(13)] = 1012,
  [SMALL_STATE(14)] = 1080,
  [SMALL_STATE(15)] = 1124,
  [SMALL_STATE(16)] = 1168,
  [SMALL_STATE(17)] = 1212,
  [SMALL_STATE(18)] = 1261,
  [SMALL_STATE(19)] = 1300,
  [SMALL_STATE(20)] = 1339,
  [SMALL_STATE(21)] = 1378,
  [SMALL_STATE(22)] = 1441,
  [SMALL_STATE(23)] = 1480,
  [SMALL_STATE(24)] = 1519,
  [SMALL_STATE(25)] = 1558,
  [SMALL_STATE(26)] = 1597,
  [SMALL_STATE(27)] = 1650,
  [SMALL_STATE(28)] = 1699,
  [SMALL_STATE(29)] = 1738,
  [SMALL_STATE(30)] = 1797,
  [SMALL_STATE(31)] = 1858,
  [SMALL_STATE(32)] = 1897,
  [SMALL_STATE(33)] = 1936,
  [SMALL_STATE(34)] = 1993,
  [SMALL_STATE(35)] = 2031,
  [SMALL_STATE(36)] = 2068,
  [SMALL_STATE(37)] = 2105,
  [SMALL_STATE(38)] = 2142,
  [SMALL_STATE(39)] = 2179,
  [SMALL_STATE(40)] = 2216,
  [SMALL_STATE(41)] = 2253,
  [SMALL_STATE(42)] = 2290,
  [SMALL_STATE(43)] = 2353,
  [SMALL_STATE(44)] = 2406,
  [SMALL_STATE(45)] = 2445,
  [SMALL_STATE(46)] = 2503,
  [SMALL_STATE(47)] = 2565,
  [SMALL_STATE(48)] = 2623,
  [SMALL_STATE(49)] = 2660,
  [SMALL_STATE(50)] = 2695,
  [SMALL_STATE(51)] = 2751,
  [SMALL_STATE(52)] = 2807,
  [SMALL_STATE(53)] = 2861,
  [SMALL_STATE(54)] = 2915,
  [SMALL_STATE(55)] = 2957,
  [SMALL_STATE(56)] = 3011,
  [SMALL_STATE(57)] = 3065,
  [SMALL_STATE(58)] = 3107,
  [SMALL_STATE(59)] = 3146,
  [SMALL_STATE(60)] = 3199,
  [SMALL_STATE(61)] = 3252,
  [SMALL_STATE(62)] = 3291,
  [SMALL_STATE(63)] = 3344,
  [SMALL_STATE(64)] = 3397,
  [SMALL_STATE(65)] = 3433,
  [SMALL_STATE(66)] = 3469,
  [SMALL_STATE(67)] = 3505,
  [SMALL_STATE(68)] = 3541,
  [SMALL_STATE(69)] = 3577,
  [SMALL_STATE(70)] = 3613,
  [SMALL_STATE(71)] = 3649,
  [SMALL_STATE(72)] = 3685,
  [SMALL_STATE(73)] = 3721,
  [SMALL_STATE(74)] = 3757,
  [SMALL_STATE(75)] = 3789,
  [SMALL_STATE(76)] = 3825,
  [SMALL_STATE(77)] = 3861,
  [SMALL_STATE(78)] = 3897,
  [SMALL_STATE(79)] = 3933,
  [SMALL_STATE(80)] = 3969,
  [SMALL_STATE(81)] = 4001,
  [SMALL_STATE(82)] = 4037,
  [SMALL_STATE(83)] = 4073,
  [SMALL_STATE(84)] = 4103,
  [SMALL_STATE(85)] = 4133,
  [SMALL_STATE(86)] = 4158,
  [SMALL_STATE(87)] = 4182,
  [SMALL_STATE(88)] = 4224,
  [SMALL_STATE(89)] = 4266,
  [SMALL_STATE(90)] = 4302,
  [SMALL_STATE(91)] = 4338,
  [SMALL_STATE(92)] = 4365,
  [SMALL_STATE(93)] = 4392,
  [SMALL_STATE(94)] = 4412,
  [SMALL_STATE(95)] = 4432,
  [SMALL_STATE(96)] = 4452,
  [SMALL_STATE(97)] = 4472,
  [SMALL_STATE(98)] = 4499,
  [SMALL_STATE(99)] = 4526,
  [SMALL_STATE(100)] = 4553,
  [SMALL_STATE(101)] = 4580,
  [SMALL_STATE(102)] = 4607,
  [SMALL_STATE(103)] = 4621,
  [SMALL_STATE(104)] = 4635,
  [SMALL_STATE(105)] = 4657,
  [SMALL_STATE(106)] = 4679,
  [SMALL_STATE(107)] = 4701,
  [SMALL_STATE(108)] = 4715,
  [SMALL_STATE(109)] = 4735,
  [SMALL_STATE(110)] = 4757,
  [SMALL_STATE(111)] = 4777,
  [SMALL_STATE(112)] = 4797,
  [SMALL_STATE(113)] = 4817,
  [SMALL_STATE(114)] = 4839,
  [SMALL_STATE(115)] = 4863,
  [SMALL_STATE(116)] = 4877,
  [SMALL_STATE(117)] = 4899,
  [SMALL_STATE(118)] = 4918,
  [SMALL_STATE(119)] = 4937,
  [SMALL_STATE(120)] = 4956,
  [SMALL_STATE(121)] = 4975,
  [SMALL_STATE(122)] = 4992,
  [SMALL_STATE(123)] = 5011,
  [SMALL_STATE(124)] = 5032,
  [SMALL_STATE(125)] = 5048,
  [SMALL_STATE(126)] = 5064,
  [SMALL_STATE(127)] = 5080,
  [SMALL_STATE(128)] = 5096,
  [SMALL_STATE(129)] = 5116,
  [SMALL_STATE(130)] = 5132,
  [SMALL_STATE(131)] = 5148,
  [SMALL_STATE(132)] = 5164,
  [SMALL_STATE(133)] = 5180,
  [SMALL_STATE(134)] = 5196,
  [SMALL_STATE(135)] = 5216,
  [SMALL_STATE(136)] = 5232,
  [SMALL_STATE(137)] = 5244,
  [SMALL_STATE(138)] = 5260,
  [SMALL_STATE(139)] = 5271,
  [SMALL_STATE(140)] = 5282,
  [SMALL_STATE(141)] = 5297,
  [SMALL_STATE(142)] = 5308,
  [SMALL_STATE(143)] = 5325,
  [SMALL_STATE(144)] = 5342,
  [SMALL_STATE(145)] = 5359,
  [SMALL_STATE(146)] = 5370,
  [SMALL_STATE(147)] = 5381,
  [SMALL_STATE(148)] = 5398,
  [SMALL_STATE(149)] = 5415,
  [SMALL_STATE(150)] = 5426,
  [SMALL_STATE(151)] = 5443,
  [SMALL_STATE(152)] = 5460,
  [SMALL_STATE(153)] = 5471,
  [SMALL_STATE(154)] = 5488,
  [SMALL_STATE(155)] = 5505,
  [SMALL_STATE(156)] = 5516,
  [SMALL_STATE(157)] = 5531,
  [SMALL_STATE(158)] = 5542,
  [SMALL_STATE(159)] = 5559,
  [SMALL_STATE(160)] = 5576,
  [SMALL_STATE(161)] = 5593,
  [SMALL_STATE(162)] = 5610,
  [SMALL_STATE(163)] = 5627,
  [SMALL_STATE(164)] = 5638,
  [SMALL_STATE(165)] = 5649,
  [SMALL_STATE(166)] = 5666,
  [SMALL_STATE(167)] = 5677,
  [SMALL_STATE(168)] = 5692,
  [SMALL_STATE(169)] = 5703,
  [SMALL_STATE(170)] = 5720,
  [SMALL_STATE(171)] = 5731,
  [SMALL_STATE(172)] = 5742,
  [SMALL_STATE(173)] = 5753,
  [SMALL_STATE(174)] = 5764,
  [SMALL_STATE(175)] = 5775,
  [SMALL_STATE(176)] = 5786,
  [SMALL_STATE(177)] = 5797,
  [SMALL_STATE(178)] = 5814,
  [SMALL_STATE(179)] = 5831,
  [SMALL_STATE(180)] = 5842,
  [SMALL_STATE(181)] = 5857,
  [SMALL_STATE(182)] = 5874,
  [SMALL_STATE(183)] = 5885,
  [SMALL_STATE(184)] = 5896,
  [SMALL_STATE(185)] = 5907,
  [SMALL_STATE(186)] = 5924,
  [SMALL_STATE(187)] = 5935,
  [SMALL_STATE(188)] = 5952,
  [SMALL_STATE(189)] = 5969,
  [SMALL_STATE(190)] = 5986,
  [SMALL_STATE(191)] = 5997,
  [SMALL_STATE(192)] = 6014,
  [SMALL_STATE(193)] = 6031,
  [SMALL_STATE(194)] = 6048,
  [SMALL_STATE(195)] = 6065,
  [SMALL_STATE(196)] = 6082,
  [SMALL_STATE(197)] = 6099,
  [SMALL_STATE(198)] = 6110,
  [SMALL_STATE(199)] = 6127,
  [SMALL_STATE(200)] = 6144,
  [SMALL_STATE(201)] = 6161,
  [SMALL_STATE(202)] = 6178,
  [SMALL_STATE(203)] = 6195,
  [SMALL_STATE(204)] = 6212,
  [SMALL_STATE(205)] = 6223,
  [SMALL_STATE(206)] = 6240,
  [SMALL_STATE(207)] = 6255,
  [SMALL_STATE(208)] = 6265,
  [SMALL_STATE(209)] = 6275,
  [SMALL_STATE(210)] = 6285,
  [SMALL_STATE(211)] = 6297,
  [SMALL_STATE(212)] = 6311,
  [SMALL_STATE(213)] = 6321,
  [SMALL_STATE(214)] = 6333,
  [SMALL_STATE(215)] = 6347,
  [SMALL_STATE(216)] = 6359,
  [SMALL_STATE(217)] = 6369,
  [SMALL_STATE(218)] = 6383,
  [SMALL_STATE(219)] = 6393,
  [SMALL_STATE(220)] = 6407,
  [SMALL_STATE(221)] = 6421,
  [SMALL_STATE(222)] = 6435,
  [SMALL_STATE(223)] = 6449,
  [SMALL_STATE(224)] = 6459,
  [SMALL_STATE(225)] = 6470,
  [SMALL_STATE(226)] = 6479,
  [SMALL_STATE(227)] = 6488,
  [SMALL_STATE(228)] = 6497,
  [SMALL_STATE(229)] = 6508,
  [SMALL_STATE(230)] = 6517,
  [SMALL_STATE(231)] = 6526,
  [SMALL_STATE(232)] = 6535,
  [SMALL_STATE(233)] = 6544,
  [SMALL_STATE(234)] = 6553,
  [SMALL_STATE(235)] = 6562,
  [SMALL_STATE(236)] = 6573,
  [SMALL_STATE(237)] = 6582,
  [SMALL_STATE(238)] = 6591,
  [SMALL_STATE(239)] = 6600,
  [SMALL_STATE(240)] = 6609,
  [SMALL_STATE(241)] = 6618,
  [SMALL_STATE(242)] = 6626,
  [SMALL_STATE(243)] = 6634,
  [SMALL_STATE(244)] = 6642,
  [SMALL_STATE(245)] = 6650,
  [SMALL_STATE(246)] = 6658,
  [SMALL_STATE(247)] = 6666,
  [SMALL_STATE(248)] = 6674,
  [SMALL_STATE(249)] = 6682,
  [SMALL_STATE(250)] = 6690,
  [SMALL_STATE(251)] = 6698,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(235),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(249),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [19] = {.entry = {.count = 1, .reusable = false}}, SHIFT(85),
  [21] = {.entry = {.count = 1, .reusable = false}}, SHIFT(86),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(73),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(92),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(245),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(251),
  [31] = {.entry = {.count = 1, .reusable = false}}, SHIFT(110),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(129),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(250),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(163),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(152),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(138),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(155),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(190),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(204),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [61] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 2, .production_id = 4),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 2, .production_id = 4),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(219),
  [67] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 2, .production_id = 15),
  [69] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 2, .production_id = 15),
  [71] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 3, .production_id = 26),
  [73] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 3, .production_id = 26),
  [75] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 7),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 7),
  [79] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 7), SHIFT_REPEAT(219),
  [82] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 1, .production_id = 1),
  [84] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 1, .production_id = 1),
  [86] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_op, 2, .production_id = 14),
  [88] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_op, 2, .production_id = 14),
  [90] = {.entry = {.count = 1, .reusable = false}}, SHIFT(242),
  [92] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [94] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [96] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 7, .production_id = 53),
  [98] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 7, .production_id = 53),
  [100] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 3),
  [102] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 3),
  [104] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 6, .production_id = 52),
  [106] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 6, .production_id = 52),
  [108] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_op, 3, .production_id = 29),
  [110] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_op, 3, .production_id = 29),
  [112] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [114] = {.entry = {.count = 1, .reusable = false}}, SHIFT(82),
  [116] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [118] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [120] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [122] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [124] = {.entry = {.count = 1, .reusable = false}}, SHIFT(77),
  [126] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 6, .production_id = 51),
  [128] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 6, .production_id = 51),
  [130] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 5, .production_id = 8),
  [132] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 5, .production_id = 8),
  [134] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 5, .production_id = 50),
  [136] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 5, .production_id = 50),
  [138] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 5, .production_id = 39),
  [140] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 5, .production_id = 39),
  [142] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 5),
  [144] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 5),
  [146] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 4, .production_id = 5),
  [148] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 4, .production_id = 5),
  [150] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 4, .production_id = 31),
  [152] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 4, .production_id = 31),
  [154] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 25),
  [156] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 25),
  [158] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_op, 2, .production_id = 18),
  [160] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_op, 2, .production_id = 18),
  [162] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 2),
  [164] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 2),
  [166] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 3, .production_id = 5),
  [168] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 3, .production_id = 5),
  [170] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 25),
  [172] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 25),
  [174] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 4, .production_id = 8),
  [176] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 4, .production_id = 8),
  [178] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_access, 3, .production_id = 30),
  [180] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_access, 3, .production_id = 30),
  [182] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 2, .production_id = 19),
  [184] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 2, .production_id = 19),
  [186] = {.entry = {.count = 1, .reusable = false}}, SHIFT(81),
  [188] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_latency_specifier, 2, .production_id = 25),
  [190] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_latency_specifier, 2, .production_id = 25),
  [192] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [194] = {.entry = {.count = 1, .reusable = true}}, SHIFT(242),
  [196] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [198] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__linebreak, 2),
  [200] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__linebreak, 2),
  [202] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__linebreak, 2), SHIFT_REPEAT(44),
  [205] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_to, 2, .production_id = 16),
  [207] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_to, 2, .production_id = 16),
  [209] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [211] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [213] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_to, 1, .production_id = 10),
  [215] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_to, 1, .production_id = 10),
  [217] = {.entry = {.count = 1, .reusable = false}}, SHIFT(68),
  [219] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__type, 1),
  [221] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expression, 1),
  [223] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expression, 1),
  [225] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym__type, 1), REDUCE(sym__expression, 1),
  [228] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_value_param, 1, .production_id = 38),
  [230] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_decl_assign_statement, 3, .production_id = 27),
  [232] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [234] = {.entry = {.count = 1, .reusable = true}}, SHIFT(113),
  [236] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [238] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 5),
  [240] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_value_param, 3, .production_id = 48),
  [242] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [244] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [246] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [248] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [250] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [252] = {.entry = {.count = 1, .reusable = true}}, SHIFT(223),
  [254] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [256] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [258] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [260] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [262] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [264] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [266] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [268] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [270] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [272] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [274] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [276] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [278] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__comma, 1),
  [280] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__comma, 1),
  [282] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [284] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [286] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [288] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [290] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [292] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [294] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__comma, 2),
  [296] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__comma, 2),
  [298] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [300] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [302] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 7),
  [304] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 7), SHIFT_REPEAT(85),
  [307] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 7),
  [309] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_write_modifiers, 1, .production_id = 11),
  [311] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_write_modifiers, 1, .production_id = 11),
  [313] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 1, .production_id = 1),
  [315] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_write_modifiers_repeat1, 1, .production_id = 1),
  [317] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_write_modifiers, 1, .production_id = 1),
  [319] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_write_modifiers, 1, .production_id = 1),
  [321] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [323] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [325] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [327] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 2, .production_id = 17),
  [329] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [331] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 35),
  [333] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 23),
  [335] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 24),
  [337] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 4, .production_id = 8),
  [339] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [341] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [343] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 1),
  [345] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 4),
  [347] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 5),
  [349] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 37),
  [351] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 5, .production_id = 45),
  [353] = {.entry = {.count = 1, .reusable = true}}, SHIFT(118),
  [355] = {.entry = {.count = 1, .reusable = true}}, SHIFT(173),
  [357] = {.entry = {.count = 1, .reusable = true}}, SHIFT(244),
  [359] = {.entry = {.count = 1, .reusable = true}}, SHIFT(183),
  [361] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [363] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 28),
  [365] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration_list, 2, .production_id = 4),
  [367] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [369] = {.entry = {.count = 1, .reusable = false}}, SHIFT(126),
  [371] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 7),
  [373] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 7), SHIFT_REPEAT(74),
  [376] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration_list, 1, .production_id = 1),
  [378] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [380] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 36),
  [382] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [384] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 2, .production_id = 4),
  [386] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [388] = {.entry = {.count = 1, .reusable = true}}, SHIFT(211),
  [390] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 7),
  [392] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 7), SHIFT_REPEAT(74),
  [395] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 1, .production_id = 1),
  [397] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__linebreak, 2), SHIFT_REPEAT(121),
  [400] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_statement, 2, .production_id = 13),
  [402] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [404] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 44),
  [406] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [408] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2, .production_id = 33),
  [410] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [412] = {.entry = {.count = 1, .reusable = true}}, SHIFT(179),
  [414] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [416] = {.entry = {.count = 1, .reusable = true}}, SHIFT(141),
  [418] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [420] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 7), SHIFT_REPEAT(211),
  [423] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 5),
  [425] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 6, .production_id = 39),
  [427] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [429] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 31),
  [431] = {.entry = {.count = 1, .reusable = true}}, SHIFT(164),
  [433] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [435] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [437] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [439] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 5),
  [441] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 8),
  [443] = {.entry = {.count = 1, .reusable = true}}, SHIFT(168),
  [445] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 7),
  [447] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 7), SHIFT_REPEAT(10),
  [450] = {.entry = {.count = 1, .reusable = true}}, SHIFT(170),
  [452] = {.entry = {.count = 1, .reusable = true}}, SHIFT(172),
  [454] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 5),
  [456] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, .production_id = 1),
  [458] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [460] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [462] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 8),
  [464] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_statement, 3, .production_id = 22),
  [466] = {.entry = {.count = 1, .reusable = true}}, SHIFT(175),
  [468] = {.entry = {.count = 1, .reusable = true}}, SHIFT(182),
  [470] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_declaration_arguments_repeat1, 2, .production_id = 7),
  [472] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_declaration_arguments_repeat1, 2, .production_id = 7), SHIFT_REPEAT(74),
  [475] = {.entry = {.count = 1, .reusable = true}}, SHIFT(184),
  [477] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [479] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 31),
  [481] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 39),
  [483] = {.entry = {.count = 1, .reusable = true}}, SHIFT(222),
  [485] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_type, 3, .production_id = 20),
  [487] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [489] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [491] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type, 2, .production_id = 18),
  [493] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [495] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [497] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [499] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [501] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, .production_id = 5),
  [503] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_type_param, 1, .production_id = 38),
  [505] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [507] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [509] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 4),
  [511] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [513] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 7),
  [515] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 7), SHIFT_REPEAT(74),
  [518] = {.entry = {.count = 1, .reusable = true}}, SHIFT(243),
  [520] = {.entry = {.count = 1, .reusable = true}}, SHIFT(166),
  [522] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [524] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 5),
  [526] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [528] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 7),
  [530] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 7), SHIFT_REPEAT(114),
  [533] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [535] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 8),
  [537] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [539] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [541] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [543] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [545] = {.entry = {.count = 1, .reusable = true}}, SHIFT(241),
  [547] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [549] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_params_repeat2, 2, .production_id = 7),
  [551] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_params_repeat2, 2, .production_id = 7), SHIFT_REPEAT(74),
  [554] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [556] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_params_repeat1, 2, .production_id = 7),
  [558] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_params_repeat1, 2, .production_id = 7), SHIFT_REPEAT(74),
  [561] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_type_param, 3, .production_id = 48),
  [563] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__interface_ports_output, 3, .production_id = 46),
  [565] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 42),
  [567] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2, .production_id = 32),
  [569] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 3, .production_id = 21),
  [571] = {.entry = {.count = 1, .reusable = true}}, SHIFT(140),
  [573] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [575] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [577] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__interface_ports_output, 2, .production_id = 41),
  [579] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_type, 1, .production_id = 9),
  [581] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [583] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [585] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 5),
  [587] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 4, .production_id = 47),
  [589] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [591] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 43),
  [593] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [595] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [597] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [599] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [601] = {.entry = {.count = 1, .reusable = true}}, SHIFT(213),
  [603] = {.entry = {.count = 1, .reusable = true}}, SHIFT(247),
  [605] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 4, .production_id = 12),
  [607] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_params_repeat2, 2, .production_id = 5),
  [609] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_domain_statement, 2, .production_id = 13),
  [611] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 3, .production_id = 6),
  [613] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 5),
  [615] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_statement, 7, .production_id = 49),
  [617] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_statement, 4, .production_id = 34),
  [619] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 5, .production_id = 40),
  [621] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_declaration_arguments_repeat1, 2, .production_id = 5),
  [623] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_obj, 2, .production_id = 3),
  [625] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_obj, 1, .production_id = 2),
  [627] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_params_repeat1, 2, .production_id = 5),
  [629] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_statement, 3, .production_id = 6),
  [631] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 3, .production_id = 5),
  [633] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [635] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 4, .production_id = 8),
  [637] = {.entry = {.count = 1, .reusable = true}}, SHIFT(131),
  [639] = {.entry = {.count = 1, .reusable = true}}, SHIFT(227),
  [641] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [643] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 2),
  [645] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [647] = {.entry = {.count = 1, .reusable = true}}, SHIFT(165),
  [649] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [651] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
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
