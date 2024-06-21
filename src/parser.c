#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 247
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 92
#define ALIAS_COUNT 0
#define TOKEN_COUNT 48
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 30
#define MAX_ALIAS_SEQUENCE_LENGTH 7
#define PRODUCTION_ID_COUNT 51

enum ts_symbol_identifiers {
  sym_identifier = 1,
  anon_sym_module = 2,
  anon_sym_COLON = 3,
  anon_sym_DASH_GT = 4,
  anon_sym_LT = 5,
  anon_sym_GT = 6,
  anon_sym_LBRACE = 7,
  anon_sym_RBRACE = 8,
  anon_sym_interface = 9,
  anon_sym_EQ = 10,
  anon_sym_reg = 11,
  anon_sym_initial = 12,
  anon_sym_if = 13,
  anon_sym_else = 14,
  anon_sym_for = 15,
  anon_sym_in = 16,
  anon_sym_DOT_DOT = 17,
  anon_sym_input = 18,
  anon_sym_output = 19,
  anon_sym_state = 20,
  anon_sym_gen = 21,
  anon_sym_SQUOTE = 22,
  anon_sym_PLUS = 23,
  anon_sym_DASH = 24,
  anon_sym_STAR = 25,
  anon_sym_BANG = 26,
  anon_sym_PIPE = 27,
  anon_sym_AMP = 28,
  anon_sym_CARET = 29,
  anon_sym_EQ_EQ = 30,
  anon_sym_BANG_EQ = 31,
  anon_sym_LT_EQ = 32,
  anon_sym_GT_EQ = 33,
  anon_sym_SLASH = 34,
  anon_sym_PERCENT = 35,
  anon_sym_DOT = 36,
  anon_sym_LPAREN = 37,
  anon_sym_RPAREN = 38,
  anon_sym_LBRACK = 39,
  anon_sym_RBRACK = 40,
  anon_sym_COLON_COLON = 41,
  anon_sym_SEMI = 42,
  sym_number = 43,
  anon_sym_COMMA = 44,
  anon_sym_LF = 45,
  sym_single_line_comment = 46,
  sym_multi_line_comment = 47,
  sym_source_file = 48,
  sym_module = 49,
  sym_interface_ports = 50,
  sym__interface_ports_output = 51,
  sym_template_declaration_arguments = 52,
  sym_template_declaration_type = 53,
  sym_block = 54,
  sym_interface_statement = 55,
  sym_decl_assign_statement = 56,
  sym_assign_left_side = 57,
  sym_assign_to = 58,
  sym_write_modifiers = 59,
  sym_if_statement = 60,
  sym_for_statement = 61,
  sym_declaration_list = 62,
  sym_declaration = 63,
  sym_latency_specifier = 64,
  sym__type = 65,
  sym_array_type = 66,
  sym__expression = 67,
  sym_unary_op = 68,
  sym_binary_op = 69,
  sym_array_op = 70,
  sym_func_call = 71,
  sym_field_access = 72,
  sym_parenthesis_expression_list = 73,
  sym_parenthesis_expression = 74,
  sym_array_bracket_expression = 75,
  sym_template_global = 76,
  sym_template_type_param = 77,
  sym_template_value_param = 78,
  sym_template_params = 79,
  sym__comma = 80,
  aux_sym__linebreak = 81,
  aux_sym_source_file_repeat1 = 82,
  aux_sym_template_declaration_arguments_repeat1 = 83,
  aux_sym_block_repeat1 = 84,
  aux_sym_assign_left_side_repeat1 = 85,
  aux_sym_write_modifiers_repeat1 = 86,
  aux_sym_declaration_list_repeat1 = 87,
  aux_sym_parenthesis_expression_list_repeat1 = 88,
  aux_sym_template_global_repeat1 = 89,
  aux_sym_template_params_repeat1 = 90,
  aux_sym_template_params_repeat2 = 91,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [anon_sym_module] = "module",
  [anon_sym_COLON] = ":",
  [anon_sym_DASH_GT] = "->",
  [anon_sym_LT] = "<",
  [anon_sym_GT] = ">",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_interface] = "interface",
  [anon_sym_EQ] = "=",
  [anon_sym_reg] = "reg",
  [anon_sym_initial] = "initial",
  [anon_sym_if] = "if",
  [anon_sym_else] = "else",
  [anon_sym_for] = "for",
  [anon_sym_in] = "in",
  [anon_sym_DOT_DOT] = "..",
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
  [sym_module] = "module",
  [sym_interface_ports] = "interface_ports",
  [sym__interface_ports_output] = "_interface_ports_output",
  [sym_template_declaration_arguments] = "template_declaration_arguments",
  [sym_template_declaration_type] = "template_declaration_type",
  [sym_block] = "block",
  [sym_interface_statement] = "interface_statement",
  [sym_decl_assign_statement] = "decl_assign_statement",
  [sym_assign_left_side] = "assign_left_side",
  [sym_assign_to] = "assign_to",
  [sym_write_modifiers] = "write_modifiers",
  [sym_if_statement] = "if_statement",
  [sym_for_statement] = "for_statement",
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
  [anon_sym_module] = anon_sym_module,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_DASH_GT] = anon_sym_DASH_GT,
  [anon_sym_LT] = anon_sym_LT,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_interface] = anon_sym_interface,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_reg] = anon_sym_reg,
  [anon_sym_initial] = anon_sym_initial,
  [anon_sym_if] = anon_sym_if,
  [anon_sym_else] = anon_sym_else,
  [anon_sym_for] = anon_sym_for,
  [anon_sym_in] = anon_sym_in,
  [anon_sym_DOT_DOT] = anon_sym_DOT_DOT,
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
  [sym_module] = sym_module,
  [sym_interface_ports] = sym_interface_ports,
  [sym__interface_ports_output] = sym__interface_ports_output,
  [sym_template_declaration_arguments] = sym_template_declaration_arguments,
  [sym_template_declaration_type] = sym_template_declaration_type,
  [sym_block] = sym_block,
  [sym_interface_statement] = sym_interface_statement,
  [sym_decl_assign_statement] = sym_decl_assign_statement,
  [sym_assign_left_side] = sym_assign_left_side,
  [sym_assign_to] = sym_assign_to,
  [sym_write_modifiers] = sym_write_modifiers,
  [sym_if_statement] = sym_if_statement,
  [sym_for_statement] = sym_for_statement,
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
  [anon_sym_module] = {
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
  [anon_sym_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
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
  [anon_sym_interface] = {
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
  [sym_module] = {
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
  [sym_interface_statement] = {
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
  field_else_block = 11,
  field_expr_or_decl = 12,
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
  field_operator = 23,
  field_outputs = 24,
  field_right = 25,
  field_template_declaration_arguments = 26,
  field_then_block = 27,
  field_to = 28,
  field_type = 29,
  field_write_modifiers = 30,
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
  [field_else_block] = "else_block",
  [field_expr_or_decl] = "expr_or_decl",
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
  [2] = {.index = 1, .length = 2},
  [3] = {.index = 3, .length = 1},
  [4] = {.index = 4, .length = 2},
  [5] = {.index = 6, .length = 2},
  [6] = {.index = 8, .length = 2},
  [7] = {.index = 10, .length = 1},
  [8] = {.index = 11, .length = 1},
  [9] = {.index = 12, .length = 1},
  [10] = {.index = 13, .length = 1},
  [11] = {.index = 14, .length = 3},
  [12] = {.index = 17, .length = 3},
  [13] = {.index = 20, .length = 1},
  [14] = {.index = 21, .length = 2},
  [15] = {.index = 23, .length = 2},
  [16] = {.index = 25, .length = 2},
  [17] = {.index = 27, .length = 2},
  [18] = {.index = 29, .length = 1},
  [19] = {.index = 30, .length = 1},
  [20] = {.index = 31, .length = 1},
  [21] = {.index = 32, .length = 2},
  [22] = {.index = 34, .length = 2},
  [23] = {.index = 36, .length = 2},
  [24] = {.index = 38, .length = 4},
  [25] = {.index = 42, .length = 1},
  [26] = {.index = 43, .length = 3},
  [27] = {.index = 46, .length = 3},
  [28] = {.index = 49, .length = 3},
  [29] = {.index = 52, .length = 3},
  [30] = {.index = 55, .length = 2},
  [31] = {.index = 57, .length = 2},
  [32] = {.index = 59, .length = 2},
  [33] = {.index = 61, .length = 1},
  [34] = {.index = 62, .length = 2},
  [35] = {.index = 64, .length = 3},
  [36] = {.index = 67, .length = 2},
  [37] = {.index = 69, .length = 1},
  [38] = {.index = 70, .length = 4},
  [39] = {.index = 74, .length = 4},
  [40] = {.index = 78, .length = 4},
  [41] = {.index = 82, .length = 1},
  [42] = {.index = 83, .length = 2},
  [43] = {.index = 85, .length = 5},
  [44] = {.index = 90, .length = 3},
  [45] = {.index = 93, .length = 2},
  [46] = {.index = 95, .length = 2},
  [47] = {.index = 97, .length = 4},
  [48] = {.index = 101, .length = 3},
  [49] = {.index = 104, .length = 3},
  [50] = {.index = 107, .length = 4},
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
  [6] =
    {field_item, 0, .inherited = true},
    {field_item, 1, .inherited = true},
  [8] =
    {field_item, 1},
    {field_item, 2, .inherited = true},
  [10] =
    {field_outputs, 1, .inherited = true},
  [11] =
    {field_inputs, 1},
  [12] =
    {field_expr_or_decl, 0},
  [13] =
    {field_item, 0, .inherited = true},
  [14] =
    {field_block, 3},
    {field_interface_ports, 2},
    {field_name, 1},
  [17] =
    {field_block, 3},
    {field_name, 1},
    {field_template_declaration_arguments, 2},
  [20] =
    {field_outputs, 1},
  [21] =
    {field_is_global_path, 0},
    {field_item, 1},
  [23] =
    {field_inputs, 1},
    {field_outputs, 2, .inherited = true},
  [25] =
    {field_name, 1},
    {field_type, 0},
  [27] =
    {field_arr, 0},
    {field_arr_idx, 1},
  [29] =
    {field_outputs, 2, .inherited = true},
  [30] =
    {field_inputs, 2},
  [31] =
    {field_name, 1},
  [32] =
    {field_operator, 0},
    {field_right, 1},
  [34] =
    {field_expr_or_decl, 1},
    {field_write_modifiers, 0},
  [36] =
    {field_arguments, 1},
    {field_name, 0},
  [38] =
    {field_block, 4},
    {field_interface_ports, 3},
    {field_name, 1},
    {field_template_declaration_arguments, 2},
  [42] =
    {field_outputs, 2},
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
    {field_item, 1},
    {field_item, 2, .inherited = true},
  [52] =
    {field_latency_specifier, 2},
    {field_name, 1},
    {field_type, 0},
  [55] =
    {field_inputs, 2},
    {field_outputs, 3, .inherited = true},
  [57] =
    {field_interface_ports, 2},
    {field_name, 1},
  [59] =
    {field_condition, 1},
    {field_then_block, 2},
  [61] =
    {field_content, 1},
  [62] =
    {field_assign_left, 0},
    {field_assign_value, 2},
  [64] =
    {field_left, 0},
    {field_operator, 1},
    {field_right, 2},
  [67] =
    {field_left, 0},
    {field_name, 2},
  [69] =
    {field_item, 2},
  [70] =
    {field_declaration_modifiers, 1},
    {field_io_port_modifiers, 0},
    {field_name, 3},
    {field_type, 2},
  [74] =
    {field_io_port_modifiers, 0},
    {field_latency_specifier, 3},
    {field_name, 2},
    {field_type, 1},
  [78] =
    {field_declaration_modifiers, 0},
    {field_latency_specifier, 3},
    {field_name, 2},
    {field_type, 1},
  [82] =
    {field_arg, 0},
  [83] =
    {field_item, 2},
    {field_item, 3, .inherited = true},
  [85] =
    {field_declaration_modifiers, 1},
    {field_io_port_modifiers, 0},
    {field_latency_specifier, 4},
    {field_name, 3},
    {field_type, 2},
  [90] =
    {field_condition, 1},
    {field_else_block, 4},
    {field_then_block, 2},
  [93] =
    {field_arg, 2},
    {field_name, 0},
  [95] =
    {field_item, 1},
    {field_item, 3},
  [97] =
    {field_block, 6},
    {field_for_decl, 1},
    {field_from, 3},
    {field_to, 5},
  [101] =
    {field_item, 1},
    {field_item, 3},
    {field_item, 4, .inherited = true},
  [104] =
    {field_item, 1},
    {field_item, 2, .inherited = true},
    {field_item, 4},
  [107] =
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
  [56] = 50,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 58,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 72,
  [73] = 69,
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
  [100] = 100,
  [101] = 101,
  [102] = 102,
  [103] = 103,
  [104] = 97,
  [105] = 105,
  [106] = 99,
  [107] = 107,
  [108] = 108,
  [109] = 109,
  [110] = 108,
  [111] = 111,
  [112] = 112,
  [113] = 113,
  [114] = 114,
  [115] = 115,
  [116] = 116,
  [117] = 117,
  [118] = 15,
  [119] = 119,
  [120] = 13,
  [121] = 12,
  [122] = 122,
  [123] = 11,
  [124] = 124,
  [125] = 125,
  [126] = 126,
  [127] = 127,
  [128] = 128,
  [129] = 14,
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
  [150] = 29,
  [151] = 151,
  [152] = 152,
  [153] = 153,
  [154] = 154,
  [155] = 33,
  [156] = 25,
  [157] = 157,
  [158] = 27,
  [159] = 159,
  [160] = 160,
  [161] = 161,
  [162] = 32,
  [163] = 163,
  [164] = 164,
  [165] = 165,
  [166] = 166,
  [167] = 23,
  [168] = 168,
  [169] = 169,
  [170] = 170,
  [171] = 171,
  [172] = 172,
  [173] = 173,
  [174] = 141,
  [175] = 175,
  [176] = 176,
  [177] = 177,
  [178] = 163,
  [179] = 179,
  [180] = 180,
  [181] = 181,
  [182] = 182,
  [183] = 183,
  [184] = 184,
  [185] = 185,
  [186] = 22,
  [187] = 187,
  [188] = 145,
  [189] = 44,
  [190] = 190,
  [191] = 153,
  [192] = 192,
  [193] = 193,
  [194] = 194,
  [195] = 195,
  [196] = 20,
  [197] = 197,
  [198] = 26,
  [199] = 160,
  [200] = 28,
  [201] = 179,
  [202] = 202,
  [203] = 154,
  [204] = 180,
  [205] = 205,
  [206] = 34,
  [207] = 207,
  [208] = 208,
  [209] = 209,
  [210] = 210,
  [211] = 211,
  [212] = 212,
  [213] = 213,
  [214] = 214,
  [215] = 211,
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
  [242] = 241,
  [243] = 243,
  [244] = 244,
  [245] = 245,
  [246] = 246,
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
      if (lookahead == ':') ADVANCE(9);
      if (lookahead == ';') ADVANCE(40);
      if (lookahead == '<') ADVANCE(11);
      if (lookahead == '=') ADVANCE(17);
      if (lookahead == '>') ADVANCE(13);
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
      if (lookahead == '=') ADVANCE(16);
      if (lookahead == '>') ADVANCE(12);
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
      if (lookahead == '<') ADVANCE(11);
      if (lookahead == '=') ADVANCE(17);
      if (lookahead == '>') ADVANCE(13);
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
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(30);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(31);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead == '=') ADVANCE(28);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_DOT_DOT);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '>') ADVANCE(10);
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
      if (lookahead == '.') ADVANCE(18);
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
      if (lookahead == 'e') ADVANCE(1);
      if (lookahead == 'f') ADVANCE(2);
      if (lookahead == 'g') ADVANCE(3);
      if (lookahead == 'i') ADVANCE(4);
      if (lookahead == 'm') ADVANCE(5);
      if (lookahead == 'o') ADVANCE(6);
      if (lookahead == 'r') ADVANCE(7);
      if (lookahead == 's') ADVANCE(8);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == 'l') ADVANCE(9);
      END_STATE();
    case 2:
      if (lookahead == 'o') ADVANCE(10);
      END_STATE();
    case 3:
      if (lookahead == 'e') ADVANCE(11);
      END_STATE();
    case 4:
      if (lookahead == 'f') ADVANCE(12);
      if (lookahead == 'n') ADVANCE(13);
      END_STATE();
    case 5:
      if (lookahead == 'o') ADVANCE(14);
      END_STATE();
    case 6:
      if (lookahead == 'u') ADVANCE(15);
      END_STATE();
    case 7:
      if (lookahead == 'e') ADVANCE(16);
      END_STATE();
    case 8:
      if (lookahead == 't') ADVANCE(17);
      END_STATE();
    case 9:
      if (lookahead == 's') ADVANCE(18);
      END_STATE();
    case 10:
      if (lookahead == 'r') ADVANCE(19);
      END_STATE();
    case 11:
      if (lookahead == 'n') ADVANCE(20);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_in);
      if (lookahead == 'i') ADVANCE(21);
      if (lookahead == 'p') ADVANCE(22);
      if (lookahead == 't') ADVANCE(23);
      END_STATE();
    case 14:
      if (lookahead == 'd') ADVANCE(24);
      END_STATE();
    case 15:
      if (lookahead == 't') ADVANCE(25);
      END_STATE();
    case 16:
      if (lookahead == 'g') ADVANCE(26);
      END_STATE();
    case 17:
      if (lookahead == 'a') ADVANCE(27);
      END_STATE();
    case 18:
      if (lookahead == 'e') ADVANCE(28);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_for);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_gen);
      END_STATE();
    case 21:
      if (lookahead == 't') ADVANCE(29);
      END_STATE();
    case 22:
      if (lookahead == 'u') ADVANCE(30);
      END_STATE();
    case 23:
      if (lookahead == 'e') ADVANCE(31);
      END_STATE();
    case 24:
      if (lookahead == 'u') ADVANCE(32);
      END_STATE();
    case 25:
      if (lookahead == 'p') ADVANCE(33);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_reg);
      END_STATE();
    case 27:
      if (lookahead == 't') ADVANCE(34);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 29:
      if (lookahead == 'i') ADVANCE(35);
      END_STATE();
    case 30:
      if (lookahead == 't') ADVANCE(36);
      END_STATE();
    case 31:
      if (lookahead == 'r') ADVANCE(37);
      END_STATE();
    case 32:
      if (lookahead == 'l') ADVANCE(38);
      END_STATE();
    case 33:
      if (lookahead == 'u') ADVANCE(39);
      END_STATE();
    case 34:
      if (lookahead == 'e') ADVANCE(40);
      END_STATE();
    case 35:
      if (lookahead == 'a') ADVANCE(41);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_input);
      END_STATE();
    case 37:
      if (lookahead == 'f') ADVANCE(42);
      END_STATE();
    case 38:
      if (lookahead == 'e') ADVANCE(43);
      END_STATE();
    case 39:
      if (lookahead == 't') ADVANCE(44);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(anon_sym_state);
      END_STATE();
    case 41:
      if (lookahead == 'l') ADVANCE(45);
      END_STATE();
    case 42:
      if (lookahead == 'a') ADVANCE(46);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(anon_sym_module);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(anon_sym_output);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(anon_sym_initial);
      END_STATE();
    case 46:
      if (lookahead == 'c') ADVANCE(47);
      END_STATE();
    case 47:
      if (lookahead == 'e') ADVANCE(48);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(anon_sym_interface);
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
  [13] = {.lex_state = 2},
  [14] = {.lex_state = 2},
  [15] = {.lex_state = 2},
  [16] = {.lex_state = 1},
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
  [42] = {.lex_state = 1},
  [43] = {.lex_state = 2},
  [44] = {.lex_state = 1},
  [45] = {.lex_state = 2},
  [46] = {.lex_state = 2},
  [47] = {.lex_state = 2},
  [48] = {.lex_state = 2},
  [49] = {.lex_state = 2},
  [50] = {.lex_state = 1},
  [51] = {.lex_state = 2},
  [52] = {.lex_state = 2},
  [53] = {.lex_state = 2},
  [54] = {.lex_state = 2},
  [55] = {.lex_state = 2},
  [56] = {.lex_state = 1},
  [57] = {.lex_state = 2},
  [58] = {.lex_state = 2},
  [59] = {.lex_state = 1},
  [60] = {.lex_state = 2},
  [61] = {.lex_state = 2},
  [62] = {.lex_state = 1},
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
  [97] = {.lex_state = 1},
  [98] = {.lex_state = 0},
  [99] = {.lex_state = 1},
  [100] = {.lex_state = 0},
  [101] = {.lex_state = 0},
  [102] = {.lex_state = 0},
  [103] = {.lex_state = 0},
  [104] = {.lex_state = 1},
  [105] = {.lex_state = 1},
  [106] = {.lex_state = 1},
  [107] = {.lex_state = 0},
  [108] = {.lex_state = 1},
  [109] = {.lex_state = 0},
  [110] = {.lex_state = 1},
  [111] = {.lex_state = 1},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 0},
  [115] = {.lex_state = 0},
  [116] = {.lex_state = 1},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 1},
  [119] = {.lex_state = 0},
  [120] = {.lex_state = 1},
  [121] = {.lex_state = 1},
  [122] = {.lex_state = 0},
  [123] = {.lex_state = 1},
  [124] = {.lex_state = 0},
  [125] = {.lex_state = 1},
  [126] = {.lex_state = 0},
  [127] = {.lex_state = 0},
  [128] = {.lex_state = 1},
  [129] = {.lex_state = 1},
  [130] = {.lex_state = 0},
  [131] = {.lex_state = 0},
  [132] = {.lex_state = 0},
  [133] = {.lex_state = 0},
  [134] = {.lex_state = 1},
  [135] = {.lex_state = 0},
  [136] = {.lex_state = 0},
  [137] = {.lex_state = 0},
  [138] = {.lex_state = 0},
  [139] = {.lex_state = 0},
  [140] = {.lex_state = 0},
  [141] = {.lex_state = 1},
  [142] = {.lex_state = 0},
  [143] = {.lex_state = 1},
  [144] = {.lex_state = 1},
  [145] = {.lex_state = 1},
  [146] = {.lex_state = 0},
  [147] = {.lex_state = 0},
  [148] = {.lex_state = 0},
  [149] = {.lex_state = 0},
  [150] = {.lex_state = 1},
  [151] = {.lex_state = 0},
  [152] = {.lex_state = 1},
  [153] = {.lex_state = 1},
  [154] = {.lex_state = 0},
  [155] = {.lex_state = 1},
  [156] = {.lex_state = 1},
  [157] = {.lex_state = 0},
  [158] = {.lex_state = 1},
  [159] = {.lex_state = 0},
  [160] = {.lex_state = 1},
  [161] = {.lex_state = 0},
  [162] = {.lex_state = 1},
  [163] = {.lex_state = 0},
  [164] = {.lex_state = 0},
  [165] = {.lex_state = 0},
  [166] = {.lex_state = 0},
  [167] = {.lex_state = 1},
  [168] = {.lex_state = 0},
  [169] = {.lex_state = 0},
  [170] = {.lex_state = 0},
  [171] = {.lex_state = 0},
  [172] = {.lex_state = 0},
  [173] = {.lex_state = 1},
  [174] = {.lex_state = 1},
  [175] = {.lex_state = 0},
  [176] = {.lex_state = 0},
  [177] = {.lex_state = 0},
  [178] = {.lex_state = 0},
  [179] = {.lex_state = 1},
  [180] = {.lex_state = 1},
  [181] = {.lex_state = 0},
  [182] = {.lex_state = 0},
  [183] = {.lex_state = 0},
  [184] = {.lex_state = 0},
  [185] = {.lex_state = 0},
  [186] = {.lex_state = 1},
  [187] = {.lex_state = 0},
  [188] = {.lex_state = 1},
  [189] = {.lex_state = 0},
  [190] = {.lex_state = 0},
  [191] = {.lex_state = 1},
  [192] = {.lex_state = 1},
  [193] = {.lex_state = 0},
  [194] = {.lex_state = 0},
  [195] = {.lex_state = 1},
  [196] = {.lex_state = 1},
  [197] = {.lex_state = 1},
  [198] = {.lex_state = 1},
  [199] = {.lex_state = 1},
  [200] = {.lex_state = 1},
  [201] = {.lex_state = 1},
  [202] = {.lex_state = 0},
  [203] = {.lex_state = 0},
  [204] = {.lex_state = 1},
  [205] = {.lex_state = 0},
  [206] = {.lex_state = 1},
  [207] = {.lex_state = 0},
  [208] = {.lex_state = 1},
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
  [221] = {.lex_state = 1},
  [222] = {.lex_state = 0},
  [223] = {.lex_state = 0},
  [224] = {.lex_state = 0},
  [225] = {.lex_state = 0},
  [226] = {.lex_state = 0},
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
  [237] = {.lex_state = 1},
  [238] = {.lex_state = 0},
  [239] = {.lex_state = 0},
  [240] = {.lex_state = 0},
  [241] = {.lex_state = 0},
  [242] = {.lex_state = 0},
  [243] = {.lex_state = 0},
  [244] = {.lex_state = 0},
  [245] = {.lex_state = 0},
  [246] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_module] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_DASH_GT] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_interface] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_reg] = ACTIONS(1),
    [anon_sym_initial] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [anon_sym_for] = ACTIONS(1),
    [anon_sym_in] = ACTIONS(1),
    [anon_sym_DOT_DOT] = ACTIONS(1),
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
    [sym_source_file] = STATE(246),
    [sym_module] = STATE(202),
    [aux_sym__linebreak] = STATE(132),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_module] = ACTIONS(7),
    [anon_sym_LF] = ACTIONS(9),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 26,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      anon_sym_RBRACE,
    ACTIONS(17), 1,
      anon_sym_interface,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(37), 1,
      sym_number,
    ACTIONS(39), 1,
      anon_sym_LF,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(48), 1,
      sym_template_global,
    STATE(83), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(115), 1,
      sym_assign_to,
    STATE(137), 1,
      sym_declaration,
    STATE(213), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 2,
      sym__type,
      sym_array_type,
    STATE(239), 5,
      sym_block,
      sym_interface_statement,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
    ACTIONS(31), 7,
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
  [99] = 26,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(17), 1,
      anon_sym_interface,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(37), 1,
      sym_number,
    ACTIONS(39), 1,
      anon_sym_LF,
    ACTIONS(41), 1,
      anon_sym_RBRACE,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(48), 1,
      sym_template_global,
    STATE(83), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(115), 1,
      sym_assign_to,
    STATE(137), 1,
      sym_declaration,
    STATE(213), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 2,
      sym__type,
      sym_array_type,
    STATE(239), 5,
      sym_block,
      sym_interface_statement,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
    ACTIONS(31), 7,
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
  [198] = 26,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(17), 1,
      anon_sym_interface,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(37), 1,
      sym_number,
    ACTIONS(39), 1,
      anon_sym_LF,
    ACTIONS(43), 1,
      anon_sym_RBRACE,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(48), 1,
      sym_template_global,
    STATE(83), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(115), 1,
      sym_assign_to,
    STATE(119), 1,
      sym_assign_left_side,
    STATE(137), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 2,
      sym__type,
      sym_array_type,
    STATE(136), 5,
      sym_block,
      sym_interface_statement,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
    ACTIONS(31), 7,
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
  [297] = 26,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(17), 1,
      anon_sym_interface,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(37), 1,
      sym_number,
    ACTIONS(39), 1,
      anon_sym_LF,
    ACTIONS(45), 1,
      anon_sym_RBRACE,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(48), 1,
      sym_template_global,
    STATE(83), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(115), 1,
      sym_assign_to,
    STATE(137), 1,
      sym_declaration,
    STATE(213), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 2,
      sym__type,
      sym_array_type,
    STATE(239), 5,
      sym_block,
      sym_interface_statement,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
    ACTIONS(31), 7,
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
  [396] = 26,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(17), 1,
      anon_sym_interface,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(37), 1,
      sym_number,
    ACTIONS(39), 1,
      anon_sym_LF,
    ACTIONS(47), 1,
      anon_sym_RBRACE,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(48), 1,
      sym_template_global,
    STATE(83), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(115), 1,
      sym_assign_to,
    STATE(137), 1,
      sym_declaration,
    STATE(213), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 2,
      sym__type,
      sym_array_type,
    STATE(239), 5,
      sym_block,
      sym_interface_statement,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
    ACTIONS(31), 7,
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
  [495] = 26,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(17), 1,
      anon_sym_interface,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(37), 1,
      sym_number,
    ACTIONS(39), 1,
      anon_sym_LF,
    ACTIONS(49), 1,
      anon_sym_RBRACE,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(48), 1,
      sym_template_global,
    STATE(83), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(115), 1,
      sym_assign_to,
    STATE(137), 1,
      sym_declaration,
    STATE(213), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 2,
      sym__type,
      sym_array_type,
    STATE(239), 5,
      sym_block,
      sym_interface_statement,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
    ACTIONS(31), 7,
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
  [594] = 26,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(17), 1,
      anon_sym_interface,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(37), 1,
      sym_number,
    ACTIONS(51), 1,
      anon_sym_RBRACE,
    ACTIONS(53), 1,
      anon_sym_LF,
    STATE(4), 1,
      aux_sym__linebreak,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(48), 1,
      sym_template_global,
    STATE(83), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(115), 1,
      sym_assign_to,
    STATE(126), 1,
      sym_assign_left_side,
    STATE(137), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 2,
      sym__type,
      sym_array_type,
    STATE(140), 5,
      sym_block,
      sym_interface_statement,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
    ACTIONS(31), 7,
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
  [693] = 26,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(17), 1,
      anon_sym_interface,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(37), 1,
      sym_number,
    ACTIONS(39), 1,
      anon_sym_LF,
    ACTIONS(55), 1,
      anon_sym_RBRACE,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(48), 1,
      sym_template_global,
    STATE(83), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(115), 1,
      sym_assign_to,
    STATE(137), 1,
      sym_declaration,
    STATE(213), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 2,
      sym__type,
      sym_array_type,
    STATE(239), 5,
      sym_block,
      sym_interface_statement,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
    ACTIONS(31), 7,
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
  [792] = 25,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(17), 1,
      anon_sym_interface,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(23), 1,
      anon_sym_if,
    ACTIONS(25), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(37), 1,
      sym_number,
    ACTIONS(39), 1,
      anon_sym_LF,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(48), 1,
      sym_template_global,
    STATE(83), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(115), 1,
      sym_assign_to,
    STATE(137), 1,
      sym_declaration,
    STATE(213), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 2,
      sym__type,
      sym_array_type,
    STATE(239), 5,
      sym_block,
      sym_interface_statement,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
    ACTIONS(31), 7,
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
  [888] = 5,
    ACTIONS(61), 1,
      anon_sym_COLON_COLON,
    STATE(11), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(57), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(59), 21,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
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
  [932] = 5,
    ACTIONS(68), 1,
      anon_sym_COLON_COLON,
    STATE(14), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(64), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(66), 21,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
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
  [976] = 5,
    ACTIONS(68), 1,
      anon_sym_COLON_COLON,
    STATE(15), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(70), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(72), 21,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
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
  [1020] = 5,
    ACTIONS(68), 1,
      anon_sym_COLON_COLON,
    STATE(11), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(74), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(76), 21,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
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
  [1064] = 5,
    ACTIONS(68), 1,
      anon_sym_COLON_COLON,
    STATE(11), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(78), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(80), 21,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
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
  [1108] = 17,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(19), 1,
      anon_sym_reg,
    ACTIONS(21), 1,
      anon_sym_initial,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(37), 1,
      sym_number,
    STATE(42), 1,
      sym_write_modifiers,
    STATE(48), 1,
      sym_template_global,
    STATE(83), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(137), 1,
      sym_declaration,
    STATE(139), 1,
      sym_assign_to,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 2,
      sym__type,
      sym_array_type,
    ACTIONS(31), 7,
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
  [1176] = 12,
    ACTIONS(86), 1,
      anon_sym_PLUS,
    ACTIONS(88), 1,
      anon_sym_DASH,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(94), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(84), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(82), 17,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
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
  [1233] = 14,
    ACTIONS(86), 1,
      anon_sym_PLUS,
    ACTIONS(88), 1,
      anon_sym_DASH,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(94), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_CARET,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(84), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(82), 15,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
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
  [1294] = 15,
    ACTIONS(86), 1,
      anon_sym_PLUS,
    ACTIONS(88), 1,
      anon_sym_DASH,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(94), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_CARET,
    ACTIONS(104), 1,
      anon_sym_AMP,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(84), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(82), 14,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_LF,
  [1357] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(108), 22,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
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
  [1396] = 8,
    ACTIONS(94), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
    ACTIONS(110), 20,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
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
  [1445] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(114), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(116), 22,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
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
  [1484] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(118), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(120), 22,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
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
  [1523] = 10,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(94), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(84), 4,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
    ACTIONS(82), 18,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
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
  [1576] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(122), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(124), 22,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
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
  [1615] = 3,
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
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
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
  [1654] = 3,
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
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
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
  [1693] = 3,
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
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
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
  [1732] = 3,
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
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
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
  [1771] = 8,
    ACTIONS(94), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(84), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
    ACTIONS(82), 20,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
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
  [1820] = 13,
    ACTIONS(86), 1,
      anon_sym_PLUS,
    ACTIONS(88), 1,
      anon_sym_DASH,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(94), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(102), 1,
      anon_sym_CARET,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(84), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(82), 16,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
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
  [1879] = 3,
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
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
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
  [1918] = 3,
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
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
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
  [1957] = 3,
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
    ACTIONS(152), 21,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
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
  [1995] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(156), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(154), 22,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
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
  [2032] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(160), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(158), 22,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
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
  [2069] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(164), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(162), 22,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
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
  [2106] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(168), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(166), 22,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
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
  [2143] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(172), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(170), 22,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
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
  [2180] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(176), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(174), 22,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
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
  [2217] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(180), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(178), 22,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
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
  [2254] = 12,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(182), 1,
      sym_number,
    STATE(48), 1,
      sym_template_global,
    STATE(172), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 2,
      sym__type,
      sym_array_type,
    ACTIONS(31), 7,
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
  [2307] = 17,
    ACTIONS(86), 1,
      anon_sym_PLUS,
    ACTIONS(88), 1,
      anon_sym_DASH,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_CARET,
    ACTIONS(104), 1,
      anon_sym_AMP,
    ACTIONS(188), 1,
      anon_sym_EQ,
    ACTIONS(192), 1,
      anon_sym_DOT,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(190), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
    ACTIONS(184), 6,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_COMMA,
      anon_sym_LF,
  [2370] = 5,
    ACTIONS(198), 1,
      anon_sym_LF,
    STATE(44), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(194), 11,
      anon_sym_interface,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_DASH,
      sym_identifier,
    ACTIONS(196), 12,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_COLON_COLON,
      sym_number,
  [2408] = 16,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_CARET,
    ACTIONS(104), 1,
      anon_sym_AMP,
    ACTIONS(192), 1,
      anon_sym_DOT,
    ACTIONS(203), 1,
      anon_sym_EQ,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(201), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_LF,
    ACTIONS(190), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2466] = 18,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_CARET,
    ACTIONS(104), 1,
      anon_sym_AMP,
    ACTIONS(192), 1,
      anon_sym_DOT,
    ACTIONS(205), 1,
      anon_sym_RPAREN,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    STATE(81), 1,
      sym__comma,
    STATE(164), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(190), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2528] = 16,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_CARET,
    ACTIONS(104), 1,
      anon_sym_AMP,
    ACTIONS(192), 1,
      anon_sym_DOT,
    ACTIONS(211), 1,
      anon_sym_EQ,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(209), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_LF,
    ACTIONS(190), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2586] = 5,
    ACTIONS(213), 1,
      sym_identifier,
    ACTIONS(219), 1,
      anon_sym_LBRACK,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(215), 4,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_SLASH,
    ACTIONS(217), 16,
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
  [2621] = 6,
    ACTIONS(68), 1,
      anon_sym_COLON_COLON,
    ACTIONS(222), 1,
      anon_sym_EQ,
    STATE(14), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(64), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(66), 16,
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
  [2658] = 9,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(224), 1,
      sym_identifier,
    ACTIONS(226), 1,
      anon_sym_SEMI,
    ACTIONS(228), 1,
      sym_number,
    STATE(203), 1,
      sym_template_value_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [2700] = 16,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_CARET,
    ACTIONS(104), 1,
      anon_sym_AMP,
    ACTIONS(192), 1,
      anon_sym_DOT,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    STATE(223), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(190), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2756] = 15,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_CARET,
    ACTIONS(104), 1,
      anon_sym_AMP,
    ACTIONS(192), 1,
      anon_sym_DOT,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(230), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
    ACTIONS(190), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2810] = 15,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_CARET,
    ACTIONS(104), 1,
      anon_sym_AMP,
    ACTIONS(192), 1,
      anon_sym_DOT,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(232), 2,
      anon_sym_SEMI,
      anon_sym_COMMA,
    ACTIONS(190), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2864] = 16,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_CARET,
    ACTIONS(104), 1,
      anon_sym_AMP,
    ACTIONS(192), 1,
      anon_sym_DOT,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    STATE(207), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(190), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2920] = 15,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_CARET,
    ACTIONS(104), 1,
      anon_sym_AMP,
    ACTIONS(192), 1,
      anon_sym_DOT,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(234), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(190), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2974] = 9,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(224), 1,
      sym_identifier,
    ACTIONS(228), 1,
      sym_number,
    ACTIONS(236), 1,
      anon_sym_SEMI,
    STATE(154), 1,
      sym_template_value_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [3016] = 15,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_CARET,
    ACTIONS(104), 1,
      anon_sym_AMP,
    ACTIONS(192), 1,
      anon_sym_DOT,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(238), 2,
      anon_sym_SEMI,
      anon_sym_COMMA,
    ACTIONS(190), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3070] = 15,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_CARET,
    ACTIONS(104), 1,
      anon_sym_AMP,
    ACTIONS(192), 1,
      anon_sym_DOT,
    ACTIONS(240), 1,
      anon_sym_RBRACK,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(190), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3123] = 8,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(224), 1,
      sym_identifier,
    ACTIONS(228), 1,
      sym_number,
    STATE(232), 1,
      sym_template_value_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [3162] = 15,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_CARET,
    ACTIONS(104), 1,
      anon_sym_AMP,
    ACTIONS(192), 1,
      anon_sym_DOT,
    ACTIONS(242), 1,
      anon_sym_RPAREN,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(190), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3215] = 15,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(94), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_CARET,
    ACTIONS(104), 1,
      anon_sym_AMP,
    ACTIONS(244), 1,
      anon_sym_DOT_DOT,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(190), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3268] = 8,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(248), 1,
      anon_sym_RPAREN,
    ACTIONS(250), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [3307] = 15,
    ACTIONS(92), 1,
      anon_sym_SLASH,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_CARET,
    ACTIONS(104), 1,
      anon_sym_AMP,
    ACTIONS(192), 1,
      anon_sym_DOT,
    ACTIONS(252), 1,
      anon_sym_RBRACK,
    STATE(36), 1,
      sym_parenthesis_expression_list,
    STATE(39), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(90), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(186), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(190), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3360] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(254), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(43), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3396] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(256), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [3432] = 5,
    ACTIONS(262), 1,
      anon_sym_LF,
    STATE(71), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(258), 7,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(260), 10,
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
  [3464] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(264), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [3500] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(266), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [3536] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(268), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [3572] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(270), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [3608] = 5,
    ACTIONS(39), 1,
      anon_sym_LF,
    STATE(44), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(272), 7,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(274), 10,
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
  [3640] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(276), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [3676] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(278), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(58), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3712] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(280), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(61), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3748] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(282), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [3784] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(284), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [3820] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(286), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [3856] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(288), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [3892] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(290), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(31), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3928] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(292), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [3964] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(294), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [4000] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(296), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(31), 7,
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
  [4036] = 5,
    ACTIONS(19), 1,
      anon_sym_reg,
    STATE(84), 1,
      aux_sym_write_modifiers_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(298), 5,
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
  [4066] = 5,
    ACTIONS(304), 1,
      anon_sym_reg,
    STATE(84), 1,
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
  [4096] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(309), 6,
      anon_sym_reg,
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
  [4121] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(313), 5,
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
  [4145] = 12,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(317), 1,
      anon_sym_DASH_GT,
    ACTIONS(319), 1,
      anon_sym_LF,
    STATE(88), 1,
      aux_sym__linebreak,
    STATE(109), 1,
      sym_declaration,
    STATE(122), 1,
      sym_declaration_list,
    STATE(209), 1,
      sym__interface_ports_output,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4187] = 12,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(39), 1,
      anon_sym_LF,
    ACTIONS(317), 1,
      anon_sym_DASH_GT,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(109), 1,
      sym_declaration,
    STATE(131), 1,
      sym_declaration_list,
    STATE(220), 1,
      sym__interface_ports_output,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4229] = 10,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(321), 1,
      anon_sym_LF,
    STATE(90), 1,
      aux_sym__linebreak,
    STATE(109), 1,
      sym_declaration,
    STATE(212), 1,
      sym_declaration_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4265] = 10,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(39), 1,
      anon_sym_LF,
    STATE(44), 1,
      aux_sym__linebreak,
    STATE(109), 1,
      sym_declaration,
    STATE(219), 1,
      sym_declaration_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4301] = 7,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    STATE(244), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4328] = 7,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    STATE(133), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(27), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(29), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(210), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4355] = 4,
    ACTIONS(325), 1,
      anon_sym_SQUOTE,
    STATE(98), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(323), 7,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_COMMA,
      anon_sym_LF,
  [4375] = 4,
    ACTIONS(325), 1,
      anon_sym_SQUOTE,
    STATE(100), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(327), 7,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_COMMA,
      anon_sym_LF,
  [4395] = 4,
    ACTIONS(325), 1,
      anon_sym_SQUOTE,
    STATE(102), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(329), 7,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_COMMA,
      anon_sym_LF,
  [4415] = 4,
    ACTIONS(325), 1,
      anon_sym_SQUOTE,
    STATE(101), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(331), 7,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_COMMA,
      anon_sym_LF,
  [4435] = 6,
    ACTIONS(333), 1,
      sym_identifier,
    ACTIONS(335), 1,
      anon_sym_GT,
    ACTIONS(337), 1,
      anon_sym_COLON_COLON,
    STATE(153), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(173), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4457] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(339), 7,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_COMMA,
      anon_sym_LF,
  [4471] = 6,
    ACTIONS(333), 1,
      sym_identifier,
    ACTIONS(337), 1,
      anon_sym_COLON_COLON,
    ACTIONS(341), 1,
      anon_sym_GT,
    STATE(174), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(173), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4493] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(343), 7,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_COMMA,
      anon_sym_LF,
  [4507] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(345), 7,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_COMMA,
      anon_sym_LF,
  [4521] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(347), 7,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_COMMA,
      anon_sym_LF,
  [4535] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    STATE(92), 1,
      sym__comma,
    STATE(107), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(349), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [4555] = 6,
    ACTIONS(333), 1,
      sym_identifier,
    ACTIONS(337), 1,
      anon_sym_COLON_COLON,
    ACTIONS(351), 1,
      anon_sym_GT,
    STATE(191), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(173), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4577] = 5,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(353), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(205), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4597] = 6,
    ACTIONS(333), 1,
      sym_identifier,
    ACTIONS(337), 1,
      anon_sym_COLON_COLON,
    ACTIONS(355), 1,
      anon_sym_GT,
    STATE(141), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(173), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4619] = 5,
    ACTIONS(359), 1,
      anon_sym_COMMA,
    STATE(92), 1,
      sym__comma,
    STATE(107), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(357), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [4639] = 6,
    ACTIONS(333), 1,
      sym_identifier,
    ACTIONS(337), 1,
      anon_sym_COLON_COLON,
    ACTIONS(362), 1,
      anon_sym_GT,
    STATE(201), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(173), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4661] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    STATE(92), 1,
      sym__comma,
    STATE(103), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(364), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [4681] = 6,
    ACTIONS(333), 1,
      sym_identifier,
    ACTIONS(337), 1,
      anon_sym_COLON_COLON,
    ACTIONS(366), 1,
      anon_sym_GT,
    STATE(179), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(173), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4703] = 5,
    ACTIONS(368), 1,
      anon_sym_EQ,
    ACTIONS(370), 1,
      anon_sym_COLON_COLON,
    STATE(129), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(66), 3,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COMMA,
  [4722] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    STATE(16), 1,
      sym__comma,
    STATE(113), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(372), 3,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_LF,
  [4741] = 5,
    ACTIONS(376), 1,
      anon_sym_COMMA,
    STATE(16), 1,
      sym__comma,
    STATE(113), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(374), 3,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_LF,
  [4760] = 7,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(379), 1,
      anon_sym_COLON,
    ACTIONS(381), 1,
      anon_sym_LT,
    STATE(181), 1,
      sym_template_declaration_arguments,
    STATE(224), 1,
      sym_interface_ports,
    STATE(230), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4783] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    STATE(16), 1,
      sym__comma,
    STATE(112), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(383), 3,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_LF,
  [4802] = 5,
    ACTIONS(333), 1,
      sym_identifier,
    ACTIONS(337), 1,
      anon_sym_COLON_COLON,
    STATE(236), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(173), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4821] = 6,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(385), 1,
      ts_builtin_sym_end,
    ACTIONS(387), 1,
      anon_sym_LF,
    STATE(189), 1,
      aux_sym__linebreak,
    STATE(231), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4841] = 4,
    ACTIONS(370), 1,
      anon_sym_COLON_COLON,
    STATE(123), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(80), 3,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COMMA,
  [4857] = 6,
    ACTIONS(389), 1,
      anon_sym_RBRACE,
    ACTIONS(391), 1,
      anon_sym_EQ,
    ACTIONS(393), 1,
      anon_sym_LF,
    STATE(7), 1,
      aux_sym__linebreak,
    STATE(147), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4877] = 4,
    ACTIONS(370), 1,
      anon_sym_COLON_COLON,
    STATE(118), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(72), 3,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COMMA,
  [4893] = 4,
    ACTIONS(370), 1,
      anon_sym_COLON_COLON,
    STATE(129), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(66), 3,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COMMA,
  [4909] = 4,
    ACTIONS(317), 1,
      anon_sym_DASH_GT,
    STATE(216), 1,
      sym__interface_ports_output,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(395), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [4925] = 4,
    ACTIONS(397), 1,
      anon_sym_COLON_COLON,
    STATE(123), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(59), 3,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COMMA,
  [4941] = 6,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(387), 1,
      anon_sym_LF,
    ACTIONS(400), 1,
      ts_builtin_sym_end,
    STATE(189), 1,
      aux_sym__linebreak,
    STATE(231), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4961] = 4,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(214), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4977] = 6,
    ACTIONS(391), 1,
      anon_sym_EQ,
    ACTIONS(402), 1,
      anon_sym_RBRACE,
    ACTIONS(404), 1,
      anon_sym_LF,
    STATE(3), 1,
      aux_sym__linebreak,
    STATE(176), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4997] = 6,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(387), 1,
      anon_sym_LF,
    ACTIONS(406), 1,
      ts_builtin_sym_end,
    STATE(189), 1,
      aux_sym__linebreak,
    STATE(231), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5017] = 4,
    ACTIONS(337), 1,
      anon_sym_COLON_COLON,
    ACTIONS(408), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(195), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [5033] = 4,
    ACTIONS(370), 1,
      anon_sym_COLON_COLON,
    STATE(123), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(76), 3,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COMMA,
  [5049] = 6,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(387), 1,
      anon_sym_LF,
    ACTIONS(410), 1,
      ts_builtin_sym_end,
    STATE(189), 1,
      aux_sym__linebreak,
    STATE(231), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5069] = 4,
    ACTIONS(317), 1,
      anon_sym_DASH_GT,
    STATE(217), 1,
      sym__interface_ports_output,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(412), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5085] = 6,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(387), 1,
      anon_sym_LF,
    ACTIONS(414), 1,
      ts_builtin_sym_end,
    STATE(189), 1,
      aux_sym__linebreak,
    STATE(190), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5105] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(416), 5,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_LF,
  [5117] = 4,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(246), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(218), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [5133] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(418), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5144] = 5,
    ACTIONS(389), 1,
      anon_sym_RBRACE,
    ACTIONS(393), 1,
      anon_sym_LF,
    STATE(7), 1,
      aux_sym__linebreak,
    STATE(146), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5161] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(201), 4,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_COMMA,
      anon_sym_LF,
  [5172] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(420), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5183] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(422), 4,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_COMMA,
      anon_sym_LF,
  [5194] = 5,
    ACTIONS(402), 1,
      anon_sym_RBRACE,
    ACTIONS(404), 1,
      anon_sym_LF,
    STATE(3), 1,
      aux_sym__linebreak,
    STATE(177), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5211] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(424), 1,
      anon_sym_GT,
    STATE(116), 1,
      sym__comma,
    STATE(145), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5228] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(426), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5239] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(428), 1,
      anon_sym_GT,
    STATE(192), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(225), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5256] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(430), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      sym_identifier,
      anon_sym_COMMA,
  [5267] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(432), 1,
      anon_sym_GT,
    STATE(116), 1,
      sym__comma,
    STATE(197), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5284] = 5,
    ACTIONS(434), 1,
      anon_sym_RBRACE,
    ACTIONS(436), 1,
      anon_sym_LF,
    STATE(2), 1,
      aux_sym__linebreak,
    STATE(170), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5301] = 5,
    ACTIONS(438), 1,
      anon_sym_RBRACE,
    ACTIONS(440), 1,
      anon_sym_LF,
    STATE(9), 1,
      aux_sym__linebreak,
    STATE(170), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5318] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(420), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5329] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(442), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5340] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(140), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5351] = 5,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(387), 1,
      anon_sym_LF,
    STATE(189), 1,
      aux_sym__linebreak,
    STATE(231), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5368] = 5,
    ACTIONS(444), 1,
      anon_sym_GT,
    ACTIONS(446), 1,
      anon_sym_COMMA,
    STATE(152), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(225), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5385] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(449), 1,
      anon_sym_GT,
    STATE(116), 1,
      sym__comma,
    STATE(160), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5402] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(451), 1,
      anon_sym_SEMI,
    STATE(59), 1,
      sym__comma,
    STATE(178), 1,
      aux_sym_template_params_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5419] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(148), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5430] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(124), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5441] = 4,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(453), 1,
      anon_sym_if,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(228), 2,
      sym_block,
      sym_if_statement,
  [5456] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(132), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5467] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(455), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5478] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(457), 1,
      anon_sym_GT,
    STATE(116), 1,
      sym__comma,
    STATE(197), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5495] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(455), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5506] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(144), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5517] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(459), 1,
      anon_sym_SEMI,
    STATE(59), 1,
      sym__comma,
    STATE(193), 1,
      aux_sym_template_params_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5534] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(461), 1,
      anon_sym_RPAREN,
    STATE(81), 1,
      sym__comma,
    STATE(182), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5551] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(463), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5562] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(465), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5573] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(120), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5584] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(465), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5595] = 5,
    ACTIONS(467), 1,
      ts_builtin_sym_end,
    ACTIONS(469), 1,
      anon_sym_LF,
    STATE(117), 1,
      aux_sym__linebreak,
    STATE(175), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5612] = 5,
    ACTIONS(471), 1,
      anon_sym_RBRACE,
    ACTIONS(473), 1,
      anon_sym_LF,
    STATE(10), 1,
      aux_sym__linebreak,
    STATE(170), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5629] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(476), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5640] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(209), 4,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_COMMA,
      anon_sym_LF,
  [5651] = 4,
    ACTIONS(480), 1,
      anon_sym_LBRACK,
    STATE(144), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(478), 2,
      anon_sym_GT,
      anon_sym_COMMA,
  [5666] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(482), 1,
      anon_sym_GT,
    STATE(116), 1,
      sym__comma,
    STATE(188), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5683] = 5,
    ACTIONS(484), 1,
      ts_builtin_sym_end,
    ACTIONS(486), 1,
      anon_sym_LF,
    STATE(151), 1,
      aux_sym__linebreak,
    STATE(175), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5700] = 5,
    ACTIONS(489), 1,
      anon_sym_RBRACE,
    ACTIONS(491), 1,
      anon_sym_LF,
    STATE(6), 1,
      aux_sym__linebreak,
    STATE(170), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5717] = 5,
    ACTIONS(493), 1,
      anon_sym_RBRACE,
    ACTIONS(495), 1,
      anon_sym_LF,
    STATE(5), 1,
      aux_sym__linebreak,
    STATE(170), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5734] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(497), 1,
      anon_sym_SEMI,
    STATE(59), 1,
      sym__comma,
    STATE(193), 1,
      aux_sym_template_params_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5751] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(499), 1,
      anon_sym_GT,
    STATE(116), 1,
      sym__comma,
    STATE(180), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5768] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(501), 1,
      anon_sym_GT,
    STATE(116), 1,
      sym__comma,
    STATE(197), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5785] = 5,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(379), 1,
      anon_sym_COLON,
    STATE(222), 1,
      sym_interface_ports,
    STATE(233), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5802] = 5,
    ACTIONS(503), 1,
      anon_sym_RPAREN,
    ACTIONS(505), 1,
      anon_sym_COMMA,
    STATE(81), 1,
      sym__comma,
    STATE(182), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5819] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(508), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5830] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(508), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5841] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(510), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5852] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(116), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [5863] = 4,
    ACTIONS(379), 1,
      anon_sym_COLON,
    STATE(227), 1,
      sym_interface_ports,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(512), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5878] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(514), 1,
      anon_sym_GT,
    STATE(116), 1,
      sym__comma,
    STATE(197), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5895] = 4,
    ACTIONS(516), 1,
      anon_sym_LF,
    STATE(189), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(196), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [5910] = 5,
    ACTIONS(519), 1,
      ts_builtin_sym_end,
    ACTIONS(521), 1,
      anon_sym_LF,
    STATE(124), 1,
      aux_sym__linebreak,
    STATE(169), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5927] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(523), 1,
      anon_sym_GT,
    STATE(116), 1,
      sym__comma,
    STATE(199), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5944] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(525), 1,
      anon_sym_GT,
    STATE(152), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(225), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5961] = 5,
    ACTIONS(527), 1,
      anon_sym_SEMI,
    ACTIONS(529), 1,
      anon_sym_COMMA,
    STATE(59), 1,
      sym__comma,
    STATE(193), 1,
      aux_sym_template_params_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5978] = 5,
    ACTIONS(532), 1,
      ts_builtin_sym_end,
    ACTIONS(534), 1,
      anon_sym_LF,
    STATE(127), 1,
      aux_sym__linebreak,
    STATE(175), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5995] = 4,
    ACTIONS(480), 1,
      anon_sym_LBRACK,
    STATE(144), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(536), 2,
      anon_sym_GT,
      anon_sym_COMMA,
  [6010] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(108), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [6021] = 5,
    ACTIONS(538), 1,
      anon_sym_GT,
    ACTIONS(540), 1,
      anon_sym_COMMA,
    STATE(116), 1,
      sym__comma,
    STATE(197), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6038] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(128), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [6049] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(543), 1,
      anon_sym_GT,
    STATE(116), 1,
      sym__comma,
    STATE(197), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6066] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(136), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
  [6077] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(545), 1,
      anon_sym_GT,
    STATE(116), 1,
      sym__comma,
    STATE(204), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6094] = 5,
    ACTIONS(547), 1,
      ts_builtin_sym_end,
    ACTIONS(549), 1,
      anon_sym_LF,
    STATE(130), 1,
      aux_sym__linebreak,
    STATE(194), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6111] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(551), 1,
      anon_sym_SEMI,
    STATE(59), 1,
      sym__comma,
    STATE(163), 1,
      aux_sym_template_params_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6128] = 5,
    ACTIONS(207), 1,
      anon_sym_COMMA,
    ACTIONS(553), 1,
      anon_sym_GT,
    STATE(116), 1,
      sym__comma,
    STATE(197), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6145] = 4,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(555), 1,
      sym_identifier,
    STATE(144), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6159] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(152), 3,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COMMA,
  [6169] = 3,
    ACTIONS(559), 1,
      anon_sym_else,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(557), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6181] = 4,
    ACTIONS(561), 1,
      sym_identifier,
    ACTIONS(563), 1,
      anon_sym_GT,
    STATE(143), 1,
      sym_template_declaration_type,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6195] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(565), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6205] = 4,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(567), 1,
      sym_identifier,
    STATE(144), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6219] = 4,
    ACTIONS(569), 1,
      sym_identifier,
    ACTIONS(571), 1,
      anon_sym_LT,
    STATE(167), 1,
      sym_template_params,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6233] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(573), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6243] = 3,
    ACTIONS(391), 1,
      anon_sym_EQ,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(575), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6255] = 4,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(577), 1,
      sym_identifier,
    STATE(144), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6269] = 4,
    ACTIONS(579), 1,
      sym_identifier,
    ACTIONS(581), 1,
      anon_sym_LT,
    STATE(23), 1,
      sym_template_params,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6283] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(583), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6293] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(585), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6303] = 4,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(587), 1,
      sym_identifier,
    STATE(144), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6317] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(589), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6327] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(591), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6337] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(593), 2,
      anon_sym_GT,
      anon_sym_COMMA,
  [6346] = 3,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    STATE(234), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6357] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(595), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6366] = 3,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    STATE(235), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6377] = 3,
    ACTIONS(561), 1,
      sym_identifier,
    STATE(221), 1,
      sym_template_declaration_type,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6388] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(597), 2,
      anon_sym_COLON,
      anon_sym_LBRACE,
  [6397] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(599), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6406] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(601), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6415] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(603), 2,
      anon_sym_COLON,
      anon_sym_LBRACE,
  [6424] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(605), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6433] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(607), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6442] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(609), 2,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [6451] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(611), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6460] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(613), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6469] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(615), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6478] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(617), 2,
      anon_sym_GT,
      anon_sym_COMMA,
  [6487] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(619), 2,
      anon_sym_GT,
      anon_sym_COMMA,
  [6496] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(621), 2,
      anon_sym_COLON,
      anon_sym_LBRACE,
  [6505] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(575), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6514] = 2,
    ACTIONS(623), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6522] = 2,
    ACTIONS(625), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6530] = 2,
    ACTIONS(627), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6538] = 2,
    ACTIONS(629), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6546] = 2,
    ACTIONS(631), 1,
      anon_sym_in,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6554] = 2,
    ACTIONS(633), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6562] = 2,
    ACTIONS(635), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 99,
  [SMALL_STATE(4)] = 198,
  [SMALL_STATE(5)] = 297,
  [SMALL_STATE(6)] = 396,
  [SMALL_STATE(7)] = 495,
  [SMALL_STATE(8)] = 594,
  [SMALL_STATE(9)] = 693,
  [SMALL_STATE(10)] = 792,
  [SMALL_STATE(11)] = 888,
  [SMALL_STATE(12)] = 932,
  [SMALL_STATE(13)] = 976,
  [SMALL_STATE(14)] = 1020,
  [SMALL_STATE(15)] = 1064,
  [SMALL_STATE(16)] = 1108,
  [SMALL_STATE(17)] = 1176,
  [SMALL_STATE(18)] = 1233,
  [SMALL_STATE(19)] = 1294,
  [SMALL_STATE(20)] = 1357,
  [SMALL_STATE(21)] = 1396,
  [SMALL_STATE(22)] = 1445,
  [SMALL_STATE(23)] = 1484,
  [SMALL_STATE(24)] = 1523,
  [SMALL_STATE(25)] = 1576,
  [SMALL_STATE(26)] = 1615,
  [SMALL_STATE(27)] = 1654,
  [SMALL_STATE(28)] = 1693,
  [SMALL_STATE(29)] = 1732,
  [SMALL_STATE(30)] = 1771,
  [SMALL_STATE(31)] = 1820,
  [SMALL_STATE(32)] = 1879,
  [SMALL_STATE(33)] = 1918,
  [SMALL_STATE(34)] = 1957,
  [SMALL_STATE(35)] = 1995,
  [SMALL_STATE(36)] = 2032,
  [SMALL_STATE(37)] = 2069,
  [SMALL_STATE(38)] = 2106,
  [SMALL_STATE(39)] = 2143,
  [SMALL_STATE(40)] = 2180,
  [SMALL_STATE(41)] = 2217,
  [SMALL_STATE(42)] = 2254,
  [SMALL_STATE(43)] = 2307,
  [SMALL_STATE(44)] = 2370,
  [SMALL_STATE(45)] = 2408,
  [SMALL_STATE(46)] = 2466,
  [SMALL_STATE(47)] = 2528,
  [SMALL_STATE(48)] = 2586,
  [SMALL_STATE(49)] = 2621,
  [SMALL_STATE(50)] = 2658,
  [SMALL_STATE(51)] = 2700,
  [SMALL_STATE(52)] = 2756,
  [SMALL_STATE(53)] = 2810,
  [SMALL_STATE(54)] = 2864,
  [SMALL_STATE(55)] = 2920,
  [SMALL_STATE(56)] = 2974,
  [SMALL_STATE(57)] = 3016,
  [SMALL_STATE(58)] = 3070,
  [SMALL_STATE(59)] = 3123,
  [SMALL_STATE(60)] = 3162,
  [SMALL_STATE(61)] = 3215,
  [SMALL_STATE(62)] = 3268,
  [SMALL_STATE(63)] = 3307,
  [SMALL_STATE(64)] = 3360,
  [SMALL_STATE(65)] = 3396,
  [SMALL_STATE(66)] = 3432,
  [SMALL_STATE(67)] = 3464,
  [SMALL_STATE(68)] = 3500,
  [SMALL_STATE(69)] = 3536,
  [SMALL_STATE(70)] = 3572,
  [SMALL_STATE(71)] = 3608,
  [SMALL_STATE(72)] = 3640,
  [SMALL_STATE(73)] = 3676,
  [SMALL_STATE(74)] = 3712,
  [SMALL_STATE(75)] = 3748,
  [SMALL_STATE(76)] = 3784,
  [SMALL_STATE(77)] = 3820,
  [SMALL_STATE(78)] = 3856,
  [SMALL_STATE(79)] = 3892,
  [SMALL_STATE(80)] = 3928,
  [SMALL_STATE(81)] = 3964,
  [SMALL_STATE(82)] = 4000,
  [SMALL_STATE(83)] = 4036,
  [SMALL_STATE(84)] = 4066,
  [SMALL_STATE(85)] = 4096,
  [SMALL_STATE(86)] = 4121,
  [SMALL_STATE(87)] = 4145,
  [SMALL_STATE(88)] = 4187,
  [SMALL_STATE(89)] = 4229,
  [SMALL_STATE(90)] = 4265,
  [SMALL_STATE(91)] = 4301,
  [SMALL_STATE(92)] = 4328,
  [SMALL_STATE(93)] = 4355,
  [SMALL_STATE(94)] = 4375,
  [SMALL_STATE(95)] = 4395,
  [SMALL_STATE(96)] = 4415,
  [SMALL_STATE(97)] = 4435,
  [SMALL_STATE(98)] = 4457,
  [SMALL_STATE(99)] = 4471,
  [SMALL_STATE(100)] = 4493,
  [SMALL_STATE(101)] = 4507,
  [SMALL_STATE(102)] = 4521,
  [SMALL_STATE(103)] = 4535,
  [SMALL_STATE(104)] = 4555,
  [SMALL_STATE(105)] = 4577,
  [SMALL_STATE(106)] = 4597,
  [SMALL_STATE(107)] = 4619,
  [SMALL_STATE(108)] = 4639,
  [SMALL_STATE(109)] = 4661,
  [SMALL_STATE(110)] = 4681,
  [SMALL_STATE(111)] = 4703,
  [SMALL_STATE(112)] = 4722,
  [SMALL_STATE(113)] = 4741,
  [SMALL_STATE(114)] = 4760,
  [SMALL_STATE(115)] = 4783,
  [SMALL_STATE(116)] = 4802,
  [SMALL_STATE(117)] = 4821,
  [SMALL_STATE(118)] = 4841,
  [SMALL_STATE(119)] = 4857,
  [SMALL_STATE(120)] = 4877,
  [SMALL_STATE(121)] = 4893,
  [SMALL_STATE(122)] = 4909,
  [SMALL_STATE(123)] = 4925,
  [SMALL_STATE(124)] = 4941,
  [SMALL_STATE(125)] = 4961,
  [SMALL_STATE(126)] = 4977,
  [SMALL_STATE(127)] = 4997,
  [SMALL_STATE(128)] = 5017,
  [SMALL_STATE(129)] = 5033,
  [SMALL_STATE(130)] = 5049,
  [SMALL_STATE(131)] = 5069,
  [SMALL_STATE(132)] = 5085,
  [SMALL_STATE(133)] = 5105,
  [SMALL_STATE(134)] = 5117,
  [SMALL_STATE(135)] = 5133,
  [SMALL_STATE(136)] = 5144,
  [SMALL_STATE(137)] = 5161,
  [SMALL_STATE(138)] = 5172,
  [SMALL_STATE(139)] = 5183,
  [SMALL_STATE(140)] = 5194,
  [SMALL_STATE(141)] = 5211,
  [SMALL_STATE(142)] = 5228,
  [SMALL_STATE(143)] = 5239,
  [SMALL_STATE(144)] = 5256,
  [SMALL_STATE(145)] = 5267,
  [SMALL_STATE(146)] = 5284,
  [SMALL_STATE(147)] = 5301,
  [SMALL_STATE(148)] = 5318,
  [SMALL_STATE(149)] = 5329,
  [SMALL_STATE(150)] = 5340,
  [SMALL_STATE(151)] = 5351,
  [SMALL_STATE(152)] = 5368,
  [SMALL_STATE(153)] = 5385,
  [SMALL_STATE(154)] = 5402,
  [SMALL_STATE(155)] = 5419,
  [SMALL_STATE(156)] = 5430,
  [SMALL_STATE(157)] = 5441,
  [SMALL_STATE(158)] = 5456,
  [SMALL_STATE(159)] = 5467,
  [SMALL_STATE(160)] = 5478,
  [SMALL_STATE(161)] = 5495,
  [SMALL_STATE(162)] = 5506,
  [SMALL_STATE(163)] = 5517,
  [SMALL_STATE(164)] = 5534,
  [SMALL_STATE(165)] = 5551,
  [SMALL_STATE(166)] = 5562,
  [SMALL_STATE(167)] = 5573,
  [SMALL_STATE(168)] = 5584,
  [SMALL_STATE(169)] = 5595,
  [SMALL_STATE(170)] = 5612,
  [SMALL_STATE(171)] = 5629,
  [SMALL_STATE(172)] = 5640,
  [SMALL_STATE(173)] = 5651,
  [SMALL_STATE(174)] = 5666,
  [SMALL_STATE(175)] = 5683,
  [SMALL_STATE(176)] = 5700,
  [SMALL_STATE(177)] = 5717,
  [SMALL_STATE(178)] = 5734,
  [SMALL_STATE(179)] = 5751,
  [SMALL_STATE(180)] = 5768,
  [SMALL_STATE(181)] = 5785,
  [SMALL_STATE(182)] = 5802,
  [SMALL_STATE(183)] = 5819,
  [SMALL_STATE(184)] = 5830,
  [SMALL_STATE(185)] = 5841,
  [SMALL_STATE(186)] = 5852,
  [SMALL_STATE(187)] = 5863,
  [SMALL_STATE(188)] = 5878,
  [SMALL_STATE(189)] = 5895,
  [SMALL_STATE(190)] = 5910,
  [SMALL_STATE(191)] = 5927,
  [SMALL_STATE(192)] = 5944,
  [SMALL_STATE(193)] = 5961,
  [SMALL_STATE(194)] = 5978,
  [SMALL_STATE(195)] = 5995,
  [SMALL_STATE(196)] = 6010,
  [SMALL_STATE(197)] = 6021,
  [SMALL_STATE(198)] = 6038,
  [SMALL_STATE(199)] = 6049,
  [SMALL_STATE(200)] = 6066,
  [SMALL_STATE(201)] = 6077,
  [SMALL_STATE(202)] = 6094,
  [SMALL_STATE(203)] = 6111,
  [SMALL_STATE(204)] = 6128,
  [SMALL_STATE(205)] = 6145,
  [SMALL_STATE(206)] = 6159,
  [SMALL_STATE(207)] = 6169,
  [SMALL_STATE(208)] = 6181,
  [SMALL_STATE(209)] = 6195,
  [SMALL_STATE(210)] = 6205,
  [SMALL_STATE(211)] = 6219,
  [SMALL_STATE(212)] = 6233,
  [SMALL_STATE(213)] = 6243,
  [SMALL_STATE(214)] = 6255,
  [SMALL_STATE(215)] = 6269,
  [SMALL_STATE(216)] = 6283,
  [SMALL_STATE(217)] = 6293,
  [SMALL_STATE(218)] = 6303,
  [SMALL_STATE(219)] = 6317,
  [SMALL_STATE(220)] = 6327,
  [SMALL_STATE(221)] = 6337,
  [SMALL_STATE(222)] = 6346,
  [SMALL_STATE(223)] = 6357,
  [SMALL_STATE(224)] = 6366,
  [SMALL_STATE(225)] = 6377,
  [SMALL_STATE(226)] = 6388,
  [SMALL_STATE(227)] = 6397,
  [SMALL_STATE(228)] = 6406,
  [SMALL_STATE(229)] = 6415,
  [SMALL_STATE(230)] = 6424,
  [SMALL_STATE(231)] = 6433,
  [SMALL_STATE(232)] = 6442,
  [SMALL_STATE(233)] = 6451,
  [SMALL_STATE(234)] = 6460,
  [SMALL_STATE(235)] = 6469,
  [SMALL_STATE(236)] = 6478,
  [SMALL_STATE(237)] = 6487,
  [SMALL_STATE(238)] = 6496,
  [SMALL_STATE(239)] = 6505,
  [SMALL_STATE(240)] = 6514,
  [SMALL_STATE(241)] = 6522,
  [SMALL_STATE(242)] = 6530,
  [SMALL_STATE(243)] = 6538,
  [SMALL_STATE(244)] = 6546,
  [SMALL_STATE(245)] = 6554,
  [SMALL_STATE(246)] = 6562,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(243),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(132),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(183),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(240),
  [19] = {.entry = {.count = 1, .reusable = false}}, SHIFT(85),
  [21] = {.entry = {.count = 1, .reusable = false}}, SHIFT(86),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(80),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(91),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(105),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(125),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(241),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(159),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(161),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(165),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(142),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(184),
  [57] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 5),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 5),
  [61] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 5), SHIFT_REPEAT(215),
  [64] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 1, .production_id = 1),
  [66] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 1, .production_id = 1),
  [68] = {.entry = {.count = 1, .reusable = true}}, SHIFT(215),
  [70] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 2, .production_id = 14),
  [72] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 2, .production_id = 14),
  [74] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 2, .production_id = 2),
  [76] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 2, .production_id = 2),
  [78] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 3, .production_id = 28),
  [80] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 3, .production_id = 28),
  [82] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_op, 3, .production_id = 35),
  [84] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_op, 3, .production_id = 35),
  [86] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [88] = {.entry = {.count = 1, .reusable = false}}, SHIFT(76),
  [90] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [92] = {.entry = {.count = 1, .reusable = false}}, SHIFT(77),
  [94] = {.entry = {.count = 1, .reusable = false}}, SHIFT(245),
  [96] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [98] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [100] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [102] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [104] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [106] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 4, .production_id = 37),
  [108] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 4, .production_id = 37),
  [110] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_op, 2, .production_id = 21),
  [112] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_op, 2, .production_id = 21),
  [114] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 3),
  [116] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 3),
  [118] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 3),
  [120] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 3),
  [122] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 6, .production_id = 48),
  [124] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 6, .production_id = 48),
  [126] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 4, .production_id = 3),
  [128] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 4, .production_id = 3),
  [130] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 5, .production_id = 6),
  [132] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 5, .production_id = 6),
  [134] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 5, .production_id = 42),
  [136] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 5, .production_id = 42),
  [138] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 7, .production_id = 50),
  [140] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 7, .production_id = 50),
  [142] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 5, .production_id = 46),
  [144] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 5, .production_id = 46),
  [146] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 6, .production_id = 49),
  [148] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 6, .production_id = 49),
  [150] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 33),
  [152] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 33),
  [154] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_access, 3, .production_id = 36),
  [156] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_access, 3, .production_id = 36),
  [158] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 2, .production_id = 23),
  [160] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 2, .production_id = 23),
  [162] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 2),
  [164] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 2),
  [166] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 33),
  [168] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 33),
  [170] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_op, 2, .production_id = 17),
  [172] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_op, 2, .production_id = 17),
  [174] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 3, .production_id = 3),
  [176] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 3, .production_id = 3),
  [178] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 4, .production_id = 6),
  [180] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 4, .production_id = 6),
  [182] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [184] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_latency_specifier, 2, .production_id = 33),
  [186] = {.entry = {.count = 1, .reusable = false}}, SHIFT(75),
  [188] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_latency_specifier, 2, .production_id = 33),
  [190] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [192] = {.entry = {.count = 1, .reusable = true}}, SHIFT(245),
  [194] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__linebreak, 2),
  [196] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__linebreak, 2),
  [198] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__linebreak, 2), SHIFT_REPEAT(44),
  [201] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_to, 1, .production_id = 9),
  [203] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_to, 1, .production_id = 9),
  [205] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [207] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [209] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_to, 2, .production_id = 22),
  [211] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_to, 2, .production_id = 22),
  [213] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__type, 1),
  [215] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expression, 1),
  [217] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expression, 1),
  [219] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym__type, 1), REDUCE(sym__expression, 1),
  [222] = {.entry = {.count = 1, .reusable = false}}, SHIFT(65),
  [224] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [226] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [228] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [230] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_decl_assign_statement, 3, .production_id = 34),
  [232] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_value_param, 3, .production_id = 45),
  [234] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 3),
  [236] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [238] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_value_param, 1, .production_id = 41),
  [240] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [242] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [244] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [246] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [248] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [250] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [252] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [254] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [256] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [258] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__comma, 1),
  [260] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__comma, 1),
  [262] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [264] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [266] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [268] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [270] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [272] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__comma, 2),
  [274] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__comma, 2),
  [276] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [278] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [280] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [282] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [284] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [286] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [288] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [290] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [292] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [294] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [296] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [298] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_write_modifiers, 1, .production_id = 10),
  [300] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_write_modifiers, 1, .production_id = 10),
  [302] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 5),
  [304] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 5), SHIFT_REPEAT(85),
  [307] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 5),
  [309] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 1, .production_id = 1),
  [311] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_write_modifiers_repeat1, 1, .production_id = 1),
  [313] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_write_modifiers, 1, .production_id = 1),
  [315] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_write_modifiers, 1, .production_id = 1),
  [317] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [319] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [321] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [323] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 27),
  [325] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [327] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 2, .production_id = 16),
  [329] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 26),
  [331] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 38),
  [333] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [335] = {.entry = {.count = 1, .reusable = true}}, SHIFT(198),
  [337] = {.entry = {.count = 1, .reusable = true}}, SHIFT(242),
  [339] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 40),
  [341] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [343] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 29),
  [345] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 5, .production_id = 43),
  [347] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 39),
  [349] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration_list, 2, .production_id = 2),
  [351] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [353] = {.entry = {.count = 1, .reusable = false}}, SHIFT(134),
  [355] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [357] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 5),
  [359] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 5), SHIFT_REPEAT(66),
  [362] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [364] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration_list, 1, .production_id = 1),
  [366] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [368] = {.entry = {.count = 1, .reusable = true}}, SHIFT(128),
  [370] = {.entry = {.count = 1, .reusable = true}}, SHIFT(211),
  [372] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 2, .production_id = 2),
  [374] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 5),
  [376] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 5), SHIFT_REPEAT(66),
  [379] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [381] = {.entry = {.count = 1, .reusable = true}}, SHIFT(208),
  [383] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 1, .production_id = 1),
  [385] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 4, .production_id = 6),
  [387] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [389] = {.entry = {.count = 1, .reusable = true}}, SHIFT(135),
  [391] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [393] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [395] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2, .production_id = 8),
  [397] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 5), SHIFT_REPEAT(211),
  [400] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 3),
  [402] = {.entry = {.count = 1, .reusable = true}}, SHIFT(185),
  [404] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [406] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 2),
  [408] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [410] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 1),
  [412] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 19),
  [414] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [416] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 3),
  [418] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 37),
  [420] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 6),
  [422] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 3),
  [424] = {.entry = {.count = 1, .reusable = true}}, SHIFT(196),
  [426] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [428] = {.entry = {.count = 1, .reusable = true}}, SHIFT(226),
  [430] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type, 2, .production_id = 17),
  [432] = {.entry = {.count = 1, .reusable = true}}, SHIFT(200),
  [434] = {.entry = {.count = 1, .reusable = true}}, SHIFT(166),
  [436] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [438] = {.entry = {.count = 1, .reusable = true}}, SHIFT(168),
  [440] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [442] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 3),
  [444] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_declaration_arguments_repeat1, 2, .production_id = 5),
  [446] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_declaration_arguments_repeat1, 2, .production_id = 5), SHIFT_REPEAT(66),
  [449] = {.entry = {.count = 1, .reusable = true}}, SHIFT(162),
  [451] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [453] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [455] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 6),
  [457] = {.entry = {.count = 1, .reusable = true}}, SHIFT(156),
  [459] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [461] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [463] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 37),
  [465] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 42),
  [467] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 6),
  [469] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [471] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 5),
  [473] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 5), SHIFT_REPEAT(10),
  [476] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [478] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_type_param, 1, .production_id = 41),
  [480] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [482] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [484] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 5),
  [486] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 5), SHIFT_REPEAT(151),
  [489] = {.entry = {.count = 1, .reusable = true}}, SHIFT(138),
  [491] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [493] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [495] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [497] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [499] = {.entry = {.count = 1, .reusable = true}}, SHIFT(155),
  [501] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [503] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 5),
  [505] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 5), SHIFT_REPEAT(66),
  [508] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 6, .production_id = 42),
  [510] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, .production_id = 3),
  [512] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_statement, 2, .production_id = 20),
  [514] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [516] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__linebreak, 2), SHIFT_REPEAT(189),
  [519] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 3),
  [521] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [523] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [525] = {.entry = {.count = 1, .reusable = true}}, SHIFT(229),
  [527] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_params_repeat1, 2, .production_id = 5),
  [529] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_params_repeat1, 2, .production_id = 5), SHIFT_REPEAT(66),
  [532] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 2),
  [534] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [536] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_type_param, 3, .production_id = 45),
  [538] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_params_repeat2, 2, .production_id = 5),
  [540] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_params_repeat2, 2, .production_id = 5), SHIFT_REPEAT(66),
  [543] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [545] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [547] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, .production_id = 1),
  [549] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [551] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [553] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [555] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [557] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 3, .production_id = 32),
  [559] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [561] = {.entry = {.count = 1, .reusable = true}}, SHIFT(237),
  [563] = {.entry = {.count = 1, .reusable = true}}, SHIFT(238),
  [565] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2, .production_id = 7),
  [567] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [569] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [571] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [573] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__interface_ports_output, 2, .production_id = 13),
  [575] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 3),
  [577] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [579] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [581] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [583] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 15),
  [585] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 4, .production_id = 30),
  [587] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [589] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__interface_ports_output, 3, .production_id = 25),
  [591] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 18),
  [593] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_declaration_arguments_repeat1, 2, .production_id = 3),
  [595] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_statement, 7, .production_id = 47),
  [597] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 3, .production_id = 3),
  [599] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_statement, 3, .production_id = 31),
  [601] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 5, .production_id = 44),
  [603] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 4, .production_id = 6),
  [605] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 3, .production_id = 4),
  [607] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 3),
  [609] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_params_repeat1, 2, .production_id = 3),
  [611] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 4, .production_id = 12),
  [613] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 5, .production_id = 24),
  [615] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 4, .production_id = 11),
  [617] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_params_repeat2, 2, .production_id = 3),
  [619] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_type, 1),
  [621] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 2),
  [623] = {.entry = {.count = 1, .reusable = true}}, SHIFT(187),
  [625] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [627] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [629] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [631] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [633] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [635] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
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
