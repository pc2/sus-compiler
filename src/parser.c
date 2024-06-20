#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 253
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
  [56] = 56,
  [57] = 54,
  [58] = 58,
  [59] = 53,
  [60] = 60,
  [61] = 61,
  [62] = 56,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 66,
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
  [82] = 75,
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
  [105] = 104,
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
  [119] = 15,
  [120] = 14,
  [121] = 12,
  [122] = 13,
  [123] = 16,
  [124] = 124,
  [125] = 125,
  [126] = 36,
  [127] = 127,
  [128] = 128,
  [129] = 129,
  [130] = 130,
  [131] = 131,
  [132] = 31,
  [133] = 28,
  [134] = 24,
  [135] = 135,
  [136] = 136,
  [137] = 137,
  [138] = 138,
  [139] = 139,
  [140] = 17,
  [141] = 141,
  [142] = 142,
  [143] = 143,
  [144] = 144,
  [145] = 21,
  [146] = 29,
  [147] = 26,
  [148] = 25,
  [149] = 23,
  [150] = 18,
  [151] = 22,
  [152] = 152,
  [153] = 20,
  [154] = 154,
  [155] = 155,
  [156] = 125,
  [157] = 135,
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
  [195] = 185,
  [196] = 196,
  [197] = 197,
  [198] = 184,
  [199] = 37,
  [200] = 200,
  [201] = 179,
  [202] = 202,
  [203] = 203,
  [204] = 204,
  [205] = 205,
  [206] = 167,
  [207] = 207,
  [208] = 208,
  [209] = 182,
  [210] = 181,
  [211] = 47,
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
  [222] = 213,
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
  [242] = 242,
  [243] = 243,
  [244] = 244,
  [245] = 245,
  [246] = 246,
  [247] = 247,
  [248] = 248,
  [249] = 249,
  [250] = 250,
  [251] = 247,
  [252] = 252,
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
  [40] = {.lex_state = 2},
  [41] = {.lex_state = 2},
  [42] = {.lex_state = 2},
  [43] = {.lex_state = 2},
  [44] = {.lex_state = 2},
  [45] = {.lex_state = 2},
  [46] = {.lex_state = 1},
  [47] = {.lex_state = 1},
  [48] = {.lex_state = 2},
  [49] = {.lex_state = 2},
  [50] = {.lex_state = 2},
  [51] = {.lex_state = 2},
  [52] = {.lex_state = 2},
  [53] = {.lex_state = 1},
  [54] = {.lex_state = 1},
  [55] = {.lex_state = 2},
  [56] = {.lex_state = 1},
  [57] = {.lex_state = 1},
  [58] = {.lex_state = 2},
  [59] = {.lex_state = 1},
  [60] = {.lex_state = 2},
  [61] = {.lex_state = 2},
  [62] = {.lex_state = 1},
  [63] = {.lex_state = 1},
  [64] = {.lex_state = 2},
  [65] = {.lex_state = 2},
  [66] = {.lex_state = 2},
  [67] = {.lex_state = 2},
  [68] = {.lex_state = 1},
  [69] = {.lex_state = 2},
  [70] = {.lex_state = 2},
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
  [93] = {.lex_state = 1},
  [94] = {.lex_state = 1},
  [95] = {.lex_state = 1},
  [96] = {.lex_state = 1},
  [97] = {.lex_state = 1},
  [98] = {.lex_state = 1},
  [99] = {.lex_state = 1},
  [100] = {.lex_state = 0},
  [101] = {.lex_state = 0},
  [102] = {.lex_state = 0},
  [103] = {.lex_state = 0},
  [104] = {.lex_state = 1},
  [105] = {.lex_state = 1},
  [106] = {.lex_state = 0},
  [107] = {.lex_state = 1},
  [108] = {.lex_state = 0},
  [109] = {.lex_state = 0},
  [110] = {.lex_state = 0},
  [111] = {.lex_state = 1},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 0},
  [115] = {.lex_state = 1},
  [116] = {.lex_state = 0},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 0},
  [119] = {.lex_state = 1},
  [120] = {.lex_state = 1},
  [121] = {.lex_state = 1},
  [122] = {.lex_state = 1},
  [123] = {.lex_state = 1},
  [124] = {.lex_state = 0},
  [125] = {.lex_state = 1},
  [126] = {.lex_state = 1},
  [127] = {.lex_state = 0},
  [128] = {.lex_state = 0},
  [129] = {.lex_state = 1},
  [130] = {.lex_state = 0},
  [131] = {.lex_state = 0},
  [132] = {.lex_state = 1},
  [133] = {.lex_state = 1},
  [134] = {.lex_state = 1},
  [135] = {.lex_state = 1},
  [136] = {.lex_state = 0},
  [137] = {.lex_state = 1},
  [138] = {.lex_state = 1},
  [139] = {.lex_state = 1},
  [140] = {.lex_state = 1},
  [141] = {.lex_state = 0},
  [142] = {.lex_state = 1},
  [143] = {.lex_state = 0},
  [144] = {.lex_state = 0},
  [145] = {.lex_state = 1},
  [146] = {.lex_state = 1},
  [147] = {.lex_state = 1},
  [148] = {.lex_state = 1},
  [149] = {.lex_state = 1},
  [150] = {.lex_state = 1},
  [151] = {.lex_state = 1},
  [152] = {.lex_state = 0},
  [153] = {.lex_state = 1},
  [154] = {.lex_state = 1},
  [155] = {.lex_state = 0},
  [156] = {.lex_state = 1},
  [157] = {.lex_state = 1},
  [158] = {.lex_state = 1},
  [159] = {.lex_state = 0},
  [160] = {.lex_state = 0},
  [161] = {.lex_state = 0},
  [162] = {.lex_state = 0},
  [163] = {.lex_state = 1},
  [164] = {.lex_state = 0},
  [165] = {.lex_state = 0},
  [166] = {.lex_state = 0},
  [167] = {.lex_state = 1},
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
  [179] = {.lex_state = 1},
  [180] = {.lex_state = 0},
  [181] = {.lex_state = 1},
  [182] = {.lex_state = 1},
  [183] = {.lex_state = 0},
  [184] = {.lex_state = 1},
  [185] = {.lex_state = 1},
  [186] = {.lex_state = 0},
  [187] = {.lex_state = 0},
  [188] = {.lex_state = 0},
  [189] = {.lex_state = 0},
  [190] = {.lex_state = 0},
  [191] = {.lex_state = 0},
  [192] = {.lex_state = 0},
  [193] = {.lex_state = 0},
  [194] = {.lex_state = 1},
  [195] = {.lex_state = 1},
  [196] = {.lex_state = 0},
  [197] = {.lex_state = 0},
  [198] = {.lex_state = 1},
  [199] = {.lex_state = 1},
  [200] = {.lex_state = 0},
  [201] = {.lex_state = 1},
  [202] = {.lex_state = 0},
  [203] = {.lex_state = 1},
  [204] = {.lex_state = 0},
  [205] = {.lex_state = 0},
  [206] = {.lex_state = 1},
  [207] = {.lex_state = 1},
  [208] = {.lex_state = 0},
  [209] = {.lex_state = 1},
  [210] = {.lex_state = 1},
  [211] = {.lex_state = 0},
  [212] = {.lex_state = 1},
  [213] = {.lex_state = 0},
  [214] = {.lex_state = 0},
  [215] = {.lex_state = 0},
  [216] = {.lex_state = 0},
  [217] = {.lex_state = 0},
  [218] = {.lex_state = 0},
  [219] = {.lex_state = 0},
  [220] = {.lex_state = 1},
  [221] = {.lex_state = 0},
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
  [235] = {.lex_state = 1},
  [236] = {.lex_state = 0},
  [237] = {.lex_state = 0},
  [238] = {.lex_state = 1},
  [239] = {.lex_state = 0},
  [240] = {.lex_state = 0},
  [241] = {.lex_state = 0},
  [242] = {.lex_state = 1},
  [243] = {.lex_state = 0},
  [244] = {.lex_state = 0},
  [245] = {.lex_state = 0},
  [246] = {.lex_state = 0},
  [247] = {.lex_state = 0},
  [248] = {.lex_state = 0},
  [249] = {.lex_state = 0},
  [250] = {.lex_state = 0},
  [251] = {.lex_state = 0},
  [252] = {.lex_state = 0},
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
    [sym_source_file] = STATE(248),
    [sym_module] = STATE(200),
    [aux_sym__linebreak] = STATE(155),
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
    STATE(46), 1,
      sym_write_modifiers,
    STATE(47), 1,
      aux_sym__linebreak,
    STATE(51), 1,
      sym_template_global,
    STATE(91), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(180), 1,
      sym_declaration,
    STATE(221), 1,
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
    STATE(224), 2,
      sym__type,
      sym_array_type,
    STATE(245), 5,
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
    STATE(50), 7,
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
    STATE(46), 1,
      sym_write_modifiers,
    STATE(47), 1,
      aux_sym__linebreak,
    STATE(51), 1,
      sym_template_global,
    STATE(91), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(180), 1,
      sym_declaration,
    STATE(221), 1,
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
    STATE(224), 2,
      sym__type,
      sym_array_type,
    STATE(245), 5,
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
    STATE(50), 7,
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
    STATE(46), 1,
      sym_write_modifiers,
    STATE(47), 1,
      aux_sym__linebreak,
    STATE(51), 1,
      sym_template_global,
    STATE(91), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(180), 1,
      sym_declaration,
    STATE(221), 1,
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
    STATE(224), 2,
      sym__type,
      sym_array_type,
    STATE(245), 5,
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
    STATE(50), 7,
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
    STATE(46), 1,
      sym_write_modifiers,
    STATE(47), 1,
      aux_sym__linebreak,
    STATE(51), 1,
      sym_template_global,
    STATE(91), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(180), 1,
      sym_declaration,
    STATE(221), 1,
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
    STATE(224), 2,
      sym__type,
      sym_array_type,
    STATE(245), 5,
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
    STATE(50), 7,
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
    STATE(46), 1,
      sym_write_modifiers,
    STATE(47), 1,
      aux_sym__linebreak,
    STATE(51), 1,
      sym_template_global,
    STATE(91), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(131), 1,
      sym_assign_left_side,
    STATE(180), 1,
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
    STATE(224), 2,
      sym__type,
      sym_array_type,
    STATE(196), 5,
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
    STATE(50), 7,
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
    STATE(46), 1,
      sym_write_modifiers,
    STATE(47), 1,
      aux_sym__linebreak,
    STATE(51), 1,
      sym_template_global,
    STATE(91), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(180), 1,
      sym_declaration,
    STATE(221), 1,
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
    STATE(224), 2,
      sym__type,
      sym_array_type,
    STATE(245), 5,
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
    STATE(50), 7,
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
    STATE(6), 1,
      aux_sym__linebreak,
    STATE(46), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(91), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(143), 1,
      sym_assign_left_side,
    STATE(180), 1,
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
    STATE(224), 2,
      sym__type,
      sym_array_type,
    STATE(177), 5,
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
    STATE(50), 7,
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
    STATE(46), 1,
      sym_write_modifiers,
    STATE(47), 1,
      aux_sym__linebreak,
    STATE(51), 1,
      sym_template_global,
    STATE(91), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(180), 1,
      sym_declaration,
    STATE(221), 1,
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
    STATE(224), 2,
      sym__type,
      sym_array_type,
    STATE(245), 5,
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
    STATE(50), 7,
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
    STATE(46), 1,
      sym_write_modifiers,
    STATE(47), 1,
      aux_sym__linebreak,
    STATE(51), 1,
      sym_template_global,
    STATE(91), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(124), 1,
      sym_assign_to,
    STATE(180), 1,
      sym_declaration,
    STATE(221), 1,
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
    STATE(224), 2,
      sym__type,
      sym_array_type,
    STATE(245), 5,
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
    STATE(50), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [888] = 17,
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
    STATE(46), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(91), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(180), 1,
      sym_declaration,
    STATE(183), 1,
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
    STATE(224), 2,
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
    STATE(50), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [956] = 5,
    ACTIONS(61), 1,
      anon_sym_COLON_COLON,
    STATE(15), 1,
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
    ACTIONS(59), 20,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [999] = 5,
    ACTIONS(61), 1,
      anon_sym_COLON_COLON,
    STATE(14), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(63), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(65), 20,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1042] = 5,
    ACTIONS(61), 1,
      anon_sym_COLON_COLON,
    STATE(15), 1,
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
    ACTIONS(69), 20,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1085] = 5,
    ACTIONS(75), 1,
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
    ACTIONS(73), 20,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1128] = 5,
    ACTIONS(61), 1,
      anon_sym_COLON_COLON,
    STATE(12), 1,
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
    ACTIONS(80), 20,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1171] = 3,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1209] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(88), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1247] = 8,
    ACTIONS(94), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(92), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
    ACTIONS(90), 19,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1295] = 3,
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
    ACTIONS(102), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1333] = 3,
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
    ACTIONS(106), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1371] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(108), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(110), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1409] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(114), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1447] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(116), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(118), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1485] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(120), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(122), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1523] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(124), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(126), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1561] = 15,
    ACTIONS(94), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(132), 1,
      anon_sym_PLUS,
    ACTIONS(134), 1,
      anon_sym_DASH,
    ACTIONS(138), 1,
      anon_sym_PIPE,
    ACTIONS(140), 1,
      anon_sym_AMP,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(130), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(128), 13,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1623] = 3,
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
    ACTIONS(148), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1661] = 3,
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
      anon_sym_COLON_COLON,
      anon_sym_COMMA,
      anon_sym_LF,
  [1699] = 10,
    ACTIONS(94), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(130), 4,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
    ACTIONS(128), 17,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1751] = 3,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1789] = 8,
    ACTIONS(94), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(130), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
    ACTIONS(128), 19,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1837] = 13,
    ACTIONS(94), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(132), 1,
      anon_sym_PLUS,
    ACTIONS(134), 1,
      anon_sym_DASH,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(130), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(128), 15,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1895] = 14,
    ACTIONS(94), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(132), 1,
      anon_sym_PLUS,
    ACTIONS(134), 1,
      anon_sym_DASH,
    ACTIONS(138), 1,
      anon_sym_PIPE,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(130), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(128), 14,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [1955] = 12,
    ACTIONS(94), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(132), 1,
      anon_sym_PLUS,
    ACTIONS(134), 1,
      anon_sym_DASH,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(130), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(128), 16,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [2011] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(158), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(160), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [2049] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(162), 8,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(164), 20,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [2086] = 3,
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
    ACTIONS(166), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [2122] = 3,
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
    ACTIONS(170), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [2158] = 3,
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
    ACTIONS(174), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [2194] = 3,
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
    ACTIONS(178), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [2230] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(184), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(182), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [2266] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(188), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(186), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [2302] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(192), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(190), 21,
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
      anon_sym_COMMA,
      anon_sym_LF,
  [2338] = 17,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(132), 1,
      anon_sym_PLUS,
    ACTIONS(134), 1,
      anon_sym_DASH,
    ACTIONS(138), 1,
      anon_sym_PIPE,
    ACTIONS(140), 1,
      anon_sym_AMP,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    ACTIONS(198), 1,
      anon_sym_EQ,
    ACTIONS(202), 1,
      anon_sym_DOT,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(196), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(200), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
    ACTIONS(194), 6,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_COMMA,
      anon_sym_LF,
  [2401] = 12,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(204), 1,
      sym_number,
    STATE(51), 1,
      sym_template_global,
    STATE(178), 1,
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
    STATE(224), 2,
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
    STATE(49), 7,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
  [2454] = 5,
    ACTIONS(210), 1,
      anon_sym_LF,
    STATE(47), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(206), 11,
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
    ACTIONS(208), 12,
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
  [2492] = 18,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(138), 1,
      anon_sym_PIPE,
    ACTIONS(140), 1,
      anon_sym_AMP,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    ACTIONS(202), 1,
      anon_sym_DOT,
    ACTIONS(213), 1,
      anon_sym_RPAREN,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    STATE(86), 1,
      sym__comma,
    STATE(169), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(132), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(196), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(200), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2554] = 16,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(138), 1,
      anon_sym_PIPE,
    ACTIONS(140), 1,
      anon_sym_AMP,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    ACTIONS(202), 1,
      anon_sym_DOT,
    ACTIONS(219), 1,
      anon_sym_EQ,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(132), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(196), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(217), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_LF,
    ACTIONS(200), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2612] = 16,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(138), 1,
      anon_sym_PIPE,
    ACTIONS(140), 1,
      anon_sym_AMP,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    ACTIONS(202), 1,
      anon_sym_DOT,
    ACTIONS(223), 1,
      anon_sym_EQ,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(132), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(196), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(221), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_LF,
    ACTIONS(200), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2670] = 5,
    ACTIONS(225), 1,
      sym_identifier,
    ACTIONS(231), 1,
      anon_sym_LBRACK,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(227), 4,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_SLASH,
    ACTIONS(229), 16,
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
  [2705] = 15,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(138), 1,
      anon_sym_PIPE,
    ACTIONS(140), 1,
      anon_sym_AMP,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    ACTIONS(202), 1,
      anon_sym_DOT,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(132), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(196), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(234), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
    ACTIONS(200), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2759] = 9,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(236), 1,
      sym_identifier,
    ACTIONS(238), 1,
      anon_sym_GT,
    ACTIONS(240), 1,
      sym_number,
    STATE(179), 1,
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
    STATE(65), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [2801] = 9,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(236), 1,
      sym_identifier,
    ACTIONS(240), 1,
      sym_number,
    ACTIONS(242), 1,
      anon_sym_GT,
    STATE(181), 1,
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
    STATE(65), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [2843] = 15,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(138), 1,
      anon_sym_PIPE,
    ACTIONS(140), 1,
      anon_sym_AMP,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    ACTIONS(202), 1,
      anon_sym_DOT,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(132), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(196), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(244), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(200), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2897] = 9,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(236), 1,
      sym_identifier,
    ACTIONS(240), 1,
      sym_number,
    ACTIONS(246), 1,
      anon_sym_GT,
    STATE(198), 1,
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
    STATE(65), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [2939] = 9,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(236), 1,
      sym_identifier,
    ACTIONS(240), 1,
      sym_number,
    ACTIONS(248), 1,
      anon_sym_GT,
    STATE(210), 1,
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
    STATE(65), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [2981] = 6,
    ACTIONS(61), 1,
      anon_sym_COLON_COLON,
    ACTIONS(250), 1,
      anon_sym_EQ,
    STATE(12), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(78), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(80), 15,
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
      anon_sym_COMMA,
  [3017] = 9,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(236), 1,
      sym_identifier,
    ACTIONS(240), 1,
      sym_number,
    ACTIONS(252), 1,
      anon_sym_GT,
    STATE(201), 1,
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
    STATE(65), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3059] = 16,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(138), 1,
      anon_sym_PIPE,
    ACTIONS(140), 1,
      anon_sym_AMP,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    ACTIONS(202), 1,
      anon_sym_DOT,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    STATE(215), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(132), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(196), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(200), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3115] = 16,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(138), 1,
      anon_sym_PIPE,
    ACTIONS(140), 1,
      anon_sym_AMP,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    ACTIONS(202), 1,
      anon_sym_DOT,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    STATE(231), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(132), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(196), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(200), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3171] = 9,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(236), 1,
      sym_identifier,
    ACTIONS(240), 1,
      sym_number,
    ACTIONS(254), 1,
      anon_sym_GT,
    STATE(184), 1,
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
    STATE(65), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3213] = 8,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(236), 1,
      sym_identifier,
    ACTIONS(240), 1,
      sym_number,
    STATE(242), 1,
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
    STATE(65), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3252] = 15,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(138), 1,
      anon_sym_PIPE,
    ACTIONS(140), 1,
      anon_sym_AMP,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    ACTIONS(202), 1,
      anon_sym_DOT,
    ACTIONS(256), 1,
      anon_sym_COMMA,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(132), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(196), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(200), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3305] = 15,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(138), 1,
      anon_sym_PIPE,
    ACTIONS(140), 1,
      anon_sym_AMP,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    ACTIONS(202), 1,
      anon_sym_DOT,
    ACTIONS(258), 1,
      anon_sym_COMMA,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(132), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(196), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(200), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3358] = 15,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(138), 1,
      anon_sym_PIPE,
    ACTIONS(140), 1,
      anon_sym_AMP,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    ACTIONS(202), 1,
      anon_sym_DOT,
    ACTIONS(260), 1,
      anon_sym_RBRACK,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(132), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(196), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(200), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3411] = 15,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(138), 1,
      anon_sym_PIPE,
    ACTIONS(140), 1,
      anon_sym_AMP,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    ACTIONS(202), 1,
      anon_sym_DOT,
    ACTIONS(262), 1,
      anon_sym_RBRACK,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(132), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(196), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(200), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3464] = 8,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
      sym_identifier,
    ACTIONS(266), 1,
      anon_sym_RPAREN,
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
    STATE(48), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3503] = 15,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(138), 1,
      anon_sym_PIPE,
    ACTIONS(140), 1,
      anon_sym_AMP,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    ACTIONS(202), 1,
      anon_sym_DOT,
    ACTIONS(270), 1,
      anon_sym_RPAREN,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(132), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(196), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(200), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3556] = 15,
    ACTIONS(94), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_LPAREN,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(138), 1,
      anon_sym_PIPE,
    ACTIONS(140), 1,
      anon_sym_AMP,
    ACTIONS(142), 1,
      anon_sym_CARET,
    ACTIONS(144), 1,
      anon_sym_SLASH,
    ACTIONS(272), 1,
      anon_sym_DOT_DOT,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(43), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(132), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(136), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(196), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(200), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3609] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
      sym_identifier,
    ACTIONS(274), 1,
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
    STATE(70), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3645] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
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
    STATE(19), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3681] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
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
    STATE(45), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3717] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
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
    STATE(64), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3753] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
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
    STATE(67), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3789] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
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
    STATE(35), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3825] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
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
    STATE(34), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3861] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
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
    STATE(33), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3897] = 5,
    ACTIONS(294), 1,
      anon_sym_LF,
    STATE(88), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(290), 7,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(292), 10,
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
  [3929] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
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
    STATE(32), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3965] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
      sym_identifier,
    ACTIONS(298), 1,
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
  [4001] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
      sym_identifier,
    ACTIONS(300), 1,
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
    STATE(66), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [4037] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
      sym_identifier,
    ACTIONS(302), 1,
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
    STATE(27), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [4073] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
      sym_identifier,
    ACTIONS(304), 1,
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
  [4109] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
      sym_identifier,
    ACTIONS(306), 1,
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
  [4145] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
      sym_identifier,
    ACTIONS(308), 1,
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
  [4181] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
      sym_identifier,
    ACTIONS(310), 1,
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
    STATE(69), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [4217] = 5,
    ACTIONS(39), 1,
      anon_sym_LF,
    STATE(47), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(312), 7,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(314), 10,
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
  [4249] = 7,
    ACTIONS(33), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
      sym_identifier,
    ACTIONS(316), 1,
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
  [4285] = 5,
    ACTIONS(320), 1,
      anon_sym_reg,
    STATE(90), 1,
      aux_sym_write_modifiers_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(318), 5,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(323), 10,
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
  [4315] = 5,
    ACTIONS(19), 1,
      anon_sym_reg,
    STATE(90), 1,
      aux_sym_write_modifiers_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(325), 5,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(327), 10,
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
  [4345] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(329), 6,
      anon_sym_reg,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(331), 10,
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
  [4370] = 12,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(333), 1,
      anon_sym_DASH_GT,
    ACTIONS(335), 1,
      anon_sym_LF,
    STATE(94), 1,
      aux_sym__linebreak,
    STATE(110), 1,
      sym_declaration,
    STATE(141), 1,
      sym_declaration_list,
    STATE(223), 1,
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
    STATE(224), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4412] = 12,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(39), 1,
      anon_sym_LF,
    ACTIONS(333), 1,
      anon_sym_DASH_GT,
    STATE(47), 1,
      aux_sym__linebreak,
    STATE(110), 1,
      sym_declaration,
    STATE(152), 1,
      sym_declaration_list,
    STATE(214), 1,
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
    STATE(224), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4454] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(337), 5,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(339), 10,
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
  [4478] = 10,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(39), 1,
      anon_sym_LF,
    STATE(47), 1,
      aux_sym__linebreak,
    STATE(110), 1,
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
    STATE(224), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4514] = 10,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(341), 1,
      anon_sym_LF,
    STATE(96), 1,
      aux_sym__linebreak,
    STATE(110), 1,
      sym_declaration,
    STATE(227), 1,
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
    STATE(224), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4550] = 7,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    STATE(136), 1,
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
    STATE(224), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4577] = 7,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    STATE(252), 1,
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
    STATE(224), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4604] = 4,
    ACTIONS(345), 1,
      anon_sym_SQUOTE,
    STATE(108), 1,
      sym_latency_specifier,
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
  [4624] = 4,
    ACTIONS(345), 1,
      anon_sym_SQUOTE,
    STATE(112), 1,
      sym_latency_specifier,
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
  [4644] = 4,
    ACTIONS(345), 1,
      anon_sym_SQUOTE,
    STATE(109), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(349), 7,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_COMMA,
      anon_sym_LF,
  [4664] = 4,
    ACTIONS(345), 1,
      anon_sym_SQUOTE,
    STATE(114), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(351), 7,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_COMMA,
      anon_sym_LF,
  [4684] = 7,
    ACTIONS(353), 1,
      sym_identifier,
    ACTIONS(355), 1,
      anon_sym_GT,
    ACTIONS(357), 1,
      anon_sym_COLON_COLON,
    ACTIONS(359), 1,
      anon_sym_SEMI,
    STATE(156), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(158), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4709] = 7,
    ACTIONS(353), 1,
      sym_identifier,
    ACTIONS(357), 1,
      anon_sym_COLON_COLON,
    ACTIONS(361), 1,
      anon_sym_GT,
    ACTIONS(363), 1,
      anon_sym_SEMI,
    STATE(125), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(158), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4734] = 5,
    ACTIONS(367), 1,
      anon_sym_COMMA,
    STATE(98), 1,
      sym__comma,
    STATE(106), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(365), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [4754] = 5,
    ACTIONS(370), 1,
      anon_sym_EQ,
    ACTIONS(372), 1,
      anon_sym_COLON_COLON,
    STATE(121), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(80), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [4774] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(374), 7,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_COMMA,
      anon_sym_LF,
  [4788] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(376), 7,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_COMMA,
      anon_sym_LF,
  [4802] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    STATE(98), 1,
      sym__comma,
    STATE(113), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(378), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [4822] = 5,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(380), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(225), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4842] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(382), 7,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_COMMA,
      anon_sym_LF,
  [4856] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    STATE(98), 1,
      sym__comma,
    STATE(106), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(384), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [4876] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(386), 7,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_COMMA,
      anon_sym_LF,
  [4890] = 5,
    ACTIONS(353), 1,
      sym_identifier,
    ACTIONS(357), 1,
      anon_sym_COLON_COLON,
    STATE(220), 1,
      sym_template_type_param,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(158), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4909] = 5,
    ACTIONS(390), 1,
      anon_sym_COMMA,
    STATE(11), 1,
      sym__comma,
    STATE(116), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(388), 3,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_LF,
  [4928] = 7,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(393), 1,
      anon_sym_COLON,
    ACTIONS(395), 1,
      anon_sym_LT,
    STATE(204), 1,
      sym_template_declaration_arguments,
    STATE(229), 1,
      sym_interface_ports,
    STATE(243), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4951] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    STATE(11), 1,
      sym__comma,
    STATE(116), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(397), 3,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_LF,
  [4970] = 4,
    ACTIONS(399), 1,
      anon_sym_COLON_COLON,
    STATE(119), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(73), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [4987] = 4,
    ACTIONS(372), 1,
      anon_sym_COLON_COLON,
    STATE(119), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(69), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5004] = 4,
    ACTIONS(372), 1,
      anon_sym_COLON_COLON,
    STATE(119), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(59), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5021] = 4,
    ACTIONS(372), 1,
      anon_sym_COLON_COLON,
    STATE(120), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(65), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5038] = 4,
    ACTIONS(372), 1,
      anon_sym_COLON_COLON,
    STATE(121), 1,
      aux_sym_template_global_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(80), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5055] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    STATE(11), 1,
      sym__comma,
    STATE(118), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(402), 3,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_LF,
  [5074] = 6,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(404), 1,
      anon_sym_GT,
    ACTIONS(406), 1,
      anon_sym_SEMI,
    STATE(115), 1,
      sym__comma,
    STATE(135), 1,
      aux_sym_template_params_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5094] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(160), 5,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5106] = 6,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(408), 1,
      ts_builtin_sym_end,
    ACTIONS(410), 1,
      anon_sym_LF,
    STATE(211), 1,
      aux_sym__linebreak,
    STATE(244), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5126] = 6,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(410), 1,
      anon_sym_LF,
    ACTIONS(412), 1,
      ts_builtin_sym_end,
    STATE(211), 1,
      aux_sym__linebreak,
    STATE(244), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5146] = 4,
    ACTIONS(416), 1,
      anon_sym_LBRACK,
    STATE(154), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(414), 3,
      anon_sym_GT,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5162] = 6,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(410), 1,
      anon_sym_LF,
    ACTIONS(418), 1,
      ts_builtin_sym_end,
    STATE(211), 1,
      aux_sym__linebreak,
    STATE(244), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5182] = 6,
    ACTIONS(420), 1,
      anon_sym_RBRACE,
    ACTIONS(422), 1,
      anon_sym_EQ,
    ACTIONS(424), 1,
      anon_sym_LF,
    STATE(7), 1,
      aux_sym__linebreak,
    STATE(172), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5202] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(156), 5,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5214] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(148), 5,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5226] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(118), 5,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5238] = 6,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(426), 1,
      anon_sym_GT,
    ACTIONS(428), 1,
      anon_sym_SEMI,
    STATE(115), 1,
      sym__comma,
    STATE(142), 1,
      aux_sym_template_params_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5258] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(430), 5,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_LF,
  [5270] = 4,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(226), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [5286] = 4,
    ACTIONS(35), 1,
      anon_sym_COLON_COLON,
    ACTIONS(264), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(218), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [5302] = 4,
    ACTIONS(357), 1,
      anon_sym_COLON_COLON,
    ACTIONS(432), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(129), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [5318] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(84), 5,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5330] = 4,
    ACTIONS(333), 1,
      anon_sym_DASH_GT,
    STATE(216), 1,
      sym__interface_ports_output,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(434), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5346] = 5,
    ACTIONS(438), 1,
      anon_sym_COMMA,
    STATE(115), 1,
      sym__comma,
    STATE(142), 1,
      aux_sym_template_params_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(436), 2,
      anon_sym_GT,
      anon_sym_SEMI,
  [5364] = 6,
    ACTIONS(422), 1,
      anon_sym_EQ,
    ACTIONS(441), 1,
      anon_sym_RBRACE,
    ACTIONS(443), 1,
      anon_sym_LF,
    STATE(3), 1,
      aux_sym__linebreak,
    STATE(174), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5384] = 6,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(410), 1,
      anon_sym_LF,
    ACTIONS(445), 1,
      ts_builtin_sym_end,
    STATE(211), 1,
      aux_sym__linebreak,
    STATE(244), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5404] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 5,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5416] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(152), 5,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5428] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(126), 5,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5440] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(122), 5,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5452] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(114), 5,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5464] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(88), 5,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5476] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(110), 5,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5488] = 4,
    ACTIONS(333), 1,
      anon_sym_DASH_GT,
    STATE(217), 1,
      sym__interface_ports_output,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(447), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5504] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(102), 5,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_COLON_COLON,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5516] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(449), 5,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_SEMI,
      sym_identifier,
      anon_sym_COMMA,
  [5528] = 6,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(410), 1,
      anon_sym_LF,
    ACTIONS(451), 1,
      ts_builtin_sym_end,
    STATE(208), 1,
      sym_module,
    STATE(211), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5548] = 6,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(453), 1,
      anon_sym_GT,
    ACTIONS(455), 1,
      anon_sym_SEMI,
    STATE(115), 1,
      sym__comma,
    STATE(157), 1,
      aux_sym_template_params_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5568] = 6,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(457), 1,
      anon_sym_GT,
    ACTIONS(459), 1,
      anon_sym_SEMI,
    STATE(115), 1,
      sym__comma,
    STATE(142), 1,
      aux_sym_template_params_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5588] = 4,
    ACTIONS(416), 1,
      anon_sym_LBRACK,
    STATE(154), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(461), 3,
      anon_sym_GT,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [5604] = 4,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(463), 1,
      anon_sym_if,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(237), 2,
      sym_block,
      sym_if_statement,
  [5619] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(465), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5630] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(467), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5641] = 4,
    ACTIONS(393), 1,
      anon_sym_COLON,
    STATE(230), 1,
      sym_interface_ports,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(469), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5656] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(471), 1,
      anon_sym_GT,
    STATE(194), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(239), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5673] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(473), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5684] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(475), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5695] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(473), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5706] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(477), 1,
      anon_sym_GT,
    STATE(63), 1,
      sym__comma,
    STATE(207), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5723] = 5,
    ACTIONS(479), 1,
      anon_sym_RBRACE,
    ACTIONS(481), 1,
      anon_sym_LF,
    STATE(4), 1,
      aux_sym__linebreak,
    STATE(188), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5740] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(483), 1,
      anon_sym_RPAREN,
    STATE(86), 1,
      sym__comma,
    STATE(189), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5757] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(485), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5768] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(487), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5779] = 5,
    ACTIONS(489), 1,
      anon_sym_RBRACE,
    ACTIONS(491), 1,
      anon_sym_LF,
    STATE(2), 1,
      aux_sym__linebreak,
    STATE(188), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5796] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(487), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5807] = 5,
    ACTIONS(493), 1,
      anon_sym_RBRACE,
    ACTIONS(495), 1,
      anon_sym_LF,
    STATE(5), 1,
      aux_sym__linebreak,
    STATE(188), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5824] = 5,
    ACTIONS(497), 1,
      anon_sym_RBRACE,
    ACTIONS(499), 1,
      anon_sym_LF,
    STATE(9), 1,
      aux_sym__linebreak,
    STATE(188), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5841] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(501), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5852] = 5,
    ACTIONS(441), 1,
      anon_sym_RBRACE,
    ACTIONS(443), 1,
      anon_sym_LF,
    STATE(3), 1,
      aux_sym__linebreak,
    STATE(168), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5869] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(217), 4,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_COMMA,
      anon_sym_LF,
  [5880] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(503), 1,
      anon_sym_GT,
    STATE(63), 1,
      sym__comma,
    STATE(195), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5897] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(221), 4,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_COMMA,
      anon_sym_LF,
  [5908] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(505), 1,
      anon_sym_GT,
    STATE(63), 1,
      sym__comma,
    STATE(167), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5925] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(507), 1,
      anon_sym_GT,
    STATE(63), 1,
      sym__comma,
    STATE(207), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5942] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(509), 4,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_COMMA,
      anon_sym_LF,
  [5953] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(511), 1,
      anon_sym_GT,
    STATE(63), 1,
      sym__comma,
    STATE(182), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5970] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(513), 1,
      anon_sym_GT,
    STATE(63), 1,
      sym__comma,
    STATE(207), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5987] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(465), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5998] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(515), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [6009] = 5,
    ACTIONS(517), 1,
      anon_sym_RBRACE,
    ACTIONS(519), 1,
      anon_sym_LF,
    STATE(10), 1,
      aux_sym__linebreak,
    STATE(188), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6026] = 5,
    ACTIONS(522), 1,
      anon_sym_RPAREN,
    ACTIONS(524), 1,
      anon_sym_COMMA,
    STATE(86), 1,
      sym__comma,
    STATE(189), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6043] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(527), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [6054] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(527), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [6065] = 5,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(410), 1,
      anon_sym_LF,
    STATE(211), 1,
      aux_sym__linebreak,
    STATE(244), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6082] = 5,
    ACTIONS(529), 1,
      ts_builtin_sym_end,
    ACTIONS(531), 1,
      anon_sym_LF,
    STATE(127), 1,
      aux_sym__linebreak,
    STATE(197), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6099] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(533), 1,
      anon_sym_GT,
    STATE(203), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(239), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6116] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(535), 1,
      anon_sym_GT,
    STATE(63), 1,
      sym__comma,
    STATE(207), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6133] = 5,
    ACTIONS(420), 1,
      anon_sym_RBRACE,
    ACTIONS(424), 1,
      anon_sym_LF,
    STATE(7), 1,
      aux_sym__linebreak,
    STATE(175), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6150] = 5,
    ACTIONS(537), 1,
      ts_builtin_sym_end,
    ACTIONS(539), 1,
      anon_sym_LF,
    STATE(192), 1,
      aux_sym__linebreak,
    STATE(197), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6167] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(542), 1,
      anon_sym_GT,
    STATE(63), 1,
      sym__comma,
    STATE(209), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6184] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(164), 4,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [6195] = 5,
    ACTIONS(544), 1,
      ts_builtin_sym_end,
    ACTIONS(546), 1,
      anon_sym_LF,
    STATE(144), 1,
      aux_sym__linebreak,
    STATE(205), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6212] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(548), 1,
      anon_sym_GT,
    STATE(63), 1,
      sym__comma,
    STATE(185), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6229] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(550), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [6240] = 5,
    ACTIONS(552), 1,
      anon_sym_GT,
    ACTIONS(554), 1,
      anon_sym_COMMA,
    STATE(203), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(239), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6257] = 5,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(393), 1,
      anon_sym_COLON,
    STATE(236), 1,
      sym_interface_ports,
    STATE(241), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6274] = 5,
    ACTIONS(557), 1,
      ts_builtin_sym_end,
    ACTIONS(559), 1,
      anon_sym_LF,
    STATE(128), 1,
      aux_sym__linebreak,
    STATE(197), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6291] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(561), 1,
      anon_sym_GT,
    STATE(63), 1,
      sym__comma,
    STATE(207), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6308] = 5,
    ACTIONS(563), 1,
      anon_sym_GT,
    ACTIONS(565), 1,
      anon_sym_COMMA,
    STATE(63), 1,
      sym__comma,
    STATE(207), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6325] = 5,
    ACTIONS(568), 1,
      ts_builtin_sym_end,
    ACTIONS(570), 1,
      anon_sym_LF,
    STATE(130), 1,
      aux_sym__linebreak,
    STATE(193), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6342] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(572), 1,
      anon_sym_GT,
    STATE(63), 1,
      sym__comma,
    STATE(207), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6359] = 5,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(574), 1,
      anon_sym_GT,
    STATE(63), 1,
      sym__comma,
    STATE(206), 1,
      aux_sym_template_params_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6376] = 4,
    ACTIONS(576), 1,
      anon_sym_LF,
    STATE(211), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(208), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [6391] = 4,
    ACTIONS(579), 1,
      sym_identifier,
    ACTIONS(581), 1,
      anon_sym_GT,
    STATE(163), 1,
      sym_template_declaration_type,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6405] = 4,
    ACTIONS(583), 1,
      sym_identifier,
    ACTIONS(585), 1,
      anon_sym_LT,
    STATE(153), 1,
      sym_template_params,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6419] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(587), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6429] = 3,
    ACTIONS(591), 1,
      anon_sym_else,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(589), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6441] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(593), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6451] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(595), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6461] = 4,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(597), 1,
      sym_identifier,
    STATE(154), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6475] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(599), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6485] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(601), 3,
      anon_sym_GT,
      anon_sym_SEMI,
      anon_sym_COMMA,
  [6495] = 3,
    ACTIONS(422), 1,
      anon_sym_EQ,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(603), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6507] = 4,
    ACTIONS(605), 1,
      sym_identifier,
    ACTIONS(607), 1,
      anon_sym_LT,
    STATE(20), 1,
      sym_template_params,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6521] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(609), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6531] = 4,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(611), 1,
      sym_identifier,
    STATE(154), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6545] = 4,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(613), 1,
      sym_identifier,
    STATE(154), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6559] = 4,
    ACTIONS(98), 1,
      anon_sym_LBRACK,
    ACTIONS(615), 1,
      sym_identifier,
    STATE(154), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6573] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(617), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6583] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(619), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6592] = 3,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    STATE(228), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6603] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(621), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6612] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(623), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6621] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(625), 2,
      anon_sym_COLON,
      anon_sym_LBRACE,
  [6630] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(627), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6639] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(629), 2,
      anon_sym_COLON,
      anon_sym_LBRACE,
  [6648] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(631), 2,
      anon_sym_GT,
      anon_sym_COMMA,
  [6657] = 3,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    STATE(233), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6668] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(633), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6677] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(635), 2,
      anon_sym_GT,
      anon_sym_COMMA,
  [6686] = 3,
    ACTIONS(579), 1,
      sym_identifier,
    STATE(235), 1,
      sym_template_declaration_type,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6697] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(637), 2,
      anon_sym_COLON,
      anon_sym_LBRACE,
  [6706] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(639), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6715] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(641), 2,
      anon_sym_GT,
      anon_sym_COMMA,
  [6724] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(643), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6733] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(645), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6742] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(603), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6751] = 2,
    ACTIONS(647), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6759] = 2,
    ACTIONS(649), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6767] = 2,
    ACTIONS(651), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6775] = 2,
    ACTIONS(653), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6783] = 2,
    ACTIONS(655), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6791] = 2,
    ACTIONS(657), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6799] = 2,
    ACTIONS(659), 1,
      anon_sym_in,
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
  [SMALL_STATE(12)] = 956,
  [SMALL_STATE(13)] = 999,
  [SMALL_STATE(14)] = 1042,
  [SMALL_STATE(15)] = 1085,
  [SMALL_STATE(16)] = 1128,
  [SMALL_STATE(17)] = 1171,
  [SMALL_STATE(18)] = 1209,
  [SMALL_STATE(19)] = 1247,
  [SMALL_STATE(20)] = 1295,
  [SMALL_STATE(21)] = 1333,
  [SMALL_STATE(22)] = 1371,
  [SMALL_STATE(23)] = 1409,
  [SMALL_STATE(24)] = 1447,
  [SMALL_STATE(25)] = 1485,
  [SMALL_STATE(26)] = 1523,
  [SMALL_STATE(27)] = 1561,
  [SMALL_STATE(28)] = 1623,
  [SMALL_STATE(29)] = 1661,
  [SMALL_STATE(30)] = 1699,
  [SMALL_STATE(31)] = 1751,
  [SMALL_STATE(32)] = 1789,
  [SMALL_STATE(33)] = 1837,
  [SMALL_STATE(34)] = 1895,
  [SMALL_STATE(35)] = 1955,
  [SMALL_STATE(36)] = 2011,
  [SMALL_STATE(37)] = 2049,
  [SMALL_STATE(38)] = 2086,
  [SMALL_STATE(39)] = 2122,
  [SMALL_STATE(40)] = 2158,
  [SMALL_STATE(41)] = 2194,
  [SMALL_STATE(42)] = 2230,
  [SMALL_STATE(43)] = 2266,
  [SMALL_STATE(44)] = 2302,
  [SMALL_STATE(45)] = 2338,
  [SMALL_STATE(46)] = 2401,
  [SMALL_STATE(47)] = 2454,
  [SMALL_STATE(48)] = 2492,
  [SMALL_STATE(49)] = 2554,
  [SMALL_STATE(50)] = 2612,
  [SMALL_STATE(51)] = 2670,
  [SMALL_STATE(52)] = 2705,
  [SMALL_STATE(53)] = 2759,
  [SMALL_STATE(54)] = 2801,
  [SMALL_STATE(55)] = 2843,
  [SMALL_STATE(56)] = 2897,
  [SMALL_STATE(57)] = 2939,
  [SMALL_STATE(58)] = 2981,
  [SMALL_STATE(59)] = 3017,
  [SMALL_STATE(60)] = 3059,
  [SMALL_STATE(61)] = 3115,
  [SMALL_STATE(62)] = 3171,
  [SMALL_STATE(63)] = 3213,
  [SMALL_STATE(64)] = 3252,
  [SMALL_STATE(65)] = 3305,
  [SMALL_STATE(66)] = 3358,
  [SMALL_STATE(67)] = 3411,
  [SMALL_STATE(68)] = 3464,
  [SMALL_STATE(69)] = 3503,
  [SMALL_STATE(70)] = 3556,
  [SMALL_STATE(71)] = 3609,
  [SMALL_STATE(72)] = 3645,
  [SMALL_STATE(73)] = 3681,
  [SMALL_STATE(74)] = 3717,
  [SMALL_STATE(75)] = 3753,
  [SMALL_STATE(76)] = 3789,
  [SMALL_STATE(77)] = 3825,
  [SMALL_STATE(78)] = 3861,
  [SMALL_STATE(79)] = 3897,
  [SMALL_STATE(80)] = 3929,
  [SMALL_STATE(81)] = 3965,
  [SMALL_STATE(82)] = 4001,
  [SMALL_STATE(83)] = 4037,
  [SMALL_STATE(84)] = 4073,
  [SMALL_STATE(85)] = 4109,
  [SMALL_STATE(86)] = 4145,
  [SMALL_STATE(87)] = 4181,
  [SMALL_STATE(88)] = 4217,
  [SMALL_STATE(89)] = 4249,
  [SMALL_STATE(90)] = 4285,
  [SMALL_STATE(91)] = 4315,
  [SMALL_STATE(92)] = 4345,
  [SMALL_STATE(93)] = 4370,
  [SMALL_STATE(94)] = 4412,
  [SMALL_STATE(95)] = 4454,
  [SMALL_STATE(96)] = 4478,
  [SMALL_STATE(97)] = 4514,
  [SMALL_STATE(98)] = 4550,
  [SMALL_STATE(99)] = 4577,
  [SMALL_STATE(100)] = 4604,
  [SMALL_STATE(101)] = 4624,
  [SMALL_STATE(102)] = 4644,
  [SMALL_STATE(103)] = 4664,
  [SMALL_STATE(104)] = 4684,
  [SMALL_STATE(105)] = 4709,
  [SMALL_STATE(106)] = 4734,
  [SMALL_STATE(107)] = 4754,
  [SMALL_STATE(108)] = 4774,
  [SMALL_STATE(109)] = 4788,
  [SMALL_STATE(110)] = 4802,
  [SMALL_STATE(111)] = 4822,
  [SMALL_STATE(112)] = 4842,
  [SMALL_STATE(113)] = 4856,
  [SMALL_STATE(114)] = 4876,
  [SMALL_STATE(115)] = 4890,
  [SMALL_STATE(116)] = 4909,
  [SMALL_STATE(117)] = 4928,
  [SMALL_STATE(118)] = 4951,
  [SMALL_STATE(119)] = 4970,
  [SMALL_STATE(120)] = 4987,
  [SMALL_STATE(121)] = 5004,
  [SMALL_STATE(122)] = 5021,
  [SMALL_STATE(123)] = 5038,
  [SMALL_STATE(124)] = 5055,
  [SMALL_STATE(125)] = 5074,
  [SMALL_STATE(126)] = 5094,
  [SMALL_STATE(127)] = 5106,
  [SMALL_STATE(128)] = 5126,
  [SMALL_STATE(129)] = 5146,
  [SMALL_STATE(130)] = 5162,
  [SMALL_STATE(131)] = 5182,
  [SMALL_STATE(132)] = 5202,
  [SMALL_STATE(133)] = 5214,
  [SMALL_STATE(134)] = 5226,
  [SMALL_STATE(135)] = 5238,
  [SMALL_STATE(136)] = 5258,
  [SMALL_STATE(137)] = 5270,
  [SMALL_STATE(138)] = 5286,
  [SMALL_STATE(139)] = 5302,
  [SMALL_STATE(140)] = 5318,
  [SMALL_STATE(141)] = 5330,
  [SMALL_STATE(142)] = 5346,
  [SMALL_STATE(143)] = 5364,
  [SMALL_STATE(144)] = 5384,
  [SMALL_STATE(145)] = 5404,
  [SMALL_STATE(146)] = 5416,
  [SMALL_STATE(147)] = 5428,
  [SMALL_STATE(148)] = 5440,
  [SMALL_STATE(149)] = 5452,
  [SMALL_STATE(150)] = 5464,
  [SMALL_STATE(151)] = 5476,
  [SMALL_STATE(152)] = 5488,
  [SMALL_STATE(153)] = 5504,
  [SMALL_STATE(154)] = 5516,
  [SMALL_STATE(155)] = 5528,
  [SMALL_STATE(156)] = 5548,
  [SMALL_STATE(157)] = 5568,
  [SMALL_STATE(158)] = 5588,
  [SMALL_STATE(159)] = 5604,
  [SMALL_STATE(160)] = 5619,
  [SMALL_STATE(161)] = 5630,
  [SMALL_STATE(162)] = 5641,
  [SMALL_STATE(163)] = 5656,
  [SMALL_STATE(164)] = 5673,
  [SMALL_STATE(165)] = 5684,
  [SMALL_STATE(166)] = 5695,
  [SMALL_STATE(167)] = 5706,
  [SMALL_STATE(168)] = 5723,
  [SMALL_STATE(169)] = 5740,
  [SMALL_STATE(170)] = 5757,
  [SMALL_STATE(171)] = 5768,
  [SMALL_STATE(172)] = 5779,
  [SMALL_STATE(173)] = 5796,
  [SMALL_STATE(174)] = 5807,
  [SMALL_STATE(175)] = 5824,
  [SMALL_STATE(176)] = 5841,
  [SMALL_STATE(177)] = 5852,
  [SMALL_STATE(178)] = 5869,
  [SMALL_STATE(179)] = 5880,
  [SMALL_STATE(180)] = 5897,
  [SMALL_STATE(181)] = 5908,
  [SMALL_STATE(182)] = 5925,
  [SMALL_STATE(183)] = 5942,
  [SMALL_STATE(184)] = 5953,
  [SMALL_STATE(185)] = 5970,
  [SMALL_STATE(186)] = 5987,
  [SMALL_STATE(187)] = 5998,
  [SMALL_STATE(188)] = 6009,
  [SMALL_STATE(189)] = 6026,
  [SMALL_STATE(190)] = 6043,
  [SMALL_STATE(191)] = 6054,
  [SMALL_STATE(192)] = 6065,
  [SMALL_STATE(193)] = 6082,
  [SMALL_STATE(194)] = 6099,
  [SMALL_STATE(195)] = 6116,
  [SMALL_STATE(196)] = 6133,
  [SMALL_STATE(197)] = 6150,
  [SMALL_STATE(198)] = 6167,
  [SMALL_STATE(199)] = 6184,
  [SMALL_STATE(200)] = 6195,
  [SMALL_STATE(201)] = 6212,
  [SMALL_STATE(202)] = 6229,
  [SMALL_STATE(203)] = 6240,
  [SMALL_STATE(204)] = 6257,
  [SMALL_STATE(205)] = 6274,
  [SMALL_STATE(206)] = 6291,
  [SMALL_STATE(207)] = 6308,
  [SMALL_STATE(208)] = 6325,
  [SMALL_STATE(209)] = 6342,
  [SMALL_STATE(210)] = 6359,
  [SMALL_STATE(211)] = 6376,
  [SMALL_STATE(212)] = 6391,
  [SMALL_STATE(213)] = 6405,
  [SMALL_STATE(214)] = 6419,
  [SMALL_STATE(215)] = 6429,
  [SMALL_STATE(216)] = 6441,
  [SMALL_STATE(217)] = 6451,
  [SMALL_STATE(218)] = 6461,
  [SMALL_STATE(219)] = 6475,
  [SMALL_STATE(220)] = 6485,
  [SMALL_STATE(221)] = 6495,
  [SMALL_STATE(222)] = 6507,
  [SMALL_STATE(223)] = 6521,
  [SMALL_STATE(224)] = 6531,
  [SMALL_STATE(225)] = 6545,
  [SMALL_STATE(226)] = 6559,
  [SMALL_STATE(227)] = 6573,
  [SMALL_STATE(228)] = 6583,
  [SMALL_STATE(229)] = 6592,
  [SMALL_STATE(230)] = 6603,
  [SMALL_STATE(231)] = 6612,
  [SMALL_STATE(232)] = 6621,
  [SMALL_STATE(233)] = 6630,
  [SMALL_STATE(234)] = 6639,
  [SMALL_STATE(235)] = 6648,
  [SMALL_STATE(236)] = 6657,
  [SMALL_STATE(237)] = 6668,
  [SMALL_STATE(238)] = 6677,
  [SMALL_STATE(239)] = 6686,
  [SMALL_STATE(240)] = 6697,
  [SMALL_STATE(241)] = 6706,
  [SMALL_STATE(242)] = 6715,
  [SMALL_STATE(243)] = 6724,
  [SMALL_STATE(244)] = 6733,
  [SMALL_STATE(245)] = 6742,
  [SMALL_STATE(246)] = 6751,
  [SMALL_STATE(247)] = 6759,
  [SMALL_STATE(248)] = 6767,
  [SMALL_STATE(249)] = 6775,
  [SMALL_STATE(250)] = 6783,
  [SMALL_STATE(251)] = 6791,
  [SMALL_STATE(252)] = 6799,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(249),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(155),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(191),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(250),
  [19] = {.entry = {.count = 1, .reusable = false}}, SHIFT(92),
  [21] = {.entry = {.count = 1, .reusable = false}}, SHIFT(95),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(89),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(99),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(111),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(137),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(251),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(202),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(164),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(166),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(187),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(170),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(161),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(190),
  [57] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 2, .production_id = 2),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 2, .production_id = 2),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(222),
  [63] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 2, .production_id = 14),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 2, .production_id = 14),
  [67] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 3, .production_id = 28),
  [69] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 3, .production_id = 28),
  [71] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 5),
  [73] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 5),
  [75] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 5), SHIFT_REPEAT(222),
  [78] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 1, .production_id = 1),
  [80] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 1, .production_id = 1),
  [82] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 4, .production_id = 3),
  [84] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 4, .production_id = 3),
  [86] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 6, .production_id = 49),
  [88] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 6, .production_id = 49),
  [90] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_op, 2, .production_id = 21),
  [92] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_op, 2, .production_id = 21),
  [94] = {.entry = {.count = 1, .reusable = false}}, SHIFT(246),
  [96] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [98] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [100] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 3),
  [102] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 3),
  [104] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 4, .production_id = 6),
  [106] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 4, .production_id = 6),
  [108] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 7, .production_id = 50),
  [110] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 7, .production_id = 50),
  [112] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 6, .production_id = 48),
  [114] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 6, .production_id = 48),
  [116] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 4, .production_id = 37),
  [118] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 4, .production_id = 37),
  [120] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 5, .production_id = 6),
  [122] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 5, .production_id = 6),
  [124] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 5, .production_id = 46),
  [126] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 5, .production_id = 46),
  [128] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_op, 3, .production_id = 35),
  [130] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_op, 3, .production_id = 35),
  [132] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [134] = {.entry = {.count = 1, .reusable = false}}, SHIFT(81),
  [136] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [138] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [140] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [142] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [144] = {.entry = {.count = 1, .reusable = false}}, SHIFT(80),
  [146] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 3, .production_id = 3),
  [148] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 3, .production_id = 3),
  [150] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 5, .production_id = 42),
  [152] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 5, .production_id = 42),
  [154] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 3),
  [156] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 3),
  [158] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_params, 2),
  [160] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_params, 2),
  [162] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 33),
  [164] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 33),
  [166] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 4, .production_id = 6),
  [168] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 4, .production_id = 6),
  [170] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 2, .production_id = 23),
  [172] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 2, .production_id = 23),
  [174] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 2),
  [176] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 2),
  [178] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_access, 3, .production_id = 36),
  [180] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_access, 3, .production_id = 36),
  [182] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 3, .production_id = 3),
  [184] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 3, .production_id = 3),
  [186] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_op, 2, .production_id = 17),
  [188] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_op, 2, .production_id = 17),
  [190] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 33),
  [192] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 33),
  [194] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_latency_specifier, 2, .production_id = 33),
  [196] = {.entry = {.count = 1, .reusable = false}}, SHIFT(83),
  [198] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_latency_specifier, 2, .production_id = 33),
  [200] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [202] = {.entry = {.count = 1, .reusable = true}}, SHIFT(246),
  [204] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [206] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__linebreak, 2),
  [208] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__linebreak, 2),
  [210] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__linebreak, 2), SHIFT_REPEAT(47),
  [213] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [215] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [217] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_to, 2, .production_id = 22),
  [219] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_to, 2, .production_id = 22),
  [221] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_to, 1, .production_id = 9),
  [223] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_to, 1, .production_id = 9),
  [225] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__type, 1),
  [227] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expression, 1),
  [229] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expression, 1),
  [231] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym__type, 1), REDUCE(sym__expression, 1),
  [234] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_decl_assign_statement, 3, .production_id = 34),
  [236] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [238] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [240] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [242] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [244] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 3),
  [246] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [248] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [250] = {.entry = {.count = 1, .reusable = false}}, SHIFT(74),
  [252] = {.entry = {.count = 1, .reusable = true}}, SHIFT(132),
  [254] = {.entry = {.count = 1, .reusable = true}}, SHIFT(140),
  [256] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_value_param, 3, .production_id = 45),
  [258] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_value_param, 1, .production_id = 41),
  [260] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [262] = {.entry = {.count = 1, .reusable = true}}, SHIFT(199),
  [264] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [266] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [268] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [270] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [272] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [274] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [276] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [278] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [280] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [282] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [284] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [286] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [288] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [290] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__comma, 1),
  [292] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__comma, 1),
  [294] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [296] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [298] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [300] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [302] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [304] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [306] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [308] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [310] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [312] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__comma, 2),
  [314] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__comma, 2),
  [316] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [318] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 5),
  [320] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 5), SHIFT_REPEAT(92),
  [323] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 5),
  [325] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_write_modifiers, 1, .production_id = 10),
  [327] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_write_modifiers, 1, .production_id = 10),
  [329] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 1, .production_id = 1),
  [331] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_write_modifiers_repeat1, 1, .production_id = 1),
  [333] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [335] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [337] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_write_modifiers, 1, .production_id = 1),
  [339] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_write_modifiers, 1, .production_id = 1),
  [341] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [343] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 38),
  [345] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [347] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 27),
  [349] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 2, .production_id = 16),
  [351] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 26),
  [353] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
  [355] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [357] = {.entry = {.count = 1, .reusable = true}}, SHIFT(247),
  [359] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [361] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [363] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [365] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 5),
  [367] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 5), SHIFT_REPEAT(79),
  [370] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [372] = {.entry = {.count = 1, .reusable = true}}, SHIFT(213),
  [374] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 5, .production_id = 43),
  [376] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 29),
  [378] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration_list, 1, .production_id = 1),
  [380] = {.entry = {.count = 1, .reusable = false}}, SHIFT(138),
  [382] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 40),
  [384] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration_list, 2, .production_id = 2),
  [386] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 39),
  [388] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 5),
  [390] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 5), SHIFT_REPEAT(79),
  [393] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [395] = {.entry = {.count = 1, .reusable = true}}, SHIFT(212),
  [397] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 2, .production_id = 2),
  [399] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_global_repeat1, 2, .production_id = 5), SHIFT_REPEAT(213),
  [402] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 1, .production_id = 1),
  [404] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [406] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [408] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 4, .production_id = 6),
  [410] = {.entry = {.count = 1, .reusable = true}}, SHIFT(211),
  [412] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 2),
  [414] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_type_param, 3, .production_id = 45),
  [416] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [418] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 3),
  [420] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [422] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [424] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [426] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [428] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [430] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 3),
  [432] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
  [434] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2, .production_id = 8),
  [436] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_params_repeat1, 2, .production_id = 5),
  [438] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_params_repeat1, 2, .production_id = 5), SHIFT_REPEAT(79),
  [441] = {.entry = {.count = 1, .reusable = true}}, SHIFT(165),
  [443] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [445] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 1),
  [447] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 19),
  [449] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type, 2, .production_id = 17),
  [451] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [453] = {.entry = {.count = 1, .reusable = true}}, SHIFT(133),
  [455] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [457] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [459] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [461] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_type_param, 1, .production_id = 41),
  [463] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [465] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 6),
  [467] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [469] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_statement, 2, .production_id = 20),
  [471] = {.entry = {.count = 1, .reusable = true}}, SHIFT(240),
  [473] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 6),
  [475] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, .production_id = 3),
  [477] = {.entry = {.count = 1, .reusable = true}}, SHIFT(151),
  [479] = {.entry = {.count = 1, .reusable = true}}, SHIFT(160),
  [481] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [483] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [485] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 37),
  [487] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 42),
  [489] = {.entry = {.count = 1, .reusable = true}}, SHIFT(173),
  [491] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [493] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [495] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [497] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [499] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [501] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 37),
  [503] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [505] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [507] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [509] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 3),
  [511] = {.entry = {.count = 1, .reusable = true}}, SHIFT(147),
  [513] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [515] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [517] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 5),
  [519] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 5), SHIFT_REPEAT(10),
  [522] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 5),
  [524] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 5), SHIFT_REPEAT(79),
  [527] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 6, .production_id = 42),
  [529] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 6),
  [531] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [533] = {.entry = {.count = 1, .reusable = true}}, SHIFT(232),
  [535] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [537] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 5),
  [539] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 5), SHIFT_REPEAT(192),
  [542] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [544] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, .production_id = 1),
  [546] = {.entry = {.count = 1, .reusable = true}}, SHIFT(144),
  [548] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
  [550] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 3),
  [552] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_declaration_arguments_repeat1, 2, .production_id = 5),
  [554] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_declaration_arguments_repeat1, 2, .production_id = 5), SHIFT_REPEAT(79),
  [557] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 2),
  [559] = {.entry = {.count = 1, .reusable = true}}, SHIFT(128),
  [561] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [563] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_params_repeat2, 2, .production_id = 5),
  [565] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_params_repeat2, 2, .production_id = 5), SHIFT_REPEAT(79),
  [568] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 3),
  [570] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [572] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [574] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [576] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__linebreak, 2), SHIFT_REPEAT(211),
  [579] = {.entry = {.count = 1, .reusable = true}}, SHIFT(238),
  [581] = {.entry = {.count = 1, .reusable = true}}, SHIFT(234),
  [583] = {.entry = {.count = 1, .reusable = true}}, SHIFT(153),
  [585] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [587] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 18),
  [589] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 3, .production_id = 32),
  [591] = {.entry = {.count = 1, .reusable = true}}, SHIFT(159),
  [593] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 15),
  [595] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 4, .production_id = 30),
  [597] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [599] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__interface_ports_output, 3, .production_id = 25),
  [601] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_params_repeat1, 2, .production_id = 3),
  [603] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 3),
  [605] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [607] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [609] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2, .production_id = 7),
  [611] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [613] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [615] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [617] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__interface_ports_output, 2, .production_id = 13),
  [619] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 4, .production_id = 11),
  [621] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_statement, 3, .production_id = 31),
  [623] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_statement, 7, .production_id = 47),
  [625] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 4, .production_id = 6),
  [627] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 5, .production_id = 24),
  [629] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 2),
  [631] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_declaration_arguments_repeat1, 2, .production_id = 3),
  [633] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 5, .production_id = 44),
  [635] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_type, 1),
  [637] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 3, .production_id = 3),
  [639] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 4, .production_id = 12),
  [641] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_params_repeat2, 2, .production_id = 3),
  [643] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 3, .production_id = 4),
  [645] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 3),
  [647] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [649] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [651] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [653] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [655] = {.entry = {.count = 1, .reusable = true}}, SHIFT(162),
  [657] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [659] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
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
