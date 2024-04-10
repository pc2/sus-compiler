#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 156
#define LARGE_STATE_COUNT 11
#define SYMBOL_COUNT 76
#define ALIAS_COUNT 0
#define TOKEN_COUNT 43
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 26
#define MAX_ALIAS_SEQUENCE_LENGTH 7
#define PRODUCTION_ID_COUNT 35

enum ts_symbol_identifiers {
  sym_identifier = 1,
  anon_sym_COMMA = 2,
  anon_sym_COLON = 3,
  anon_sym_DASH_GT = 4,
  anon_sym_module = 5,
  sym_number = 6,
  anon_sym_COLON_COLON = 7,
  anon_sym_SQUOTE = 8,
  anon_sym_state = 9,
  anon_sym_gen = 10,
  anon_sym_PLUS = 11,
  anon_sym_DASH = 12,
  anon_sym_STAR = 13,
  anon_sym_BANG = 14,
  anon_sym_PIPE = 15,
  anon_sym_AMP = 16,
  anon_sym_CARET = 17,
  anon_sym_EQ_EQ = 18,
  anon_sym_BANG_EQ = 19,
  anon_sym_LT = 20,
  anon_sym_LT_EQ = 21,
  anon_sym_GT = 22,
  anon_sym_GT_EQ = 23,
  anon_sym_SLASH = 24,
  anon_sym_PERCENT = 25,
  anon_sym_LPAREN = 26,
  anon_sym_RPAREN = 27,
  anon_sym_LBRACK = 28,
  anon_sym_RBRACK = 29,
  aux_sym__linebreak_token1 = 30,
  anon_sym_LBRACE = 31,
  anon_sym_RBRACE = 32,
  anon_sym_reg = 33,
  anon_sym_initial = 34,
  anon_sym_EQ = 35,
  anon_sym_if = 36,
  anon_sym_else = 37,
  anon_sym_for = 38,
  anon_sym_in = 39,
  anon_sym_DOT_DOT = 40,
  sym_single_line_comment = 41,
  sym_multi_line_comment = 42,
  sym_source_file = 43,
  sym__comma = 44,
  sym_interface_ports = 45,
  sym_declaration_list = 46,
  sym_module = 47,
  sym_global_identifier = 48,
  sym_array_type = 49,
  sym__type = 50,
  sym_latency_specifier = 51,
  sym_declaration = 52,
  sym_unary_op = 53,
  sym_binary_op = 54,
  sym_array_op = 55,
  sym_func_call = 56,
  sym_parenthesis_expression_list = 57,
  sym_parenthesis_expression = 58,
  sym_array_bracket_expression = 59,
  sym__expression = 60,
  aux_sym__linebreak = 61,
  sym_block = 62,
  sym_write_modifiers = 63,
  sym_assign_to = 64,
  sym_assign_left_side = 65,
  sym_decl_assign_statement = 66,
  sym_if_statement = 67,
  sym_for_statement = 68,
  aux_sym_source_file_repeat1 = 69,
  aux_sym_declaration_list_repeat1 = 70,
  aux_sym_global_identifier_repeat1 = 71,
  aux_sym_parenthesis_expression_list_repeat1 = 72,
  aux_sym_block_repeat1 = 73,
  aux_sym_write_modifiers_repeat1 = 74,
  aux_sym_assign_left_side_repeat1 = 75,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [anon_sym_COMMA] = ",",
  [anon_sym_COLON] = ":",
  [anon_sym_DASH_GT] = "->",
  [anon_sym_module] = "module",
  [sym_number] = "number",
  [anon_sym_COLON_COLON] = "::",
  [anon_sym_SQUOTE] = "'",
  [anon_sym_state] = "state",
  [anon_sym_gen] = "gen",
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
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [aux_sym__linebreak_token1] = "_linebreak_token1",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_reg] = "reg",
  [anon_sym_initial] = "initial",
  [anon_sym_EQ] = "=",
  [anon_sym_if] = "if",
  [anon_sym_else] = "else",
  [anon_sym_for] = "for",
  [anon_sym_in] = "in",
  [anon_sym_DOT_DOT] = "..",
  [sym_single_line_comment] = "single_line_comment",
  [sym_multi_line_comment] = "multi_line_comment",
  [sym_source_file] = "source_file",
  [sym__comma] = "_comma",
  [sym_interface_ports] = "interface_ports",
  [sym_declaration_list] = "declaration_list",
  [sym_module] = "module",
  [sym_global_identifier] = "global_identifier",
  [sym_array_type] = "array_type",
  [sym__type] = "_type",
  [sym_latency_specifier] = "latency_specifier",
  [sym_declaration] = "declaration",
  [sym_unary_op] = "unary_op",
  [sym_binary_op] = "binary_op",
  [sym_array_op] = "array_op",
  [sym_func_call] = "func_call",
  [sym_parenthesis_expression_list] = "parenthesis_expression_list",
  [sym_parenthesis_expression] = "parenthesis_expression",
  [sym_array_bracket_expression] = "array_bracket_expression",
  [sym__expression] = "_expression",
  [aux_sym__linebreak] = "_linebreak",
  [sym_block] = "block",
  [sym_write_modifiers] = "write_modifiers",
  [sym_assign_to] = "assign_to",
  [sym_assign_left_side] = "assign_left_side",
  [sym_decl_assign_statement] = "decl_assign_statement",
  [sym_if_statement] = "if_statement",
  [sym_for_statement] = "for_statement",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_declaration_list_repeat1] = "declaration_list_repeat1",
  [aux_sym_global_identifier_repeat1] = "global_identifier_repeat1",
  [aux_sym_parenthesis_expression_list_repeat1] = "parenthesis_expression_list_repeat1",
  [aux_sym_block_repeat1] = "block_repeat1",
  [aux_sym_write_modifiers_repeat1] = "write_modifiers_repeat1",
  [aux_sym_assign_left_side_repeat1] = "assign_left_side_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_identifier] = sym_identifier,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_DASH_GT] = anon_sym_DASH_GT,
  [anon_sym_module] = anon_sym_module,
  [sym_number] = sym_number,
  [anon_sym_COLON_COLON] = anon_sym_COLON_COLON,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [anon_sym_state] = anon_sym_state,
  [anon_sym_gen] = anon_sym_gen,
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
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [aux_sym__linebreak_token1] = aux_sym__linebreak_token1,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_reg] = anon_sym_reg,
  [anon_sym_initial] = anon_sym_initial,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_if] = anon_sym_if,
  [anon_sym_else] = anon_sym_else,
  [anon_sym_for] = anon_sym_for,
  [anon_sym_in] = anon_sym_in,
  [anon_sym_DOT_DOT] = anon_sym_DOT_DOT,
  [sym_single_line_comment] = sym_single_line_comment,
  [sym_multi_line_comment] = sym_multi_line_comment,
  [sym_source_file] = sym_source_file,
  [sym__comma] = sym__comma,
  [sym_interface_ports] = sym_interface_ports,
  [sym_declaration_list] = sym_declaration_list,
  [sym_module] = sym_module,
  [sym_global_identifier] = sym_global_identifier,
  [sym_array_type] = sym_array_type,
  [sym__type] = sym__type,
  [sym_latency_specifier] = sym_latency_specifier,
  [sym_declaration] = sym_declaration,
  [sym_unary_op] = sym_unary_op,
  [sym_binary_op] = sym_binary_op,
  [sym_array_op] = sym_array_op,
  [sym_func_call] = sym_func_call,
  [sym_parenthesis_expression_list] = sym_parenthesis_expression_list,
  [sym_parenthesis_expression] = sym_parenthesis_expression,
  [sym_array_bracket_expression] = sym_array_bracket_expression,
  [sym__expression] = sym__expression,
  [aux_sym__linebreak] = aux_sym__linebreak,
  [sym_block] = sym_block,
  [sym_write_modifiers] = sym_write_modifiers,
  [sym_assign_to] = sym_assign_to,
  [sym_assign_left_side] = sym_assign_left_side,
  [sym_decl_assign_statement] = sym_decl_assign_statement,
  [sym_if_statement] = sym_if_statement,
  [sym_for_statement] = sym_for_statement,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_declaration_list_repeat1] = aux_sym_declaration_list_repeat1,
  [aux_sym_global_identifier_repeat1] = aux_sym_global_identifier_repeat1,
  [aux_sym_parenthesis_expression_list_repeat1] = aux_sym_parenthesis_expression_list_repeat1,
  [aux_sym_block_repeat1] = aux_sym_block_repeat1,
  [aux_sym_write_modifiers_repeat1] = aux_sym_write_modifiers_repeat1,
  [aux_sym_assign_left_side_repeat1] = aux_sym_assign_left_side_repeat1,
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
  [anon_sym_COMMA] = {
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
  [anon_sym_module] = {
    .visible = true,
    .named = false,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_COLON_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SQUOTE] = {
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
  [aux_sym__linebreak_token1] = {
    .visible = false,
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
  [anon_sym_EQ] = {
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
  [sym__comma] = {
    .visible = false,
    .named = true,
  },
  [sym_interface_ports] = {
    .visible = true,
    .named = true,
  },
  [sym_declaration_list] = {
    .visible = true,
    .named = true,
  },
  [sym_module] = {
    .visible = true,
    .named = true,
  },
  [sym_global_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_array_type] = {
    .visible = true,
    .named = true,
  },
  [sym__type] = {
    .visible = false,
    .named = true,
  },
  [sym_latency_specifier] = {
    .visible = true,
    .named = true,
  },
  [sym_declaration] = {
    .visible = true,
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
  [sym__expression] = {
    .visible = false,
    .named = true,
  },
  [aux_sym__linebreak] = {
    .visible = false,
    .named = false,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym_write_modifiers] = {
    .visible = true,
    .named = true,
  },
  [sym_assign_to] = {
    .visible = true,
    .named = true,
  },
  [sym_assign_left_side] = {
    .visible = true,
    .named = true,
  },
  [sym_decl_assign_statement] = {
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
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_declaration_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_global_identifier_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_parenthesis_expression_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_block_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_write_modifiers_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_assign_left_side_repeat1] = {
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
  field_for_decl = 12,
  field_from = 13,
  field_inputs = 14,
  field_interface_ports = 15,
  field_item = 16,
  field_latency_specifier = 17,
  field_left = 18,
  field_name = 19,
  field_operator = 20,
  field_outputs = 21,
  field_right = 22,
  field_then_block = 23,
  field_to = 24,
  field_type = 25,
  field_write_modifiers = 26,
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
  [field_for_decl] = "for_decl",
  [field_from] = "from",
  [field_inputs] = "inputs",
  [field_interface_ports] = "interface_ports",
  [field_item] = "item",
  [field_latency_specifier] = "latency_specifier",
  [field_left] = "left",
  [field_name] = "name",
  [field_operator] = "operator",
  [field_outputs] = "outputs",
  [field_right] = "right",
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
  [10] = {.index = 13, .length = 3},
  [11] = {.index = 16, .length = 1},
  [12] = {.index = 17, .length = 2},
  [13] = {.index = 19, .length = 2},
  [14] = {.index = 21, .length = 1},
  [15] = {.index = 22, .length = 2},
  [16] = {.index = 24, .length = 2},
  [17] = {.index = 26, .length = 2},
  [18] = {.index = 28, .length = 1},
  [19] = {.index = 29, .length = 3},
  [20] = {.index = 32, .length = 2},
  [21] = {.index = 34, .length = 3},
  [22] = {.index = 37, .length = 1},
  [23] = {.index = 38, .length = 2},
  [24] = {.index = 40, .length = 3},
  [25] = {.index = 43, .length = 1},
  [26] = {.index = 44, .length = 2},
  [27] = {.index = 46, .length = 4},
  [28] = {.index = 50, .length = 2},
  [29] = {.index = 52, .length = 1},
  [30] = {.index = 53, .length = 2},
  [31] = {.index = 55, .length = 2},
  [32] = {.index = 57, .length = 2},
  [33] = {.index = 59, .length = 3},
  [34] = {.index = 62, .length = 4},
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
    {field_inputs, 1},
  [11] =
    {field_expr_or_decl, 0},
  [12] =
    {field_item, 0, .inherited = true},
  [13] =
    {field_block, 3},
    {field_interface_ports, 2},
    {field_name, 1},
  [16] =
    {field_outputs, 2},
  [17] =
    {field_name, 1},
    {field_type, 0},
  [19] =
    {field_arr, 0},
    {field_arr_idx, 1},
  [21] =
    {field_inputs, 2},
  [22] =
    {field_operator, 0},
    {field_right, 1},
  [24] =
    {field_arguments, 1},
    {field_name, 0},
  [26] =
    {field_expr_or_decl, 1},
    {field_write_modifiers, 0},
  [28] =
    {field_outputs, 3},
  [29] =
    {field_declaration_modifiers, 0},
    {field_name, 2},
    {field_type, 1},
  [32] =
    {field_inputs, 1},
    {field_outputs, 3},
  [34] =
    {field_latency_specifier, 2},
    {field_name, 1},
    {field_type, 0},
  [37] =
    {field_content, 1},
  [38] =
    {field_condition, 1},
    {field_then_block, 2},
  [40] =
    {field_left, 0},
    {field_operator, 1},
    {field_right, 2},
  [43] =
    {field_item, 2},
  [44] =
    {field_assign_left, 0},
    {field_assign_value, 2},
  [46] =
    {field_declaration_modifiers, 0},
    {field_latency_specifier, 3},
    {field_name, 2},
    {field_type, 1},
  [50] =
    {field_inputs, 1},
    {field_outputs, 4},
  [52] =
    {field_outputs, 4},
  [53] =
    {field_inputs, 2},
    {field_outputs, 4},
  [55] =
    {field_item, 2},
    {field_item, 3, .inherited = true},
  [57] =
    {field_inputs, 2},
    {field_outputs, 5},
  [59] =
    {field_condition, 1},
    {field_else_block, 4},
    {field_then_block, 2},
  [62] =
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
      if (lookahead == '\n') ADVANCE(38);
      if (lookahead == '!') ADVANCE(22);
      if (lookahead == '%') ADVANCE(33);
      if (lookahead == '&') ADVANCE(24);
      if (lookahead == '\'') ADVANCE(17);
      if (lookahead == '(') ADVANCE(34);
      if (lookahead == ')') ADVANCE(35);
      if (lookahead == '*') ADVANCE(20);
      if (lookahead == '+') ADVANCE(18);
      if (lookahead == ',') ADVANCE(10);
      if (lookahead == '-') ADVANCE(19);
      if (lookahead == '.') ADVANCE(5);
      if (lookahead == '/') ADVANCE(32);
      if (lookahead == ':') ADVANCE(12);
      if (lookahead == '<') ADVANCE(28);
      if (lookahead == '=') ADVANCE(41);
      if (lookahead == '>') ADVANCE(30);
      if (lookahead == '[') ADVANCE(36);
      if (lookahead == ']') ADVANCE(37);
      if (lookahead == '^') ADVANCE(25);
      if (lookahead == '{') ADVANCE(39);
      if (lookahead == '|') ADVANCE(23);
      if (lookahead == '}') ADVANCE(40);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(15);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(14);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(38);
      if (lookahead == '!') ADVANCE(7);
      if (lookahead == '%') ADVANCE(33);
      if (lookahead == '&') ADVANCE(24);
      if (lookahead == '(') ADVANCE(34);
      if (lookahead == ')') ADVANCE(35);
      if (lookahead == '*') ADVANCE(20);
      if (lookahead == '+') ADVANCE(18);
      if (lookahead == ',') ADVANCE(10);
      if (lookahead == '-') ADVANCE(19);
      if (lookahead == '.') ADVANCE(5);
      if (lookahead == '/') ADVANCE(32);
      if (lookahead == ':') ADVANCE(6);
      if (lookahead == '<') ADVANCE(28);
      if (lookahead == '=') ADVANCE(41);
      if (lookahead == '>') ADVANCE(30);
      if (lookahead == '[') ADVANCE(36);
      if (lookahead == ']') ADVANCE(37);
      if (lookahead == '^') ADVANCE(25);
      if (lookahead == '{') ADVANCE(39);
      if (lookahead == '|') ADVANCE(23);
      if (lookahead == '}') ADVANCE(40);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(1)
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(14);
      END_STATE();
    case 2:
      if (lookahead == '*') ADVANCE(4);
      if (lookahead == '/') ADVANCE(43);
      END_STATE();
    case 3:
      if (lookahead == '*') ADVANCE(3);
      if (lookahead == '/') ADVANCE(44);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 4:
      if (lookahead == '*') ADVANCE(3);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 5:
      if (lookahead == '.') ADVANCE(42);
      END_STATE();
    case 6:
      if (lookahead == ':') ADVANCE(16);
      END_STATE();
    case 7:
      if (lookahead == '=') ADVANCE(27);
      END_STATE();
    case 8:
      if (eof) ADVANCE(9);
      if (lookahead == '\n') ADVANCE(38);
      if (lookahead == '!') ADVANCE(21);
      if (lookahead == '&') ADVANCE(24);
      if (lookahead == '(') ADVANCE(34);
      if (lookahead == ')') ADVANCE(35);
      if (lookahead == '*') ADVANCE(20);
      if (lookahead == '+') ADVANCE(18);
      if (lookahead == '-') ADVANCE(19);
      if (lookahead == '/') ADVANCE(2);
      if (lookahead == ':') ADVANCE(11);
      if (lookahead == '^') ADVANCE(25);
      if (lookahead == '{') ADVANCE(39);
      if (lookahead == '|') ADVANCE(23);
      if (lookahead == '}') ADVANCE(40);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(8)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(15);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(14);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_COLON);
      if (lookahead == ':') ADVANCE(16);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(14);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(15);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_COLON_COLON);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '>') ADVANCE(13);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_BANG);
      if (lookahead == '=') ADVANCE(27);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_AMP);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(29);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(31);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(anon_sym_GT_EQ);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '*') ADVANCE(4);
      if (lookahead == '/') ADVANCE(43);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(aux_sym__linebreak_token1);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead == '=') ADVANCE(26);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(anon_sym_DOT_DOT);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(sym_single_line_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(43);
      END_STATE();
    case 44:
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
      if (lookahead == 'r') ADVANCE(6);
      if (lookahead == 's') ADVANCE(7);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == 'l') ADVANCE(8);
      END_STATE();
    case 2:
      if (lookahead == 'o') ADVANCE(9);
      END_STATE();
    case 3:
      if (lookahead == 'e') ADVANCE(10);
      END_STATE();
    case 4:
      if (lookahead == 'f') ADVANCE(11);
      if (lookahead == 'n') ADVANCE(12);
      END_STATE();
    case 5:
      if (lookahead == 'o') ADVANCE(13);
      END_STATE();
    case 6:
      if (lookahead == 'e') ADVANCE(14);
      END_STATE();
    case 7:
      if (lookahead == 't') ADVANCE(15);
      END_STATE();
    case 8:
      if (lookahead == 's') ADVANCE(16);
      END_STATE();
    case 9:
      if (lookahead == 'r') ADVANCE(17);
      END_STATE();
    case 10:
      if (lookahead == 'n') ADVANCE(18);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_in);
      if (lookahead == 'i') ADVANCE(19);
      END_STATE();
    case 13:
      if (lookahead == 'd') ADVANCE(20);
      END_STATE();
    case 14:
      if (lookahead == 'g') ADVANCE(21);
      END_STATE();
    case 15:
      if (lookahead == 'a') ADVANCE(22);
      END_STATE();
    case 16:
      if (lookahead == 'e') ADVANCE(23);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_for);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_gen);
      END_STATE();
    case 19:
      if (lookahead == 't') ADVANCE(24);
      END_STATE();
    case 20:
      if (lookahead == 'u') ADVANCE(25);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_reg);
      END_STATE();
    case 22:
      if (lookahead == 't') ADVANCE(26);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 24:
      if (lookahead == 'i') ADVANCE(27);
      END_STATE();
    case 25:
      if (lookahead == 'l') ADVANCE(28);
      END_STATE();
    case 26:
      if (lookahead == 'e') ADVANCE(29);
      END_STATE();
    case 27:
      if (lookahead == 'a') ADVANCE(30);
      END_STATE();
    case 28:
      if (lookahead == 'e') ADVANCE(31);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(anon_sym_state);
      END_STATE();
    case 30:
      if (lookahead == 'l') ADVANCE(32);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(anon_sym_module);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_initial);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 8},
  [3] = {.lex_state = 8},
  [4] = {.lex_state = 8},
  [5] = {.lex_state = 8},
  [6] = {.lex_state = 8},
  [7] = {.lex_state = 8},
  [8] = {.lex_state = 8},
  [9] = {.lex_state = 8},
  [10] = {.lex_state = 8},
  [11] = {.lex_state = 1},
  [12] = {.lex_state = 1},
  [13] = {.lex_state = 1},
  [14] = {.lex_state = 1},
  [15] = {.lex_state = 1},
  [16] = {.lex_state = 1},
  [17] = {.lex_state = 1},
  [18] = {.lex_state = 1},
  [19] = {.lex_state = 1},
  [20] = {.lex_state = 1},
  [21] = {.lex_state = 1},
  [22] = {.lex_state = 8},
  [23] = {.lex_state = 1},
  [24] = {.lex_state = 1},
  [25] = {.lex_state = 1},
  [26] = {.lex_state = 1},
  [27] = {.lex_state = 1},
  [28] = {.lex_state = 1},
  [29] = {.lex_state = 1},
  [30] = {.lex_state = 1},
  [31] = {.lex_state = 8},
  [32] = {.lex_state = 1},
  [33] = {.lex_state = 1},
  [34] = {.lex_state = 1},
  [35] = {.lex_state = 8},
  [36] = {.lex_state = 1},
  [37] = {.lex_state = 1},
  [38] = {.lex_state = 1},
  [39] = {.lex_state = 1},
  [40] = {.lex_state = 1},
  [41] = {.lex_state = 1},
  [42] = {.lex_state = 1},
  [43] = {.lex_state = 1},
  [44] = {.lex_state = 8},
  [45] = {.lex_state = 8},
  [46] = {.lex_state = 8},
  [47] = {.lex_state = 8},
  [48] = {.lex_state = 8},
  [49] = {.lex_state = 8},
  [50] = {.lex_state = 8},
  [51] = {.lex_state = 8},
  [52] = {.lex_state = 8},
  [53] = {.lex_state = 8},
  [54] = {.lex_state = 8},
  [55] = {.lex_state = 8},
  [56] = {.lex_state = 8},
  [57] = {.lex_state = 8},
  [58] = {.lex_state = 8},
  [59] = {.lex_state = 8},
  [60] = {.lex_state = 8},
  [61] = {.lex_state = 8},
  [62] = {.lex_state = 8},
  [63] = {.lex_state = 8},
  [64] = {.lex_state = 8},
  [65] = {.lex_state = 0},
  [66] = {.lex_state = 0},
  [67] = {.lex_state = 8},
  [68] = {.lex_state = 0},
  [69] = {.lex_state = 0},
  [70] = {.lex_state = 0},
  [71] = {.lex_state = 0},
  [72] = {.lex_state = 0},
  [73] = {.lex_state = 0},
  [74] = {.lex_state = 0},
  [75] = {.lex_state = 0},
  [76] = {.lex_state = 0},
  [77] = {.lex_state = 0},
  [78] = {.lex_state = 0},
  [79] = {.lex_state = 0},
  [80] = {.lex_state = 0},
  [81] = {.lex_state = 0},
  [82] = {.lex_state = 0},
  [83] = {.lex_state = 0},
  [84] = {.lex_state = 0},
  [85] = {.lex_state = 0},
  [86] = {.lex_state = 0},
  [87] = {.lex_state = 0},
  [88] = {.lex_state = 0},
  [89] = {.lex_state = 0},
  [90] = {.lex_state = 0},
  [91] = {.lex_state = 0},
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
  [103] = {.lex_state = 0},
  [104] = {.lex_state = 0},
  [105] = {.lex_state = 0},
  [106] = {.lex_state = 0},
  [107] = {.lex_state = 8},
  [108] = {.lex_state = 0},
  [109] = {.lex_state = 0},
  [110] = {.lex_state = 0},
  [111] = {.lex_state = 0},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 0},
  [115] = {.lex_state = 0},
  [116] = {.lex_state = 0},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 0},
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
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_DASH_GT] = ACTIONS(1),
    [anon_sym_module] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
    [anon_sym_COLON_COLON] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [anon_sym_state] = ACTIONS(1),
    [anon_sym_gen] = ACTIONS(1),
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
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [aux_sym__linebreak_token1] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_reg] = ACTIONS(1),
    [anon_sym_initial] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [anon_sym_for] = ACTIONS(1),
    [anon_sym_in] = ACTIONS(1),
    [anon_sym_DOT_DOT] = ACTIONS(1),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(155),
    [sym_module] = STATE(106),
    [aux_sym__linebreak] = STATE(89),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_module] = ACTIONS(7),
    [aux_sym__linebreak_token1] = ACTIONS(9),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [2] = {
    [sym_global_identifier] = STATE(36),
    [sym_array_type] = STATE(130),
    [sym__type] = STATE(130),
    [sym_declaration] = STATE(125),
    [sym_unary_op] = STATE(33),
    [sym_binary_op] = STATE(33),
    [sym_array_op] = STATE(33),
    [sym_func_call] = STATE(33),
    [sym_parenthesis_expression] = STATE(33),
    [sym__expression] = STATE(33),
    [aux_sym__linebreak] = STATE(31),
    [sym_block] = STATE(140),
    [sym_write_modifiers] = STATE(35),
    [sym_assign_to] = STATE(82),
    [sym_assign_left_side] = STATE(131),
    [sym_decl_assign_statement] = STATE(140),
    [sym_if_statement] = STATE(140),
    [sym_for_statement] = STATE(140),
    [aux_sym_write_modifiers_repeat1] = STATE(63),
    [sym_identifier] = ACTIONS(11),
    [sym_number] = ACTIONS(13),
    [anon_sym_state] = ACTIONS(15),
    [anon_sym_gen] = ACTIONS(15),
    [anon_sym_PLUS] = ACTIONS(17),
    [anon_sym_DASH] = ACTIONS(17),
    [anon_sym_STAR] = ACTIONS(17),
    [anon_sym_BANG] = ACTIONS(17),
    [anon_sym_PIPE] = ACTIONS(17),
    [anon_sym_AMP] = ACTIONS(17),
    [anon_sym_CARET] = ACTIONS(17),
    [anon_sym_LPAREN] = ACTIONS(19),
    [aux_sym__linebreak_token1] = ACTIONS(21),
    [anon_sym_LBRACE] = ACTIONS(23),
    [anon_sym_RBRACE] = ACTIONS(25),
    [anon_sym_reg] = ACTIONS(27),
    [anon_sym_initial] = ACTIONS(29),
    [anon_sym_if] = ACTIONS(31),
    [anon_sym_for] = ACTIONS(33),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [3] = {
    [sym_global_identifier] = STATE(36),
    [sym_array_type] = STATE(130),
    [sym__type] = STATE(130),
    [sym_declaration] = STATE(125),
    [sym_unary_op] = STATE(33),
    [sym_binary_op] = STATE(33),
    [sym_array_op] = STATE(33),
    [sym_func_call] = STATE(33),
    [sym_parenthesis_expression] = STATE(33),
    [sym__expression] = STATE(33),
    [aux_sym__linebreak] = STATE(31),
    [sym_block] = STATE(140),
    [sym_write_modifiers] = STATE(35),
    [sym_assign_to] = STATE(82),
    [sym_assign_left_side] = STATE(131),
    [sym_decl_assign_statement] = STATE(140),
    [sym_if_statement] = STATE(140),
    [sym_for_statement] = STATE(140),
    [aux_sym_write_modifiers_repeat1] = STATE(63),
    [sym_identifier] = ACTIONS(11),
    [sym_number] = ACTIONS(13),
    [anon_sym_state] = ACTIONS(15),
    [anon_sym_gen] = ACTIONS(15),
    [anon_sym_PLUS] = ACTIONS(17),
    [anon_sym_DASH] = ACTIONS(17),
    [anon_sym_STAR] = ACTIONS(17),
    [anon_sym_BANG] = ACTIONS(17),
    [anon_sym_PIPE] = ACTIONS(17),
    [anon_sym_AMP] = ACTIONS(17),
    [anon_sym_CARET] = ACTIONS(17),
    [anon_sym_LPAREN] = ACTIONS(19),
    [aux_sym__linebreak_token1] = ACTIONS(21),
    [anon_sym_LBRACE] = ACTIONS(23),
    [anon_sym_RBRACE] = ACTIONS(35),
    [anon_sym_reg] = ACTIONS(27),
    [anon_sym_initial] = ACTIONS(29),
    [anon_sym_if] = ACTIONS(31),
    [anon_sym_for] = ACTIONS(33),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [4] = {
    [sym_global_identifier] = STATE(36),
    [sym_array_type] = STATE(130),
    [sym__type] = STATE(130),
    [sym_declaration] = STATE(125),
    [sym_unary_op] = STATE(33),
    [sym_binary_op] = STATE(33),
    [sym_array_op] = STATE(33),
    [sym_func_call] = STATE(33),
    [sym_parenthesis_expression] = STATE(33),
    [sym__expression] = STATE(33),
    [aux_sym__linebreak] = STATE(31),
    [sym_block] = STATE(140),
    [sym_write_modifiers] = STATE(35),
    [sym_assign_to] = STATE(82),
    [sym_assign_left_side] = STATE(131),
    [sym_decl_assign_statement] = STATE(140),
    [sym_if_statement] = STATE(140),
    [sym_for_statement] = STATE(140),
    [aux_sym_write_modifiers_repeat1] = STATE(63),
    [sym_identifier] = ACTIONS(11),
    [sym_number] = ACTIONS(13),
    [anon_sym_state] = ACTIONS(15),
    [anon_sym_gen] = ACTIONS(15),
    [anon_sym_PLUS] = ACTIONS(17),
    [anon_sym_DASH] = ACTIONS(17),
    [anon_sym_STAR] = ACTIONS(17),
    [anon_sym_BANG] = ACTIONS(17),
    [anon_sym_PIPE] = ACTIONS(17),
    [anon_sym_AMP] = ACTIONS(17),
    [anon_sym_CARET] = ACTIONS(17),
    [anon_sym_LPAREN] = ACTIONS(19),
    [aux_sym__linebreak_token1] = ACTIONS(21),
    [anon_sym_LBRACE] = ACTIONS(23),
    [anon_sym_RBRACE] = ACTIONS(37),
    [anon_sym_reg] = ACTIONS(27),
    [anon_sym_initial] = ACTIONS(29),
    [anon_sym_if] = ACTIONS(31),
    [anon_sym_for] = ACTIONS(33),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [5] = {
    [sym_global_identifier] = STATE(36),
    [sym_array_type] = STATE(130),
    [sym__type] = STATE(130),
    [sym_declaration] = STATE(125),
    [sym_unary_op] = STATE(33),
    [sym_binary_op] = STATE(33),
    [sym_array_op] = STATE(33),
    [sym_func_call] = STATE(33),
    [sym_parenthesis_expression] = STATE(33),
    [sym__expression] = STATE(33),
    [aux_sym__linebreak] = STATE(31),
    [sym_block] = STATE(140),
    [sym_write_modifiers] = STATE(35),
    [sym_assign_to] = STATE(82),
    [sym_assign_left_side] = STATE(131),
    [sym_decl_assign_statement] = STATE(140),
    [sym_if_statement] = STATE(140),
    [sym_for_statement] = STATE(140),
    [aux_sym_write_modifiers_repeat1] = STATE(63),
    [sym_identifier] = ACTIONS(11),
    [sym_number] = ACTIONS(13),
    [anon_sym_state] = ACTIONS(15),
    [anon_sym_gen] = ACTIONS(15),
    [anon_sym_PLUS] = ACTIONS(17),
    [anon_sym_DASH] = ACTIONS(17),
    [anon_sym_STAR] = ACTIONS(17),
    [anon_sym_BANG] = ACTIONS(17),
    [anon_sym_PIPE] = ACTIONS(17),
    [anon_sym_AMP] = ACTIONS(17),
    [anon_sym_CARET] = ACTIONS(17),
    [anon_sym_LPAREN] = ACTIONS(19),
    [aux_sym__linebreak_token1] = ACTIONS(21),
    [anon_sym_LBRACE] = ACTIONS(23),
    [anon_sym_RBRACE] = ACTIONS(39),
    [anon_sym_reg] = ACTIONS(27),
    [anon_sym_initial] = ACTIONS(29),
    [anon_sym_if] = ACTIONS(31),
    [anon_sym_for] = ACTIONS(33),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [6] = {
    [sym_global_identifier] = STATE(36),
    [sym_array_type] = STATE(130),
    [sym__type] = STATE(130),
    [sym_declaration] = STATE(125),
    [sym_unary_op] = STATE(33),
    [sym_binary_op] = STATE(33),
    [sym_array_op] = STATE(33),
    [sym_func_call] = STATE(33),
    [sym_parenthesis_expression] = STATE(33),
    [sym__expression] = STATE(33),
    [aux_sym__linebreak] = STATE(31),
    [sym_block] = STATE(140),
    [sym_write_modifiers] = STATE(35),
    [sym_assign_to] = STATE(82),
    [sym_assign_left_side] = STATE(131),
    [sym_decl_assign_statement] = STATE(140),
    [sym_if_statement] = STATE(140),
    [sym_for_statement] = STATE(140),
    [aux_sym_write_modifiers_repeat1] = STATE(63),
    [sym_identifier] = ACTIONS(11),
    [sym_number] = ACTIONS(13),
    [anon_sym_state] = ACTIONS(15),
    [anon_sym_gen] = ACTIONS(15),
    [anon_sym_PLUS] = ACTIONS(17),
    [anon_sym_DASH] = ACTIONS(17),
    [anon_sym_STAR] = ACTIONS(17),
    [anon_sym_BANG] = ACTIONS(17),
    [anon_sym_PIPE] = ACTIONS(17),
    [anon_sym_AMP] = ACTIONS(17),
    [anon_sym_CARET] = ACTIONS(17),
    [anon_sym_LPAREN] = ACTIONS(19),
    [aux_sym__linebreak_token1] = ACTIONS(21),
    [anon_sym_LBRACE] = ACTIONS(23),
    [anon_sym_RBRACE] = ACTIONS(41),
    [anon_sym_reg] = ACTIONS(27),
    [anon_sym_initial] = ACTIONS(29),
    [anon_sym_if] = ACTIONS(31),
    [anon_sym_for] = ACTIONS(33),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [7] = {
    [sym_global_identifier] = STATE(36),
    [sym_array_type] = STATE(130),
    [sym__type] = STATE(130),
    [sym_declaration] = STATE(125),
    [sym_unary_op] = STATE(33),
    [sym_binary_op] = STATE(33),
    [sym_array_op] = STATE(33),
    [sym_func_call] = STATE(33),
    [sym_parenthesis_expression] = STATE(33),
    [sym__expression] = STATE(33),
    [aux_sym__linebreak] = STATE(31),
    [sym_block] = STATE(104),
    [sym_write_modifiers] = STATE(35),
    [sym_assign_to] = STATE(82),
    [sym_assign_left_side] = STATE(91),
    [sym_decl_assign_statement] = STATE(104),
    [sym_if_statement] = STATE(104),
    [sym_for_statement] = STATE(104),
    [aux_sym_write_modifiers_repeat1] = STATE(63),
    [sym_identifier] = ACTIONS(11),
    [sym_number] = ACTIONS(13),
    [anon_sym_state] = ACTIONS(15),
    [anon_sym_gen] = ACTIONS(15),
    [anon_sym_PLUS] = ACTIONS(17),
    [anon_sym_DASH] = ACTIONS(17),
    [anon_sym_STAR] = ACTIONS(17),
    [anon_sym_BANG] = ACTIONS(17),
    [anon_sym_PIPE] = ACTIONS(17),
    [anon_sym_AMP] = ACTIONS(17),
    [anon_sym_CARET] = ACTIONS(17),
    [anon_sym_LPAREN] = ACTIONS(19),
    [aux_sym__linebreak_token1] = ACTIONS(21),
    [anon_sym_LBRACE] = ACTIONS(23),
    [anon_sym_RBRACE] = ACTIONS(43),
    [anon_sym_reg] = ACTIONS(27),
    [anon_sym_initial] = ACTIONS(29),
    [anon_sym_if] = ACTIONS(31),
    [anon_sym_for] = ACTIONS(33),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [8] = {
    [sym_global_identifier] = STATE(36),
    [sym_array_type] = STATE(130),
    [sym__type] = STATE(130),
    [sym_declaration] = STATE(125),
    [sym_unary_op] = STATE(33),
    [sym_binary_op] = STATE(33),
    [sym_array_op] = STATE(33),
    [sym_func_call] = STATE(33),
    [sym_parenthesis_expression] = STATE(33),
    [sym__expression] = STATE(33),
    [aux_sym__linebreak] = STATE(7),
    [sym_block] = STATE(123),
    [sym_write_modifiers] = STATE(35),
    [sym_assign_to] = STATE(82),
    [sym_assign_left_side] = STATE(90),
    [sym_decl_assign_statement] = STATE(123),
    [sym_if_statement] = STATE(123),
    [sym_for_statement] = STATE(123),
    [aux_sym_write_modifiers_repeat1] = STATE(63),
    [sym_identifier] = ACTIONS(11),
    [sym_number] = ACTIONS(13),
    [anon_sym_state] = ACTIONS(15),
    [anon_sym_gen] = ACTIONS(15),
    [anon_sym_PLUS] = ACTIONS(17),
    [anon_sym_DASH] = ACTIONS(17),
    [anon_sym_STAR] = ACTIONS(17),
    [anon_sym_BANG] = ACTIONS(17),
    [anon_sym_PIPE] = ACTIONS(17),
    [anon_sym_AMP] = ACTIONS(17),
    [anon_sym_CARET] = ACTIONS(17),
    [anon_sym_LPAREN] = ACTIONS(19),
    [aux_sym__linebreak_token1] = ACTIONS(45),
    [anon_sym_LBRACE] = ACTIONS(23),
    [anon_sym_RBRACE] = ACTIONS(47),
    [anon_sym_reg] = ACTIONS(27),
    [anon_sym_initial] = ACTIONS(29),
    [anon_sym_if] = ACTIONS(31),
    [anon_sym_for] = ACTIONS(33),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [9] = {
    [sym_global_identifier] = STATE(36),
    [sym_array_type] = STATE(130),
    [sym__type] = STATE(130),
    [sym_declaration] = STATE(125),
    [sym_unary_op] = STATE(33),
    [sym_binary_op] = STATE(33),
    [sym_array_op] = STATE(33),
    [sym_func_call] = STATE(33),
    [sym_parenthesis_expression] = STATE(33),
    [sym__expression] = STATE(33),
    [aux_sym__linebreak] = STATE(31),
    [sym_block] = STATE(140),
    [sym_write_modifiers] = STATE(35),
    [sym_assign_to] = STATE(82),
    [sym_assign_left_side] = STATE(131),
    [sym_decl_assign_statement] = STATE(140),
    [sym_if_statement] = STATE(140),
    [sym_for_statement] = STATE(140),
    [aux_sym_write_modifiers_repeat1] = STATE(63),
    [sym_identifier] = ACTIONS(11),
    [sym_number] = ACTIONS(13),
    [anon_sym_state] = ACTIONS(15),
    [anon_sym_gen] = ACTIONS(15),
    [anon_sym_PLUS] = ACTIONS(17),
    [anon_sym_DASH] = ACTIONS(17),
    [anon_sym_STAR] = ACTIONS(17),
    [anon_sym_BANG] = ACTIONS(17),
    [anon_sym_PIPE] = ACTIONS(17),
    [anon_sym_AMP] = ACTIONS(17),
    [anon_sym_CARET] = ACTIONS(17),
    [anon_sym_LPAREN] = ACTIONS(19),
    [aux_sym__linebreak_token1] = ACTIONS(21),
    [anon_sym_LBRACE] = ACTIONS(23),
    [anon_sym_RBRACE] = ACTIONS(49),
    [anon_sym_reg] = ACTIONS(27),
    [anon_sym_initial] = ACTIONS(29),
    [anon_sym_if] = ACTIONS(31),
    [anon_sym_for] = ACTIONS(33),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [10] = {
    [sym_global_identifier] = STATE(36),
    [sym_array_type] = STATE(130),
    [sym__type] = STATE(130),
    [sym_declaration] = STATE(125),
    [sym_unary_op] = STATE(33),
    [sym_binary_op] = STATE(33),
    [sym_array_op] = STATE(33),
    [sym_func_call] = STATE(33),
    [sym_parenthesis_expression] = STATE(33),
    [sym__expression] = STATE(33),
    [aux_sym__linebreak] = STATE(31),
    [sym_block] = STATE(140),
    [sym_write_modifiers] = STATE(35),
    [sym_assign_to] = STATE(82),
    [sym_assign_left_side] = STATE(131),
    [sym_decl_assign_statement] = STATE(140),
    [sym_if_statement] = STATE(140),
    [sym_for_statement] = STATE(140),
    [aux_sym_write_modifiers_repeat1] = STATE(63),
    [sym_identifier] = ACTIONS(11),
    [sym_number] = ACTIONS(13),
    [anon_sym_state] = ACTIONS(15),
    [anon_sym_gen] = ACTIONS(15),
    [anon_sym_PLUS] = ACTIONS(17),
    [anon_sym_DASH] = ACTIONS(17),
    [anon_sym_STAR] = ACTIONS(17),
    [anon_sym_BANG] = ACTIONS(17),
    [anon_sym_PIPE] = ACTIONS(17),
    [anon_sym_AMP] = ACTIONS(17),
    [anon_sym_CARET] = ACTIONS(17),
    [anon_sym_LPAREN] = ACTIONS(19),
    [aux_sym__linebreak_token1] = ACTIONS(21),
    [anon_sym_LBRACE] = ACTIONS(23),
    [anon_sym_reg] = ACTIONS(27),
    [anon_sym_initial] = ACTIONS(29),
    [anon_sym_if] = ACTIONS(31),
    [anon_sym_for] = ACTIONS(33),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 5,
    ACTIONS(55), 1,
      anon_sym_COLON_COLON,
    STATE(13), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(51), 7,
      sym_identifier,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
      anon_sym_in,
    ACTIONS(53), 20,
      anon_sym_COMMA,
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
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
  [42] = 5,
    ACTIONS(55), 1,
      anon_sym_COLON_COLON,
    STATE(11), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(57), 7,
      sym_identifier,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
      anon_sym_in,
    ACTIONS(59), 20,
      anon_sym_COMMA,
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
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
  [84] = 5,
    ACTIONS(65), 1,
      anon_sym_COLON_COLON,
    STATE(13), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(61), 7,
      sym_identifier,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
      anon_sym_in,
    ACTIONS(63), 20,
      anon_sym_COMMA,
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
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
  [126] = 11,
    ACTIONS(70), 1,
      anon_sym_PLUS,
    ACTIONS(72), 1,
      anon_sym_DASH,
    ACTIONS(78), 1,
      anon_sym_SLASH,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(74), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(76), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(68), 16,
      anon_sym_COMMA,
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
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
  [179] = 7,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(84), 19,
      anon_sym_COMMA,
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
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
  [224] = 13,
    ACTIONS(70), 1,
      anon_sym_PLUS,
    ACTIONS(72), 1,
      anon_sym_DASH,
    ACTIONS(78), 1,
      anon_sym_SLASH,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    ACTIONS(88), 1,
      anon_sym_PIPE,
    ACTIONS(90), 1,
      anon_sym_CARET,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(74), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(76), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(68), 14,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_RBRACK,
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
  [281] = 14,
    ACTIONS(70), 1,
      anon_sym_PLUS,
    ACTIONS(72), 1,
      anon_sym_DASH,
    ACTIONS(78), 1,
      anon_sym_SLASH,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    ACTIONS(88), 1,
      anon_sym_PIPE,
    ACTIONS(90), 1,
      anon_sym_CARET,
    ACTIONS(92), 1,
      anon_sym_AMP,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(74), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(76), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(68), 13,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_RBRACK,
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
  [340] = 12,
    ACTIONS(70), 1,
      anon_sym_PLUS,
    ACTIONS(72), 1,
      anon_sym_DASH,
    ACTIONS(78), 1,
      anon_sym_SLASH,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    ACTIONS(90), 1,
      anon_sym_CARET,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(74), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(76), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(68), 15,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_RBRACK,
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
  [395] = 9,
    ACTIONS(78), 1,
      anon_sym_SLASH,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(74), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(76), 4,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(68), 17,
      anon_sym_COMMA,
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
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
  [444] = 7,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(76), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(68), 19,
      anon_sym_COMMA,
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
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
  [489] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(94), 7,
      sym_identifier,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
      anon_sym_in,
    ACTIONS(96), 21,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_COLON_COLON,
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
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
  [526] = 15,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_reg,
    ACTIONS(29), 1,
      anon_sym_initial,
    STATE(35), 1,
      sym_write_modifiers,
    STATE(36), 1,
      sym_global_identifier,
    STATE(63), 1,
      aux_sym_write_modifiers_repeat1,
    STATE(121), 1,
      sym_assign_to,
    STATE(125), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(130), 2,
      sym_array_type,
      sym__type,
    STATE(33), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [586] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(98), 7,
      sym_identifier,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
      anon_sym_in,
    ACTIONS(100), 20,
      anon_sym_COMMA,
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
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_DOT_DOT,
  [622] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(104), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(102), 21,
      anon_sym_COMMA,
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
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
  [657] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(108), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(106), 21,
      anon_sym_COMMA,
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
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
  [692] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(112), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(110), 21,
      anon_sym_COMMA,
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
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
  [727] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(116), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(114), 21,
      anon_sym_COMMA,
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
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
  [762] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(120), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(118), 21,
      anon_sym_COMMA,
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
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
  [797] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(124), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(122), 21,
      anon_sym_COMMA,
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
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
      anon_sym_DOT_DOT,
  [832] = 16,
    ACTIONS(70), 1,
      anon_sym_PLUS,
    ACTIONS(72), 1,
      anon_sym_DASH,
    ACTIONS(78), 1,
      anon_sym_SLASH,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    ACTIONS(88), 1,
      anon_sym_PIPE,
    ACTIONS(90), 1,
      anon_sym_CARET,
    ACTIONS(92), 1,
      anon_sym_AMP,
    ACTIONS(132), 1,
      anon_sym_EQ,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(74), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(130), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(128), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
    ACTIONS(126), 6,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_in,
  [892] = 5,
    ACTIONS(138), 1,
      aux_sym__linebreak_token1,
    STATE(31), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(136), 9,
      anon_sym_module,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_DASH,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(134), 12,
      ts_builtin_sym_end,
      anon_sym_DASH_GT,
      sym_number,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
  [928] = 17,
    ACTIONS(78), 1,
      anon_sym_SLASH,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    ACTIONS(88), 1,
      anon_sym_PIPE,
    ACTIONS(90), 1,
      anon_sym_CARET,
    ACTIONS(92), 1,
      anon_sym_AMP,
    ACTIONS(141), 1,
      anon_sym_COMMA,
    ACTIONS(143), 1,
      anon_sym_RPAREN,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    STATE(49), 1,
      sym__comma,
    STATE(114), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(70), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(74), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(130), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(128), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [987] = 15,
    ACTIONS(78), 1,
      anon_sym_SLASH,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    ACTIONS(88), 1,
      anon_sym_PIPE,
    ACTIONS(90), 1,
      anon_sym_CARET,
    ACTIONS(92), 1,
      anon_sym_AMP,
    ACTIONS(147), 1,
      anon_sym_EQ,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(70), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(74), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(130), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(145), 3,
      anon_sym_COMMA,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
    ACTIONS(128), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [1042] = 15,
    ACTIONS(78), 1,
      anon_sym_SLASH,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    ACTIONS(88), 1,
      anon_sym_PIPE,
    ACTIONS(90), 1,
      anon_sym_CARET,
    ACTIONS(92), 1,
      anon_sym_AMP,
    ACTIONS(151), 1,
      anon_sym_EQ,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(70), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(74), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(130), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(149), 3,
      anon_sym_COMMA,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
    ACTIONS(128), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [1097] = 10,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(153), 1,
      sym_number,
    STATE(36), 1,
      sym_global_identifier,
    STATE(97), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(130), 2,
      sym_array_type,
      sym__type,
    STATE(34), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1142] = 5,
    ACTIONS(155), 1,
      sym_identifier,
    ACTIONS(161), 1,
      anon_sym_LBRACK,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(159), 4,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(157), 15,
      anon_sym_COMMA,
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
      anon_sym_LPAREN,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
  [1176] = 14,
    ACTIONS(78), 1,
      anon_sym_SLASH,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    ACTIONS(88), 1,
      anon_sym_PIPE,
    ACTIONS(90), 1,
      anon_sym_CARET,
    ACTIONS(92), 1,
      anon_sym_AMP,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(70), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(74), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(130), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(164), 2,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
    ACTIONS(128), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [1227] = 15,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(78), 1,
      anon_sym_SLASH,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    ACTIONS(88), 1,
      anon_sym_PIPE,
    ACTIONS(90), 1,
      anon_sym_CARET,
    ACTIONS(92), 1,
      anon_sym_AMP,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    STATE(134), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(70), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(74), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(130), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(128), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [1280] = 14,
    ACTIONS(78), 1,
      anon_sym_SLASH,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    ACTIONS(88), 1,
      anon_sym_PIPE,
    ACTIONS(90), 1,
      anon_sym_CARET,
    ACTIONS(92), 1,
      anon_sym_AMP,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(70), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(74), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(130), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(166), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
    ACTIONS(128), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [1331] = 15,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(78), 1,
      anon_sym_SLASH,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    ACTIONS(88), 1,
      anon_sym_PIPE,
    ACTIONS(90), 1,
      anon_sym_CARET,
    ACTIONS(92), 1,
      anon_sym_AMP,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    STATE(144), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(70), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(74), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(130), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(128), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [1384] = 14,
    ACTIONS(78), 1,
      anon_sym_SLASH,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    ACTIONS(88), 1,
      anon_sym_PIPE,
    ACTIONS(90), 1,
      anon_sym_CARET,
    ACTIONS(92), 1,
      anon_sym_AMP,
    ACTIONS(168), 1,
      anon_sym_DOT_DOT,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(70), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(74), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(130), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(128), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [1434] = 14,
    ACTIONS(78), 1,
      anon_sym_SLASH,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    ACTIONS(88), 1,
      anon_sym_PIPE,
    ACTIONS(90), 1,
      anon_sym_CARET,
    ACTIONS(92), 1,
      anon_sym_AMP,
    ACTIONS(170), 1,
      anon_sym_RPAREN,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(70), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(74), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(130), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(128), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [1484] = 14,
    ACTIONS(78), 1,
      anon_sym_SLASH,
    ACTIONS(80), 1,
      anon_sym_LPAREN,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    ACTIONS(88), 1,
      anon_sym_PIPE,
    ACTIONS(90), 1,
      anon_sym_CARET,
    ACTIONS(92), 1,
      anon_sym_AMP,
    ACTIONS(172), 1,
      anon_sym_RBRACK,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(26), 1,
      sym_parenthesis_expression_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(70), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(74), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(130), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(128), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [1534] = 7,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(176), 1,
      sym_number,
    ACTIONS(178), 1,
      anon_sym_RPAREN,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(32), 7,
      sym_global_identifier,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
  [1569] = 6,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(180), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(14), 7,
      sym_global_identifier,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
  [1601] = 6,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(182), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(37), 7,
      sym_global_identifier,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
  [1633] = 6,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(184), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(40), 7,
      sym_global_identifier,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
  [1665] = 6,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(186), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(30), 7,
      sym_global_identifier,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
  [1697] = 6,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(188), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(39), 7,
      sym_global_identifier,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
  [1729] = 6,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(190), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(41), 7,
      sym_global_identifier,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
  [1761] = 6,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(192), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(15), 7,
      sym_global_identifier,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
  [1793] = 6,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(194), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(17), 7,
      sym_global_identifier,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
  [1825] = 6,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(196), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(43), 7,
      sym_global_identifier,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
  [1857] = 6,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(198), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(42), 7,
      sym_global_identifier,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
  [1889] = 6,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(200), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(16), 7,
      sym_global_identifier,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
  [1921] = 6,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(202), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(18), 7,
      sym_global_identifier,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
  [1953] = 6,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(204), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(20), 7,
      sym_global_identifier,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
  [1985] = 6,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(206), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(19), 7,
      sym_global_identifier,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
  [2017] = 6,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(208), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
    STATE(38), 7,
      sym_global_identifier,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
  [2049] = 5,
    ACTIONS(21), 1,
      aux_sym__linebreak_token1,
    STATE(31), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(210), 5,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
    ACTIONS(212), 9,
      sym_number,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
  [2078] = 5,
    ACTIONS(218), 1,
      aux_sym__linebreak_token1,
    STATE(60), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(214), 5,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
    ACTIONS(216), 9,
      sym_number,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
  [2107] = 5,
    ACTIONS(224), 1,
      anon_sym_reg,
    STATE(62), 1,
      aux_sym_write_modifiers_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(220), 3,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
    ACTIONS(222), 9,
      sym_number,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
  [2134] = 5,
    ACTIONS(27), 1,
      anon_sym_reg,
    STATE(62), 1,
      aux_sym_write_modifiers_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(227), 3,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
    ACTIONS(229), 9,
      sym_number,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
  [2161] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(231), 4,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
    ACTIONS(233), 9,
      sym_number,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
  [2183] = 10,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(21), 1,
      aux_sym__linebreak_token1,
    ACTIONS(235), 1,
      anon_sym_DASH_GT,
    ACTIONS(237), 1,
      anon_sym_LBRACE,
    STATE(31), 1,
      aux_sym__linebreak,
    STATE(93), 1,
      sym_declaration,
    STATE(143), 1,
      sym_declaration_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(130), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [2218] = 10,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(239), 1,
      anon_sym_DASH_GT,
    ACTIONS(241), 1,
      aux_sym__linebreak_token1,
    ACTIONS(243), 1,
      anon_sym_LBRACE,
    STATE(65), 1,
      aux_sym__linebreak,
    STATE(93), 1,
      sym_declaration,
    STATE(137), 1,
      sym_declaration_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(130), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [2253] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(245), 3,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
    ACTIONS(247), 9,
      sym_number,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
  [2274] = 8,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(249), 1,
      aux_sym__linebreak_token1,
    STATE(69), 1,
      aux_sym__linebreak,
    STATE(93), 1,
      sym_declaration,
    STATE(154), 1,
      sym_declaration_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(130), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [2303] = 8,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(21), 1,
      aux_sym__linebreak_token1,
    STATE(31), 1,
      aux_sym__linebreak,
    STATE(93), 1,
      sym_declaration,
    STATE(145), 1,
      sym_declaration_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(130), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [2332] = 8,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(251), 1,
      aux_sym__linebreak_token1,
    STATE(73), 1,
      aux_sym__linebreak,
    STATE(93), 1,
      sym_declaration,
    STATE(147), 1,
      sym_declaration_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(130), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [2361] = 8,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(253), 1,
      aux_sym__linebreak_token1,
    STATE(72), 1,
      aux_sym__linebreak,
    STATE(93), 1,
      sym_declaration,
    STATE(149), 1,
      sym_declaration_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(130), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [2390] = 8,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(21), 1,
      aux_sym__linebreak_token1,
    STATE(31), 1,
      aux_sym__linebreak,
    STATE(93), 1,
      sym_declaration,
    STATE(148), 1,
      sym_declaration_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(130), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [2419] = 8,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(21), 1,
      aux_sym__linebreak_token1,
    STATE(31), 1,
      aux_sym__linebreak,
    STATE(93), 1,
      sym_declaration,
    STATE(150), 1,
      sym_declaration_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(130), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [2448] = 8,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(21), 1,
      aux_sym__linebreak_token1,
    STATE(31), 1,
      aux_sym__linebreak,
    STATE(93), 1,
      sym_declaration,
    STATE(146), 1,
      sym_declaration_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(130), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [2477] = 8,
    ACTIONS(11), 1,
      sym_identifier,
    ACTIONS(255), 1,
      aux_sym__linebreak_token1,
    STATE(74), 1,
      aux_sym__linebreak,
    STATE(93), 1,
      sym_declaration,
    STATE(148), 1,
      sym_declaration_list,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(130), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [2506] = 4,
    ACTIONS(259), 1,
      anon_sym_SQUOTE,
    STATE(80), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(257), 7,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
  [2526] = 4,
    ACTIONS(259), 1,
      anon_sym_SQUOTE,
    STATE(79), 1,
      sym_latency_specifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(261), 7,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
  [2546] = 5,
    ACTIONS(11), 1,
      sym_identifier,
    STATE(152), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(130), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [2566] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(263), 7,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
  [2580] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(265), 7,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      aux_sym__linebreak_token1,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
      anon_sym_in,
  [2594] = 5,
    ACTIONS(11), 1,
      sym_identifier,
    STATE(132), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(130), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [2614] = 5,
    ACTIONS(141), 1,
      anon_sym_COMMA,
    STATE(22), 1,
      sym__comma,
    STATE(83), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(267), 3,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [2633] = 5,
    ACTIONS(141), 1,
      anon_sym_COMMA,
    STATE(22), 1,
      sym__comma,
    STATE(84), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(269), 3,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [2652] = 5,
    ACTIONS(271), 1,
      anon_sym_COMMA,
    STATE(22), 1,
      sym__comma,
    STATE(84), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(274), 3,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [2671] = 6,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(21), 1,
      aux_sym__linebreak_token1,
    ACTIONS(276), 1,
      ts_builtin_sym_end,
    STATE(31), 1,
      aux_sym__linebreak,
    STATE(136), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [2691] = 6,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(21), 1,
      aux_sym__linebreak_token1,
    ACTIONS(278), 1,
      ts_builtin_sym_end,
    STATE(31), 1,
      aux_sym__linebreak,
    STATE(136), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [2711] = 6,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(21), 1,
      aux_sym__linebreak_token1,
    ACTIONS(280), 1,
      ts_builtin_sym_end,
    STATE(31), 1,
      aux_sym__linebreak,
    STATE(136), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [2731] = 6,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(21), 1,
      aux_sym__linebreak_token1,
    ACTIONS(282), 1,
      ts_builtin_sym_end,
    STATE(31), 1,
      aux_sym__linebreak,
    STATE(136), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [2751] = 6,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(21), 1,
      aux_sym__linebreak_token1,
    ACTIONS(284), 1,
      ts_builtin_sym_end,
    STATE(31), 1,
      aux_sym__linebreak,
    STATE(112), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [2771] = 6,
    ACTIONS(286), 1,
      aux_sym__linebreak_token1,
    ACTIONS(288), 1,
      anon_sym_RBRACE,
    ACTIONS(290), 1,
      anon_sym_EQ,
    STATE(4), 1,
      aux_sym__linebreak,
    STATE(100), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [2791] = 6,
    ACTIONS(290), 1,
      anon_sym_EQ,
    ACTIONS(292), 1,
      aux_sym__linebreak_token1,
    ACTIONS(294), 1,
      anon_sym_RBRACE,
    STATE(2), 1,
      aux_sym__linebreak,
    STATE(96), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [2811] = 5,
    ACTIONS(296), 1,
      anon_sym_COMMA,
    STATE(81), 1,
      sym__comma,
    STATE(92), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(299), 2,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
  [2829] = 5,
    ACTIONS(141), 1,
      anon_sym_COMMA,
    STATE(81), 1,
      sym__comma,
    STATE(94), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(301), 2,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
  [2847] = 5,
    ACTIONS(141), 1,
      anon_sym_COMMA,
    STATE(81), 1,
      sym__comma,
    STATE(92), 1,
      aux_sym_declaration_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(303), 2,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
  [2865] = 5,
    ACTIONS(305), 1,
      aux_sym__linebreak_token1,
    ACTIONS(307), 1,
      anon_sym_RBRACE,
    STATE(3), 1,
      aux_sym__linebreak,
    STATE(120), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [2882] = 5,
    ACTIONS(309), 1,
      aux_sym__linebreak_token1,
    ACTIONS(311), 1,
      anon_sym_RBRACE,
    STATE(5), 1,
      aux_sym__linebreak,
    STATE(120), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [2899] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(149), 4,
      anon_sym_COMMA,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [2910] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(313), 4,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_else,
  [2921] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(313), 4,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_else,
  [2932] = 5,
    ACTIONS(315), 1,
      aux_sym__linebreak_token1,
    ACTIONS(317), 1,
      anon_sym_RBRACE,
    STATE(6), 1,
      aux_sym__linebreak,
    STATE(120), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [2949] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(319), 4,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_else,
  [2960] = 5,
    ACTIONS(321), 1,
      anon_sym_COMMA,
    ACTIONS(324), 1,
      anon_sym_RPAREN,
    STATE(49), 1,
      sym__comma,
    STATE(102), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [2977] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(326), 4,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_else,
  [2988] = 5,
    ACTIONS(292), 1,
      aux_sym__linebreak_token1,
    ACTIONS(294), 1,
      anon_sym_RBRACE,
    STATE(2), 1,
      aux_sym__linebreak,
    STATE(113), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3005] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(328), 4,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_else,
  [3016] = 5,
    ACTIONS(330), 1,
      ts_builtin_sym_end,
    ACTIONS(332), 1,
      aux_sym__linebreak_token1,
    STATE(87), 1,
      aux_sym__linebreak,
    STATE(108), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3033] = 5,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(334), 1,
      anon_sym_COLON,
    STATE(135), 1,
      sym_block,
    STATE(138), 1,
      sym_interface_ports,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3050] = 5,
    ACTIONS(336), 1,
      ts_builtin_sym_end,
    ACTIONS(338), 1,
      aux_sym__linebreak_token1,
    STATE(85), 1,
      aux_sym__linebreak,
    STATE(117), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3067] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(340), 4,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_else,
  [3078] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(342), 4,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_else,
  [3089] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(344), 4,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_else,
  [3100] = 5,
    ACTIONS(346), 1,
      ts_builtin_sym_end,
    ACTIONS(348), 1,
      aux_sym__linebreak_token1,
    STATE(86), 1,
      aux_sym__linebreak,
    STATE(127), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3117] = 5,
    ACTIONS(350), 1,
      aux_sym__linebreak_token1,
    ACTIONS(352), 1,
      anon_sym_RBRACE,
    STATE(9), 1,
      aux_sym__linebreak,
    STATE(120), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3134] = 5,
    ACTIONS(141), 1,
      anon_sym_COMMA,
    ACTIONS(354), 1,
      anon_sym_RPAREN,
    STATE(49), 1,
      sym__comma,
    STATE(102), 1,
      aux_sym_parenthesis_expression_list_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3151] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(356), 4,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_else,
  [3162] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(342), 4,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_else,
  [3173] = 5,
    ACTIONS(358), 1,
      ts_builtin_sym_end,
    ACTIONS(360), 1,
      aux_sym__linebreak_token1,
    STATE(117), 1,
      aux_sym_source_file_repeat1,
    STATE(119), 1,
      aux_sym__linebreak,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3190] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(363), 4,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_else,
  [3201] = 5,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(21), 1,
      aux_sym__linebreak_token1,
    STATE(31), 1,
      aux_sym__linebreak,
    STATE(136), 1,
      sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3218] = 5,
    ACTIONS(365), 1,
      aux_sym__linebreak_token1,
    ACTIONS(368), 1,
      anon_sym_RBRACE,
    STATE(10), 1,
      aux_sym__linebreak,
    STATE(120), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3235] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(370), 4,
      anon_sym_COMMA,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [3246] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(319), 4,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_else,
  [3257] = 5,
    ACTIONS(286), 1,
      aux_sym__linebreak_token1,
    ACTIONS(288), 1,
      anon_sym_RBRACE,
    STATE(4), 1,
      aux_sym__linebreak,
    STATE(95), 1,
      aux_sym_block_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3274] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(363), 4,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_else,
  [3285] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 4,
      anon_sym_COMMA,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [3296] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(372), 4,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
      anon_sym_else,
  [3307] = 5,
    ACTIONS(374), 1,
      ts_builtin_sym_end,
    ACTIONS(376), 1,
      aux_sym__linebreak_token1,
    STATE(88), 1,
      aux_sym__linebreak,
    STATE(117), 1,
      aux_sym_source_file_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3324] = 3,
    ACTIONS(174), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(133), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3337] = 4,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(378), 1,
      anon_sym_if,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(141), 2,
      sym_block,
      sym_if_statement,
  [3352] = 4,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    ACTIONS(380), 1,
      sym_identifier,
    STATE(142), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3366] = 3,
    ACTIONS(290), 1,
      anon_sym_EQ,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(382), 2,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
  [3378] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(384), 3,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
  [3388] = 4,
    ACTIONS(82), 1,
      anon_sym_LBRACK,
    ACTIONS(386), 1,
      sym_identifier,
    STATE(142), 1,
      sym_array_bracket_expression,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3402] = 3,
    ACTIONS(390), 1,
      anon_sym_else,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(388), 2,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
  [3414] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(392), 2,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
  [3423] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(394), 2,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
  [3432] = 3,
    ACTIONS(396), 1,
      anon_sym_DASH_GT,
    ACTIONS(398), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3443] = 3,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    STATE(139), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3454] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(400), 2,
      ts_builtin_sym_end,
      aux_sym__linebreak_token1,
  [3463] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(382), 2,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
  [3472] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(402), 2,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
  [3481] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(404), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3490] = 3,
    ACTIONS(406), 1,
      anon_sym_DASH_GT,
    ACTIONS(408), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3501] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(410), 2,
      aux_sym__linebreak_token1,
      anon_sym_RBRACE,
  [3510] = 2,
    ACTIONS(412), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3518] = 2,
    ACTIONS(414), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3526] = 2,
    ACTIONS(416), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3534] = 2,
    ACTIONS(418), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3542] = 2,
    ACTIONS(420), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3550] = 2,
    ACTIONS(422), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3558] = 2,
    ACTIONS(424), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3566] = 2,
    ACTIONS(426), 1,
      anon_sym_in,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3574] = 2,
    ACTIONS(428), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3582] = 2,
    ACTIONS(430), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3590] = 2,
    ACTIONS(432), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(11)] = 0,
  [SMALL_STATE(12)] = 42,
  [SMALL_STATE(13)] = 84,
  [SMALL_STATE(14)] = 126,
  [SMALL_STATE(15)] = 179,
  [SMALL_STATE(16)] = 224,
  [SMALL_STATE(17)] = 281,
  [SMALL_STATE(18)] = 340,
  [SMALL_STATE(19)] = 395,
  [SMALL_STATE(20)] = 444,
  [SMALL_STATE(21)] = 489,
  [SMALL_STATE(22)] = 526,
  [SMALL_STATE(23)] = 586,
  [SMALL_STATE(24)] = 622,
  [SMALL_STATE(25)] = 657,
  [SMALL_STATE(26)] = 692,
  [SMALL_STATE(27)] = 727,
  [SMALL_STATE(28)] = 762,
  [SMALL_STATE(29)] = 797,
  [SMALL_STATE(30)] = 832,
  [SMALL_STATE(31)] = 892,
  [SMALL_STATE(32)] = 928,
  [SMALL_STATE(33)] = 987,
  [SMALL_STATE(34)] = 1042,
  [SMALL_STATE(35)] = 1097,
  [SMALL_STATE(36)] = 1142,
  [SMALL_STATE(37)] = 1176,
  [SMALL_STATE(38)] = 1227,
  [SMALL_STATE(39)] = 1280,
  [SMALL_STATE(40)] = 1331,
  [SMALL_STATE(41)] = 1384,
  [SMALL_STATE(42)] = 1434,
  [SMALL_STATE(43)] = 1484,
  [SMALL_STATE(44)] = 1534,
  [SMALL_STATE(45)] = 1569,
  [SMALL_STATE(46)] = 1601,
  [SMALL_STATE(47)] = 1633,
  [SMALL_STATE(48)] = 1665,
  [SMALL_STATE(49)] = 1697,
  [SMALL_STATE(50)] = 1729,
  [SMALL_STATE(51)] = 1761,
  [SMALL_STATE(52)] = 1793,
  [SMALL_STATE(53)] = 1825,
  [SMALL_STATE(54)] = 1857,
  [SMALL_STATE(55)] = 1889,
  [SMALL_STATE(56)] = 1921,
  [SMALL_STATE(57)] = 1953,
  [SMALL_STATE(58)] = 1985,
  [SMALL_STATE(59)] = 2017,
  [SMALL_STATE(60)] = 2049,
  [SMALL_STATE(61)] = 2078,
  [SMALL_STATE(62)] = 2107,
  [SMALL_STATE(63)] = 2134,
  [SMALL_STATE(64)] = 2161,
  [SMALL_STATE(65)] = 2183,
  [SMALL_STATE(66)] = 2218,
  [SMALL_STATE(67)] = 2253,
  [SMALL_STATE(68)] = 2274,
  [SMALL_STATE(69)] = 2303,
  [SMALL_STATE(70)] = 2332,
  [SMALL_STATE(71)] = 2361,
  [SMALL_STATE(72)] = 2390,
  [SMALL_STATE(73)] = 2419,
  [SMALL_STATE(74)] = 2448,
  [SMALL_STATE(75)] = 2477,
  [SMALL_STATE(76)] = 2506,
  [SMALL_STATE(77)] = 2526,
  [SMALL_STATE(78)] = 2546,
  [SMALL_STATE(79)] = 2566,
  [SMALL_STATE(80)] = 2580,
  [SMALL_STATE(81)] = 2594,
  [SMALL_STATE(82)] = 2614,
  [SMALL_STATE(83)] = 2633,
  [SMALL_STATE(84)] = 2652,
  [SMALL_STATE(85)] = 2671,
  [SMALL_STATE(86)] = 2691,
  [SMALL_STATE(87)] = 2711,
  [SMALL_STATE(88)] = 2731,
  [SMALL_STATE(89)] = 2751,
  [SMALL_STATE(90)] = 2771,
  [SMALL_STATE(91)] = 2791,
  [SMALL_STATE(92)] = 2811,
  [SMALL_STATE(93)] = 2829,
  [SMALL_STATE(94)] = 2847,
  [SMALL_STATE(95)] = 2865,
  [SMALL_STATE(96)] = 2882,
  [SMALL_STATE(97)] = 2899,
  [SMALL_STATE(98)] = 2910,
  [SMALL_STATE(99)] = 2921,
  [SMALL_STATE(100)] = 2932,
  [SMALL_STATE(101)] = 2949,
  [SMALL_STATE(102)] = 2960,
  [SMALL_STATE(103)] = 2977,
  [SMALL_STATE(104)] = 2988,
  [SMALL_STATE(105)] = 3005,
  [SMALL_STATE(106)] = 3016,
  [SMALL_STATE(107)] = 3033,
  [SMALL_STATE(108)] = 3050,
  [SMALL_STATE(109)] = 3067,
  [SMALL_STATE(110)] = 3078,
  [SMALL_STATE(111)] = 3089,
  [SMALL_STATE(112)] = 3100,
  [SMALL_STATE(113)] = 3117,
  [SMALL_STATE(114)] = 3134,
  [SMALL_STATE(115)] = 3151,
  [SMALL_STATE(116)] = 3162,
  [SMALL_STATE(117)] = 3173,
  [SMALL_STATE(118)] = 3190,
  [SMALL_STATE(119)] = 3201,
  [SMALL_STATE(120)] = 3218,
  [SMALL_STATE(121)] = 3235,
  [SMALL_STATE(122)] = 3246,
  [SMALL_STATE(123)] = 3257,
  [SMALL_STATE(124)] = 3274,
  [SMALL_STATE(125)] = 3285,
  [SMALL_STATE(126)] = 3296,
  [SMALL_STATE(127)] = 3307,
  [SMALL_STATE(128)] = 3324,
  [SMALL_STATE(129)] = 3337,
  [SMALL_STATE(130)] = 3352,
  [SMALL_STATE(131)] = 3366,
  [SMALL_STATE(132)] = 3378,
  [SMALL_STATE(133)] = 3388,
  [SMALL_STATE(134)] = 3402,
  [SMALL_STATE(135)] = 3414,
  [SMALL_STATE(136)] = 3423,
  [SMALL_STATE(137)] = 3432,
  [SMALL_STATE(138)] = 3443,
  [SMALL_STATE(139)] = 3454,
  [SMALL_STATE(140)] = 3463,
  [SMALL_STATE(141)] = 3472,
  [SMALL_STATE(142)] = 3481,
  [SMALL_STATE(143)] = 3490,
  [SMALL_STATE(144)] = 3501,
  [SMALL_STATE(145)] = 3510,
  [SMALL_STATE(146)] = 3518,
  [SMALL_STATE(147)] = 3526,
  [SMALL_STATE(148)] = 3534,
  [SMALL_STATE(149)] = 3542,
  [SMALL_STATE(150)] = 3550,
  [SMALL_STATE(151)] = 3558,
  [SMALL_STATE(152)] = 3566,
  [SMALL_STATE(153)] = 3574,
  [SMALL_STATE(154)] = 3582,
  [SMALL_STATE(155)] = 3590,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(153),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(128),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(64),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(67),
  [31] = {.entry = {.count = 1, .reusable = false}}, SHIFT(59),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(78),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(115),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [51] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 2, .production_id = 2),
  [53] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 2, .production_id = 2),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(151),
  [57] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 1, .production_id = 1),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 1, .production_id = 1),
  [61] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_global_identifier_repeat1, 2, .production_id = 5),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_global_identifier_repeat1, 2, .production_id = 5),
  [65] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_global_identifier_repeat1, 2, .production_id = 5), SHIFT_REPEAT(151),
  [68] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_op, 3, .production_id = 24),
  [70] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [72] = {.entry = {.count = 1, .reusable = false}}, SHIFT(58),
  [74] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [76] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_op, 3, .production_id = 24),
  [78] = {.entry = {.count = 1, .reusable = false}}, SHIFT(57),
  [80] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [82] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [84] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_op, 2, .production_id = 15),
  [86] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_op, 2, .production_id = 15),
  [88] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [90] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [92] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [94] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_global_identifier_repeat1, 2, .production_id = 3),
  [96] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_global_identifier_repeat1, 2, .production_id = 3),
  [98] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 22),
  [100] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 22),
  [102] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 3, .production_id = 3),
  [104] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 3, .production_id = 3),
  [106] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_op, 2, .production_id = 13),
  [108] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_op, 2, .production_id = 13),
  [110] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 2, .production_id = 16),
  [112] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 2, .production_id = 16),
  [114] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 4, .production_id = 6),
  [116] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 4, .production_id = 6),
  [118] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 22),
  [120] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 22),
  [122] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression_list, 2),
  [124] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression_list, 2),
  [126] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_latency_specifier, 2, .production_id = 22),
  [128] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [130] = {.entry = {.count = 1, .reusable = false}}, SHIFT(52),
  [132] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_latency_specifier, 2, .production_id = 22),
  [134] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__linebreak, 2),
  [136] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__linebreak, 2),
  [138] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__linebreak, 2), SHIFT_REPEAT(31),
  [141] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [143] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_to, 1, .production_id = 8),
  [147] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_to, 1, .production_id = 8),
  [149] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_to, 2, .production_id = 17),
  [151] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_to, 2, .production_id = 17),
  [153] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__type, 1),
  [157] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expression, 1),
  [159] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expression, 1),
  [161] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym__type, 1), REDUCE(sym__expression, 1),
  [164] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_decl_assign_statement, 3, .production_id = 26),
  [166] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 3),
  [168] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [170] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [172] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [174] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [176] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [178] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [180] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [182] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [184] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [186] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [188] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [190] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [192] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [194] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [196] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [198] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [200] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [202] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [204] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [206] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [208] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [210] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__comma, 2),
  [212] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__comma, 2),
  [214] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__comma, 1),
  [216] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__comma, 1),
  [218] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [220] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 5),
  [222] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 5),
  [224] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 2, .production_id = 5), SHIFT_REPEAT(64),
  [227] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_write_modifiers, 1, .production_id = 9),
  [229] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_write_modifiers, 1, .production_id = 9),
  [231] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_write_modifiers_repeat1, 1, .production_id = 1),
  [233] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_write_modifiers_repeat1, 1, .production_id = 1),
  [235] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [237] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2),
  [239] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [241] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [243] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 1),
  [245] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_write_modifiers, 1, .production_id = 1),
  [247] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_write_modifiers, 1, .production_id = 1),
  [249] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [251] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [253] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [255] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [257] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 2, .production_id = 12),
  [259] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [261] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 19),
  [263] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 27),
  [265] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 21),
  [267] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 1, .production_id = 1),
  [269] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 2, .production_id = 2),
  [271] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 5), SHIFT_REPEAT(61),
  [274] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 5),
  [276] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 2),
  [278] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 3),
  [280] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 1),
  [282] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 4, .production_id = 6),
  [284] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [286] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [288] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [290] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [292] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [294] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [296] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 5), SHIFT_REPEAT(61),
  [299] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 5),
  [301] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration_list, 1, .production_id = 1),
  [303] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration_list, 2, .production_id = 2),
  [305] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [307] = {.entry = {.count = 1, .reusable = true}}, SHIFT(118),
  [309] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [311] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [313] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 6, .production_id = 31),
  [315] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [317] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [319] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 31),
  [321] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 5), SHIFT_REPEAT(61),
  [324] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parenthesis_expression_list_repeat1, 2, .production_id = 5),
  [326] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, .production_id = 3),
  [328] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [330] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, .production_id = 1),
  [332] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [334] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [336] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 2),
  [338] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [340] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 25),
  [342] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5, .production_id = 6),
  [344] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 25),
  [346] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, .production_id = 3),
  [348] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [350] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [352] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [354] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [356] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 3),
  [358] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 5),
  [360] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 5), SHIFT_REPEAT(119),
  [363] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 6),
  [365] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 5), SHIFT_REPEAT(10),
  [368] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 5),
  [370] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2, .production_id = 3),
  [372] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [374] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, .production_id = 6),
  [376] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [378] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [380] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [382] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 3),
  [384] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_list_repeat1, 2, .production_id = 3),
  [386] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [388] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 3, .production_id = 23),
  [390] = {.entry = {.count = 1, .reusable = true}}, SHIFT(129),
  [392] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 3, .production_id = 4),
  [394] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 3),
  [396] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [398] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2, .production_id = 7),
  [400] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 4, .production_id = 10),
  [402] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 5, .production_id = 33),
  [404] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type, 2, .production_id = 13),
  [406] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [408] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 14),
  [410] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_statement, 7, .production_id = 34),
  [412] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 5, .production_id = 28),
  [414] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 5, .production_id = 29),
  [416] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 5, .production_id = 30),
  [418] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 4, .production_id = 18),
  [420] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 11),
  [422] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 6, .production_id = 32),
  [424] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [426] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [428] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
  [430] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 4, .production_id = 20),
  [432] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
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
