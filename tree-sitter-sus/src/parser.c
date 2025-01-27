#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 230
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 100
#define ALIAS_COUNT 0
#define TOKEN_COUNT 55
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 37
#define MAX_ALIAS_SEQUENCE_LENGTH 7
#define PRODUCTION_ID_COUNT 51

enum ts_symbol_identifiers {
  sym_identifier = 1,
  anon_sym___builtin__ = 2,
  anon_sym_extern = 3,
  anon_sym_module = 4,
  anon_sym_struct = 5,
  anon_sym_const = 6,
  anon_sym_POUND_LPAREN = 7,
  anon_sym_RPAREN = 8,
  anon_sym_LBRACE = 9,
  anon_sym_RBRACE = 10,
  anon_sym_EQ = 11,
  anon_sym_reg = 12,
  anon_sym_initial = 13,
  anon_sym_when = 14,
  anon_sym_if = 15,
  anon_sym_else = 16,
  anon_sym_for = 17,
  anon_sym_in = 18,
  anon_sym_DOT_DOT = 19,
  anon_sym_domain = 20,
  anon_sym_interface = 21,
  anon_sym_COLON = 22,
  anon_sym_DASH_GT = 23,
  anon_sym_input = 24,
  anon_sym_output = 25,
  anon_sym_state = 26,
  anon_sym_gen = 27,
  anon_sym_SQUOTE = 28,
  anon_sym_PLUS = 29,
  anon_sym_DASH = 30,
  anon_sym_STAR = 31,
  anon_sym_BANG = 32,
  anon_sym_PIPE = 33,
  anon_sym_AMP = 34,
  anon_sym_CARET = 35,
  anon_sym_EQ_EQ = 36,
  anon_sym_BANG_EQ = 37,
  anon_sym_LT = 38,
  anon_sym_LT_EQ = 39,
  anon_sym_GT = 40,
  anon_sym_GT_EQ = 41,
  anon_sym_SLASH = 42,
  anon_sym_PERCENT = 43,
  anon_sym_DOT = 44,
  anon_sym_LPAREN = 45,
  anon_sym_LBRACK = 46,
  anon_sym_RBRACK = 47,
  anon_sym_COLON_COLON = 48,
  anon_sym_type = 49,
  sym_number = 50,
  anon_sym_COMMA = 51,
  anon_sym_LF = 52,
  sym_single_line_comment = 53,
  sym_multi_line_comment = 54,
  sym_source_file = 55,
  sym_global_object = 56,
  sym_const_and_type = 57,
  sym_template_declaration_arguments = 58,
  sym_template_declaration_type = 59,
  sym_block = 60,
  sym_decl_assign_statement = 61,
  sym_assign_left_side = 62,
  sym_assign_to = 63,
  sym_write_modifiers = 64,
  sym_if_statement = 65,
  sym_for_statement = 66,
  sym_domain_statement = 67,
  sym_interface_statement = 68,
  sym_interface_ports = 69,
  sym__interface_ports_output = 70,
  sym_declaration_list = 71,
  sym_declaration = 72,
  sym_latency_specifier = 73,
  sym__type = 74,
  sym_array_type = 75,
  sym__expression = 76,
  sym_unary_op = 77,
  sym_binary_op = 78,
  sym_array_op = 79,
  sym_func_call = 80,
  sym_field_access = 81,
  sym_parenthesis_expression_list = 82,
  sym_parenthesis_expression = 83,
  sym_array_bracket_expression = 84,
  sym_namespace_list = 85,
  sym_template_global = 86,
  sym_template_args = 87,
  sym_template_arg = 88,
  sym__comma = 89,
  aux_sym__linebreak = 90,
  aux_sym_source_file_repeat1 = 91,
  aux_sym_template_declaration_arguments_repeat1 = 92,
  aux_sym_block_repeat1 = 93,
  aux_sym_assign_left_side_repeat1 = 94,
  aux_sym_write_modifiers_repeat1 = 95,
  aux_sym_declaration_list_repeat1 = 96,
  aux_sym_parenthesis_expression_list_repeat1 = 97,
  aux_sym_namespace_list_repeat1 = 98,
  aux_sym_template_args_repeat1 = 99,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [anon_sym___builtin__] = "__builtin__",
  [anon_sym_extern] = "extern",
  [anon_sym_module] = "module",
  [anon_sym_struct] = "struct",
  [anon_sym_const] = "const",
  [anon_sym_POUND_LPAREN] = "#(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_EQ] = "=",
  [anon_sym_reg] = "reg",
  [anon_sym_initial] = "initial",
  [anon_sym_when] = "when",
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
  [sym_const_and_type] = "const_and_type",
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
  [anon_sym_struct] = anon_sym_struct,
  [anon_sym_const] = anon_sym_const,
  [anon_sym_POUND_LPAREN] = anon_sym_POUND_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_reg] = anon_sym_reg,
  [anon_sym_initial] = anon_sym_initial,
  [anon_sym_when] = anon_sym_when,
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
  [sym_const_and_type] = sym_const_and_type,
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
  [anon_sym_struct] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_const] = {
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
  [anon_sym_when] = {
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
  [sym_const_and_type] = {
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
  field_const_type = 8,
  field_content = 9,
  field_declaration_modifiers = 10,
  field_else_block = 11,
  field_expr_or_decl = 12,
  field_extern_marker = 13,
  field_for_decl = 14,
  field_from = 15,
  field_inputs = 16,
  field_interface_ports = 17,
  field_io_port_modifiers = 18,
  field_is_global_path = 19,
  field_item = 20,
  field_latency_specifier = 21,
  field_left = 22,
  field_name = 23,
  field_namespace_list = 24,
  field_object_type = 25,
  field_operator = 26,
  field_outputs = 27,
  field_right = 28,
  field_statement_type = 29,
  field_template_args = 30,
  field_template_declaration_arguments = 31,
  field_then_block = 32,
  field_to = 33,
  field_type = 34,
  field_type_arg = 35,
  field_val_arg = 36,
  field_write_modifiers = 37,
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
  [field_const_type] = "const_type",
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
  [field_statement_type] = "statement_type",
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
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 1},
  [4] = {.index = 3, .length = 2},
  [5] = {.index = 5, .length = 1},
  [6] = {.index = 6, .length = 3},
  [7] = {.index = 9, .length = 2},
  [8] = {.index = 11, .length = 2},
  [9] = {.index = 13, .length = 2},
  [10] = {.index = 15, .length = 2},
  [11] = {.index = 17, .length = 2},
  [12] = {.index = 19, .length = 4},
  [13] = {.index = 23, .length = 1},
  [14] = {.index = 24, .length = 1},
  [15] = {.index = 25, .length = 1},
  [16] = {.index = 26, .length = 4},
  [17] = {.index = 30, .length = 3},
  [18] = {.index = 33, .length = 5},
  [19] = {.index = 38, .length = 2},
  [20] = {.index = 40, .length = 1},
  [21] = {.index = 41, .length = 2},
  [22] = {.index = 43, .length = 2},
  [23] = {.index = 45, .length = 2},
  [24] = {.index = 47, .length = 1},
  [25] = {.index = 48, .length = 3},
  [26] = {.index = 51, .length = 3},
  [27] = {.index = 54, .length = 3},
  [28] = {.index = 57, .length = 1},
  [29] = {.index = 58, .length = 3},
  [30] = {.index = 61, .length = 2},
  [31] = {.index = 63, .length = 2},
  [32] = {.index = 65, .length = 3},
  [33] = {.index = 68, .length = 2},
  [34] = {.index = 70, .length = 2},
  [35] = {.index = 72, .length = 4},
  [36] = {.index = 76, .length = 4},
  [37] = {.index = 80, .length = 4},
  [38] = {.index = 84, .length = 2},
  [39] = {.index = 86, .length = 1},
  [40] = {.index = 87, .length = 1},
  [41] = {.index = 88, .length = 2},
  [42] = {.index = 90, .length = 5},
  [43] = {.index = 95, .length = 4},
  [44] = {.index = 99, .length = 1},
  [45] = {.index = 100, .length = 2},
  [46] = {.index = 102, .length = 1},
  [47] = {.index = 103, .length = 1},
  [48] = {.index = 104, .length = 1},
  [49] = {.index = 105, .length = 2},
  [50] = {.index = 107, .length = 4},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_item, 0},
  [1] =
    {field_const_type, 1},
  [2] =
    {field_namespace_list, 0},
  [3] =
    {field_item, 0},
    {field_item, 1, .inherited = true},
  [5] =
    {field_item, 1},
  [6] =
    {field_block, 2},
    {field_name, 1},
    {field_object_type, 0},
  [9] =
    {field_is_global_path, 0},
    {field_namespace_list, 1},
  [11] =
    {field_arr, 0},
    {field_arr_idx, 1},
  [13] =
    {field_namespace_list, 0},
    {field_template_args, 1},
  [15] =
    {field_item, 0, .inherited = true},
    {field_item, 1, .inherited = true},
  [17] =
    {field_item, 1},
    {field_item, 2, .inherited = true},
  [19] =
    {field_block, 3},
    {field_extern_marker, 0},
    {field_name, 2},
    {field_object_type, 1},
  [23] =
    {field_name, 0},
  [24] =
    {field_expr_or_decl, 0},
  [25] =
    {field_item, 0, .inherited = true},
  [26] =
    {field_block, 3},
    {field_name, 1},
    {field_object_type, 0},
    {field_template_declaration_arguments, 2},
  [30] =
    {field_is_global_path, 0},
    {field_namespace_list, 1},
    {field_template_args, 2},
  [33] =
    {field_block, 4},
    {field_extern_marker, 0},
    {field_name, 2},
    {field_object_type, 1},
    {field_template_declaration_arguments, 3},
  [38] =
    {field_name, 1},
    {field_type, 0},
  [40] =
    {field_name, 1},
  [41] =
    {field_operator, 0},
    {field_right, 1},
  [43] =
    {field_expr_or_decl, 1},
    {field_write_modifiers, 0},
  [45] =
    {field_arguments, 1},
    {field_name, 0},
  [47] =
    {field_content, 1},
  [48] =
    {field_io_port_modifiers, 0},
    {field_name, 2},
    {field_type, 1},
  [51] =
    {field_declaration_modifiers, 0},
    {field_name, 2},
    {field_type, 1},
  [54] =
    {field_latency_specifier, 2},
    {field_name, 1},
    {field_type, 0},
  [57] =
    {field_item, 2},
  [58] =
    {field_condition, 1},
    {field_statement_type, 0},
    {field_then_block, 2},
  [61] =
    {field_interface_ports, 2},
    {field_name, 1},
  [63] =
    {field_assign_left, 0},
    {field_assign_value, 2},
  [65] =
    {field_left, 0},
    {field_operator, 1},
    {field_right, 2},
  [68] =
    {field_left, 0},
    {field_name, 2},
  [70] =
    {field_name, 0},
    {field_val_arg, 2},
  [72] =
    {field_declaration_modifiers, 1},
    {field_io_port_modifiers, 0},
    {field_name, 3},
    {field_type, 2},
  [76] =
    {field_io_port_modifiers, 0},
    {field_latency_specifier, 3},
    {field_name, 2},
    {field_type, 1},
  [80] =
    {field_declaration_modifiers, 0},
    {field_latency_specifier, 3},
    {field_name, 2},
    {field_type, 1},
  [84] =
    {field_item, 2},
    {field_item, 3, .inherited = true},
  [86] =
    {field_outputs, 1, .inherited = true},
  [87] =
    {field_inputs, 1},
  [88] =
    {field_name, 0},
    {field_type_arg, 3},
  [90] =
    {field_declaration_modifiers, 1},
    {field_io_port_modifiers, 0},
    {field_latency_specifier, 4},
    {field_name, 3},
    {field_type, 2},
  [95] =
    {field_condition, 1},
    {field_else_block, 4},
    {field_statement_type, 0},
    {field_then_block, 2},
  [99] =
    {field_outputs, 1},
  [100] =
    {field_inputs, 1},
    {field_outputs, 2, .inherited = true},
  [102] =
    {field_outputs, 2, .inherited = true},
  [103] =
    {field_inputs, 2},
  [104] =
    {field_outputs, 2},
  [105] =
    {field_inputs, 2},
    {field_outputs, 3, .inherited = true},
  [107] =
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
  [98] = 25,
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
  [228] = 228,
  [229] = 229,
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
      if (lookahead == 'c') ADVANCE(2);
      if (lookahead == 'd') ADVANCE(3);
      if (lookahead == 'e') ADVANCE(4);
      if (lookahead == 'f') ADVANCE(5);
      if (lookahead == 'g') ADVANCE(6);
      if (lookahead == 'i') ADVANCE(7);
      if (lookahead == 'm') ADVANCE(8);
      if (lookahead == 'o') ADVANCE(9);
      if (lookahead == 'r') ADVANCE(10);
      if (lookahead == 's') ADVANCE(11);
      if (lookahead == 't') ADVANCE(12);
      if (lookahead == 'w') ADVANCE(13);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == '_') ADVANCE(14);
      END_STATE();
    case 2:
      if (lookahead == 'o') ADVANCE(15);
      END_STATE();
    case 3:
      if (lookahead == 'o') ADVANCE(16);
      END_STATE();
    case 4:
      if (lookahead == 'l') ADVANCE(17);
      if (lookahead == 'x') ADVANCE(18);
      END_STATE();
    case 5:
      if (lookahead == 'o') ADVANCE(19);
      END_STATE();
    case 6:
      if (lookahead == 'e') ADVANCE(20);
      END_STATE();
    case 7:
      if (lookahead == 'f') ADVANCE(21);
      if (lookahead == 'n') ADVANCE(22);
      END_STATE();
    case 8:
      if (lookahead == 'o') ADVANCE(23);
      END_STATE();
    case 9:
      if (lookahead == 'u') ADVANCE(24);
      END_STATE();
    case 10:
      if (lookahead == 'e') ADVANCE(25);
      END_STATE();
    case 11:
      if (lookahead == 't') ADVANCE(26);
      END_STATE();
    case 12:
      if (lookahead == 'y') ADVANCE(27);
      END_STATE();
    case 13:
      if (lookahead == 'h') ADVANCE(28);
      END_STATE();
    case 14:
      if (lookahead == 'b') ADVANCE(29);
      END_STATE();
    case 15:
      if (lookahead == 'n') ADVANCE(30);
      END_STATE();
    case 16:
      if (lookahead == 'm') ADVANCE(31);
      END_STATE();
    case 17:
      if (lookahead == 's') ADVANCE(32);
      END_STATE();
    case 18:
      if (lookahead == 't') ADVANCE(33);
      END_STATE();
    case 19:
      if (lookahead == 'r') ADVANCE(34);
      END_STATE();
    case 20:
      if (lookahead == 'n') ADVANCE(35);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_in);
      if (lookahead == 'i') ADVANCE(36);
      if (lookahead == 'p') ADVANCE(37);
      if (lookahead == 't') ADVANCE(38);
      END_STATE();
    case 23:
      if (lookahead == 'd') ADVANCE(39);
      END_STATE();
    case 24:
      if (lookahead == 't') ADVANCE(40);
      END_STATE();
    case 25:
      if (lookahead == 'g') ADVANCE(41);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(42);
      if (lookahead == 'r') ADVANCE(43);
      END_STATE();
    case 27:
      if (lookahead == 'p') ADVANCE(44);
      END_STATE();
    case 28:
      if (lookahead == 'e') ADVANCE(45);
      END_STATE();
    case 29:
      if (lookahead == 'u') ADVANCE(46);
      END_STATE();
    case 30:
      if (lookahead == 's') ADVANCE(47);
      END_STATE();
    case 31:
      if (lookahead == 'a') ADVANCE(48);
      END_STATE();
    case 32:
      if (lookahead == 'e') ADVANCE(49);
      END_STATE();
    case 33:
      if (lookahead == 'e') ADVANCE(50);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_for);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_gen);
      END_STATE();
    case 36:
      if (lookahead == 't') ADVANCE(51);
      END_STATE();
    case 37:
      if (lookahead == 'u') ADVANCE(52);
      END_STATE();
    case 38:
      if (lookahead == 'e') ADVANCE(53);
      END_STATE();
    case 39:
      if (lookahead == 'u') ADVANCE(54);
      END_STATE();
    case 40:
      if (lookahead == 'p') ADVANCE(55);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(anon_sym_reg);
      END_STATE();
    case 42:
      if (lookahead == 't') ADVANCE(56);
      END_STATE();
    case 43:
      if (lookahead == 'u') ADVANCE(57);
      END_STATE();
    case 44:
      if (lookahead == 'e') ADVANCE(58);
      END_STATE();
    case 45:
      if (lookahead == 'n') ADVANCE(59);
      END_STATE();
    case 46:
      if (lookahead == 'i') ADVANCE(60);
      END_STATE();
    case 47:
      if (lookahead == 't') ADVANCE(61);
      END_STATE();
    case 48:
      if (lookahead == 'i') ADVANCE(62);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 50:
      if (lookahead == 'r') ADVANCE(63);
      END_STATE();
    case 51:
      if (lookahead == 'i') ADVANCE(64);
      END_STATE();
    case 52:
      if (lookahead == 't') ADVANCE(65);
      END_STATE();
    case 53:
      if (lookahead == 'r') ADVANCE(66);
      END_STATE();
    case 54:
      if (lookahead == 'l') ADVANCE(67);
      END_STATE();
    case 55:
      if (lookahead == 'u') ADVANCE(68);
      END_STATE();
    case 56:
      if (lookahead == 'e') ADVANCE(69);
      END_STATE();
    case 57:
      if (lookahead == 'c') ADVANCE(70);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(anon_sym_type);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(anon_sym_when);
      END_STATE();
    case 60:
      if (lookahead == 'l') ADVANCE(71);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(anon_sym_const);
      END_STATE();
    case 62:
      if (lookahead == 'n') ADVANCE(72);
      END_STATE();
    case 63:
      if (lookahead == 'n') ADVANCE(73);
      END_STATE();
    case 64:
      if (lookahead == 'a') ADVANCE(74);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(anon_sym_input);
      END_STATE();
    case 66:
      if (lookahead == 'f') ADVANCE(75);
      END_STATE();
    case 67:
      if (lookahead == 'e') ADVANCE(76);
      END_STATE();
    case 68:
      if (lookahead == 't') ADVANCE(77);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(anon_sym_state);
      END_STATE();
    case 70:
      if (lookahead == 't') ADVANCE(78);
      END_STATE();
    case 71:
      if (lookahead == 't') ADVANCE(79);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(anon_sym_domain);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(anon_sym_extern);
      END_STATE();
    case 74:
      if (lookahead == 'l') ADVANCE(80);
      END_STATE();
    case 75:
      if (lookahead == 'a') ADVANCE(81);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(anon_sym_module);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(anon_sym_output);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(anon_sym_struct);
      END_STATE();
    case 79:
      if (lookahead == 'i') ADVANCE(82);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(anon_sym_initial);
      END_STATE();
    case 81:
      if (lookahead == 'c') ADVANCE(83);
      END_STATE();
    case 82:
      if (lookahead == 'n') ADVANCE(84);
      END_STATE();
    case 83:
      if (lookahead == 'e') ADVANCE(85);
      END_STATE();
    case 84:
      if (lookahead == '_') ADVANCE(86);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(anon_sym_interface);
      END_STATE();
    case 86:
      if (lookahead == '_') ADVANCE(87);
      END_STATE();
    case 87:
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
  [25] = {.lex_state = 1},
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
  [47] = {.lex_state = 2},
  [48] = {.lex_state = 2},
  [49] = {.lex_state = 2},
  [50] = {.lex_state = 2},
  [51] = {.lex_state = 2},
  [52] = {.lex_state = 2},
  [53] = {.lex_state = 2},
  [54] = {.lex_state = 1},
  [55] = {.lex_state = 1},
  [56] = {.lex_state = 2},
  [57] = {.lex_state = 2},
  [58] = {.lex_state = 2},
  [59] = {.lex_state = 1},
  [60] = {.lex_state = 1},
  [61] = {.lex_state = 1},
  [62] = {.lex_state = 1},
  [63] = {.lex_state = 1},
  [64] = {.lex_state = 1},
  [65] = {.lex_state = 1},
  [66] = {.lex_state = 1},
  [67] = {.lex_state = 1},
  [68] = {.lex_state = 1},
  [69] = {.lex_state = 1},
  [70] = {.lex_state = 1},
  [71] = {.lex_state = 1},
  [72] = {.lex_state = 1},
  [73] = {.lex_state = 2},
  [74] = {.lex_state = 2},
  [75] = {.lex_state = 1},
  [76] = {.lex_state = 2},
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
  [111] = {.lex_state = 0},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 1},
  [115] = {.lex_state = 0},
  [116] = {.lex_state = 0},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 0},
  [119] = {.lex_state = 0},
  [120] = {.lex_state = 0},
  [121] = {.lex_state = 0},
  [122] = {.lex_state = 0},
  [123] = {.lex_state = 1},
  [124] = {.lex_state = 1},
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
  [228] = {.lex_state = 0},
  [229] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym___builtin__] = ACTIONS(1),
    [anon_sym_extern] = ACTIONS(1),
    [anon_sym_module] = ACTIONS(1),
    [anon_sym_struct] = ACTIONS(1),
    [anon_sym_const] = ACTIONS(1),
    [anon_sym_POUND_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_reg] = ACTIONS(1),
    [anon_sym_initial] = ACTIONS(1),
    [anon_sym_when] = ACTIONS(1),
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
    [sym_global_object] = STATE(143),
    [sym_const_and_type] = STATE(223),
    [aux_sym__linebreak] = STATE(94),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym___builtin__] = ACTIONS(7),
    [anon_sym_extern] = ACTIONS(7),
    [anon_sym_module] = ACTIONS(9),
    [anon_sym_struct] = ACTIONS(9),
    [anon_sym_const] = ACTIONS(11),
    [anon_sym_LF] = ACTIONS(13),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 28,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LBRACE,
    ACTIONS(19), 1,
      anon_sym_RBRACE,
    ACTIONS(21), 1,
      anon_sym_reg,
    ACTIONS(23), 1,
      anon_sym_initial,
    ACTIONS(27), 1,
      anon_sym_for,
    ACTIONS(29), 1,
      anon_sym_domain,
    ACTIONS(31), 1,
      anon_sym_interface,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(43), 1,
      sym_number,
    ACTIONS(45), 1,
      anon_sym_LF,
    STATE(3), 1,
      aux_sym__linebreak,
    STATE(17), 1,
      sym_namespace_list,
    STATE(46), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(80), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(117), 1,
      sym_assign_to,
    STATE(127), 1,
      sym_assign_left_side,
    STATE(150), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(25), 2,
      anon_sym_when,
      anon_sym_if,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 2,
      sym__type,
      sym_array_type,
    STATE(144), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(37), 7,
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
  [107] = 28,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LBRACE,
    ACTIONS(21), 1,
      anon_sym_reg,
    ACTIONS(23), 1,
      anon_sym_initial,
    ACTIONS(27), 1,
      anon_sym_for,
    ACTIONS(29), 1,
      anon_sym_domain,
    ACTIONS(31), 1,
      anon_sym_interface,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(43), 1,
      sym_number,
    ACTIONS(47), 1,
      anon_sym_RBRACE,
    ACTIONS(49), 1,
      anon_sym_LF,
    STATE(17), 1,
      sym_namespace_list,
    STATE(25), 1,
      aux_sym__linebreak,
    STATE(46), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(80), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(117), 1,
      sym_assign_to,
    STATE(129), 1,
      sym_assign_left_side,
    STATE(150), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(25), 2,
      anon_sym_when,
      anon_sym_if,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 2,
      sym__type,
      sym_array_type,
    STATE(160), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(37), 7,
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
  [214] = 28,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LBRACE,
    ACTIONS(21), 1,
      anon_sym_reg,
    ACTIONS(23), 1,
      anon_sym_initial,
    ACTIONS(27), 1,
      anon_sym_for,
    ACTIONS(29), 1,
      anon_sym_domain,
    ACTIONS(31), 1,
      anon_sym_interface,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(43), 1,
      sym_number,
    ACTIONS(49), 1,
      anon_sym_LF,
    ACTIONS(51), 1,
      anon_sym_RBRACE,
    STATE(17), 1,
      sym_namespace_list,
    STATE(25), 1,
      aux_sym__linebreak,
    STATE(46), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(80), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(117), 1,
      sym_assign_to,
    STATE(150), 1,
      sym_declaration,
    STATE(191), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(25), 2,
      anon_sym_when,
      anon_sym_if,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 2,
      sym__type,
      sym_array_type,
    STATE(209), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(37), 7,
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
  [321] = 28,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LBRACE,
    ACTIONS(21), 1,
      anon_sym_reg,
    ACTIONS(23), 1,
      anon_sym_initial,
    ACTIONS(27), 1,
      anon_sym_for,
    ACTIONS(29), 1,
      anon_sym_domain,
    ACTIONS(31), 1,
      anon_sym_interface,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(43), 1,
      sym_number,
    ACTIONS(49), 1,
      anon_sym_LF,
    ACTIONS(53), 1,
      anon_sym_RBRACE,
    STATE(17), 1,
      sym_namespace_list,
    STATE(25), 1,
      aux_sym__linebreak,
    STATE(46), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(80), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(117), 1,
      sym_assign_to,
    STATE(150), 1,
      sym_declaration,
    STATE(191), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(25), 2,
      anon_sym_when,
      anon_sym_if,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 2,
      sym__type,
      sym_array_type,
    STATE(209), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(37), 7,
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
  [428] = 28,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LBRACE,
    ACTIONS(21), 1,
      anon_sym_reg,
    ACTIONS(23), 1,
      anon_sym_initial,
    ACTIONS(27), 1,
      anon_sym_for,
    ACTIONS(29), 1,
      anon_sym_domain,
    ACTIONS(31), 1,
      anon_sym_interface,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(43), 1,
      sym_number,
    ACTIONS(49), 1,
      anon_sym_LF,
    ACTIONS(55), 1,
      anon_sym_RBRACE,
    STATE(17), 1,
      sym_namespace_list,
    STATE(25), 1,
      aux_sym__linebreak,
    STATE(46), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(80), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(117), 1,
      sym_assign_to,
    STATE(150), 1,
      sym_declaration,
    STATE(191), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(25), 2,
      anon_sym_when,
      anon_sym_if,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 2,
      sym__type,
      sym_array_type,
    STATE(209), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(37), 7,
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
  [535] = 28,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LBRACE,
    ACTIONS(21), 1,
      anon_sym_reg,
    ACTIONS(23), 1,
      anon_sym_initial,
    ACTIONS(27), 1,
      anon_sym_for,
    ACTIONS(29), 1,
      anon_sym_domain,
    ACTIONS(31), 1,
      anon_sym_interface,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(43), 1,
      sym_number,
    ACTIONS(49), 1,
      anon_sym_LF,
    ACTIONS(57), 1,
      anon_sym_RBRACE,
    STATE(17), 1,
      sym_namespace_list,
    STATE(25), 1,
      aux_sym__linebreak,
    STATE(46), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(80), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(117), 1,
      sym_assign_to,
    STATE(150), 1,
      sym_declaration,
    STATE(191), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(25), 2,
      anon_sym_when,
      anon_sym_if,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 2,
      sym__type,
      sym_array_type,
    STATE(209), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(37), 7,
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
  [642] = 28,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LBRACE,
    ACTIONS(21), 1,
      anon_sym_reg,
    ACTIONS(23), 1,
      anon_sym_initial,
    ACTIONS(27), 1,
      anon_sym_for,
    ACTIONS(29), 1,
      anon_sym_domain,
    ACTIONS(31), 1,
      anon_sym_interface,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(43), 1,
      sym_number,
    ACTIONS(49), 1,
      anon_sym_LF,
    ACTIONS(59), 1,
      anon_sym_RBRACE,
    STATE(17), 1,
      sym_namespace_list,
    STATE(25), 1,
      aux_sym__linebreak,
    STATE(46), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(80), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(117), 1,
      sym_assign_to,
    STATE(150), 1,
      sym_declaration,
    STATE(191), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(25), 2,
      anon_sym_when,
      anon_sym_if,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 2,
      sym__type,
      sym_array_type,
    STATE(209), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(37), 7,
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
  [749] = 28,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LBRACE,
    ACTIONS(21), 1,
      anon_sym_reg,
    ACTIONS(23), 1,
      anon_sym_initial,
    ACTIONS(27), 1,
      anon_sym_for,
    ACTIONS(29), 1,
      anon_sym_domain,
    ACTIONS(31), 1,
      anon_sym_interface,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(43), 1,
      sym_number,
    ACTIONS(49), 1,
      anon_sym_LF,
    ACTIONS(61), 1,
      anon_sym_RBRACE,
    STATE(17), 1,
      sym_namespace_list,
    STATE(25), 1,
      aux_sym__linebreak,
    STATE(46), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(80), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(117), 1,
      sym_assign_to,
    STATE(150), 1,
      sym_declaration,
    STATE(191), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(25), 2,
      anon_sym_when,
      anon_sym_if,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 2,
      sym__type,
      sym_array_type,
    STATE(209), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(37), 7,
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
  [856] = 27,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LBRACE,
    ACTIONS(21), 1,
      anon_sym_reg,
    ACTIONS(23), 1,
      anon_sym_initial,
    ACTIONS(27), 1,
      anon_sym_for,
    ACTIONS(29), 1,
      anon_sym_domain,
    ACTIONS(31), 1,
      anon_sym_interface,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(43), 1,
      sym_number,
    ACTIONS(49), 1,
      anon_sym_LF,
    STATE(17), 1,
      sym_namespace_list,
    STATE(25), 1,
      aux_sym__linebreak,
    STATE(46), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(80), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(117), 1,
      sym_assign_to,
    STATE(150), 1,
      sym_declaration,
    STATE(191), 1,
      sym_assign_left_side,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(25), 2,
      anon_sym_when,
      anon_sym_if,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 2,
      sym__type,
      sym_array_type,
    STATE(209), 6,
      sym_block,
      sym_decl_assign_statement,
      sym_if_statement,
      sym_for_statement,
      sym_domain_statement,
      sym_interface_statement,
    ACTIONS(37), 7,
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
  [960] = 18,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(21), 1,
      anon_sym_reg,
    ACTIONS(23), 1,
      anon_sym_initial,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(43), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    STATE(46), 1,
      sym_write_modifiers,
    STATE(51), 1,
      sym_template_global,
    STATE(80), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(142), 1,
      sym_assign_to,
    STATE(150), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 2,
      sym__type,
      sym_array_type,
    ACTIONS(37), 7,
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
  [1031] = 5,
    ACTIONS(67), 1,
      anon_sym_COLON_COLON,
    STATE(13), 1,
      aux_sym_namespace_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(63), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(65), 21,
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
  [1075] = 5,
    ACTIONS(67), 1,
      anon_sym_COLON_COLON,
    STATE(14), 1,
      aux_sym_namespace_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(69), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(71), 21,
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
  [1119] = 5,
    ACTIONS(77), 1,
      anon_sym_COLON_COLON,
    STATE(14), 1,
      aux_sym_namespace_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(73), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(75), 21,
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
  [1163] = 5,
    ACTIONS(82), 1,
      anon_sym_POUND_LPAREN,
    STATE(29), 1,
      sym_template_args,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(80), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(84), 20,
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
  [1206] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(88), 22,
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
  [1245] = 5,
    ACTIONS(82), 1,
      anon_sym_POUND_LPAREN,
    STATE(28), 1,
      sym_template_args,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(90), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(92), 20,
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
  [1288] = 8,
    ACTIONS(98), 1,
      anon_sym_DOT,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(96), 5,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(94), 19,
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
  [1336] = 12,
    ACTIONS(98), 1,
      anon_sym_DOT,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_PLUS,
    ACTIONS(110), 1,
      anon_sym_DASH,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(106), 3,
      anon_sym_EQ,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(104), 16,
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
  [1392] = 13,
    ACTIONS(98), 1,
      anon_sym_DOT,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_PLUS,
    ACTIONS(110), 1,
      anon_sym_DASH,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    ACTIONS(116), 1,
      anon_sym_AMP,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(106), 3,
      anon_sym_EQ,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(104), 15,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_PIPE,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1450] = 8,
    ACTIONS(98), 1,
      anon_sym_DOT,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(106), 5,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(104), 19,
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
  [1498] = 10,
    ACTIONS(98), 1,
      anon_sym_DOT,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(106), 4,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(104), 17,
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
  [1550] = 15,
    ACTIONS(98), 1,
      anon_sym_DOT,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_PLUS,
    ACTIONS(110), 1,
      anon_sym_DASH,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_CARET,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(106), 3,
      anon_sym_EQ,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(104), 13,
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
  [1612] = 14,
    ACTIONS(98), 1,
      anon_sym_DOT,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_PLUS,
    ACTIONS(110), 1,
      anon_sym_DASH,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(106), 3,
      anon_sym_EQ,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(104), 14,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
      anon_sym_DASH_GT,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RBRACK,
      anon_sym_COMMA,
      anon_sym_LF,
  [1672] = 5,
    ACTIONS(126), 1,
      anon_sym_LF,
    STATE(25), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(122), 13,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_when,
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
    ACTIONS(124), 13,
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
  [1713] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(129), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(131), 20,
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
  [1750] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(133), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(135), 20,
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
  [1787] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(137), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(139), 20,
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
  [1824] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(141), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(143), 20,
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
  [1861] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(147), 20,
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
  [1898] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(149), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(151), 20,
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
  [1935] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(153), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(155), 20,
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
  [1972] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(157), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(159), 20,
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
  [2009] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(161), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(163), 20,
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
  [2046] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(165), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(167), 20,
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
  [2083] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(169), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(171), 20,
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
  [2120] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(173), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(175), 20,
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
  [2157] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(177), 8,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
      sym_identifier,
    ACTIONS(179), 20,
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
  [2194] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(183), 6,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(181), 21,
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
  [2230] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(187), 6,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(185), 21,
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
  [2266] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(191), 6,
      anon_sym_EQ,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_DOT,
    ACTIONS(189), 21,
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
  [2302] = 3,
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
  [2338] = 3,
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
  [2374] = 3,
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
  [2410] = 3,
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
  [2446] = 13,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(209), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    STATE(51), 1,
      sym_template_global,
    STATE(151), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 2,
      sym__type,
      sym_array_type,
    ACTIONS(37), 7,
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
  [2502] = 17,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      anon_sym_PLUS,
    ACTIONS(110), 1,
      anon_sym_DASH,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_CARET,
    ACTIONS(213), 1,
      anon_sym_EQ,
    ACTIONS(219), 1,
      anon_sym_DOT,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(217), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
    ACTIONS(211), 6,
      anon_sym_RPAREN,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [2565] = 18,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_CARET,
    ACTIONS(219), 1,
      anon_sym_DOT,
    ACTIONS(221), 1,
      anon_sym_RPAREN,
    ACTIONS(223), 1,
      anon_sym_COMMA,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    STATE(75), 1,
      sym__comma,
    STATE(166), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(108), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(217), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2627] = 16,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_CARET,
    ACTIONS(219), 1,
      anon_sym_DOT,
    ACTIONS(227), 1,
      anon_sym_EQ,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(108), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(217), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(225), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_LF,
    ACTIONS(215), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2685] = 16,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_CARET,
    ACTIONS(219), 1,
      anon_sym_DOT,
    ACTIONS(231), 1,
      anon_sym_EQ,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(108), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(217), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(229), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_LF,
    ACTIONS(215), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2743] = 5,
    ACTIONS(233), 1,
      sym_identifier,
    ACTIONS(239), 1,
      anon_sym_LBRACK,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(237), 4,
      anon_sym_EQ,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(235), 16,
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
  [2778] = 15,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_CARET,
    ACTIONS(219), 1,
      anon_sym_DOT,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(108), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(217), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(242), 3,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LF,
    ACTIONS(215), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2833] = 16,
    ACTIONS(17), 1,
      anon_sym_LBRACE,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_CARET,
    ACTIONS(219), 1,
      anon_sym_DOT,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    STATE(211), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(108), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(217), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2889] = 9,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(244), 1,
      anon_sym_type,
    ACTIONS(246), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
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
  [2931] = 9,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    ACTIONS(250), 1,
      anon_sym_RPAREN,
    ACTIONS(252), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
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
  [2973] = 16,
    ACTIONS(17), 1,
      anon_sym_LBRACE,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_CARET,
    ACTIONS(219), 1,
      anon_sym_DOT,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    STATE(175), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(108), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(217), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3029] = 15,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_CARET,
    ACTIONS(219), 1,
      anon_sym_DOT,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(108), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(217), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(254), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
    ACTIONS(215), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3083] = 15,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_CARET,
    ACTIONS(219), 1,
      anon_sym_DOT,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(108), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(217), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(256), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(215), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3137] = 8,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    ACTIONS(258), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
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
  [3176] = 8,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    ACTIONS(260), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
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
  [3215] = 8,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    ACTIONS(262), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
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
  [3254] = 8,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    ACTIONS(264), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
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
  [3293] = 8,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    ACTIONS(266), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
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
  [3332] = 8,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    ACTIONS(268), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
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
  [3371] = 8,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    ACTIONS(270), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
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
  [3410] = 8,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    ACTIONS(272), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
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
  [3449] = 8,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    ACTIONS(274), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
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
  [3488] = 8,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    ACTIONS(276), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(74), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3527] = 8,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    ACTIONS(278), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
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
  [3566] = 8,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    ACTIONS(280), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(76), 8,
      sym__expression,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_field_access,
      sym_parenthesis_expression,
      sym_template_global,
  [3605] = 8,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    ACTIONS(282), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
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
  [3644] = 8,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    ACTIONS(284), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
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
  [3683] = 15,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_CARET,
    ACTIONS(219), 1,
      anon_sym_DOT,
    ACTIONS(286), 1,
      anon_sym_RBRACK,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(108), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(217), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3736] = 15,
    ACTIONS(98), 1,
      anon_sym_DOT,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_CARET,
    ACTIONS(288), 1,
      anon_sym_DOT_DOT,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(108), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(217), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3789] = 8,
    ACTIONS(39), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    ACTIONS(290), 1,
      sym_number,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(37), 7,
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
  [3828] = 15,
    ACTIONS(100), 1,
      anon_sym_LPAREN,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(114), 1,
      anon_sym_SLASH,
    ACTIONS(116), 1,
      anon_sym_AMP,
    ACTIONS(118), 1,
      anon_sym_PIPE,
    ACTIONS(120), 1,
      anon_sym_CARET,
    ACTIONS(219), 1,
      anon_sym_DOT,
    ACTIONS(292), 1,
      anon_sym_RPAREN,
    STATE(39), 1,
      sym_parenthesis_expression_list,
    STATE(40), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(108), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(112), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(217), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3881] = 5,
    ACTIONS(49), 1,
      anon_sym_LF,
    STATE(25), 1,
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
  [3913] = 5,
    ACTIONS(302), 1,
      anon_sym_LF,
    STATE(77), 1,
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
  [3945] = 5,
    ACTIONS(306), 1,
      anon_sym_reg,
    STATE(79), 1,
      aux_sym_write_modifiers_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(304), 5,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(309), 10,
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
  [3975] = 5,
    ACTIONS(21), 1,
      anon_sym_reg,
    STATE(79), 1,
      aux_sym_write_modifiers_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(311), 5,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(313), 10,
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
  [4005] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(315), 6,
      anon_sym_reg,
      anon_sym_input,
      anon_sym_output,
      anon_sym_state,
      anon_sym_gen,
      sym_identifier,
    ACTIONS(317), 10,
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
  [4030] = 13,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(49), 1,
      anon_sym_LF,
    ACTIONS(319), 1,
      anon_sym_DASH_GT,
    STATE(17), 1,
      sym_namespace_list,
    STATE(25), 1,
      aux_sym__linebreak,
    STATE(113), 1,
      sym_declaration,
    STATE(170), 1,
      sym_declaration_list,
    STATE(204), 1,
      sym__interface_ports_output,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4075] = 13,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(319), 1,
      anon_sym_DASH_GT,
    ACTIONS(321), 1,
      anon_sym_LF,
    STATE(17), 1,
      sym_namespace_list,
    STATE(82), 1,
      aux_sym__linebreak,
    STATE(113), 1,
      sym_declaration,
    STATE(162), 1,
      sym_declaration_list,
    STATE(203), 1,
      sym__interface_ports_output,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4120] = 11,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(323), 1,
      sym_identifier,
    ACTIONS(325), 1,
      anon_sym_RPAREN,
    ACTIONS(327), 1,
      anon_sym_LF,
    STATE(17), 1,
      sym_namespace_list,
    STATE(86), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(125), 2,
      sym_template_declaration_type,
      sym_declaration,
    STATE(177), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4160] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(329), 5,
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
  [4184] = 11,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(49), 1,
      anon_sym_LF,
    ACTIONS(323), 1,
      sym_identifier,
    ACTIONS(333), 1,
      anon_sym_RPAREN,
    STATE(17), 1,
      sym_namespace_list,
    STATE(25), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(118), 2,
      sym_template_declaration_type,
      sym_declaration,
    STATE(177), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4224] = 11,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(335), 1,
      anon_sym_LF,
    STATE(17), 1,
      sym_namespace_list,
    STATE(88), 1,
      aux_sym__linebreak,
    STATE(113), 1,
      sym_declaration,
    STATE(201), 1,
      sym_declaration_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4263] = 11,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(49), 1,
      anon_sym_LF,
    STATE(17), 1,
      sym_namespace_list,
    STATE(25), 1,
      aux_sym__linebreak,
    STATE(113), 1,
      sym_declaration,
    STATE(208), 1,
      sym_declaration_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4302] = 8,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(323), 1,
      sym_identifier,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(184), 2,
      sym_template_declaration_type,
      sym_declaration,
    STATE(177), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4333] = 8,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    STATE(17), 1,
      sym_namespace_list,
    STATE(137), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4363] = 8,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    STATE(17), 1,
      sym_namespace_list,
    STATE(213), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(33), 2,
      anon_sym_input,
      anon_sym_output,
    ACTIONS(35), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(177), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4393] = 9,
    ACTIONS(11), 1,
      anon_sym_const,
    ACTIONS(337), 1,
      ts_builtin_sym_end,
    ACTIONS(339), 1,
      anon_sym_LF,
    STATE(98), 1,
      aux_sym__linebreak,
    STATE(207), 1,
      sym_global_object,
    STATE(223), 1,
      sym_const_and_type,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
    ACTIONS(9), 2,
      anon_sym_module,
      anon_sym_struct,
  [4424] = 9,
    ACTIONS(11), 1,
      anon_sym_const,
    ACTIONS(339), 1,
      anon_sym_LF,
    ACTIONS(341), 1,
      ts_builtin_sym_end,
    STATE(98), 1,
      aux_sym__linebreak,
    STATE(207), 1,
      sym_global_object,
    STATE(223), 1,
      sym_const_and_type,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
    ACTIONS(9), 2,
      anon_sym_module,
      anon_sym_struct,
  [4455] = 9,
    ACTIONS(11), 1,
      anon_sym_const,
    ACTIONS(339), 1,
      anon_sym_LF,
    ACTIONS(343), 1,
      ts_builtin_sym_end,
    STATE(98), 1,
      aux_sym__linebreak,
    STATE(149), 1,
      sym_global_object,
    STATE(223), 1,
      sym_const_and_type,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
    ACTIONS(9), 2,
      anon_sym_module,
      anon_sym_struct,
  [4486] = 9,
    ACTIONS(11), 1,
      anon_sym_const,
    ACTIONS(339), 1,
      anon_sym_LF,
    ACTIONS(345), 1,
      ts_builtin_sym_end,
    STATE(98), 1,
      aux_sym__linebreak,
    STATE(207), 1,
      sym_global_object,
    STATE(223), 1,
      sym_const_and_type,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
    ACTIONS(9), 2,
      anon_sym_module,
      anon_sym_struct,
  [4517] = 9,
    ACTIONS(11), 1,
      anon_sym_const,
    ACTIONS(339), 1,
      anon_sym_LF,
    ACTIONS(347), 1,
      ts_builtin_sym_end,
    STATE(98), 1,
      aux_sym__linebreak,
    STATE(207), 1,
      sym_global_object,
    STATE(223), 1,
      sym_const_and_type,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
    ACTIONS(9), 2,
      anon_sym_module,
      anon_sym_struct,
  [4548] = 8,
    ACTIONS(11), 1,
      anon_sym_const,
    ACTIONS(339), 1,
      anon_sym_LF,
    STATE(98), 1,
      aux_sym__linebreak,
    STATE(207), 1,
      sym_global_object,
    STATE(223), 1,
      sym_const_and_type,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(7), 2,
      anon_sym___builtin__,
      anon_sym_extern,
    ACTIONS(9), 2,
      anon_sym_module,
      anon_sym_struct,
  [4576] = 4,
    ACTIONS(349), 1,
      anon_sym_LF,
    STATE(98), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(124), 7,
      ts_builtin_sym_end,
      anon_sym___builtin__,
      anon_sym_extern,
      anon_sym_module,
      anon_sym_struct,
      anon_sym_const,
      anon_sym_RPAREN,
  [4596] = 4,
    ACTIONS(354), 1,
      anon_sym_SQUOTE,
    STATE(107), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(352), 7,
      anon_sym_RPAREN,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4616] = 4,
    ACTIONS(354), 1,
      anon_sym_SQUOTE,
    STATE(108), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(356), 7,
      anon_sym_RPAREN,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4636] = 4,
    ACTIONS(354), 1,
      anon_sym_SQUOTE,
    STATE(106), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(358), 7,
      anon_sym_RPAREN,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4656] = 4,
    ACTIONS(354), 1,
      anon_sym_SQUOTE,
    STATE(105), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(360), 7,
      anon_sym_RPAREN,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4676] = 5,
    ACTIONS(67), 1,
      anon_sym_COLON_COLON,
    STATE(13), 1,
      aux_sym_namespace_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(65), 3,
      anon_sym_POUND_LPAREN,
      anon_sym_LBRACK,
      sym_identifier,
    ACTIONS(362), 3,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LF,
  [4697] = 6,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(364), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(180), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4720] = 2,
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
  [4734] = 2,
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
  [4748] = 2,
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
  [4762] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(372), 7,
      anon_sym_RPAREN,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [4776] = 7,
    ACTIONS(223), 1,
      anon_sym_COMMA,
    ACTIONS(374), 1,
      anon_sym_RPAREN,
    ACTIONS(376), 1,
      anon_sym_LF,
    STATE(132), 1,
      aux_sym_template_args_repeat1,
    STATE(186), 1,
      aux_sym__linebreak,
    STATE(193), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4799] = 5,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(131), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4818] = 7,
    ACTIONS(223), 1,
      anon_sym_COMMA,
    ACTIONS(378), 1,
      anon_sym_RPAREN,
    ACTIONS(380), 1,
      anon_sym_LF,
    STATE(89), 1,
      sym__comma,
    STATE(130), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(185), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4841] = 5,
    ACTIONS(384), 1,
      anon_sym_COMMA,
    STATE(11), 1,
      sym__comma,
    STATE(112), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(382), 3,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_LF,
  [4860] = 5,
    ACTIONS(223), 1,
      anon_sym_COMMA,
    STATE(90), 1,
      sym__comma,
    STATE(121), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(387), 3,
      anon_sym_RBRACE,
      anon_sym_DASH_GT,
      anon_sym_LF,
  [4879] = 5,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(181), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [4898] = 7,
    ACTIONS(223), 1,
      anon_sym_COMMA,
    ACTIONS(389), 1,
      anon_sym_RPAREN,
    ACTIONS(391), 1,
      anon_sym_LF,
    STATE(109), 1,
      aux_sym_template_args_repeat1,
    STATE(178), 1,
      aux_sym__linebreak,
    STATE(193), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4921] = 7,
    ACTIONS(223), 1,
      anon_sym_COMMA,
    ACTIONS(393), 1,
      anon_sym_RPAREN,
    ACTIONS(395), 1,
      anon_sym_LF,
    STATE(132), 1,
      aux_sym_template_args_repeat1,
    STATE(183), 1,
      aux_sym__linebreak,
    STATE(193), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4944] = 5,
    ACTIONS(223), 1,
      anon_sym_COMMA,
    STATE(11), 1,
      sym__comma,
    STATE(120), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(397), 3,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_LF,
  [4963] = 7,
    ACTIONS(223), 1,
      anon_sym_COMMA,
    ACTIONS(399), 1,
      anon_sym_RPAREN,
    ACTIONS(401), 1,
      anon_sym_LF,
    STATE(89), 1,
      sym__comma,
    STATE(119), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(190), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4986] = 7,
    ACTIONS(223), 1,
      anon_sym_COMMA,
    ACTIONS(403), 1,
      anon_sym_RPAREN,
    ACTIONS(405), 1,
      anon_sym_LF,
    STATE(89), 1,
      sym__comma,
    STATE(130), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(176), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5009] = 5,
    ACTIONS(223), 1,
      anon_sym_COMMA,
    STATE(11), 1,
      sym__comma,
    STATE(112), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(407), 3,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_LF,
  [5028] = 5,
    ACTIONS(223), 1,
      anon_sym_COMMA,
    STATE(90), 1,
      sym__comma,
    STATE(126), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(409), 3,
      anon_sym_RBRACE,
      anon_sym_DASH_GT,
      anon_sym_LF,
  [5047] = 7,
    ACTIONS(223), 1,
      anon_sym_COMMA,
    ACTIONS(411), 1,
      anon_sym_RPAREN,
    ACTIONS(413), 1,
      anon_sym_LF,
    STATE(116), 1,
      aux_sym_template_args_repeat1,
    STATE(189), 1,
      aux_sym__linebreak,
    STATE(193), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5070] = 5,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(182), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [5089] = 5,
    ACTIONS(41), 1,
      anon_sym_COLON_COLON,
    ACTIONS(248), 1,
      sym_identifier,
    STATE(17), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(188), 3,
      sym__type,
      sym_array_type,
      sym_template_global,
  [5108] = 7,
    ACTIONS(223), 1,
      anon_sym_COMMA,
    ACTIONS(415), 1,
      anon_sym_RPAREN,
    ACTIONS(417), 1,
      anon_sym_LF,
    STATE(89), 1,
      sym__comma,
    STATE(111), 1,
      aux_sym_template_declaration_arguments_repeat1,
    STATE(187), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5131] = 5,
    ACTIONS(421), 1,
      anon_sym_COMMA,
    STATE(90), 1,
      sym__comma,
    STATE(126), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(419), 3,
      anon_sym_RBRACE,
      anon_sym_DASH_GT,
      anon_sym_LF,
  [5150] = 6,
    ACTIONS(424), 1,
      anon_sym_RBRACE,
    ACTIONS(426), 1,
      anon_sym_EQ,
    ACTIONS(428), 1,
      anon_sym_LF,
    STATE(4), 1,
      aux_sym__linebreak,
    STATE(148), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5170] = 6,
    ACTIONS(49), 1,
      anon_sym_LF,
    ACTIONS(430), 1,
      sym_identifier,
    ACTIONS(432), 1,
      anon_sym_RPAREN,
    STATE(25), 1,
      aux_sym__linebreak,
    STATE(122), 1,
      sym_template_arg,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5190] = 6,
    ACTIONS(426), 1,
      anon_sym_EQ,
    ACTIONS(434), 1,
      anon_sym_RBRACE,
    ACTIONS(436), 1,
      anon_sym_LF,
    STATE(7), 1,
      aux_sym__linebreak,
    STATE(155), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5210] = 5,
    ACTIONS(440), 1,
      anon_sym_COMMA,
    STATE(89), 1,
      sym__comma,
    STATE(130), 1,
      aux_sym_template_declaration_arguments_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(438), 2,
      anon_sym_RPAREN,
      anon_sym_LF,
  [5228] = 4,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    STATE(134), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(443), 3,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LF,
  [5244] = 5,
    ACTIONS(447), 1,
      anon_sym_COMMA,
    STATE(132), 1,
      aux_sym_template_args_repeat1,
    STATE(193), 1,
      sym__comma,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(445), 2,
      anon_sym_RPAREN,
      anon_sym_LF,
  [5262] = 6,
    ACTIONS(430), 1,
      sym_identifier,
    ACTIONS(450), 1,
      anon_sym_RPAREN,
    ACTIONS(452), 1,
      anon_sym_LF,
    STATE(115), 1,
      sym_template_arg,
    STATE(128), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5282] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(454), 5,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      sym_identifier,
      anon_sym_COMMA,
      anon_sym_LF,
  [5294] = 4,
    ACTIONS(17), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(456), 2,
      anon_sym_when,
      anon_sym_if,
    STATE(200), 2,
      sym_block,
      sym_if_statement,
  [5310] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(458), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5321] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(460), 4,
      anon_sym_RBRACE,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LF,
  [5332] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(462), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5343] = 5,
    ACTIONS(464), 1,
      anon_sym_RBRACE,
    ACTIONS(466), 1,
      anon_sym_LF,
    STATE(10), 1,
      aux_sym__linebreak,
    STATE(139), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5360] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(462), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5371] = 4,
    ACTIONS(471), 1,
      anon_sym_COLON,
    STATE(197), 1,
      sym_interface_ports,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(469), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5386] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(473), 4,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_COMMA,
      anon_sym_LF,
  [5397] = 5,
    ACTIONS(475), 1,
      ts_builtin_sym_end,
    ACTIONS(477), 1,
      anon_sym_LF,
    STATE(92), 1,
      aux_sym__linebreak,
    STATE(146), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5414] = 5,
    ACTIONS(424), 1,
      anon_sym_RBRACE,
    ACTIONS(428), 1,
      anon_sym_LF,
    STATE(4), 1,
      aux_sym__linebreak,
    STATE(147), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5431] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(479), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5442] = 5,
    ACTIONS(481), 1,
      ts_builtin_sym_end,
    ACTIONS(483), 1,
      anon_sym_LF,
    STATE(95), 1,
      aux_sym__linebreak,
    STATE(153), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5459] = 5,
    ACTIONS(485), 1,
      anon_sym_RBRACE,
    ACTIONS(487), 1,
      anon_sym_LF,
    STATE(5), 1,
      aux_sym__linebreak,
    STATE(139), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5476] = 5,
    ACTIONS(489), 1,
      anon_sym_RBRACE,
    ACTIONS(491), 1,
      anon_sym_LF,
    STATE(9), 1,
      aux_sym__linebreak,
    STATE(139), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5493] = 5,
    ACTIONS(493), 1,
      ts_builtin_sym_end,
    ACTIONS(495), 1,
      anon_sym_LF,
    STATE(96), 1,
      aux_sym__linebreak,
    STATE(156), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5510] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(229), 4,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_COMMA,
      anon_sym_LF,
  [5521] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(225), 4,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_COMMA,
      anon_sym_LF,
  [5532] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(497), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5543] = 5,
    ACTIONS(499), 1,
      ts_builtin_sym_end,
    ACTIONS(501), 1,
      anon_sym_LF,
    STATE(97), 1,
      aux_sym__linebreak,
    STATE(153), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5560] = 5,
    ACTIONS(504), 1,
      anon_sym_RBRACE,
    ACTIONS(506), 1,
      anon_sym_LF,
    STATE(6), 1,
      aux_sym__linebreak,
    STATE(139), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5577] = 5,
    ACTIONS(508), 1,
      anon_sym_RBRACE,
    ACTIONS(510), 1,
      anon_sym_LF,
    STATE(8), 1,
      aux_sym__linebreak,
    STATE(139), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5594] = 5,
    ACTIONS(512), 1,
      ts_builtin_sym_end,
    ACTIONS(514), 1,
      anon_sym_LF,
    STATE(93), 1,
      aux_sym__linebreak,
    STATE(153), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5611] = 5,
    ACTIONS(17), 1,
      anon_sym_LBRACE,
    ACTIONS(516), 1,
      anon_sym_POUND_LPAREN,
    STATE(195), 1,
      sym_template_declaration_arguments,
    STATE(205), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5628] = 5,
    ACTIONS(17), 1,
      anon_sym_LBRACE,
    ACTIONS(516), 1,
      anon_sym_POUND_LPAREN,
    STATE(199), 1,
      sym_template_declaration_arguments,
    STATE(206), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5645] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(518), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5656] = 5,
    ACTIONS(434), 1,
      anon_sym_RBRACE,
    ACTIONS(436), 1,
      anon_sym_LF,
    STATE(7), 1,
      aux_sym__linebreak,
    STATE(154), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5673] = 3,
    ACTIONS(522), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(520), 3,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LF,
  [5686] = 4,
    ACTIONS(319), 1,
      anon_sym_DASH_GT,
    STATE(202), 1,
      sym__interface_ports_output,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(524), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5701] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(526), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5712] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(528), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5723] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(526), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5734] = 5,
    ACTIONS(223), 1,
      anon_sym_COMMA,
    ACTIONS(530), 1,
      anon_sym_RPAREN,
    STATE(75), 1,
      sym__comma,
    STATE(171), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5751] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(532), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5762] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(532), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5773] = 4,
    ACTIONS(11), 1,
      anon_sym_const,
    STATE(224), 1,
      sym_const_and_type,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(534), 2,
      anon_sym_module,
      anon_sym_struct,
  [5788] = 4,
    ACTIONS(319), 1,
      anon_sym_DASH_GT,
    STATE(210), 1,
      sym__interface_ports_output,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(536), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5803] = 5,
    ACTIONS(538), 1,
      anon_sym_RPAREN,
    ACTIONS(540), 1,
      anon_sym_COMMA,
    STATE(75), 1,
      sym__comma,
    STATE(171), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5820] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(543), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5831] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(543), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5842] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(545), 4,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      anon_sym_else,
      anon_sym_LF,
  [5853] = 3,
    ACTIONS(549), 1,
      anon_sym_else,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(547), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [5865] = 4,
    ACTIONS(339), 1,
      anon_sym_LF,
    ACTIONS(551), 1,
      anon_sym_RPAREN,
    STATE(98), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5879] = 4,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(553), 1,
      sym_identifier,
    STATE(134), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5893] = 4,
    ACTIONS(339), 1,
      anon_sym_LF,
    ACTIONS(555), 1,
      anon_sym_RPAREN,
    STATE(98), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5907] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(557), 3,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LF,
  [5917] = 4,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(559), 1,
      sym_identifier,
    STATE(134), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5931] = 4,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(561), 1,
      sym_identifier,
    STATE(134), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5945] = 4,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(563), 1,
      sym_identifier,
    STATE(134), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5959] = 4,
    ACTIONS(339), 1,
      anon_sym_LF,
    ACTIONS(565), 1,
      anon_sym_RPAREN,
    STATE(98), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5973] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(567), 3,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LF,
  [5983] = 4,
    ACTIONS(339), 1,
      anon_sym_LF,
    ACTIONS(569), 1,
      anon_sym_RPAREN,
    STATE(98), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [5997] = 4,
    ACTIONS(339), 1,
      anon_sym_LF,
    ACTIONS(571), 1,
      anon_sym_RPAREN,
    STATE(98), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6011] = 4,
    ACTIONS(339), 1,
      anon_sym_LF,
    ACTIONS(573), 1,
      anon_sym_RPAREN,
    STATE(98), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6025] = 4,
    ACTIONS(102), 1,
      anon_sym_LBRACK,
    ACTIONS(575), 1,
      sym_identifier,
    STATE(134), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6039] = 4,
    ACTIONS(339), 1,
      anon_sym_LF,
    ACTIONS(577), 1,
      anon_sym_RPAREN,
    STATE(98), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6053] = 4,
    ACTIONS(339), 1,
      anon_sym_LF,
    ACTIONS(579), 1,
      anon_sym_RPAREN,
    STATE(98), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6067] = 3,
    ACTIONS(426), 1,
      anon_sym_EQ,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(581), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6079] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(583), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6088] = 3,
    ACTIONS(430), 1,
      sym_identifier,
    STATE(179), 1,
      sym_template_arg,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6099] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(585), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6108] = 3,
    ACTIONS(17), 1,
      anon_sym_LBRACE,
    STATE(194), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6119] = 3,
    ACTIONS(248), 1,
      sym_identifier,
    STATE(15), 1,
      sym_namespace_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6130] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(587), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6139] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(589), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6148] = 3,
    ACTIONS(17), 1,
      anon_sym_LBRACE,
    STATE(192), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6159] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(591), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6168] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(593), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6177] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(595), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6186] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(597), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6195] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(599), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6204] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(601), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6213] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(603), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6222] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(605), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [6231] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(607), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6240] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(581), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6249] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(609), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6258] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(611), 2,
      anon_sym_RBRACE,
      anon_sym_LF,
  [6267] = 2,
    ACTIONS(613), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6275] = 2,
    ACTIONS(615), 1,
      anon_sym_in,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6283] = 2,
    ACTIONS(617), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6291] = 2,
    ACTIONS(619), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6299] = 2,
    ACTIONS(621), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6307] = 2,
    ACTIONS(623), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6315] = 2,
    ACTIONS(625), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6323] = 2,
    ACTIONS(627), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6331] = 2,
    ACTIONS(629), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6339] = 2,
    ACTIONS(631), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6347] = 2,
    ACTIONS(633), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6355] = 2,
    ACTIONS(635), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6363] = 2,
    ACTIONS(637), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6371] = 2,
    ACTIONS(639), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6379] = 2,
    ACTIONS(641), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6387] = 2,
    ACTIONS(643), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6395] = 2,
    ACTIONS(645), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [6403] = 2,
    ACTIONS(647), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 107,
  [SMALL_STATE(4)] = 214,
  [SMALL_STATE(5)] = 321,
  [SMALL_STATE(6)] = 428,
  [SMALL_STATE(7)] = 535,
  [SMALL_STATE(8)] = 642,
  [SMALL_STATE(9)] = 749,
  [SMALL_STATE(10)] = 856,
  [SMALL_STATE(11)] = 960,
  [SMALL_STATE(12)] = 1031,
  [SMALL_STATE(13)] = 1075,
  [SMALL_STATE(14)] = 1119,
  [SMALL_STATE(15)] = 1163,
  [SMALL_STATE(16)] = 1206,
  [SMALL_STATE(17)] = 1245,
  [SMALL_STATE(18)] = 1288,
  [SMALL_STATE(19)] = 1336,
  [SMALL_STATE(20)] = 1392,
  [SMALL_STATE(21)] = 1450,
  [SMALL_STATE(22)] = 1498,
  [SMALL_STATE(23)] = 1550,
  [SMALL_STATE(24)] = 1612,
  [SMALL_STATE(25)] = 1672,
  [SMALL_STATE(26)] = 1713,
  [SMALL_STATE(27)] = 1750,
  [SMALL_STATE(28)] = 1787,
  [SMALL_STATE(29)] = 1824,
  [SMALL_STATE(30)] = 1861,
  [SMALL_STATE(31)] = 1898,
  [SMALL_STATE(32)] = 1935,
  [SMALL_STATE(33)] = 1972,
  [SMALL_STATE(34)] = 2009,
  [SMALL_STATE(35)] = 2046,
  [SMALL_STATE(36)] = 2083,
  [SMALL_STATE(37)] = 2120,
  [SMALL_STATE(38)] = 2157,
  [SMALL_STATE(39)] = 2194,
  [SMALL_STATE(40)] = 2230,
  [SMALL_STATE(41)] = 2266,
  [SMALL_STATE(42)] = 2302,
  [SMALL_STATE(43)] = 2338,
  [SMALL_STATE(44)] = 2374,
  [SMALL_STATE(45)] = 2410,
  [SMALL_STATE(46)] = 2446,
  [SMALL_STATE(47)] = 2502,
  [SMALL_STATE(48)] = 2565,
  [SMALL_STATE(49)] = 2627,
  [SMALL_STATE(50)] = 2685,
  [SMALL_STATE(51)] = 2743,
  [SMALL_STATE(52)] = 2778,
  [SMALL_STATE(53)] = 2833,
  [SMALL_STATE(54)] = 2889,
  [SMALL_STATE(55)] = 2931,
  [SMALL_STATE(56)] = 2973,
  [SMALL_STATE(57)] = 3029,
  [SMALL_STATE(58)] = 3083,
  [SMALL_STATE(59)] = 3137,
  [SMALL_STATE(60)] = 3176,
  [SMALL_STATE(61)] = 3215,
  [SMALL_STATE(62)] = 3254,
  [SMALL_STATE(63)] = 3293,
  [SMALL_STATE(64)] = 3332,
  [SMALL_STATE(65)] = 3371,
  [SMALL_STATE(66)] = 3410,
  [SMALL_STATE(67)] = 3449,
  [SMALL_STATE(68)] = 3488,
  [SMALL_STATE(69)] = 3527,
  [SMALL_STATE(70)] = 3566,
  [SMALL_STATE(71)] = 3605,
  [SMALL_STATE(72)] = 3644,
  [SMALL_STATE(73)] = 3683,
  [SMALL_STATE(74)] = 3736,
  [SMALL_STATE(75)] = 3789,
  [SMALL_STATE(76)] = 3828,
  [SMALL_STATE(77)] = 3881,
  [SMALL_STATE(78)] = 3913,
  [SMALL_STATE(79)] = 3945,
  [SMALL_STATE(80)] = 3975,
  [SMALL_STATE(81)] = 4005,
  [SMALL_STATE(82)] = 4030,
  [SMALL_STATE(83)] = 4075,
  [SMALL_STATE(84)] = 4120,
  [SMALL_STATE(85)] = 4160,
  [SMALL_STATE(86)] = 4184,
  [SMALL_STATE(87)] = 4224,
  [SMALL_STATE(88)] = 4263,
  [SMALL_STATE(89)] = 4302,
  [SMALL_STATE(90)] = 4333,
  [SMALL_STATE(91)] = 4363,
  [SMALL_STATE(92)] = 4393,
  [SMALL_STATE(93)] = 4424,
  [SMALL_STATE(94)] = 4455,
  [SMALL_STATE(95)] = 4486,
  [SMALL_STATE(96)] = 4517,
  [SMALL_STATE(97)] = 4548,
  [SMALL_STATE(98)] = 4576,
  [SMALL_STATE(99)] = 4596,
  [SMALL_STATE(100)] = 4616,
  [SMALL_STATE(101)] = 4636,
  [SMALL_STATE(102)] = 4656,
  [SMALL_STATE(103)] = 4676,
  [SMALL_STATE(104)] = 4697,
  [SMALL_STATE(105)] = 4720,
  [SMALL_STATE(106)] = 4734,
  [SMALL_STATE(107)] = 4748,
  [SMALL_STATE(108)] = 4762,
  [SMALL_STATE(109)] = 4776,
  [SMALL_STATE(110)] = 4799,
  [SMALL_STATE(111)] = 4818,
  [SMALL_STATE(112)] = 4841,
  [SMALL_STATE(113)] = 4860,
  [SMALL_STATE(114)] = 4879,
  [SMALL_STATE(115)] = 4898,
  [SMALL_STATE(116)] = 4921,
  [SMALL_STATE(117)] = 4944,
  [SMALL_STATE(118)] = 4963,
  [SMALL_STATE(119)] = 4986,
  [SMALL_STATE(120)] = 5009,
  [SMALL_STATE(121)] = 5028,
  [SMALL_STATE(122)] = 5047,
  [SMALL_STATE(123)] = 5070,
  [SMALL_STATE(124)] = 5089,
  [SMALL_STATE(125)] = 5108,
  [SMALL_STATE(126)] = 5131,
  [SMALL_STATE(127)] = 5150,
  [SMALL_STATE(128)] = 5170,
  [SMALL_STATE(129)] = 5190,
  [SMALL_STATE(130)] = 5210,
  [SMALL_STATE(131)] = 5228,
  [SMALL_STATE(132)] = 5244,
  [SMALL_STATE(133)] = 5262,
  [SMALL_STATE(134)] = 5282,
  [SMALL_STATE(135)] = 5294,
  [SMALL_STATE(136)] = 5310,
  [SMALL_STATE(137)] = 5321,
  [SMALL_STATE(138)] = 5332,
  [SMALL_STATE(139)] = 5343,
  [SMALL_STATE(140)] = 5360,
  [SMALL_STATE(141)] = 5371,
  [SMALL_STATE(142)] = 5386,
  [SMALL_STATE(143)] = 5397,
  [SMALL_STATE(144)] = 5414,
  [SMALL_STATE(145)] = 5431,
  [SMALL_STATE(146)] = 5442,
  [SMALL_STATE(147)] = 5459,
  [SMALL_STATE(148)] = 5476,
  [SMALL_STATE(149)] = 5493,
  [SMALL_STATE(150)] = 5510,
  [SMALL_STATE(151)] = 5521,
  [SMALL_STATE(152)] = 5532,
  [SMALL_STATE(153)] = 5543,
  [SMALL_STATE(154)] = 5560,
  [SMALL_STATE(155)] = 5577,
  [SMALL_STATE(156)] = 5594,
  [SMALL_STATE(157)] = 5611,
  [SMALL_STATE(158)] = 5628,
  [SMALL_STATE(159)] = 5645,
  [SMALL_STATE(160)] = 5656,
  [SMALL_STATE(161)] = 5673,
  [SMALL_STATE(162)] = 5686,
  [SMALL_STATE(163)] = 5701,
  [SMALL_STATE(164)] = 5712,
  [SMALL_STATE(165)] = 5723,
  [SMALL_STATE(166)] = 5734,
  [SMALL_STATE(167)] = 5751,
  [SMALL_STATE(168)] = 5762,
  [SMALL_STATE(169)] = 5773,
  [SMALL_STATE(170)] = 5788,
  [SMALL_STATE(171)] = 5803,
  [SMALL_STATE(172)] = 5820,
  [SMALL_STATE(173)] = 5831,
  [SMALL_STATE(174)] = 5842,
  [SMALL_STATE(175)] = 5853,
  [SMALL_STATE(176)] = 5865,
  [SMALL_STATE(177)] = 5879,
  [SMALL_STATE(178)] = 5893,
  [SMALL_STATE(179)] = 5907,
  [SMALL_STATE(180)] = 5917,
  [SMALL_STATE(181)] = 5931,
  [SMALL_STATE(182)] = 5945,
  [SMALL_STATE(183)] = 5959,
  [SMALL_STATE(184)] = 5973,
  [SMALL_STATE(185)] = 5983,
  [SMALL_STATE(186)] = 5997,
  [SMALL_STATE(187)] = 6011,
  [SMALL_STATE(188)] = 6025,
  [SMALL_STATE(189)] = 6039,
  [SMALL_STATE(190)] = 6053,
  [SMALL_STATE(191)] = 6067,
  [SMALL_STATE(192)] = 6079,
  [SMALL_STATE(193)] = 6088,
  [SMALL_STATE(194)] = 6099,
  [SMALL_STATE(195)] = 6108,
  [SMALL_STATE(196)] = 6119,
  [SMALL_STATE(197)] = 6130,
  [SMALL_STATE(198)] = 6139,
  [SMALL_STATE(199)] = 6148,
  [SMALL_STATE(200)] = 6159,
  [SMALL_STATE(201)] = 6168,
  [SMALL_STATE(202)] = 6177,
  [SMALL_STATE(203)] = 6186,
  [SMALL_STATE(204)] = 6195,
  [SMALL_STATE(205)] = 6204,
  [SMALL_STATE(206)] = 6213,
  [SMALL_STATE(207)] = 6222,
  [SMALL_STATE(208)] = 6231,
  [SMALL_STATE(209)] = 6240,
  [SMALL_STATE(210)] = 6249,
  [SMALL_STATE(211)] = 6258,
  [SMALL_STATE(212)] = 6267,
  [SMALL_STATE(213)] = 6275,
  [SMALL_STATE(214)] = 6283,
  [SMALL_STATE(215)] = 6291,
  [SMALL_STATE(216)] = 6299,
  [SMALL_STATE(217)] = 6307,
  [SMALL_STATE(218)] = 6315,
  [SMALL_STATE(219)] = 6323,
  [SMALL_STATE(220)] = 6331,
  [SMALL_STATE(221)] = 6339,
  [SMALL_STATE(222)] = 6347,
  [SMALL_STATE(223)] = 6355,
  [SMALL_STATE(224)] = 6363,
  [SMALL_STATE(225)] = 6371,
  [SMALL_STATE(226)] = 6379,
  [SMALL_STATE(227)] = 6387,
  [SMALL_STATE(228)] = 6395,
  [SMALL_STATE(229)] = 6403,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(169),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(223),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(164),
  [21] = {.entry = {.count = 1, .reusable = false}}, SHIFT(81),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(85),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(66),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(91),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(226),
  [31] = {.entry = {.count = 1, .reusable = false}}, SHIFT(215),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(104),
  [35] = {.entry = {.count = 1, .reusable = false}}, SHIFT(123),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(196),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(159),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(174),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(163),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(172),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(173),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(165),
  [63] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_namespace_list, 1, .production_id = 1),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_namespace_list, 1, .production_id = 1),
  [67] = {.entry = {.count = 1, .reusable = true}}, SHIFT(229),
  [69] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_namespace_list, 2, .production_id = 4),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_namespace_list, 2, .production_id = 4),
  [73] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_namespace_list_repeat1, 2, .production_id = 10),
  [75] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_namespace_list_repeat1, 2, .production_id = 10),
  [77] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_namespace_list_repeat1, 2, .production_id = 10), SHIFT_REPEAT(229),
  [80] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 2, .production_id = 7),
  [82] = {.entry = {.count = 1, .reusable = true}}, SHIFT(133),
  [84] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 2, .production_id = 7),
  [86] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_namespace_list_repeat1, 2, .production_id = 5),
  [88] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_namespace_list_repeat1, 2, .production_id = 5),
  [90] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 1, .production_id = 3),
  [92] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 1, .production_id = 3),
  [94] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_op, 2, .production_id = 21),
  [96] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_op, 2, .production_id = 21),
  [98] = {.entry = {.count = 1, .reusable = false}}, SHIFT(221),
  [100] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [102] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [104] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_op, 3, .production_id = 32),
  [106] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_op, 3, .production_id = 32),
  [108] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [110] = {.entry = {.count = 1, .reusable = false}}, SHIFT(65),
  [112] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [114] = {.entry = {.count = 1, .reusable = false}}, SHIFT(60),
  [116] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [118] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [120] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [122] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__linebreak, 2),
  [124] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__linebreak, 2),
  [126] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__linebreak, 2), SHIFT_REPEAT(25),
  [129] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 4, .production_id = 5),
  [131] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 4, .production_id = 5),
  [133] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 2),
  [135] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 2),
  [137] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 2, .production_id = 9),
  [139] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 2, .production_id = 9),
  [141] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_global, 3, .production_id = 17),
  [143] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_global, 3, .production_id = 17),
  [145] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 4, .production_id = 11),
  [147] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 4, .production_id = 11),
  [149] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 4, .production_id = 28),
  [151] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 4, .production_id = 28),
  [153] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 5, .production_id = 28),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 5, .production_id = 28),
  [157] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 24),
  [159] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 24),
  [161] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 3),
  [163] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 3),
  [165] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 3, .production_id = 5),
  [167] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 3, .production_id = 5),
  [169] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 5, .production_id = 38),
  [171] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 5, .production_id = 38),
  [173] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 5, .production_id = 11),
  [175] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 5, .production_id = 11),
  [177] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_args, 6, .production_id = 38),
  [179] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_args, 6, .production_id = 38),
  [181] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 2, .production_id = 23),
  [183] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 2, .production_id = 23),
  [185] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_op, 2, .production_id = 8),
  [187] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_op, 2, .production_id = 8),
  [189] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 2),
  [191] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 2),
  [193] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_access, 3, .production_id = 33),
  [195] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_access, 3, .production_id = 33),
  [197] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 3, .production_id = 5),
  [199] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 3, .production_id = 5),
  [201] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 4, .production_id = 11),
  [203] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 4, .production_id = 11),
  [205] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 24),
  [207] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 24),
  [209] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [211] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_latency_specifier, 2, .production_id = 24),
  [213] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_latency_specifier, 2, .production_id = 24),
  [215] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [217] = {.entry = {.count = 1, .reusable = false}}, SHIFT(63),
  [219] = {.entry = {.count = 1, .reusable = true}}, SHIFT(221),
  [221] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [223] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [225] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_to, 2, .production_id = 22),
  [227] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_to, 2, .production_id = 22),
  [229] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_to, 1, .production_id = 14),
  [231] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_to, 1, .production_id = 14),
  [233] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__type, 1),
  [235] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expression, 1),
  [237] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expression, 1),
  [239] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym__type, 1), REDUCE(sym__expression, 1),
  [242] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_arg, 3, .production_id = 34),
  [244] = {.entry = {.count = 1, .reusable = false}}, SHIFT(110),
  [246] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [248] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [250] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [252] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [254] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_decl_assign_statement, 3, .production_id = 31),
  [256] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 5),
  [258] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [260] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [262] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [264] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [266] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [268] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [270] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [272] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [274] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [276] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [278] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [280] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [282] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [284] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [286] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [288] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [290] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [292] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [294] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__comma, 2),
  [296] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__comma, 2),
  [298] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__comma, 1),
  [300] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__comma, 1),
  [302] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [304] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 10),
  [306] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 10), SHIFT_REPEAT(81),
  [309] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 10),
  [311] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_write_modifiers, 1, .production_id = 15),
  [313] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_write_modifiers, 1, .production_id = 15),
  [315] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 1, .production_id = 1),
  [317] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_write_modifiers_repeat1, 1, .production_id = 1),
  [319] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [321] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [323] = {.entry = {.count = 1, .reusable = false}}, SHIFT(103),
  [325] = {.entry = {.count = 1, .reusable = true}}, SHIFT(219),
  [327] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [329] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_write_modifiers, 1, .production_id = 1),
  [331] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_write_modifiers, 1, .production_id = 1),
  [333] = {.entry = {.count = 1, .reusable = true}}, SHIFT(214),
  [335] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [337] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 1),
  [339] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [341] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 4, .production_id = 11),
  [343] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [345] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 4),
  [347] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 5),
  [349] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__linebreak, 2), SHIFT_REPEAT(98),
  [352] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 26),
  [354] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [356] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 35),
  [358] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 2, .production_id = 19),
  [360] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 25),
  [362] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_type, 1, .production_id = 13),
  [364] = {.entry = {.count = 1, .reusable = false}}, SHIFT(114),
  [366] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 36),
  [368] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 27),
  [370] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 37),
  [372] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 5, .production_id = 42),
  [374] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [376] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [378] = {.entry = {.count = 1, .reusable = true}}, SHIFT(227),
  [380] = {.entry = {.count = 1, .reusable = true}}, SHIFT(185),
  [382] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 10),
  [384] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 10), SHIFT_REPEAT(78),
  [387] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration_list, 1, .production_id = 1),
  [389] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [391] = {.entry = {.count = 1, .reusable = true}}, SHIFT(178),
  [393] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [395] = {.entry = {.count = 1, .reusable = true}}, SHIFT(183),
  [397] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 1, .production_id = 1),
  [399] = {.entry = {.count = 1, .reusable = true}}, SHIFT(222),
  [401] = {.entry = {.count = 1, .reusable = true}}, SHIFT(190),
  [403] = {.entry = {.count = 1, .reusable = true}}, SHIFT(216),
  [405] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [407] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 2, .production_id = 4),
  [409] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration_list, 2, .production_id = 4),
  [411] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [413] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [415] = {.entry = {.count = 1, .reusable = true}}, SHIFT(228),
  [417] = {.entry = {.count = 1, .reusable = true}}, SHIFT(187),
  [419] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 10),
  [421] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 10), SHIFT_REPEAT(78),
  [424] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [426] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [428] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [430] = {.entry = {.count = 1, .reusable = true}}, SHIFT(161),
  [432] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [434] = {.entry = {.count = 1, .reusable = true}}, SHIFT(152),
  [436] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [438] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_declaration_arguments_repeat1, 2, .production_id = 10),
  [440] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_declaration_arguments_repeat1, 2, .production_id = 10), SHIFT_REPEAT(78),
  [443] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_arg, 4, .production_id = 41),
  [445] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_args_repeat1, 2, .production_id = 10),
  [447] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_args_repeat1, 2, .production_id = 10), SHIFT_REPEAT(78),
  [450] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [452] = {.entry = {.count = 1, .reusable = true}}, SHIFT(128),
  [454] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type, 2, .production_id = 8),
  [456] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [458] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 28),
  [460] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 5),
  [462] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 11),
  [464] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 10),
  [466] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 10), SHIFT_REPEAT(10),
  [469] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_statement, 2, .production_id = 20),
  [471] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [473] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 5),
  [475] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, .production_id = 1),
  [477] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [479] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, .production_id = 5),
  [481] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 4),
  [483] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [485] = {.entry = {.count = 1, .reusable = true}}, SHIFT(138),
  [487] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [489] = {.entry = {.count = 1, .reusable = true}}, SHIFT(140),
  [491] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [493] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 5),
  [495] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [497] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 28),
  [499] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 10),
  [501] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 10), SHIFT_REPEAT(97),
  [504] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [506] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [508] = {.entry = {.count = 1, .reusable = true}}, SHIFT(168),
  [510] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [512] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 11),
  [514] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [516] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [518] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [520] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_arg, 1, .production_id = 13),
  [522] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [524] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2, .production_id = 40),
  [526] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 11),
  [528] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [530] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [532] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 38),
  [534] = {.entry = {.count = 1, .reusable = true}}, SHIFT(224),
  [536] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 47),
  [538] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 10),
  [540] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 10), SHIFT_REPEAT(78),
  [543] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 6, .production_id = 38),
  [545] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 5),
  [547] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 3, .production_id = 29),
  [549] = {.entry = {.count = 1, .reusable = true}}, SHIFT(135),
  [551] = {.entry = {.count = 1, .reusable = true}}, SHIFT(218),
  [553] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [555] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [557] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_args_repeat1, 2, .production_id = 5),
  [559] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [561] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [563] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [565] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [567] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_declaration_arguments_repeat1, 2, .production_id = 5),
  [569] = {.entry = {.count = 1, .reusable = true}}, SHIFT(217),
  [571] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [573] = {.entry = {.count = 1, .reusable = true}}, SHIFT(212),
  [575] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_const_and_type, 2, .production_id = 2),
  [577] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [579] = {.entry = {.count = 1, .reusable = true}}, SHIFT(220),
  [581] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 5),
  [583] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_object, 4, .production_id = 16),
  [585] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_object, 5, .production_id = 18),
  [587] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_statement, 3, .production_id = 30),
  [589] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_domain_statement, 2, .production_id = 20),
  [591] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 5, .production_id = 43),
  [593] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__interface_ports_output, 2, .production_id = 44),
  [595] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 45),
  [597] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2, .production_id = 39),
  [599] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 46),
  [601] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_object, 4, .production_id = 12),
  [603] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_object, 3, .production_id = 6),
  [605] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 5),
  [607] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__interface_ports_output, 3, .production_id = 48),
  [609] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 4, .production_id = 49),
  [611] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_statement, 7, .production_id = 50),
  [613] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 4, .production_id = 5),
  [615] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [617] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 3),
  [619] = {.entry = {.count = 1, .reusable = true}}, SHIFT(141),
  [621] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 5, .production_id = 38),
  [623] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 5, .production_id = 11),
  [625] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 6, .production_id = 38),
  [627] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 2),
  [629] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 5, .production_id = 28),
  [631] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [633] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 4, .production_id = 28),
  [635] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [637] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [639] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [641] = {.entry = {.count = 1, .reusable = true}}, SHIFT(198),
  [643] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 4, .production_id = 11),
  [645] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_declaration_arguments, 3, .production_id = 5),
  [647] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
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
