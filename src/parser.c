#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 163
#define LARGE_STATE_COUNT 9
#define SYMBOL_COUNT 72
#define ALIAS_COUNT 0
#define TOKEN_COUNT 40
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 24
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 29

enum {
  anon_sym_COLON = 1,
  anon_sym_COMMA = 2,
  anon_sym_DASH_GT = 3,
  anon_sym_module = 4,
  sym_identifier = 5,
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
  anon_sym_LBRACE = 30,
  anon_sym_RBRACE = 31,
  anon_sym_reg = 32,
  anon_sym_initial = 33,
  anon_sym_EQ = 34,
  anon_sym_if = 35,
  anon_sym_else = 36,
  anon_sym_for = 37,
  anon_sym_in = 38,
  anon_sym_SEMI = 39,
  sym_source_file = 40,
  sym_interface_ports = 41,
  sym_module = 42,
  sym_global_identifier = 43,
  sym__maybe_global_identifier = 44,
  sym_array_type = 45,
  sym__type = 46,
  sym_latency_specifier = 47,
  sym_declaration = 48,
  sym_unary_op = 49,
  sym_binary_op = 50,
  sym_array_op = 51,
  sym_func_call = 52,
  sym_parenthesis_expression = 53,
  sym_array_bracket_expression = 54,
  sym__expression = 55,
  sym_range = 56,
  sym_block = 57,
  sym__assign_left_side = 58,
  sym_decl_assign_statement = 59,
  sym_decl_statement = 60,
  sym_expression_statement = 61,
  sym_if_statement = 62,
  sym_for_statement = 63,
  sym__statement = 64,
  aux_sym_source_file_repeat1 = 65,
  aux_sym_interface_ports_repeat1 = 66,
  aux_sym_global_identifier_repeat1 = 67,
  aux_sym_func_call_repeat1 = 68,
  aux_sym_block_repeat1 = 69,
  aux_sym__assign_left_side_repeat1 = 70,
  aux_sym__assign_left_side_repeat2 = 71,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_COLON] = ":",
  [anon_sym_COMMA] = ",",
  [anon_sym_DASH_GT] = "->",
  [anon_sym_module] = "module",
  [sym_identifier] = "identifier",
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
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_reg] = "reg",
  [anon_sym_initial] = "initial",
  [anon_sym_EQ] = "=",
  [anon_sym_if] = "if",
  [anon_sym_else] = "else",
  [anon_sym_for] = "for",
  [anon_sym_in] = "in",
  [anon_sym_SEMI] = ";",
  [sym_source_file] = "source_file",
  [sym_interface_ports] = "interface_ports",
  [sym_module] = "module",
  [sym_global_identifier] = "global_identifier",
  [sym__maybe_global_identifier] = "_maybe_global_identifier",
  [sym_array_type] = "array_type",
  [sym__type] = "_type",
  [sym_latency_specifier] = "latency_specifier",
  [sym_declaration] = "declaration",
  [sym_unary_op] = "unary_op",
  [sym_binary_op] = "binary_op",
  [sym_array_op] = "array_op",
  [sym_func_call] = "func_call",
  [sym_parenthesis_expression] = "parenthesis_expression",
  [sym_array_bracket_expression] = "array_bracket_expression",
  [sym__expression] = "_expression",
  [sym_range] = "range",
  [sym_block] = "block",
  [sym__assign_left_side] = "_assign_left_side",
  [sym_decl_assign_statement] = "decl_assign_statement",
  [sym_decl_statement] = "decl_statement",
  [sym_expression_statement] = "expression_statement",
  [sym_if_statement] = "if_statement",
  [sym_for_statement] = "for_statement",
  [sym__statement] = "_statement",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_interface_ports_repeat1] = "interface_ports_repeat1",
  [aux_sym_global_identifier_repeat1] = "global_identifier_repeat1",
  [aux_sym_func_call_repeat1] = "func_call_repeat1",
  [aux_sym_block_repeat1] = "block_repeat1",
  [aux_sym__assign_left_side_repeat1] = "_assign_left_side_repeat1",
  [aux_sym__assign_left_side_repeat2] = "_assign_left_side_repeat2",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_DASH_GT] = anon_sym_DASH_GT,
  [anon_sym_module] = anon_sym_module,
  [sym_identifier] = sym_identifier,
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
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_reg] = anon_sym_reg,
  [anon_sym_initial] = anon_sym_initial,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_if] = anon_sym_if,
  [anon_sym_else] = anon_sym_else,
  [anon_sym_for] = anon_sym_for,
  [anon_sym_in] = anon_sym_in,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [sym_source_file] = sym_source_file,
  [sym_interface_ports] = sym_interface_ports,
  [sym_module] = sym_module,
  [sym_global_identifier] = sym_global_identifier,
  [sym__maybe_global_identifier] = sym__maybe_global_identifier,
  [sym_array_type] = sym_array_type,
  [sym__type] = sym__type,
  [sym_latency_specifier] = sym_latency_specifier,
  [sym_declaration] = sym_declaration,
  [sym_unary_op] = sym_unary_op,
  [sym_binary_op] = sym_binary_op,
  [sym_array_op] = sym_array_op,
  [sym_func_call] = sym_func_call,
  [sym_parenthesis_expression] = sym_parenthesis_expression,
  [sym_array_bracket_expression] = sym_array_bracket_expression,
  [sym__expression] = sym__expression,
  [sym_range] = sym_range,
  [sym_block] = sym_block,
  [sym__assign_left_side] = sym__assign_left_side,
  [sym_decl_assign_statement] = sym_decl_assign_statement,
  [sym_decl_statement] = sym_decl_statement,
  [sym_expression_statement] = sym_expression_statement,
  [sym_if_statement] = sym_if_statement,
  [sym_for_statement] = sym_for_statement,
  [sym__statement] = sym__statement,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_interface_ports_repeat1] = aux_sym_interface_ports_repeat1,
  [aux_sym_global_identifier_repeat1] = aux_sym_global_identifier_repeat1,
  [aux_sym_func_call_repeat1] = aux_sym_func_call_repeat1,
  [aux_sym_block_repeat1] = aux_sym_block_repeat1,
  [aux_sym__assign_left_side_repeat1] = aux_sym__assign_left_side_repeat1,
  [aux_sym__assign_left_side_repeat2] = aux_sym__assign_left_side_repeat2,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
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
  [sym_identifier] = {
    .visible = true,
    .named = true,
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
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_interface_ports] = {
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
  [sym__maybe_global_identifier] = {
    .visible = false,
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
  [sym_range] = {
    .visible = true,
    .named = true,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym__assign_left_side] = {
    .visible = false,
    .named = true,
  },
  [sym_decl_assign_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_decl_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_expression_statement] = {
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
  [sym__statement] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_interface_ports_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_global_identifier_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_func_call_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_block_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym__assign_left_side_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym__assign_left_side_repeat2] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_argument = 1,
  field_arr = 2,
  field_arr_idx = 3,
  field_assign_to = 4,
  field_assign_value = 5,
  field_block = 6,
  field_condition = 7,
  field_content = 8,
  field_declaration_modifiers = 9,
  field_else_block = 10,
  field_for_decl = 11,
  field_for_range = 12,
  field_from = 13,
  field_inputs = 14,
  field_interface_ports = 15,
  field_latency_specifier = 16,
  field_left = 17,
  field_name = 18,
  field_operator = 19,
  field_outputs = 20,
  field_right = 21,
  field_then_block = 22,
  field_to = 23,
  field_type = 24,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_argument] = "argument",
  [field_arr] = "arr",
  [field_arr_idx] = "arr_idx",
  [field_assign_to] = "assign_to",
  [field_assign_value] = "assign_value",
  [field_block] = "block",
  [field_condition] = "condition",
  [field_content] = "content",
  [field_declaration_modifiers] = "declaration_modifiers",
  [field_else_block] = "else_block",
  [field_for_decl] = "for_decl",
  [field_for_range] = "for_range",
  [field_from] = "from",
  [field_inputs] = "inputs",
  [field_interface_ports] = "interface_ports",
  [field_latency_specifier] = "latency_specifier",
  [field_left] = "left",
  [field_name] = "name",
  [field_operator] = "operator",
  [field_outputs] = "outputs",
  [field_right] = "right",
  [field_then_block] = "then_block",
  [field_to] = "to",
  [field_type] = "type",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 2},
  [2] = {.index = 2, .length = 1},
  [3] = {.index = 3, .length = 3},
  [4] = {.index = 6, .length = 1},
  [5] = {.index = 7, .length = 2},
  [6] = {.index = 9, .length = 2},
  [7] = {.index = 11, .length = 2},
  [8] = {.index = 13, .length = 2},
  [9] = {.index = 15, .length = 2},
  [10] = {.index = 17, .length = 3},
  [11] = {.index = 20, .length = 3},
  [12] = {.index = 23, .length = 2},
  [13] = {.index = 25, .length = 1},
  [14] = {.index = 26, .length = 2},
  [15] = {.index = 28, .length = 1},
  [16] = {.index = 29, .length = 3},
  [17] = {.index = 32, .length = 2},
  [18] = {.index = 34, .length = 4},
  [19] = {.index = 38, .length = 3},
  [20] = {.index = 41, .length = 3},
  [21] = {.index = 44, .length = 2},
  [22] = {.index = 46, .length = 4},
  [23] = {.index = 50, .length = 3},
  [24] = {.index = 53, .length = 3},
  [25] = {.index = 56, .length = 1},
  [26] = {.index = 57, .length = 3},
  [27] = {.index = 60, .length = 2},
  [28] = {.index = 62, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_block, 2},
    {field_name, 1},
  [2] =
    {field_inputs, 1},
  [3] =
    {field_block, 3},
    {field_interface_ports, 2},
    {field_name, 1},
  [6] =
    {field_outputs, 2},
  [7] =
    {field_name, 1},
    {field_type, 0},
  [9] =
    {field_arr, 0},
    {field_arr_idx, 1},
  [11] =
    {field_inputs, 1},
    {field_inputs, 2},
  [13] =
    {field_operator, 0},
    {field_right, 1},
  [15] =
    {field_outputs, 2},
    {field_outputs, 3},
  [17] =
    {field_declaration_modifiers, 0},
    {field_name, 2},
    {field_type, 1},
  [20] =
    {field_latency_specifier, 2},
    {field_name, 1},
    {field_type, 0},
  [23] =
    {field_inputs, 1},
    {field_outputs, 3},
  [25] =
    {field_content, 1},
  [26] =
    {field_condition, 1},
    {field_then_block, 2},
  [28] =
    {field_name, 0},
  [29] =
    {field_left, 0},
    {field_operator, 1},
    {field_right, 2},
  [32] =
    {field_assign_to, 0},
    {field_assign_value, 2},
  [34] =
    {field_declaration_modifiers, 0},
    {field_latency_specifier, 3},
    {field_name, 2},
    {field_type, 1},
  [38] =
    {field_inputs, 1},
    {field_outputs, 3},
    {field_outputs, 4},
  [41] =
    {field_inputs, 1},
    {field_inputs, 2},
    {field_outputs, 4},
  [44] =
    {field_argument, 2},
    {field_name, 0},
  [46] =
    {field_inputs, 1},
    {field_inputs, 2},
    {field_outputs, 4},
    {field_outputs, 5},
  [50] =
    {field_condition, 1},
    {field_else_block, 4},
    {field_then_block, 2},
  [53] =
    {field_block, 4},
    {field_for_decl, 1},
    {field_for_range, 3},
  [56] =
    {field_argument, 1},
  [57] =
    {field_argument, 2},
    {field_argument, 3, .inherited = true},
    {field_name, 0},
  [60] =
    {field_argument, 0, .inherited = true},
    {field_argument, 1, .inherited = true},
  [62] =
    {field_from, 0},
    {field_to, 2},
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
  [4] = 2,
  [5] = 3,
  [6] = 6,
  [7] = 2,
  [8] = 3,
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
  [47] = 46,
  [48] = 48,
  [49] = 11,
  [50] = 10,
  [51] = 51,
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 12,
  [56] = 13,
  [57] = 57,
  [58] = 45,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 44,
  [69] = 69,
  [70] = 15,
  [71] = 43,
  [72] = 72,
  [73] = 72,
  [74] = 39,
  [75] = 59,
  [76] = 63,
  [77] = 62,
  [78] = 61,
  [79] = 60,
  [80] = 80,
  [81] = 16,
  [82] = 82,
  [83] = 83,
  [84] = 32,
  [85] = 28,
  [86] = 26,
  [87] = 87,
  [88] = 24,
  [89] = 31,
  [90] = 35,
  [91] = 91,
  [92] = 82,
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
  [106] = 105,
  [107] = 107,
  [108] = 108,
  [109] = 109,
  [110] = 10,
  [111] = 11,
  [112] = 112,
  [113] = 113,
  [114] = 114,
  [115] = 115,
  [116] = 116,
  [117] = 117,
  [118] = 13,
  [119] = 119,
  [120] = 12,
  [121] = 121,
  [122] = 122,
  [123] = 123,
  [124] = 124,
  [125] = 102,
  [126] = 126,
  [127] = 127,
  [128] = 128,
  [129] = 129,
  [130] = 127,
  [131] = 126,
  [132] = 132,
  [133] = 133,
  [134] = 134,
  [135] = 101,
  [136] = 136,
  [137] = 16,
  [138] = 138,
  [139] = 139,
  [140] = 140,
  [141] = 141,
  [142] = 142,
  [143] = 143,
  [144] = 144,
  [145] = 145,
  [146] = 39,
  [147] = 147,
  [148] = 148,
  [149] = 149,
  [150] = 43,
  [151] = 33,
  [152] = 152,
  [153] = 153,
  [154] = 154,
  [155] = 155,
  [156] = 156,
  [157] = 155,
  [158] = 154,
  [159] = 159,
  [160] = 155,
  [161] = 154,
  [162] = 162,
};

static inline bool sym_identifier_character_set_1(int32_t c) {
  return (c < 6656
    ? (c < 2979
      ? (c < 2308
        ? (c < 1376
          ? (c < 880
            ? (c < 192
              ? (c < 170
                ? (c < '_'
                  ? (c >= 'A' && c <= 'Z')
                  : (c <= '_' || (c >= 'a' && c <= 'z')))
                : (c <= 170 || (c < 186
                  ? c == 181
                  : c <= 186)))
              : (c <= 214 || (c < 736
                ? (c < 248
                  ? (c >= 216 && c <= 246)
                  : (c <= 705 || (c >= 710 && c <= 721)))
                : (c <= 740 || (c < 750
                  ? c == 748
                  : c <= 750)))))
            : (c <= 884 || (c < 910
              ? (c < 902
                ? (c < 890
                  ? (c >= 886 && c <= 887)
                  : (c <= 893 || c == 895))
                : (c <= 902 || (c < 908
                  ? (c >= 904 && c <= 906)
                  : c <= 908)))
              : (c <= 929 || (c < 1162
                ? (c < 1015
                  ? (c >= 931 && c <= 1013)
                  : c <= 1153)
                : (c <= 1327 || (c < 1369
                  ? (c >= 1329 && c <= 1366)
                  : c <= 1369)))))))
          : (c <= 1416 || (c < 1969
            ? (c < 1765
              ? (c < 1646
                ? (c < 1519
                  ? (c >= 1488 && c <= 1514)
                  : (c <= 1522 || (c >= 1568 && c <= 1610)))
                : (c <= 1647 || (c < 1749
                  ? (c >= 1649 && c <= 1747)
                  : c <= 1749)))
              : (c <= 1766 || (c < 1808
                ? (c < 1786
                  ? (c >= 1774 && c <= 1775)
                  : (c <= 1788 || c == 1791))
                : (c <= 1808 || (c < 1869
                  ? (c >= 1810 && c <= 1839)
                  : c <= 1957)))))
            : (c <= 1969 || (c < 2088
              ? (c < 2048
                ? (c < 2036
                  ? (c >= 1994 && c <= 2026)
                  : (c <= 2037 || c == 2042))
                : (c <= 2069 || (c < 2084
                  ? c == 2074
                  : c <= 2084)))
              : (c <= 2088 || (c < 2160
                ? (c < 2144
                  ? (c >= 2112 && c <= 2136)
                  : c <= 2154)
                : (c <= 2183 || (c < 2208
                  ? (c >= 2185 && c <= 2190)
                  : c <= 2249)))))))))
        : (c <= 2361 || (c < 2693
          ? (c < 2527
            ? (c < 2451
              ? (c < 2417
                ? (c < 2384
                  ? c == 2365
                  : (c <= 2384 || (c >= 2392 && c <= 2401)))
                : (c <= 2432 || (c < 2447
                  ? (c >= 2437 && c <= 2444)
                  : c <= 2448)))
              : (c <= 2472 || (c < 2493
                ? (c < 2482
                  ? (c >= 2474 && c <= 2480)
                  : (c <= 2482 || (c >= 2486 && c <= 2489)))
                : (c <= 2493 || (c < 2524
                  ? c == 2510
                  : c <= 2525)))))
            : (c <= 2529 || (c < 2610
              ? (c < 2575
                ? (c < 2556
                  ? (c >= 2544 && c <= 2545)
                  : (c <= 2556 || (c >= 2565 && c <= 2570)))
                : (c <= 2576 || (c < 2602
                  ? (c >= 2579 && c <= 2600)
                  : c <= 2608)))
              : (c <= 2611 || (c < 2649
                ? (c < 2616
                  ? (c >= 2613 && c <= 2614)
                  : c <= 2617)
                : (c <= 2652 || (c < 2674
                  ? c == 2654
                  : c <= 2676)))))))
          : (c <= 2701 || (c < 2866
            ? (c < 2768
              ? (c < 2738
                ? (c < 2707
                  ? (c >= 2703 && c <= 2705)
                  : (c <= 2728 || (c >= 2730 && c <= 2736)))
                : (c <= 2739 || (c < 2749
                  ? (c >= 2741 && c <= 2745)
                  : c <= 2749)))
              : (c <= 2768 || (c < 2831
                ? (c < 2809
                  ? (c >= 2784 && c <= 2785)
                  : (c <= 2809 || (c >= 2821 && c <= 2828)))
                : (c <= 2832 || (c < 2858
                  ? (c >= 2835 && c <= 2856)
                  : c <= 2864)))))
            : (c <= 2867 || (c < 2949
              ? (c < 2911
                ? (c < 2877
                  ? (c >= 2869 && c <= 2873)
                  : (c <= 2877 || (c >= 2908 && c <= 2909)))
                : (c <= 2913 || (c < 2947
                  ? c == 2929
                  : c <= 2947)))
              : (c <= 2954 || (c < 2969
                ? (c < 2962
                  ? (c >= 2958 && c <= 2960)
                  : c <= 2965)
                : (c <= 2970 || (c < 2974
                  ? c == 2972
                  : c <= 2975)))))))))))
      : (c <= 2980 || (c < 4159
        ? (c < 3412
          ? (c < 3214
            ? (c < 3114
              ? (c < 3077
                ? (c < 2990
                  ? (c >= 2984 && c <= 2986)
                  : (c <= 3001 || c == 3024))
                : (c <= 3084 || (c < 3090
                  ? (c >= 3086 && c <= 3088)
                  : c <= 3112)))
              : (c <= 3129 || (c < 3168
                ? (c < 3160
                  ? c == 3133
                  : (c <= 3162 || c == 3165))
                : (c <= 3169 || (c < 3205
                  ? c == 3200
                  : c <= 3212)))))
            : (c <= 3216 || (c < 3313
              ? (c < 3261
                ? (c < 3242
                  ? (c >= 3218 && c <= 3240)
                  : (c <= 3251 || (c >= 3253 && c <= 3257)))
                : (c <= 3261 || (c < 3296
                  ? (c >= 3293 && c <= 3294)
                  : c <= 3297)))
              : (c <= 3314 || (c < 3346
                ? (c < 3342
                  ? (c >= 3332 && c <= 3340)
                  : c <= 3344)
                : (c <= 3386 || (c < 3406
                  ? c == 3389
                  : c <= 3406)))))))
          : (c <= 3414 || (c < 3724
            ? (c < 3520
              ? (c < 3482
                ? (c < 3450
                  ? (c >= 3423 && c <= 3425)
                  : (c <= 3455 || (c >= 3461 && c <= 3478)))
                : (c <= 3505 || (c < 3517
                  ? (c >= 3507 && c <= 3515)
                  : c <= 3517)))
              : (c <= 3526 || (c < 3713
                ? (c < 3634
                  ? (c >= 3585 && c <= 3632)
                  : (c <= 3635 || (c >= 3648 && c <= 3654)))
                : (c <= 3714 || (c < 3718
                  ? c == 3716
                  : c <= 3722)))))
            : (c <= 3747 || (c < 3804
              ? (c < 3773
                ? (c < 3751
                  ? c == 3749
                  : (c <= 3760 || (c >= 3762 && c <= 3763)))
                : (c <= 3773 || (c < 3782
                  ? (c >= 3776 && c <= 3780)
                  : c <= 3782)))
              : (c <= 3807 || (c < 3913
                ? (c < 3904
                  ? c == 3840
                  : c <= 3911)
                : (c <= 3948 || (c < 4096
                  ? (c >= 3976 && c <= 3980)
                  : c <= 4138)))))))))
        : (c <= 4159 || (c < 4888
          ? (c < 4688
            ? (c < 4238
              ? (c < 4197
                ? (c < 4186
                  ? (c >= 4176 && c <= 4181)
                  : (c <= 4189 || c == 4193))
                : (c <= 4198 || (c < 4213
                  ? (c >= 4206 && c <= 4208)
                  : c <= 4225)))
              : (c <= 4238 || (c < 4304
                ? (c < 4295
                  ? (c >= 4256 && c <= 4293)
                  : (c <= 4295 || c == 4301))
                : (c <= 4346 || (c < 4682
                  ? (c >= 4348 && c <= 4680)
                  : c <= 4685)))))
            : (c <= 4694 || (c < 4792
              ? (c < 4746
                ? (c < 4698
                  ? c == 4696
                  : (c <= 4701 || (c >= 4704 && c <= 4744)))
                : (c <= 4749 || (c < 4786
                  ? (c >= 4752 && c <= 4784)
                  : c <= 4789)))
              : (c <= 4798 || (c < 4808
                ? (c < 4802
                  ? c == 4800
                  : c <= 4805)
                : (c <= 4822 || (c < 4882
                  ? (c >= 4824 && c <= 4880)
                  : c <= 4885)))))))
          : (c <= 4954 || (c < 6016
            ? (c < 5792
              ? (c < 5121
                ? (c < 5024
                  ? (c >= 4992 && c <= 5007)
                  : (c <= 5109 || (c >= 5112 && c <= 5117)))
                : (c <= 5740 || (c < 5761
                  ? (c >= 5743 && c <= 5759)
                  : c <= 5786)))
              : (c <= 5866 || (c < 5952
                ? (c < 5888
                  ? (c >= 5873 && c <= 5880)
                  : (c <= 5905 || (c >= 5919 && c <= 5937)))
                : (c <= 5969 || (c < 5998
                  ? (c >= 5984 && c <= 5996)
                  : c <= 6000)))))
            : (c <= 6067 || (c < 6320
              ? (c < 6272
                ? (c < 6108
                  ? c == 6103
                  : (c <= 6108 || (c >= 6176 && c <= 6264)))
                : (c <= 6276 || (c < 6314
                  ? (c >= 6279 && c <= 6312)
                  : c <= 6314)))
              : (c <= 6389 || (c < 6512
                ? (c < 6480
                  ? (c >= 6400 && c <= 6430)
                  : c <= 6509)
                : (c <= 6516 || (c < 6576
                  ? (c >= 6528 && c <= 6571)
                  : c <= 6601)))))))))))))
    : (c <= 6678 || (c < 43259
      ? (c < 8579
        ? (c < 8031
          ? (c < 7401
            ? (c < 7098
              ? (c < 6981
                ? (c < 6823
                  ? (c >= 6688 && c <= 6740)
                  : (c <= 6823 || (c >= 6917 && c <= 6963)))
                : (c <= 6988 || (c < 7086
                  ? (c >= 7043 && c <= 7072)
                  : c <= 7087)))
              : (c <= 7141 || (c < 7296
                ? (c < 7245
                  ? (c >= 7168 && c <= 7203)
                  : (c <= 7247 || (c >= 7258 && c <= 7293)))
                : (c <= 7304 || (c < 7357
                  ? (c >= 7312 && c <= 7354)
                  : c <= 7359)))))
            : (c <= 7404 || (c < 7968
              ? (c < 7424
                ? (c < 7413
                  ? (c >= 7406 && c <= 7411)
                  : (c <= 7414 || c == 7418))
                : (c <= 7615 || (c < 7960
                  ? (c >= 7680 && c <= 7957)
                  : c <= 7965)))
              : (c <= 8005 || (c < 8025
                ? (c < 8016
                  ? (c >= 8008 && c <= 8013)
                  : c <= 8023)
                : (c <= 8025 || (c < 8029
                  ? c == 8027
                  : c <= 8029)))))))
          : (c <= 8061 || (c < 8450
            ? (c < 8150
              ? (c < 8130
                ? (c < 8118
                  ? (c >= 8064 && c <= 8116)
                  : (c <= 8124 || c == 8126))
                : (c <= 8132 || (c < 8144
                  ? (c >= 8134 && c <= 8140)
                  : c <= 8147)))
              : (c <= 8155 || (c < 8305
                ? (c < 8178
                  ? (c >= 8160 && c <= 8172)
                  : (c <= 8180 || (c >= 8182 && c <= 8188)))
                : (c <= 8305 || (c < 8336
                  ? c == 8319
                  : c <= 8348)))))
            : (c <= 8450 || (c < 8488
              ? (c < 8473
                ? (c < 8458
                  ? c == 8455
                  : (c <= 8467 || c == 8469))
                : (c <= 8477 || (c < 8486
                  ? c == 8484
                  : c <= 8486)))
              : (c <= 8488 || (c < 8508
                ? (c < 8495
                  ? (c >= 8490 && c <= 8493)
                  : c <= 8505)
                : (c <= 8511 || (c < 8526
                  ? (c >= 8517 && c <= 8521)
                  : c <= 8526)))))))))
        : (c <= 8580 || (c < 12593
          ? (c < 11712
            ? (c < 11568
              ? (c < 11520
                ? (c < 11499
                  ? (c >= 11264 && c <= 11492)
                  : (c <= 11502 || (c >= 11506 && c <= 11507)))
                : (c <= 11557 || (c < 11565
                  ? c == 11559
                  : c <= 11565)))
              : (c <= 11623 || (c < 11688
                ? (c < 11648
                  ? c == 11631
                  : (c <= 11670 || (c >= 11680 && c <= 11686)))
                : (c <= 11694 || (c < 11704
                  ? (c >= 11696 && c <= 11702)
                  : c <= 11710)))))
            : (c <= 11718 || (c < 12347
              ? (c < 11823
                ? (c < 11728
                  ? (c >= 11720 && c <= 11726)
                  : (c <= 11734 || (c >= 11736 && c <= 11742)))
                : (c <= 11823 || (c < 12337
                  ? (c >= 12293 && c <= 12294)
                  : c <= 12341)))
              : (c <= 12348 || (c < 12449
                ? (c < 12445
                  ? (c >= 12353 && c <= 12438)
                  : c <= 12447)
                : (c <= 12538 || (c < 12549
                  ? (c >= 12540 && c <= 12543)
                  : c <= 12591)))))))
          : (c <= 12686 || (c < 42775
            ? (c < 42192
              ? (c < 19903
                ? (c < 12784
                  ? (c >= 12704 && c <= 12735)
                  : (c <= 12799 || c == 13312))
                : (c <= 19903 || (c < 40959
                  ? c == 19968
                  : c <= 42124)))
              : (c <= 42237 || (c < 42560
                ? (c < 42512
                  ? (c >= 42240 && c <= 42508)
                  : (c <= 42527 || (c >= 42538 && c <= 42539)))
                : (c <= 42606 || (c < 42656
                  ? (c >= 42623 && c <= 42653)
                  : c <= 42725)))))
            : (c <= 42783 || (c < 43011
              ? (c < 42963
                ? (c < 42891
                  ? (c >= 42786 && c <= 42888)
                  : (c <= 42954 || (c >= 42960 && c <= 42961)))
                : (c <= 42963 || (c < 42994
                  ? (c >= 42965 && c <= 42969)
                  : c <= 43009)))
              : (c <= 43013 || (c < 43072
                ? (c < 43020
                  ? (c >= 43015 && c <= 43018)
                  : c <= 43042)
                : (c <= 43123 || (c < 43250
                  ? (c >= 43138 && c <= 43187)
                  : c <= 43255)))))))))))
      : (c <= 43259 || (c < 65313
        ? (c < 43808
          ? (c < 43642
            ? (c < 43488
              ? (c < 43360
                ? (c < 43274
                  ? (c >= 43261 && c <= 43262)
                  : (c <= 43301 || (c >= 43312 && c <= 43334)))
                : (c <= 43388 || (c < 43471
                  ? (c >= 43396 && c <= 43442)
                  : c <= 43471)))
              : (c <= 43492 || (c < 43584
                ? (c < 43514
                  ? (c >= 43494 && c <= 43503)
                  : (c <= 43518 || (c >= 43520 && c <= 43560)))
                : (c <= 43586 || (c < 43616
                  ? (c >= 43588 && c <= 43595)
                  : c <= 43638)))))
            : (c <= 43642 || (c < 43739
              ? (c < 43705
                ? (c < 43697
                  ? (c >= 43646 && c <= 43695)
                  : (c <= 43697 || (c >= 43701 && c <= 43702)))
                : (c <= 43709 || (c < 43714
                  ? c == 43712
                  : c <= 43714)))
              : (c <= 43741 || (c < 43777
                ? (c < 43762
                  ? (c >= 43744 && c <= 43754)
                  : c <= 43764)
                : (c <= 43782 || (c < 43793
                  ? (c >= 43785 && c <= 43790)
                  : c <= 43798)))))))
          : (c <= 43814 || (c < 64287
            ? (c < 55216
              ? (c < 43888
                ? (c < 43824
                  ? (c >= 43816 && c <= 43822)
                  : (c <= 43866 || (c >= 43868 && c <= 43881)))
                : (c <= 44002 || (c < 55203
                  ? c == 44032
                  : c <= 55203)))
              : (c <= 55238 || (c < 64256
                ? (c < 63744
                  ? (c >= 55243 && c <= 55291)
                  : (c <= 64109 || (c >= 64112 && c <= 64217)))
                : (c <= 64262 || (c < 64285
                  ? (c >= 64275 && c <= 64279)
                  : c <= 64285)))))
            : (c <= 64296 || (c < 64467
              ? (c < 64320
                ? (c < 64312
                  ? (c >= 64298 && c <= 64310)
                  : (c <= 64316 || c == 64318))
                : (c <= 64321 || (c < 64326
                  ? (c >= 64323 && c <= 64324)
                  : c <= 64433)))
              : (c <= 64829 || (c < 65008
                ? (c < 64914
                  ? (c >= 64848 && c <= 64911)
                  : c <= 64967)
                : (c <= 65019 || (c < 65142
                  ? (c >= 65136 && c <= 65140)
                  : c <= 65276)))))))))
        : (c <= 65338 || (c < 66864
          ? (c < 66176
            ? (c < 65536
              ? (c < 65482
                ? (c < 65382
                  ? (c >= 65345 && c <= 65370)
                  : (c <= 65470 || (c >= 65474 && c <= 65479)))
                : (c <= 65487 || (c < 65498
                  ? (c >= 65490 && c <= 65495)
                  : c <= 65500)))
              : (c <= 65547 || (c < 65599
                ? (c < 65576
                  ? (c >= 65549 && c <= 65574)
                  : (c <= 65594 || (c >= 65596 && c <= 65597)))
                : (c <= 65613 || (c < 65664
                  ? (c >= 65616 && c <= 65629)
                  : c <= 65786)))))
            : (c <= 66204 || (c < 66464
              ? (c < 66370
                ? (c < 66304
                  ? (c >= 66208 && c <= 66256)
                  : (c <= 66335 || (c >= 66349 && c <= 66368)))
                : (c <= 66377 || (c < 66432
                  ? (c >= 66384 && c <= 66421)
                  : c <= 66461)))
              : (c <= 66499 || (c < 66736
                ? (c < 66560
                  ? (c >= 66504 && c <= 66511)
                  : c <= 66717)
                : (c <= 66771 || (c < 66816
                  ? (c >= 66776 && c <= 66811)
                  : c <= 66855)))))))
          : (c <= 66915 || (c < 67506
            ? (c < 66995
              ? (c < 66964
                ? (c < 66940
                  ? (c >= 66928 && c <= 66938)
                  : (c <= 66954 || (c >= 66956 && c <= 66962)))
                : (c <= 66965 || (c < 66979
                  ? (c >= 66967 && c <= 66977)
                  : c <= 66993)))
              : (c <= 67001 || (c < 67424
                ? (c < 67072
                  ? (c >= 67003 && c <= 67004)
                  : (c <= 67382 || (c >= 67392 && c <= 67413)))
                : (c <= 67431 || (c < 67463
                  ? (c >= 67456 && c <= 67461)
                  : c <= 67504)))))
            : (c <= 67514 || (c < 67680
              ? (c < 67639
                ? (c < 67592
                  ? (c >= 67584 && c <= 67589)
                  : (c <= 67592 || (c >= 67594 && c <= 67637)))
                : (c <= 67640 || (c < 67647
                  ? c == 67644
                  : c <= 67669)))
              : (c <= 67702 || (c < 67828
                ? (c < 67808
                  ? (c >= 67712 && c <= 67742)
                  : c <= 67826)
                : (c <= 67829 || (c < 67872
                  ? (c >= 67840 && c <= 67861)
                  : c <= 67883)))))))))))))));
}

static inline bool sym_identifier_character_set_2(int32_t c) {
  return (c < 6656
    ? (c < 2979
      ? (c < 2308
        ? (c < 1376
          ? (c < 750
            ? (c < 186
              ? (c < 'b'
                ? (c < 'A'
                  ? (c >= '0' && c <= '9')
                  : (c <= 'Z' || c == '_'))
                : (c <= 'z' || (c < 181
                  ? c == 170
                  : c <= 181)))
              : (c <= 186 || (c < 710
                ? (c < 216
                  ? (c >= 192 && c <= 214)
                  : (c <= 246 || (c >= 248 && c <= 705)))
                : (c <= 721 || (c < 748
                  ? (c >= 736 && c <= 740)
                  : c <= 748)))))
            : (c <= 750 || (c < 908
              ? (c < 895
                ? (c < 886
                  ? (c >= 880 && c <= 884)
                  : (c <= 887 || (c >= 890 && c <= 893)))
                : (c <= 895 || (c < 904
                  ? c == 902
                  : c <= 906)))
              : (c <= 908 || (c < 1162
                ? (c < 931
                  ? (c >= 910 && c <= 929)
                  : (c <= 1013 || (c >= 1015 && c <= 1153)))
                : (c <= 1327 || (c < 1369
                  ? (c >= 1329 && c <= 1366)
                  : c <= 1369)))))))
          : (c <= 1416 || (c < 1969
            ? (c < 1765
              ? (c < 1646
                ? (c < 1519
                  ? (c >= 1488 && c <= 1514)
                  : (c <= 1522 || (c >= 1568 && c <= 1610)))
                : (c <= 1647 || (c < 1749
                  ? (c >= 1649 && c <= 1747)
                  : c <= 1749)))
              : (c <= 1766 || (c < 1808
                ? (c < 1786
                  ? (c >= 1774 && c <= 1775)
                  : (c <= 1788 || c == 1791))
                : (c <= 1808 || (c < 1869
                  ? (c >= 1810 && c <= 1839)
                  : c <= 1957)))))
            : (c <= 1969 || (c < 2088
              ? (c < 2048
                ? (c < 2036
                  ? (c >= 1994 && c <= 2026)
                  : (c <= 2037 || c == 2042))
                : (c <= 2069 || (c < 2084
                  ? c == 2074
                  : c <= 2084)))
              : (c <= 2088 || (c < 2160
                ? (c < 2144
                  ? (c >= 2112 && c <= 2136)
                  : c <= 2154)
                : (c <= 2183 || (c < 2208
                  ? (c >= 2185 && c <= 2190)
                  : c <= 2249)))))))))
        : (c <= 2361 || (c < 2693
          ? (c < 2527
            ? (c < 2451
              ? (c < 2417
                ? (c < 2384
                  ? c == 2365
                  : (c <= 2384 || (c >= 2392 && c <= 2401)))
                : (c <= 2432 || (c < 2447
                  ? (c >= 2437 && c <= 2444)
                  : c <= 2448)))
              : (c <= 2472 || (c < 2493
                ? (c < 2482
                  ? (c >= 2474 && c <= 2480)
                  : (c <= 2482 || (c >= 2486 && c <= 2489)))
                : (c <= 2493 || (c < 2524
                  ? c == 2510
                  : c <= 2525)))))
            : (c <= 2529 || (c < 2610
              ? (c < 2575
                ? (c < 2556
                  ? (c >= 2544 && c <= 2545)
                  : (c <= 2556 || (c >= 2565 && c <= 2570)))
                : (c <= 2576 || (c < 2602
                  ? (c >= 2579 && c <= 2600)
                  : c <= 2608)))
              : (c <= 2611 || (c < 2649
                ? (c < 2616
                  ? (c >= 2613 && c <= 2614)
                  : c <= 2617)
                : (c <= 2652 || (c < 2674
                  ? c == 2654
                  : c <= 2676)))))))
          : (c <= 2701 || (c < 2866
            ? (c < 2768
              ? (c < 2738
                ? (c < 2707
                  ? (c >= 2703 && c <= 2705)
                  : (c <= 2728 || (c >= 2730 && c <= 2736)))
                : (c <= 2739 || (c < 2749
                  ? (c >= 2741 && c <= 2745)
                  : c <= 2749)))
              : (c <= 2768 || (c < 2831
                ? (c < 2809
                  ? (c >= 2784 && c <= 2785)
                  : (c <= 2809 || (c >= 2821 && c <= 2828)))
                : (c <= 2832 || (c < 2858
                  ? (c >= 2835 && c <= 2856)
                  : c <= 2864)))))
            : (c <= 2867 || (c < 2949
              ? (c < 2911
                ? (c < 2877
                  ? (c >= 2869 && c <= 2873)
                  : (c <= 2877 || (c >= 2908 && c <= 2909)))
                : (c <= 2913 || (c < 2947
                  ? c == 2929
                  : c <= 2947)))
              : (c <= 2954 || (c < 2969
                ? (c < 2962
                  ? (c >= 2958 && c <= 2960)
                  : c <= 2965)
                : (c <= 2970 || (c < 2974
                  ? c == 2972
                  : c <= 2975)))))))))))
      : (c <= 2980 || (c < 4159
        ? (c < 3412
          ? (c < 3214
            ? (c < 3114
              ? (c < 3077
                ? (c < 2990
                  ? (c >= 2984 && c <= 2986)
                  : (c <= 3001 || c == 3024))
                : (c <= 3084 || (c < 3090
                  ? (c >= 3086 && c <= 3088)
                  : c <= 3112)))
              : (c <= 3129 || (c < 3168
                ? (c < 3160
                  ? c == 3133
                  : (c <= 3162 || c == 3165))
                : (c <= 3169 || (c < 3205
                  ? c == 3200
                  : c <= 3212)))))
            : (c <= 3216 || (c < 3313
              ? (c < 3261
                ? (c < 3242
                  ? (c >= 3218 && c <= 3240)
                  : (c <= 3251 || (c >= 3253 && c <= 3257)))
                : (c <= 3261 || (c < 3296
                  ? (c >= 3293 && c <= 3294)
                  : c <= 3297)))
              : (c <= 3314 || (c < 3346
                ? (c < 3342
                  ? (c >= 3332 && c <= 3340)
                  : c <= 3344)
                : (c <= 3386 || (c < 3406
                  ? c == 3389
                  : c <= 3406)))))))
          : (c <= 3414 || (c < 3724
            ? (c < 3520
              ? (c < 3482
                ? (c < 3450
                  ? (c >= 3423 && c <= 3425)
                  : (c <= 3455 || (c >= 3461 && c <= 3478)))
                : (c <= 3505 || (c < 3517
                  ? (c >= 3507 && c <= 3515)
                  : c <= 3517)))
              : (c <= 3526 || (c < 3713
                ? (c < 3634
                  ? (c >= 3585 && c <= 3632)
                  : (c <= 3635 || (c >= 3648 && c <= 3654)))
                : (c <= 3714 || (c < 3718
                  ? c == 3716
                  : c <= 3722)))))
            : (c <= 3747 || (c < 3804
              ? (c < 3773
                ? (c < 3751
                  ? c == 3749
                  : (c <= 3760 || (c >= 3762 && c <= 3763)))
                : (c <= 3773 || (c < 3782
                  ? (c >= 3776 && c <= 3780)
                  : c <= 3782)))
              : (c <= 3807 || (c < 3913
                ? (c < 3904
                  ? c == 3840
                  : c <= 3911)
                : (c <= 3948 || (c < 4096
                  ? (c >= 3976 && c <= 3980)
                  : c <= 4138)))))))))
        : (c <= 4159 || (c < 4888
          ? (c < 4688
            ? (c < 4238
              ? (c < 4197
                ? (c < 4186
                  ? (c >= 4176 && c <= 4181)
                  : (c <= 4189 || c == 4193))
                : (c <= 4198 || (c < 4213
                  ? (c >= 4206 && c <= 4208)
                  : c <= 4225)))
              : (c <= 4238 || (c < 4304
                ? (c < 4295
                  ? (c >= 4256 && c <= 4293)
                  : (c <= 4295 || c == 4301))
                : (c <= 4346 || (c < 4682
                  ? (c >= 4348 && c <= 4680)
                  : c <= 4685)))))
            : (c <= 4694 || (c < 4792
              ? (c < 4746
                ? (c < 4698
                  ? c == 4696
                  : (c <= 4701 || (c >= 4704 && c <= 4744)))
                : (c <= 4749 || (c < 4786
                  ? (c >= 4752 && c <= 4784)
                  : c <= 4789)))
              : (c <= 4798 || (c < 4808
                ? (c < 4802
                  ? c == 4800
                  : c <= 4805)
                : (c <= 4822 || (c < 4882
                  ? (c >= 4824 && c <= 4880)
                  : c <= 4885)))))))
          : (c <= 4954 || (c < 6016
            ? (c < 5792
              ? (c < 5121
                ? (c < 5024
                  ? (c >= 4992 && c <= 5007)
                  : (c <= 5109 || (c >= 5112 && c <= 5117)))
                : (c <= 5740 || (c < 5761
                  ? (c >= 5743 && c <= 5759)
                  : c <= 5786)))
              : (c <= 5866 || (c < 5952
                ? (c < 5888
                  ? (c >= 5873 && c <= 5880)
                  : (c <= 5905 || (c >= 5919 && c <= 5937)))
                : (c <= 5969 || (c < 5998
                  ? (c >= 5984 && c <= 5996)
                  : c <= 6000)))))
            : (c <= 6067 || (c < 6320
              ? (c < 6272
                ? (c < 6108
                  ? c == 6103
                  : (c <= 6108 || (c >= 6176 && c <= 6264)))
                : (c <= 6276 || (c < 6314
                  ? (c >= 6279 && c <= 6312)
                  : c <= 6314)))
              : (c <= 6389 || (c < 6512
                ? (c < 6480
                  ? (c >= 6400 && c <= 6430)
                  : c <= 6509)
                : (c <= 6516 || (c < 6576
                  ? (c >= 6528 && c <= 6571)
                  : c <= 6601)))))))))))))
    : (c <= 6678 || (c < 43259
      ? (c < 8579
        ? (c < 8031
          ? (c < 7401
            ? (c < 7098
              ? (c < 6981
                ? (c < 6823
                  ? (c >= 6688 && c <= 6740)
                  : (c <= 6823 || (c >= 6917 && c <= 6963)))
                : (c <= 6988 || (c < 7086
                  ? (c >= 7043 && c <= 7072)
                  : c <= 7087)))
              : (c <= 7141 || (c < 7296
                ? (c < 7245
                  ? (c >= 7168 && c <= 7203)
                  : (c <= 7247 || (c >= 7258 && c <= 7293)))
                : (c <= 7304 || (c < 7357
                  ? (c >= 7312 && c <= 7354)
                  : c <= 7359)))))
            : (c <= 7404 || (c < 7968
              ? (c < 7424
                ? (c < 7413
                  ? (c >= 7406 && c <= 7411)
                  : (c <= 7414 || c == 7418))
                : (c <= 7615 || (c < 7960
                  ? (c >= 7680 && c <= 7957)
                  : c <= 7965)))
              : (c <= 8005 || (c < 8025
                ? (c < 8016
                  ? (c >= 8008 && c <= 8013)
                  : c <= 8023)
                : (c <= 8025 || (c < 8029
                  ? c == 8027
                  : c <= 8029)))))))
          : (c <= 8061 || (c < 8450
            ? (c < 8150
              ? (c < 8130
                ? (c < 8118
                  ? (c >= 8064 && c <= 8116)
                  : (c <= 8124 || c == 8126))
                : (c <= 8132 || (c < 8144
                  ? (c >= 8134 && c <= 8140)
                  : c <= 8147)))
              : (c <= 8155 || (c < 8305
                ? (c < 8178
                  ? (c >= 8160 && c <= 8172)
                  : (c <= 8180 || (c >= 8182 && c <= 8188)))
                : (c <= 8305 || (c < 8336
                  ? c == 8319
                  : c <= 8348)))))
            : (c <= 8450 || (c < 8488
              ? (c < 8473
                ? (c < 8458
                  ? c == 8455
                  : (c <= 8467 || c == 8469))
                : (c <= 8477 || (c < 8486
                  ? c == 8484
                  : c <= 8486)))
              : (c <= 8488 || (c < 8508
                ? (c < 8495
                  ? (c >= 8490 && c <= 8493)
                  : c <= 8505)
                : (c <= 8511 || (c < 8526
                  ? (c >= 8517 && c <= 8521)
                  : c <= 8526)))))))))
        : (c <= 8580 || (c < 12593
          ? (c < 11712
            ? (c < 11568
              ? (c < 11520
                ? (c < 11499
                  ? (c >= 11264 && c <= 11492)
                  : (c <= 11502 || (c >= 11506 && c <= 11507)))
                : (c <= 11557 || (c < 11565
                  ? c == 11559
                  : c <= 11565)))
              : (c <= 11623 || (c < 11688
                ? (c < 11648
                  ? c == 11631
                  : (c <= 11670 || (c >= 11680 && c <= 11686)))
                : (c <= 11694 || (c < 11704
                  ? (c >= 11696 && c <= 11702)
                  : c <= 11710)))))
            : (c <= 11718 || (c < 12347
              ? (c < 11823
                ? (c < 11728
                  ? (c >= 11720 && c <= 11726)
                  : (c <= 11734 || (c >= 11736 && c <= 11742)))
                : (c <= 11823 || (c < 12337
                  ? (c >= 12293 && c <= 12294)
                  : c <= 12341)))
              : (c <= 12348 || (c < 12449
                ? (c < 12445
                  ? (c >= 12353 && c <= 12438)
                  : c <= 12447)
                : (c <= 12538 || (c < 12549
                  ? (c >= 12540 && c <= 12543)
                  : c <= 12591)))))))
          : (c <= 12686 || (c < 42775
            ? (c < 42192
              ? (c < 19903
                ? (c < 12784
                  ? (c >= 12704 && c <= 12735)
                  : (c <= 12799 || c == 13312))
                : (c <= 19903 || (c < 40959
                  ? c == 19968
                  : c <= 42124)))
              : (c <= 42237 || (c < 42560
                ? (c < 42512
                  ? (c >= 42240 && c <= 42508)
                  : (c <= 42527 || (c >= 42538 && c <= 42539)))
                : (c <= 42606 || (c < 42656
                  ? (c >= 42623 && c <= 42653)
                  : c <= 42725)))))
            : (c <= 42783 || (c < 43011
              ? (c < 42963
                ? (c < 42891
                  ? (c >= 42786 && c <= 42888)
                  : (c <= 42954 || (c >= 42960 && c <= 42961)))
                : (c <= 42963 || (c < 42994
                  ? (c >= 42965 && c <= 42969)
                  : c <= 43009)))
              : (c <= 43013 || (c < 43072
                ? (c < 43020
                  ? (c >= 43015 && c <= 43018)
                  : c <= 43042)
                : (c <= 43123 || (c < 43250
                  ? (c >= 43138 && c <= 43187)
                  : c <= 43255)))))))))))
      : (c <= 43259 || (c < 65313
        ? (c < 43808
          ? (c < 43642
            ? (c < 43488
              ? (c < 43360
                ? (c < 43274
                  ? (c >= 43261 && c <= 43262)
                  : (c <= 43301 || (c >= 43312 && c <= 43334)))
                : (c <= 43388 || (c < 43471
                  ? (c >= 43396 && c <= 43442)
                  : c <= 43471)))
              : (c <= 43492 || (c < 43584
                ? (c < 43514
                  ? (c >= 43494 && c <= 43503)
                  : (c <= 43518 || (c >= 43520 && c <= 43560)))
                : (c <= 43586 || (c < 43616
                  ? (c >= 43588 && c <= 43595)
                  : c <= 43638)))))
            : (c <= 43642 || (c < 43739
              ? (c < 43705
                ? (c < 43697
                  ? (c >= 43646 && c <= 43695)
                  : (c <= 43697 || (c >= 43701 && c <= 43702)))
                : (c <= 43709 || (c < 43714
                  ? c == 43712
                  : c <= 43714)))
              : (c <= 43741 || (c < 43777
                ? (c < 43762
                  ? (c >= 43744 && c <= 43754)
                  : c <= 43764)
                : (c <= 43782 || (c < 43793
                  ? (c >= 43785 && c <= 43790)
                  : c <= 43798)))))))
          : (c <= 43814 || (c < 64287
            ? (c < 55216
              ? (c < 43888
                ? (c < 43824
                  ? (c >= 43816 && c <= 43822)
                  : (c <= 43866 || (c >= 43868 && c <= 43881)))
                : (c <= 44002 || (c < 55203
                  ? c == 44032
                  : c <= 55203)))
              : (c <= 55238 || (c < 64256
                ? (c < 63744
                  ? (c >= 55243 && c <= 55291)
                  : (c <= 64109 || (c >= 64112 && c <= 64217)))
                : (c <= 64262 || (c < 64285
                  ? (c >= 64275 && c <= 64279)
                  : c <= 64285)))))
            : (c <= 64296 || (c < 64467
              ? (c < 64320
                ? (c < 64312
                  ? (c >= 64298 && c <= 64310)
                  : (c <= 64316 || c == 64318))
                : (c <= 64321 || (c < 64326
                  ? (c >= 64323 && c <= 64324)
                  : c <= 64433)))
              : (c <= 64829 || (c < 65008
                ? (c < 64914
                  ? (c >= 64848 && c <= 64911)
                  : c <= 64967)
                : (c <= 65019 || (c < 65142
                  ? (c >= 65136 && c <= 65140)
                  : c <= 65276)))))))))
        : (c <= 65338 || (c < 66864
          ? (c < 66176
            ? (c < 65536
              ? (c < 65482
                ? (c < 65382
                  ? (c >= 65345 && c <= 65370)
                  : (c <= 65470 || (c >= 65474 && c <= 65479)))
                : (c <= 65487 || (c < 65498
                  ? (c >= 65490 && c <= 65495)
                  : c <= 65500)))
              : (c <= 65547 || (c < 65599
                ? (c < 65576
                  ? (c >= 65549 && c <= 65574)
                  : (c <= 65594 || (c >= 65596 && c <= 65597)))
                : (c <= 65613 || (c < 65664
                  ? (c >= 65616 && c <= 65629)
                  : c <= 65786)))))
            : (c <= 66204 || (c < 66464
              ? (c < 66370
                ? (c < 66304
                  ? (c >= 66208 && c <= 66256)
                  : (c <= 66335 || (c >= 66349 && c <= 66368)))
                : (c <= 66377 || (c < 66432
                  ? (c >= 66384 && c <= 66421)
                  : c <= 66461)))
              : (c <= 66499 || (c < 66736
                ? (c < 66560
                  ? (c >= 66504 && c <= 66511)
                  : c <= 66717)
                : (c <= 66771 || (c < 66816
                  ? (c >= 66776 && c <= 66811)
                  : c <= 66855)))))))
          : (c <= 66915 || (c < 67506
            ? (c < 66995
              ? (c < 66964
                ? (c < 66940
                  ? (c >= 66928 && c <= 66938)
                  : (c <= 66954 || (c >= 66956 && c <= 66962)))
                : (c <= 66965 || (c < 66979
                  ? (c >= 66967 && c <= 66977)
                  : c <= 66993)))
              : (c <= 67001 || (c < 67424
                ? (c < 67072
                  ? (c >= 67003 && c <= 67004)
                  : (c <= 67382 || (c >= 67392 && c <= 67413)))
                : (c <= 67431 || (c < 67463
                  ? (c >= 67456 && c <= 67461)
                  : c <= 67504)))))
            : (c <= 67514 || (c < 67680
              ? (c < 67639
                ? (c < 67592
                  ? (c >= 67584 && c <= 67589)
                  : (c <= 67592 || (c >= 67594 && c <= 67637)))
                : (c <= 67640 || (c < 67647
                  ? c == 67644
                  : c <= 67669)))
              : (c <= 67702 || (c < 67828
                ? (c < 67808
                  ? (c >= 67712 && c <= 67742)
                  : c <= 67826)
                : (c <= 67829 || (c < 67872
                  ? (c >= 67840 && c <= 67861)
                  : c <= 67883)))))))))))))));
}

static inline bool sym_identifier_character_set_3(int32_t c) {
  return (c < 6656
    ? (c < 2979
      ? (c < 2308
        ? (c < 1376
          ? (c < 750
            ? (c < 186
              ? (c < 'a'
                ? (c < 'A'
                  ? (c >= '0' && c <= '9')
                  : (c <= 'Z' || c == '_'))
                : (c <= 'z' || (c < 181
                  ? c == 170
                  : c <= 181)))
              : (c <= 186 || (c < 710
                ? (c < 216
                  ? (c >= 192 && c <= 214)
                  : (c <= 246 || (c >= 248 && c <= 705)))
                : (c <= 721 || (c < 748
                  ? (c >= 736 && c <= 740)
                  : c <= 748)))))
            : (c <= 750 || (c < 908
              ? (c < 895
                ? (c < 886
                  ? (c >= 880 && c <= 884)
                  : (c <= 887 || (c >= 890 && c <= 893)))
                : (c <= 895 || (c < 904
                  ? c == 902
                  : c <= 906)))
              : (c <= 908 || (c < 1162
                ? (c < 931
                  ? (c >= 910 && c <= 929)
                  : (c <= 1013 || (c >= 1015 && c <= 1153)))
                : (c <= 1327 || (c < 1369
                  ? (c >= 1329 && c <= 1366)
                  : c <= 1369)))))))
          : (c <= 1416 || (c < 1969
            ? (c < 1765
              ? (c < 1646
                ? (c < 1519
                  ? (c >= 1488 && c <= 1514)
                  : (c <= 1522 || (c >= 1568 && c <= 1610)))
                : (c <= 1647 || (c < 1749
                  ? (c >= 1649 && c <= 1747)
                  : c <= 1749)))
              : (c <= 1766 || (c < 1808
                ? (c < 1786
                  ? (c >= 1774 && c <= 1775)
                  : (c <= 1788 || c == 1791))
                : (c <= 1808 || (c < 1869
                  ? (c >= 1810 && c <= 1839)
                  : c <= 1957)))))
            : (c <= 1969 || (c < 2088
              ? (c < 2048
                ? (c < 2036
                  ? (c >= 1994 && c <= 2026)
                  : (c <= 2037 || c == 2042))
                : (c <= 2069 || (c < 2084
                  ? c == 2074
                  : c <= 2084)))
              : (c <= 2088 || (c < 2160
                ? (c < 2144
                  ? (c >= 2112 && c <= 2136)
                  : c <= 2154)
                : (c <= 2183 || (c < 2208
                  ? (c >= 2185 && c <= 2190)
                  : c <= 2249)))))))))
        : (c <= 2361 || (c < 2693
          ? (c < 2527
            ? (c < 2451
              ? (c < 2417
                ? (c < 2384
                  ? c == 2365
                  : (c <= 2384 || (c >= 2392 && c <= 2401)))
                : (c <= 2432 || (c < 2447
                  ? (c >= 2437 && c <= 2444)
                  : c <= 2448)))
              : (c <= 2472 || (c < 2493
                ? (c < 2482
                  ? (c >= 2474 && c <= 2480)
                  : (c <= 2482 || (c >= 2486 && c <= 2489)))
                : (c <= 2493 || (c < 2524
                  ? c == 2510
                  : c <= 2525)))))
            : (c <= 2529 || (c < 2610
              ? (c < 2575
                ? (c < 2556
                  ? (c >= 2544 && c <= 2545)
                  : (c <= 2556 || (c >= 2565 && c <= 2570)))
                : (c <= 2576 || (c < 2602
                  ? (c >= 2579 && c <= 2600)
                  : c <= 2608)))
              : (c <= 2611 || (c < 2649
                ? (c < 2616
                  ? (c >= 2613 && c <= 2614)
                  : c <= 2617)
                : (c <= 2652 || (c < 2674
                  ? c == 2654
                  : c <= 2676)))))))
          : (c <= 2701 || (c < 2866
            ? (c < 2768
              ? (c < 2738
                ? (c < 2707
                  ? (c >= 2703 && c <= 2705)
                  : (c <= 2728 || (c >= 2730 && c <= 2736)))
                : (c <= 2739 || (c < 2749
                  ? (c >= 2741 && c <= 2745)
                  : c <= 2749)))
              : (c <= 2768 || (c < 2831
                ? (c < 2809
                  ? (c >= 2784 && c <= 2785)
                  : (c <= 2809 || (c >= 2821 && c <= 2828)))
                : (c <= 2832 || (c < 2858
                  ? (c >= 2835 && c <= 2856)
                  : c <= 2864)))))
            : (c <= 2867 || (c < 2949
              ? (c < 2911
                ? (c < 2877
                  ? (c >= 2869 && c <= 2873)
                  : (c <= 2877 || (c >= 2908 && c <= 2909)))
                : (c <= 2913 || (c < 2947
                  ? c == 2929
                  : c <= 2947)))
              : (c <= 2954 || (c < 2969
                ? (c < 2962
                  ? (c >= 2958 && c <= 2960)
                  : c <= 2965)
                : (c <= 2970 || (c < 2974
                  ? c == 2972
                  : c <= 2975)))))))))))
      : (c <= 2980 || (c < 4159
        ? (c < 3412
          ? (c < 3214
            ? (c < 3114
              ? (c < 3077
                ? (c < 2990
                  ? (c >= 2984 && c <= 2986)
                  : (c <= 3001 || c == 3024))
                : (c <= 3084 || (c < 3090
                  ? (c >= 3086 && c <= 3088)
                  : c <= 3112)))
              : (c <= 3129 || (c < 3168
                ? (c < 3160
                  ? c == 3133
                  : (c <= 3162 || c == 3165))
                : (c <= 3169 || (c < 3205
                  ? c == 3200
                  : c <= 3212)))))
            : (c <= 3216 || (c < 3313
              ? (c < 3261
                ? (c < 3242
                  ? (c >= 3218 && c <= 3240)
                  : (c <= 3251 || (c >= 3253 && c <= 3257)))
                : (c <= 3261 || (c < 3296
                  ? (c >= 3293 && c <= 3294)
                  : c <= 3297)))
              : (c <= 3314 || (c < 3346
                ? (c < 3342
                  ? (c >= 3332 && c <= 3340)
                  : c <= 3344)
                : (c <= 3386 || (c < 3406
                  ? c == 3389
                  : c <= 3406)))))))
          : (c <= 3414 || (c < 3724
            ? (c < 3520
              ? (c < 3482
                ? (c < 3450
                  ? (c >= 3423 && c <= 3425)
                  : (c <= 3455 || (c >= 3461 && c <= 3478)))
                : (c <= 3505 || (c < 3517
                  ? (c >= 3507 && c <= 3515)
                  : c <= 3517)))
              : (c <= 3526 || (c < 3713
                ? (c < 3634
                  ? (c >= 3585 && c <= 3632)
                  : (c <= 3635 || (c >= 3648 && c <= 3654)))
                : (c <= 3714 || (c < 3718
                  ? c == 3716
                  : c <= 3722)))))
            : (c <= 3747 || (c < 3804
              ? (c < 3773
                ? (c < 3751
                  ? c == 3749
                  : (c <= 3760 || (c >= 3762 && c <= 3763)))
                : (c <= 3773 || (c < 3782
                  ? (c >= 3776 && c <= 3780)
                  : c <= 3782)))
              : (c <= 3807 || (c < 3913
                ? (c < 3904
                  ? c == 3840
                  : c <= 3911)
                : (c <= 3948 || (c < 4096
                  ? (c >= 3976 && c <= 3980)
                  : c <= 4138)))))))))
        : (c <= 4159 || (c < 4888
          ? (c < 4688
            ? (c < 4238
              ? (c < 4197
                ? (c < 4186
                  ? (c >= 4176 && c <= 4181)
                  : (c <= 4189 || c == 4193))
                : (c <= 4198 || (c < 4213
                  ? (c >= 4206 && c <= 4208)
                  : c <= 4225)))
              : (c <= 4238 || (c < 4304
                ? (c < 4295
                  ? (c >= 4256 && c <= 4293)
                  : (c <= 4295 || c == 4301))
                : (c <= 4346 || (c < 4682
                  ? (c >= 4348 && c <= 4680)
                  : c <= 4685)))))
            : (c <= 4694 || (c < 4792
              ? (c < 4746
                ? (c < 4698
                  ? c == 4696
                  : (c <= 4701 || (c >= 4704 && c <= 4744)))
                : (c <= 4749 || (c < 4786
                  ? (c >= 4752 && c <= 4784)
                  : c <= 4789)))
              : (c <= 4798 || (c < 4808
                ? (c < 4802
                  ? c == 4800
                  : c <= 4805)
                : (c <= 4822 || (c < 4882
                  ? (c >= 4824 && c <= 4880)
                  : c <= 4885)))))))
          : (c <= 4954 || (c < 6016
            ? (c < 5792
              ? (c < 5121
                ? (c < 5024
                  ? (c >= 4992 && c <= 5007)
                  : (c <= 5109 || (c >= 5112 && c <= 5117)))
                : (c <= 5740 || (c < 5761
                  ? (c >= 5743 && c <= 5759)
                  : c <= 5786)))
              : (c <= 5866 || (c < 5952
                ? (c < 5888
                  ? (c >= 5873 && c <= 5880)
                  : (c <= 5905 || (c >= 5919 && c <= 5937)))
                : (c <= 5969 || (c < 5998
                  ? (c >= 5984 && c <= 5996)
                  : c <= 6000)))))
            : (c <= 6067 || (c < 6320
              ? (c < 6272
                ? (c < 6108
                  ? c == 6103
                  : (c <= 6108 || (c >= 6176 && c <= 6264)))
                : (c <= 6276 || (c < 6314
                  ? (c >= 6279 && c <= 6312)
                  : c <= 6314)))
              : (c <= 6389 || (c < 6512
                ? (c < 6480
                  ? (c >= 6400 && c <= 6430)
                  : c <= 6509)
                : (c <= 6516 || (c < 6576
                  ? (c >= 6528 && c <= 6571)
                  : c <= 6601)))))))))))))
    : (c <= 6678 || (c < 43259
      ? (c < 8579
        ? (c < 8031
          ? (c < 7401
            ? (c < 7098
              ? (c < 6981
                ? (c < 6823
                  ? (c >= 6688 && c <= 6740)
                  : (c <= 6823 || (c >= 6917 && c <= 6963)))
                : (c <= 6988 || (c < 7086
                  ? (c >= 7043 && c <= 7072)
                  : c <= 7087)))
              : (c <= 7141 || (c < 7296
                ? (c < 7245
                  ? (c >= 7168 && c <= 7203)
                  : (c <= 7247 || (c >= 7258 && c <= 7293)))
                : (c <= 7304 || (c < 7357
                  ? (c >= 7312 && c <= 7354)
                  : c <= 7359)))))
            : (c <= 7404 || (c < 7968
              ? (c < 7424
                ? (c < 7413
                  ? (c >= 7406 && c <= 7411)
                  : (c <= 7414 || c == 7418))
                : (c <= 7615 || (c < 7960
                  ? (c >= 7680 && c <= 7957)
                  : c <= 7965)))
              : (c <= 8005 || (c < 8025
                ? (c < 8016
                  ? (c >= 8008 && c <= 8013)
                  : c <= 8023)
                : (c <= 8025 || (c < 8029
                  ? c == 8027
                  : c <= 8029)))))))
          : (c <= 8061 || (c < 8450
            ? (c < 8150
              ? (c < 8130
                ? (c < 8118
                  ? (c >= 8064 && c <= 8116)
                  : (c <= 8124 || c == 8126))
                : (c <= 8132 || (c < 8144
                  ? (c >= 8134 && c <= 8140)
                  : c <= 8147)))
              : (c <= 8155 || (c < 8305
                ? (c < 8178
                  ? (c >= 8160 && c <= 8172)
                  : (c <= 8180 || (c >= 8182 && c <= 8188)))
                : (c <= 8305 || (c < 8336
                  ? c == 8319
                  : c <= 8348)))))
            : (c <= 8450 || (c < 8488
              ? (c < 8473
                ? (c < 8458
                  ? c == 8455
                  : (c <= 8467 || c == 8469))
                : (c <= 8477 || (c < 8486
                  ? c == 8484
                  : c <= 8486)))
              : (c <= 8488 || (c < 8508
                ? (c < 8495
                  ? (c >= 8490 && c <= 8493)
                  : c <= 8505)
                : (c <= 8511 || (c < 8526
                  ? (c >= 8517 && c <= 8521)
                  : c <= 8526)))))))))
        : (c <= 8580 || (c < 12593
          ? (c < 11712
            ? (c < 11568
              ? (c < 11520
                ? (c < 11499
                  ? (c >= 11264 && c <= 11492)
                  : (c <= 11502 || (c >= 11506 && c <= 11507)))
                : (c <= 11557 || (c < 11565
                  ? c == 11559
                  : c <= 11565)))
              : (c <= 11623 || (c < 11688
                ? (c < 11648
                  ? c == 11631
                  : (c <= 11670 || (c >= 11680 && c <= 11686)))
                : (c <= 11694 || (c < 11704
                  ? (c >= 11696 && c <= 11702)
                  : c <= 11710)))))
            : (c <= 11718 || (c < 12347
              ? (c < 11823
                ? (c < 11728
                  ? (c >= 11720 && c <= 11726)
                  : (c <= 11734 || (c >= 11736 && c <= 11742)))
                : (c <= 11823 || (c < 12337
                  ? (c >= 12293 && c <= 12294)
                  : c <= 12341)))
              : (c <= 12348 || (c < 12449
                ? (c < 12445
                  ? (c >= 12353 && c <= 12438)
                  : c <= 12447)
                : (c <= 12538 || (c < 12549
                  ? (c >= 12540 && c <= 12543)
                  : c <= 12591)))))))
          : (c <= 12686 || (c < 42775
            ? (c < 42192
              ? (c < 19903
                ? (c < 12784
                  ? (c >= 12704 && c <= 12735)
                  : (c <= 12799 || c == 13312))
                : (c <= 19903 || (c < 40959
                  ? c == 19968
                  : c <= 42124)))
              : (c <= 42237 || (c < 42560
                ? (c < 42512
                  ? (c >= 42240 && c <= 42508)
                  : (c <= 42527 || (c >= 42538 && c <= 42539)))
                : (c <= 42606 || (c < 42656
                  ? (c >= 42623 && c <= 42653)
                  : c <= 42725)))))
            : (c <= 42783 || (c < 43011
              ? (c < 42963
                ? (c < 42891
                  ? (c >= 42786 && c <= 42888)
                  : (c <= 42954 || (c >= 42960 && c <= 42961)))
                : (c <= 42963 || (c < 42994
                  ? (c >= 42965 && c <= 42969)
                  : c <= 43009)))
              : (c <= 43013 || (c < 43072
                ? (c < 43020
                  ? (c >= 43015 && c <= 43018)
                  : c <= 43042)
                : (c <= 43123 || (c < 43250
                  ? (c >= 43138 && c <= 43187)
                  : c <= 43255)))))))))))
      : (c <= 43259 || (c < 65313
        ? (c < 43808
          ? (c < 43642
            ? (c < 43488
              ? (c < 43360
                ? (c < 43274
                  ? (c >= 43261 && c <= 43262)
                  : (c <= 43301 || (c >= 43312 && c <= 43334)))
                : (c <= 43388 || (c < 43471
                  ? (c >= 43396 && c <= 43442)
                  : c <= 43471)))
              : (c <= 43492 || (c < 43584
                ? (c < 43514
                  ? (c >= 43494 && c <= 43503)
                  : (c <= 43518 || (c >= 43520 && c <= 43560)))
                : (c <= 43586 || (c < 43616
                  ? (c >= 43588 && c <= 43595)
                  : c <= 43638)))))
            : (c <= 43642 || (c < 43739
              ? (c < 43705
                ? (c < 43697
                  ? (c >= 43646 && c <= 43695)
                  : (c <= 43697 || (c >= 43701 && c <= 43702)))
                : (c <= 43709 || (c < 43714
                  ? c == 43712
                  : c <= 43714)))
              : (c <= 43741 || (c < 43777
                ? (c < 43762
                  ? (c >= 43744 && c <= 43754)
                  : c <= 43764)
                : (c <= 43782 || (c < 43793
                  ? (c >= 43785 && c <= 43790)
                  : c <= 43798)))))))
          : (c <= 43814 || (c < 64287
            ? (c < 55216
              ? (c < 43888
                ? (c < 43824
                  ? (c >= 43816 && c <= 43822)
                  : (c <= 43866 || (c >= 43868 && c <= 43881)))
                : (c <= 44002 || (c < 55203
                  ? c == 44032
                  : c <= 55203)))
              : (c <= 55238 || (c < 64256
                ? (c < 63744
                  ? (c >= 55243 && c <= 55291)
                  : (c <= 64109 || (c >= 64112 && c <= 64217)))
                : (c <= 64262 || (c < 64285
                  ? (c >= 64275 && c <= 64279)
                  : c <= 64285)))))
            : (c <= 64296 || (c < 64467
              ? (c < 64320
                ? (c < 64312
                  ? (c >= 64298 && c <= 64310)
                  : (c <= 64316 || c == 64318))
                : (c <= 64321 || (c < 64326
                  ? (c >= 64323 && c <= 64324)
                  : c <= 64433)))
              : (c <= 64829 || (c < 65008
                ? (c < 64914
                  ? (c >= 64848 && c <= 64911)
                  : c <= 64967)
                : (c <= 65019 || (c < 65142
                  ? (c >= 65136 && c <= 65140)
                  : c <= 65276)))))))))
        : (c <= 65338 || (c < 66864
          ? (c < 66176
            ? (c < 65536
              ? (c < 65482
                ? (c < 65382
                  ? (c >= 65345 && c <= 65370)
                  : (c <= 65470 || (c >= 65474 && c <= 65479)))
                : (c <= 65487 || (c < 65498
                  ? (c >= 65490 && c <= 65495)
                  : c <= 65500)))
              : (c <= 65547 || (c < 65599
                ? (c < 65576
                  ? (c >= 65549 && c <= 65574)
                  : (c <= 65594 || (c >= 65596 && c <= 65597)))
                : (c <= 65613 || (c < 65664
                  ? (c >= 65616 && c <= 65629)
                  : c <= 65786)))))
            : (c <= 66204 || (c < 66464
              ? (c < 66370
                ? (c < 66304
                  ? (c >= 66208 && c <= 66256)
                  : (c <= 66335 || (c >= 66349 && c <= 66368)))
                : (c <= 66377 || (c < 66432
                  ? (c >= 66384 && c <= 66421)
                  : c <= 66461)))
              : (c <= 66499 || (c < 66736
                ? (c < 66560
                  ? (c >= 66504 && c <= 66511)
                  : c <= 66717)
                : (c <= 66771 || (c < 66816
                  ? (c >= 66776 && c <= 66811)
                  : c <= 66855)))))))
          : (c <= 66915 || (c < 67506
            ? (c < 66995
              ? (c < 66964
                ? (c < 66940
                  ? (c >= 66928 && c <= 66938)
                  : (c <= 66954 || (c >= 66956 && c <= 66962)))
                : (c <= 66965 || (c < 66979
                  ? (c >= 66967 && c <= 66977)
                  : c <= 66993)))
              : (c <= 67001 || (c < 67424
                ? (c < 67072
                  ? (c >= 67003 && c <= 67004)
                  : (c <= 67382 || (c >= 67392 && c <= 67413)))
                : (c <= 67431 || (c < 67463
                  ? (c >= 67456 && c <= 67461)
                  : c <= 67504)))))
            : (c <= 67514 || (c < 67680
              ? (c < 67639
                ? (c < 67592
                  ? (c >= 67584 && c <= 67589)
                  : (c <= 67592 || (c >= 67594 && c <= 67637)))
                : (c <= 67640 || (c < 67647
                  ? c == 67644
                  : c <= 67669)))
              : (c <= 67702 || (c < 67828
                ? (c < 67808
                  ? (c >= 67712 && c <= 67742)
                  : c <= 67826)
                : (c <= 67829 || (c < 67872
                  ? (c >= 67840 && c <= 67861)
                  : c <= 67883)))))))))))))));
}

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(66);
      if (lookahead == '!') ADVANCE(105);
      if (lookahead == '%') ADVANCE(116);
      if (lookahead == '&') ADVANCE(107);
      if (lookahead == '\'') ADVANCE(95);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == ')') ADVANCE(118);
      if (lookahead == '*') ADVANCE(103);
      if (lookahead == '+') ADVANCE(100);
      if (lookahead == ',') ADVANCE(69);
      if (lookahead == '-') ADVANCE(102);
      if (lookahead == '/') SKIP(63)
      if (lookahead == ':') ADVANCE(68);
      if (lookahead == ';') ADVANCE(134);
      if (lookahead == '<') ADVANCE(111);
      if (lookahead == '=') ADVANCE(126);
      if (lookahead == '>') ADVANCE(113);
      if (lookahead == '[') ADVANCE(119);
      if (lookahead == ']') ADVANCE(120);
      if (lookahead == '^') ADVANCE(108);
      if (lookahead == 'e') ADVANCE(51);
      if (lookahead == 'f') ADVANCE(55);
      if (lookahead == 'g') ADVANCE(44);
      if (lookahead == 'i') ADVANCE(49);
      if (lookahead == 'm') ADVANCE(56);
      if (lookahead == 'r') ADVANCE(45);
      if (lookahead == 's') ADVANCE(59);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(106);
      if (lookahead == '}') ADVANCE(122);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(93);
      END_STATE();
    case 1:
      if (lookahead == '\n') SKIP(10)
      if (lookahead != 0) SKIP(1)
      END_STATE();
    case 2:
      if (lookahead == '\n') SKIP(11)
      if (lookahead != 0) SKIP(2)
      END_STATE();
    case 3:
      if (lookahead == '\n') SKIP(12)
      if (lookahead != 0) SKIP(3)
      END_STATE();
    case 4:
      if (lookahead == '\n') SKIP(9)
      if (lookahead != 0) SKIP(4)
      END_STATE();
    case 5:
      if (lookahead == '\n') SKIP(8)
      if (lookahead != 0) SKIP(5)
      END_STATE();
    case 6:
      if (lookahead == '\n') SKIP(13)
      if (lookahead != 0) SKIP(6)
      END_STATE();
    case 7:
      if (lookahead == '\n') SKIP(38)
      if (lookahead != 0) SKIP(7)
      END_STATE();
    case 8:
      if (lookahead == '!') ADVANCE(104);
      if (lookahead == '&') ADVANCE(107);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == ')') ADVANCE(118);
      if (lookahead == '*') ADVANCE(103);
      if (lookahead == '+') ADVANCE(100);
      if (lookahead == '-') ADVANCE(101);
      if (lookahead == '/') SKIP(35)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '[') ADVANCE(119);
      if (lookahead == '^') ADVANCE(108);
      if (lookahead == '|') ADVANCE(106);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(8)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(93);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(92);
      END_STATE();
    case 9:
      if (lookahead == '!') ADVANCE(104);
      if (lookahead == '&') ADVANCE(107);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == '*') ADVANCE(103);
      if (lookahead == '+') ADVANCE(100);
      if (lookahead == '-') ADVANCE(102);
      if (lookahead == '/') SKIP(34)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '^') ADVANCE(108);
      if (lookahead == 'g') ADVANCE(74);
      if (lookahead == 's') ADVANCE(89);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(106);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(9)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(93);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(92);
      END_STATE();
    case 10:
      if (lookahead == '!') ADVANCE(104);
      if (lookahead == '&') ADVANCE(107);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == '*') ADVANCE(103);
      if (lookahead == '+') ADVANCE(100);
      if (lookahead == '-') ADVANCE(101);
      if (lookahead == '/') SKIP(17)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '^') ADVANCE(108);
      if (lookahead == 'f') ADVANCE(86);
      if (lookahead == 'g') ADVANCE(74);
      if (lookahead == 'i') ADVANCE(78);
      if (lookahead == 'r') ADVANCE(75);
      if (lookahead == 's') ADVANCE(89);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(106);
      if (lookahead == '}') ADVANCE(122);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(10)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(93);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(92);
      END_STATE();
    case 11:
      if (lookahead == '!') ADVANCE(104);
      if (lookahead == '&') ADVANCE(107);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == '*') ADVANCE(103);
      if (lookahead == '+') ADVANCE(100);
      if (lookahead == '-') ADVANCE(101);
      if (lookahead == '/') SKIP(32)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '^') ADVANCE(108);
      if (lookahead == 'g') ADVANCE(74);
      if (lookahead == 'i') ADVANCE(84);
      if (lookahead == 'r') ADVANCE(75);
      if (lookahead == 's') ADVANCE(89);
      if (lookahead == '|') ADVANCE(106);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(11)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(93);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(92);
      END_STATE();
    case 12:
      if (lookahead == '!') ADVANCE(104);
      if (lookahead == '&') ADVANCE(107);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == '*') ADVANCE(103);
      if (lookahead == '+') ADVANCE(100);
      if (lookahead == '-') ADVANCE(101);
      if (lookahead == '/') SKIP(33)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '^') ADVANCE(108);
      if (lookahead == 'g') ADVANCE(74);
      if (lookahead == 'r') ADVANCE(75);
      if (lookahead == 's') ADVANCE(89);
      if (lookahead == '|') ADVANCE(106);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(12)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(93);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(92);
      END_STATE();
    case 13:
      if (lookahead == '!') ADVANCE(104);
      if (lookahead == '&') ADVANCE(107);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == '*') ADVANCE(103);
      if (lookahead == '+') ADVANCE(100);
      if (lookahead == '-') ADVANCE(101);
      if (lookahead == '/') SKIP(36)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '^') ADVANCE(108);
      if (lookahead == 'e') ADVANCE(83);
      if (lookahead == 'f') ADVANCE(86);
      if (lookahead == 'g') ADVANCE(74);
      if (lookahead == 'i') ADVANCE(78);
      if (lookahead == 'r') ADVANCE(75);
      if (lookahead == 's') ADVANCE(89);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(106);
      if (lookahead == '}') ADVANCE(122);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(13)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(93);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(92);
      END_STATE();
    case 14:
      if (lookahead == '!') ADVANCE(40);
      if (lookahead == '%') ADVANCE(116);
      if (lookahead == '&') ADVANCE(107);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == ')') ADVANCE(118);
      if (lookahead == '*') ADVANCE(103);
      if (lookahead == '+') ADVANCE(100);
      if (lookahead == ',') ADVANCE(69);
      if (lookahead == '-') ADVANCE(102);
      if (lookahead == '/') ADVANCE(115);
      if (lookahead == ':') ADVANCE(68);
      if (lookahead == ';') ADVANCE(134);
      if (lookahead == '<') ADVANCE(111);
      if (lookahead == '=') ADVANCE(126);
      if (lookahead == '>') ADVANCE(113);
      if (lookahead == '[') ADVANCE(119);
      if (lookahead == ']') ADVANCE(120);
      if (lookahead == '^') ADVANCE(108);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(106);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(14)
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(92);
      END_STATE();
    case 15:
      if (lookahead == '!') ADVANCE(40);
      if (lookahead == '%') ADVANCE(116);
      if (lookahead == '&') ADVANCE(107);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == ')') ADVANCE(118);
      if (lookahead == '*') ADVANCE(103);
      if (lookahead == '+') ADVANCE(100);
      if (lookahead == ',') ADVANCE(69);
      if (lookahead == '-') ADVANCE(102);
      if (lookahead == '/') ADVANCE(115);
      if (lookahead == ':') ADVANCE(67);
      if (lookahead == ';') ADVANCE(134);
      if (lookahead == '<') ADVANCE(111);
      if (lookahead == '=') ADVANCE(126);
      if (lookahead == '>') ADVANCE(113);
      if (lookahead == '[') ADVANCE(119);
      if (lookahead == ']') ADVANCE(120);
      if (lookahead == '^') ADVANCE(108);
      if (lookahead == 'i') ADVANCE(53);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(106);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(15)
      END_STATE();
    case 16:
      if (lookahead == '!') ADVANCE(40);
      if (lookahead == '%') ADVANCE(116);
      if (lookahead == '&') ADVANCE(107);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == '*') ADVANCE(103);
      if (lookahead == '+') ADVANCE(100);
      if (lookahead == '-') ADVANCE(101);
      if (lookahead == '/') ADVANCE(115);
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '<') ADVANCE(111);
      if (lookahead == '=') ADVANCE(41);
      if (lookahead == '>') ADVANCE(113);
      if (lookahead == '[') ADVANCE(119);
      if (lookahead == '^') ADVANCE(108);
      if (lookahead == 'i') ADVANCE(53);
      if (lookahead == '|') ADVANCE(106);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(16)
      END_STATE();
    case 17:
      if (lookahead == '*') SKIP(19)
      if (lookahead == '/') SKIP(1)
      END_STATE();
    case 18:
      if (lookahead == '*') SKIP(18)
      if (lookahead == '/') SKIP(10)
      if (lookahead != 0) SKIP(19)
      END_STATE();
    case 19:
      if (lookahead == '*') SKIP(18)
      if (lookahead != 0) SKIP(19)
      END_STATE();
    case 20:
      if (lookahead == '*') SKIP(20)
      if (lookahead == '/') SKIP(11)
      if (lookahead != 0) SKIP(21)
      END_STATE();
    case 21:
      if (lookahead == '*') SKIP(20)
      if (lookahead != 0) SKIP(21)
      END_STATE();
    case 22:
      if (lookahead == '*') SKIP(22)
      if (lookahead == '/') SKIP(12)
      if (lookahead != 0) SKIP(23)
      END_STATE();
    case 23:
      if (lookahead == '*') SKIP(22)
      if (lookahead != 0) SKIP(23)
      END_STATE();
    case 24:
      if (lookahead == '*') SKIP(24)
      if (lookahead == '/') SKIP(9)
      if (lookahead != 0) SKIP(25)
      END_STATE();
    case 25:
      if (lookahead == '*') SKIP(24)
      if (lookahead != 0) SKIP(25)
      END_STATE();
    case 26:
      if (lookahead == '*') SKIP(26)
      if (lookahead == '/') SKIP(8)
      if (lookahead != 0) SKIP(27)
      END_STATE();
    case 27:
      if (lookahead == '*') SKIP(26)
      if (lookahead != 0) SKIP(27)
      END_STATE();
    case 28:
      if (lookahead == '*') SKIP(28)
      if (lookahead == '/') SKIP(13)
      if (lookahead != 0) SKIP(29)
      END_STATE();
    case 29:
      if (lookahead == '*') SKIP(28)
      if (lookahead != 0) SKIP(29)
      END_STATE();
    case 30:
      if (lookahead == '*') SKIP(30)
      if (lookahead == '/') SKIP(38)
      if (lookahead != 0) SKIP(31)
      END_STATE();
    case 31:
      if (lookahead == '*') SKIP(30)
      if (lookahead != 0) SKIP(31)
      END_STATE();
    case 32:
      if (lookahead == '*') SKIP(21)
      if (lookahead == '/') SKIP(2)
      END_STATE();
    case 33:
      if (lookahead == '*') SKIP(23)
      if (lookahead == '/') SKIP(3)
      END_STATE();
    case 34:
      if (lookahead == '*') SKIP(25)
      if (lookahead == '/') SKIP(4)
      END_STATE();
    case 35:
      if (lookahead == '*') SKIP(27)
      if (lookahead == '/') SKIP(5)
      END_STATE();
    case 36:
      if (lookahead == '*') SKIP(29)
      if (lookahead == '/') SKIP(6)
      END_STATE();
    case 37:
      if (lookahead == '*') SKIP(31)
      if (lookahead == '/') SKIP(7)
      END_STATE();
    case 38:
      if (lookahead == '/') SKIP(37)
      if (lookahead == ':') ADVANCE(67);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(38)
      END_STATE();
    case 39:
      if (lookahead == ':') ADVANCE(94);
      END_STATE();
    case 40:
      if (lookahead == '=') ADVANCE(110);
      END_STATE();
    case 41:
      if (lookahead == '=') ADVANCE(109);
      END_STATE();
    case 42:
      if (lookahead == 'a') ADVANCE(60);
      END_STATE();
    case 43:
      if (lookahead == 'd') ADVANCE(61);
      END_STATE();
    case 44:
      if (lookahead == 'e') ADVANCE(54);
      END_STATE();
    case 45:
      if (lookahead == 'e') ADVANCE(50);
      END_STATE();
    case 46:
      if (lookahead == 'e') ADVANCE(129);
      END_STATE();
    case 47:
      if (lookahead == 'e') ADVANCE(96);
      END_STATE();
    case 48:
      if (lookahead == 'e') ADVANCE(71);
      END_STATE();
    case 49:
      if (lookahead == 'f') ADVANCE(127);
      if (lookahead == 'n') ADVANCE(133);
      END_STATE();
    case 50:
      if (lookahead == 'g') ADVANCE(123);
      END_STATE();
    case 51:
      if (lookahead == 'l') ADVANCE(58);
      END_STATE();
    case 52:
      if (lookahead == 'l') ADVANCE(48);
      END_STATE();
    case 53:
      if (lookahead == 'n') ADVANCE(133);
      END_STATE();
    case 54:
      if (lookahead == 'n') ADVANCE(98);
      END_STATE();
    case 55:
      if (lookahead == 'o') ADVANCE(57);
      END_STATE();
    case 56:
      if (lookahead == 'o') ADVANCE(43);
      END_STATE();
    case 57:
      if (lookahead == 'r') ADVANCE(131);
      END_STATE();
    case 58:
      if (lookahead == 's') ADVANCE(46);
      END_STATE();
    case 59:
      if (lookahead == 't') ADVANCE(42);
      END_STATE();
    case 60:
      if (lookahead == 't') ADVANCE(47);
      END_STATE();
    case 61:
      if (lookahead == 'u') ADVANCE(52);
      END_STATE();
    case 62:
      if (eof) ADVANCE(66);
      if (lookahead == '\n') SKIP(0)
      if (lookahead != 0) SKIP(62)
      END_STATE();
    case 63:
      if (eof) ADVANCE(66);
      if (lookahead == '*') SKIP(65)
      if (lookahead == '/') SKIP(62)
      END_STATE();
    case 64:
      if (eof) ADVANCE(66);
      if (lookahead == '*') SKIP(64)
      if (lookahead == '/') SKIP(0)
      if (lookahead != 0) SKIP(65)
      END_STATE();
    case 65:
      if (eof) ADVANCE(66);
      if (lookahead == '*') SKIP(64)
      if (lookahead != 0) SKIP(65)
      END_STATE();
    case 66:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(anon_sym_COLON);
      if (lookahead == ':') ADVANCE(94);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(anon_sym_module);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(82);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(92);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(91);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(92);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(85);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(79);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(97);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(130);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'f') ADVANCE(128);
      if (lookahead == 'n') ADVANCE(80);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'g') ADVANCE(124);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(90);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(72);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(125);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(88);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(80);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(99);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(87);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(132);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(77);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(73);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(81);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(76);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(93);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(anon_sym_COLON_COLON);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(anon_sym_state);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(anon_sym_state);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 98:
      ACCEPT_TOKEN(anon_sym_gen);
      END_STATE();
    case 99:
      ACCEPT_TOKEN(anon_sym_gen);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 101:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '>') ADVANCE(70);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 104:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(anon_sym_BANG);
      if (lookahead == '=') ADVANCE(110);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 107:
      ACCEPT_TOKEN(anon_sym_AMP);
      END_STATE();
    case 108:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 109:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 110:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 111:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(112);
      END_STATE();
    case 112:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 113:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(114);
      END_STATE();
    case 114:
      ACCEPT_TOKEN(anon_sym_GT_EQ);
      END_STATE();
    case 115:
      ACCEPT_TOKEN(anon_sym_SLASH);
      END_STATE();
    case 116:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 117:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 118:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 119:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 120:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 121:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 122:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 123:
      ACCEPT_TOKEN(anon_sym_reg);
      END_STATE();
    case 124:
      ACCEPT_TOKEN(anon_sym_reg);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 125:
      ACCEPT_TOKEN(anon_sym_initial);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 126:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead == '=') ADVANCE(109);
      END_STATE();
    case 127:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 128:
      ACCEPT_TOKEN(anon_sym_if);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 129:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 130:
      ACCEPT_TOKEN(anon_sym_else);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 131:
      ACCEPT_TOKEN(anon_sym_for);
      END_STATE();
    case 132:
      ACCEPT_TOKEN(anon_sym_for);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 133:
      ACCEPT_TOKEN(anon_sym_in);
      END_STATE();
    case 134:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 10},
  [3] = {.lex_state = 10},
  [4] = {.lex_state = 10},
  [5] = {.lex_state = 10},
  [6] = {.lex_state = 10},
  [7] = {.lex_state = 10},
  [8] = {.lex_state = 10},
  [9] = {.lex_state = 11},
  [10] = {.lex_state = 14},
  [11] = {.lex_state = 14},
  [12] = {.lex_state = 14},
  [13] = {.lex_state = 14},
  [14] = {.lex_state = 11},
  [15] = {.lex_state = 14},
  [16] = {.lex_state = 14},
  [17] = {.lex_state = 12},
  [18] = {.lex_state = 12},
  [19] = {.lex_state = 15},
  [20] = {.lex_state = 15},
  [21] = {.lex_state = 15},
  [22] = {.lex_state = 15},
  [23] = {.lex_state = 15},
  [24] = {.lex_state = 15},
  [25] = {.lex_state = 15},
  [26] = {.lex_state = 15},
  [27] = {.lex_state = 15},
  [28] = {.lex_state = 15},
  [29] = {.lex_state = 15},
  [30] = {.lex_state = 9},
  [31] = {.lex_state = 15},
  [32] = {.lex_state = 15},
  [33] = {.lex_state = 15},
  [34] = {.lex_state = 9},
  [35] = {.lex_state = 14},
  [36] = {.lex_state = 14},
  [37] = {.lex_state = 8},
  [38] = {.lex_state = 13},
  [39] = {.lex_state = 13},
  [40] = {.lex_state = 8},
  [41] = {.lex_state = 14},
  [42] = {.lex_state = 14},
  [43] = {.lex_state = 13},
  [44] = {.lex_state = 8},
  [45] = {.lex_state = 8},
  [46] = {.lex_state = 8},
  [47] = {.lex_state = 8},
  [48] = {.lex_state = 8},
  [49] = {.lex_state = 16},
  [50] = {.lex_state = 16},
  [51] = {.lex_state = 8},
  [52] = {.lex_state = 14},
  [53] = {.lex_state = 10},
  [54] = {.lex_state = 14},
  [55] = {.lex_state = 16},
  [56] = {.lex_state = 16},
  [57] = {.lex_state = 8},
  [58] = {.lex_state = 8},
  [59] = {.lex_state = 8},
  [60] = {.lex_state = 8},
  [61] = {.lex_state = 8},
  [62] = {.lex_state = 8},
  [63] = {.lex_state = 8},
  [64] = {.lex_state = 8},
  [65] = {.lex_state = 10},
  [66] = {.lex_state = 10},
  [67] = {.lex_state = 14},
  [68] = {.lex_state = 8},
  [69] = {.lex_state = 14},
  [70] = {.lex_state = 16},
  [71] = {.lex_state = 10},
  [72] = {.lex_state = 8},
  [73] = {.lex_state = 8},
  [74] = {.lex_state = 10},
  [75] = {.lex_state = 8},
  [76] = {.lex_state = 8},
  [77] = {.lex_state = 8},
  [78] = {.lex_state = 8},
  [79] = {.lex_state = 8},
  [80] = {.lex_state = 14},
  [81] = {.lex_state = 16},
  [82] = {.lex_state = 14},
  [83] = {.lex_state = 14},
  [84] = {.lex_state = 15},
  [85] = {.lex_state = 15},
  [86] = {.lex_state = 15},
  [87] = {.lex_state = 14},
  [88] = {.lex_state = 15},
  [89] = {.lex_state = 15},
  [90] = {.lex_state = 15},
  [91] = {.lex_state = 14},
  [92] = {.lex_state = 14},
  [93] = {.lex_state = 15},
  [94] = {.lex_state = 12},
  [95] = {.lex_state = 9},
  [96] = {.lex_state = 9},
  [97] = {.lex_state = 9},
  [98] = {.lex_state = 9},
  [99] = {.lex_state = 9},
  [100] = {.lex_state = 9},
  [101] = {.lex_state = 0},
  [102] = {.lex_state = 0},
  [103] = {.lex_state = 0},
  [104] = {.lex_state = 0},
  [105] = {.lex_state = 8},
  [106] = {.lex_state = 8},
  [107] = {.lex_state = 0},
  [108] = {.lex_state = 0},
  [109] = {.lex_state = 0},
  [110] = {.lex_state = 8},
  [111] = {.lex_state = 8},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 0},
  [115] = {.lex_state = 0},
  [116] = {.lex_state = 8},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 8},
  [119] = {.lex_state = 0},
  [120] = {.lex_state = 8},
  [121] = {.lex_state = 0},
  [122] = {.lex_state = 0},
  [123] = {.lex_state = 38},
  [124] = {.lex_state = 0},
  [125] = {.lex_state = 0},
  [126] = {.lex_state = 8},
  [127] = {.lex_state = 8},
  [128] = {.lex_state = 0},
  [129] = {.lex_state = 0},
  [130] = {.lex_state = 8},
  [131] = {.lex_state = 8},
  [132] = {.lex_state = 0},
  [133] = {.lex_state = 0},
  [134] = {.lex_state = 0},
  [135] = {.lex_state = 0},
  [136] = {.lex_state = 0},
  [137] = {.lex_state = 8},
  [138] = {.lex_state = 0},
  [139] = {.lex_state = 0},
  [140] = {.lex_state = 0},
  [141] = {.lex_state = 0},
  [142] = {.lex_state = 0},
  [143] = {.lex_state = 0},
  [144] = {.lex_state = 0},
  [145] = {.lex_state = 0},
  [146] = {.lex_state = 0},
  [147] = {.lex_state = 8},
  [148] = {.lex_state = 0},
  [149] = {.lex_state = 0},
  [150] = {.lex_state = 0},
  [151] = {.lex_state = 8},
  [152] = {.lex_state = 0},
  [153] = {.lex_state = 0},
  [154] = {.lex_state = 8},
  [155] = {.lex_state = 8},
  [156] = {.lex_state = 0},
  [157] = {.lex_state = 8},
  [158] = {.lex_state = 8},
  [159] = {.lex_state = 0},
  [160] = {.lex_state = 8},
  [161] = {.lex_state = 8},
  [162] = {.lex_state = 8},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
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
    [anon_sym_PERCENT] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_reg] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [anon_sym_for] = ACTIONS(1),
    [anon_sym_in] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(159),
    [sym_module] = STATE(112),
    [aux_sym_source_file_repeat1] = STATE(112),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_module] = ACTIONS(5),
  },
  [2] = {
    [sym_global_identifier] = STATE(36),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(126),
    [sym__type] = STATE(126),
    [sym_declaration] = STATE(115),
    [sym_unary_op] = STATE(42),
    [sym_binary_op] = STATE(42),
    [sym_array_op] = STATE(42),
    [sym_func_call] = STATE(42),
    [sym_parenthesis_expression] = STATE(42),
    [sym__expression] = STATE(42),
    [sym_block] = STATE(6),
    [sym__assign_left_side] = STATE(153),
    [sym_decl_assign_statement] = STATE(156),
    [sym_decl_statement] = STATE(156),
    [sym_expression_statement] = STATE(156),
    [sym_if_statement] = STATE(6),
    [sym_for_statement] = STATE(6),
    [sym__statement] = STATE(6),
    [aux_sym_block_repeat1] = STATE(6),
    [aux_sym__assign_left_side_repeat1] = STATE(18),
    [sym_identifier] = ACTIONS(7),
    [sym_number] = ACTIONS(9),
    [anon_sym_COLON_COLON] = ACTIONS(11),
    [anon_sym_state] = ACTIONS(13),
    [anon_sym_gen] = ACTIONS(13),
    [anon_sym_PLUS] = ACTIONS(15),
    [anon_sym_DASH] = ACTIONS(15),
    [anon_sym_STAR] = ACTIONS(15),
    [anon_sym_BANG] = ACTIONS(15),
    [anon_sym_PIPE] = ACTIONS(15),
    [anon_sym_AMP] = ACTIONS(15),
    [anon_sym_CARET] = ACTIONS(15),
    [anon_sym_LPAREN] = ACTIONS(17),
    [anon_sym_LBRACE] = ACTIONS(19),
    [anon_sym_RBRACE] = ACTIONS(21),
    [anon_sym_reg] = ACTIONS(23),
    [anon_sym_initial] = ACTIONS(25),
    [anon_sym_if] = ACTIONS(27),
    [anon_sym_for] = ACTIONS(29),
  },
  [3] = {
    [sym_global_identifier] = STATE(36),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(126),
    [sym__type] = STATE(126),
    [sym_declaration] = STATE(115),
    [sym_unary_op] = STATE(42),
    [sym_binary_op] = STATE(42),
    [sym_array_op] = STATE(42),
    [sym_func_call] = STATE(42),
    [sym_parenthesis_expression] = STATE(42),
    [sym__expression] = STATE(42),
    [sym_block] = STATE(2),
    [sym__assign_left_side] = STATE(153),
    [sym_decl_assign_statement] = STATE(156),
    [sym_decl_statement] = STATE(156),
    [sym_expression_statement] = STATE(156),
    [sym_if_statement] = STATE(2),
    [sym_for_statement] = STATE(2),
    [sym__statement] = STATE(2),
    [aux_sym_block_repeat1] = STATE(2),
    [aux_sym__assign_left_side_repeat1] = STATE(18),
    [sym_identifier] = ACTIONS(7),
    [sym_number] = ACTIONS(9),
    [anon_sym_COLON_COLON] = ACTIONS(11),
    [anon_sym_state] = ACTIONS(13),
    [anon_sym_gen] = ACTIONS(13),
    [anon_sym_PLUS] = ACTIONS(15),
    [anon_sym_DASH] = ACTIONS(15),
    [anon_sym_STAR] = ACTIONS(15),
    [anon_sym_BANG] = ACTIONS(15),
    [anon_sym_PIPE] = ACTIONS(15),
    [anon_sym_AMP] = ACTIONS(15),
    [anon_sym_CARET] = ACTIONS(15),
    [anon_sym_LPAREN] = ACTIONS(17),
    [anon_sym_LBRACE] = ACTIONS(19),
    [anon_sym_RBRACE] = ACTIONS(31),
    [anon_sym_reg] = ACTIONS(23),
    [anon_sym_initial] = ACTIONS(25),
    [anon_sym_if] = ACTIONS(27),
    [anon_sym_for] = ACTIONS(29),
  },
  [4] = {
    [sym_global_identifier] = STATE(36),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(126),
    [sym__type] = STATE(126),
    [sym_declaration] = STATE(115),
    [sym_unary_op] = STATE(42),
    [sym_binary_op] = STATE(42),
    [sym_array_op] = STATE(42),
    [sym_func_call] = STATE(42),
    [sym_parenthesis_expression] = STATE(42),
    [sym__expression] = STATE(42),
    [sym_block] = STATE(6),
    [sym__assign_left_side] = STATE(153),
    [sym_decl_assign_statement] = STATE(156),
    [sym_decl_statement] = STATE(156),
    [sym_expression_statement] = STATE(156),
    [sym_if_statement] = STATE(6),
    [sym_for_statement] = STATE(6),
    [sym__statement] = STATE(6),
    [aux_sym_block_repeat1] = STATE(6),
    [aux_sym__assign_left_side_repeat1] = STATE(18),
    [sym_identifier] = ACTIONS(7),
    [sym_number] = ACTIONS(9),
    [anon_sym_COLON_COLON] = ACTIONS(11),
    [anon_sym_state] = ACTIONS(13),
    [anon_sym_gen] = ACTIONS(13),
    [anon_sym_PLUS] = ACTIONS(15),
    [anon_sym_DASH] = ACTIONS(15),
    [anon_sym_STAR] = ACTIONS(15),
    [anon_sym_BANG] = ACTIONS(15),
    [anon_sym_PIPE] = ACTIONS(15),
    [anon_sym_AMP] = ACTIONS(15),
    [anon_sym_CARET] = ACTIONS(15),
    [anon_sym_LPAREN] = ACTIONS(17),
    [anon_sym_LBRACE] = ACTIONS(19),
    [anon_sym_RBRACE] = ACTIONS(33),
    [anon_sym_reg] = ACTIONS(23),
    [anon_sym_initial] = ACTIONS(25),
    [anon_sym_if] = ACTIONS(27),
    [anon_sym_for] = ACTIONS(29),
  },
  [5] = {
    [sym_global_identifier] = STATE(36),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(126),
    [sym__type] = STATE(126),
    [sym_declaration] = STATE(115),
    [sym_unary_op] = STATE(42),
    [sym_binary_op] = STATE(42),
    [sym_array_op] = STATE(42),
    [sym_func_call] = STATE(42),
    [sym_parenthesis_expression] = STATE(42),
    [sym__expression] = STATE(42),
    [sym_block] = STATE(4),
    [sym__assign_left_side] = STATE(153),
    [sym_decl_assign_statement] = STATE(156),
    [sym_decl_statement] = STATE(156),
    [sym_expression_statement] = STATE(156),
    [sym_if_statement] = STATE(4),
    [sym_for_statement] = STATE(4),
    [sym__statement] = STATE(4),
    [aux_sym_block_repeat1] = STATE(4),
    [aux_sym__assign_left_side_repeat1] = STATE(18),
    [sym_identifier] = ACTIONS(7),
    [sym_number] = ACTIONS(9),
    [anon_sym_COLON_COLON] = ACTIONS(11),
    [anon_sym_state] = ACTIONS(13),
    [anon_sym_gen] = ACTIONS(13),
    [anon_sym_PLUS] = ACTIONS(15),
    [anon_sym_DASH] = ACTIONS(15),
    [anon_sym_STAR] = ACTIONS(15),
    [anon_sym_BANG] = ACTIONS(15),
    [anon_sym_PIPE] = ACTIONS(15),
    [anon_sym_AMP] = ACTIONS(15),
    [anon_sym_CARET] = ACTIONS(15),
    [anon_sym_LPAREN] = ACTIONS(17),
    [anon_sym_LBRACE] = ACTIONS(19),
    [anon_sym_RBRACE] = ACTIONS(35),
    [anon_sym_reg] = ACTIONS(23),
    [anon_sym_initial] = ACTIONS(25),
    [anon_sym_if] = ACTIONS(27),
    [anon_sym_for] = ACTIONS(29),
  },
  [6] = {
    [sym_global_identifier] = STATE(36),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(126),
    [sym__type] = STATE(126),
    [sym_declaration] = STATE(115),
    [sym_unary_op] = STATE(42),
    [sym_binary_op] = STATE(42),
    [sym_array_op] = STATE(42),
    [sym_func_call] = STATE(42),
    [sym_parenthesis_expression] = STATE(42),
    [sym__expression] = STATE(42),
    [sym_block] = STATE(6),
    [sym__assign_left_side] = STATE(153),
    [sym_decl_assign_statement] = STATE(156),
    [sym_decl_statement] = STATE(156),
    [sym_expression_statement] = STATE(156),
    [sym_if_statement] = STATE(6),
    [sym_for_statement] = STATE(6),
    [sym__statement] = STATE(6),
    [aux_sym_block_repeat1] = STATE(6),
    [aux_sym__assign_left_side_repeat1] = STATE(18),
    [sym_identifier] = ACTIONS(37),
    [sym_number] = ACTIONS(40),
    [anon_sym_COLON_COLON] = ACTIONS(43),
    [anon_sym_state] = ACTIONS(46),
    [anon_sym_gen] = ACTIONS(46),
    [anon_sym_PLUS] = ACTIONS(49),
    [anon_sym_DASH] = ACTIONS(49),
    [anon_sym_STAR] = ACTIONS(49),
    [anon_sym_BANG] = ACTIONS(49),
    [anon_sym_PIPE] = ACTIONS(49),
    [anon_sym_AMP] = ACTIONS(49),
    [anon_sym_CARET] = ACTIONS(49),
    [anon_sym_LPAREN] = ACTIONS(52),
    [anon_sym_LBRACE] = ACTIONS(55),
    [anon_sym_RBRACE] = ACTIONS(58),
    [anon_sym_reg] = ACTIONS(60),
    [anon_sym_initial] = ACTIONS(63),
    [anon_sym_if] = ACTIONS(66),
    [anon_sym_for] = ACTIONS(69),
  },
  [7] = {
    [sym_global_identifier] = STATE(36),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(126),
    [sym__type] = STATE(126),
    [sym_declaration] = STATE(115),
    [sym_unary_op] = STATE(42),
    [sym_binary_op] = STATE(42),
    [sym_array_op] = STATE(42),
    [sym_func_call] = STATE(42),
    [sym_parenthesis_expression] = STATE(42),
    [sym__expression] = STATE(42),
    [sym_block] = STATE(6),
    [sym__assign_left_side] = STATE(153),
    [sym_decl_assign_statement] = STATE(156),
    [sym_decl_statement] = STATE(156),
    [sym_expression_statement] = STATE(156),
    [sym_if_statement] = STATE(6),
    [sym_for_statement] = STATE(6),
    [sym__statement] = STATE(6),
    [aux_sym_block_repeat1] = STATE(6),
    [aux_sym__assign_left_side_repeat1] = STATE(18),
    [sym_identifier] = ACTIONS(7),
    [sym_number] = ACTIONS(9),
    [anon_sym_COLON_COLON] = ACTIONS(11),
    [anon_sym_state] = ACTIONS(13),
    [anon_sym_gen] = ACTIONS(13),
    [anon_sym_PLUS] = ACTIONS(15),
    [anon_sym_DASH] = ACTIONS(15),
    [anon_sym_STAR] = ACTIONS(15),
    [anon_sym_BANG] = ACTIONS(15),
    [anon_sym_PIPE] = ACTIONS(15),
    [anon_sym_AMP] = ACTIONS(15),
    [anon_sym_CARET] = ACTIONS(15),
    [anon_sym_LPAREN] = ACTIONS(17),
    [anon_sym_LBRACE] = ACTIONS(19),
    [anon_sym_RBRACE] = ACTIONS(72),
    [anon_sym_reg] = ACTIONS(23),
    [anon_sym_initial] = ACTIONS(25),
    [anon_sym_if] = ACTIONS(27),
    [anon_sym_for] = ACTIONS(29),
  },
  [8] = {
    [sym_global_identifier] = STATE(36),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(126),
    [sym__type] = STATE(126),
    [sym_declaration] = STATE(115),
    [sym_unary_op] = STATE(42),
    [sym_binary_op] = STATE(42),
    [sym_array_op] = STATE(42),
    [sym_func_call] = STATE(42),
    [sym_parenthesis_expression] = STATE(42),
    [sym__expression] = STATE(42),
    [sym_block] = STATE(7),
    [sym__assign_left_side] = STATE(153),
    [sym_decl_assign_statement] = STATE(156),
    [sym_decl_statement] = STATE(156),
    [sym_expression_statement] = STATE(156),
    [sym_if_statement] = STATE(7),
    [sym_for_statement] = STATE(7),
    [sym__statement] = STATE(7),
    [aux_sym_block_repeat1] = STATE(7),
    [aux_sym__assign_left_side_repeat1] = STATE(18),
    [sym_identifier] = ACTIONS(7),
    [sym_number] = ACTIONS(9),
    [anon_sym_COLON_COLON] = ACTIONS(11),
    [anon_sym_state] = ACTIONS(13),
    [anon_sym_gen] = ACTIONS(13),
    [anon_sym_PLUS] = ACTIONS(15),
    [anon_sym_DASH] = ACTIONS(15),
    [anon_sym_STAR] = ACTIONS(15),
    [anon_sym_BANG] = ACTIONS(15),
    [anon_sym_PIPE] = ACTIONS(15),
    [anon_sym_AMP] = ACTIONS(15),
    [anon_sym_CARET] = ACTIONS(15),
    [anon_sym_LPAREN] = ACTIONS(17),
    [anon_sym_LBRACE] = ACTIONS(19),
    [anon_sym_RBRACE] = ACTIONS(74),
    [anon_sym_reg] = ACTIONS(23),
    [anon_sym_initial] = ACTIONS(25),
    [anon_sym_if] = ACTIONS(27),
    [anon_sym_for] = ACTIONS(29),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 15,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_reg,
    ACTIONS(25), 1,
      anon_sym_initial,
    ACTIONS(76), 1,
      sym_number,
    STATE(18), 1,
      aux_sym__assign_left_side_repeat1,
    STATE(19), 1,
      sym__maybe_global_identifier,
    STATE(36), 1,
      sym_global_identifier,
    STATE(141), 1,
      sym_declaration,
    STATE(143), 1,
      sym__assign_left_side,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(126), 2,
      sym_array_type,
      sym__type,
    STATE(54), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [59] = 4,
    ACTIONS(80), 1,
      anon_sym_COLON_COLON,
    ACTIONS(83), 1,
      anon_sym_SLASH,
    STATE(10), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(78), 24,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      sym_identifier,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [95] = 4,
    ACTIONS(87), 1,
      anon_sym_COLON_COLON,
    ACTIONS(89), 1,
      anon_sym_SLASH,
    STATE(10), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(85), 24,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      sym_identifier,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [131] = 4,
    ACTIONS(87), 1,
      anon_sym_COLON_COLON,
    ACTIONS(93), 1,
      anon_sym_SLASH,
    STATE(11), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(91), 24,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      sym_identifier,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [167] = 4,
    ACTIONS(87), 1,
      anon_sym_COLON_COLON,
    ACTIONS(93), 1,
      anon_sym_SLASH,
    STATE(10), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(91), 24,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      sym_identifier,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [203] = 14,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      sym_number,
    ACTIONS(97), 1,
      anon_sym_reg,
    ACTIONS(99), 1,
      anon_sym_initial,
    STATE(17), 1,
      aux_sym__assign_left_side_repeat1,
    STATE(19), 1,
      sym__maybe_global_identifier,
    STATE(36), 1,
      sym_global_identifier,
    STATE(136), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(126), 2,
      sym_array_type,
      sym__type,
    STATE(67), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [259] = 4,
    ACTIONS(87), 1,
      anon_sym_COLON_COLON,
    ACTIONS(103), 1,
      anon_sym_SLASH,
    STATE(13), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(101), 24,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      sym_identifier,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [295] = 2,
    ACTIONS(83), 1,
      anon_sym_SLASH,
    ACTIONS(78), 25,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      sym_identifier,
      anon_sym_COLON_COLON,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [326] = 13,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(105), 1,
      sym_number,
    ACTIONS(107), 1,
      anon_sym_reg,
    STATE(19), 1,
      sym__maybe_global_identifier,
    STATE(36), 1,
      sym_global_identifier,
    STATE(94), 1,
      aux_sym__assign_left_side_repeat1,
    STATE(140), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(126), 2,
      sym_array_type,
      sym__type,
    STATE(52), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [379] = 13,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(107), 1,
      anon_sym_reg,
    ACTIONS(109), 1,
      sym_number,
    STATE(19), 1,
      sym__maybe_global_identifier,
    STATE(36), 1,
      sym_global_identifier,
    STATE(94), 1,
      aux_sym__assign_left_side_repeat1,
    STATE(108), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(126), 2,
      sym_array_type,
      sym__type,
    STATE(41), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [432] = 3,
    ACTIONS(113), 1,
      anon_sym_SLASH,
    ACTIONS(115), 1,
      anon_sym_LPAREN,
    ACTIONS(111), 23,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [464] = 3,
    ACTIONS(119), 1,
      anon_sym_SLASH,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(117), 23,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [496] = 3,
    ACTIONS(123), 1,
      anon_sym_SLASH,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(121), 23,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [528] = 2,
    ACTIONS(127), 1,
      anon_sym_SLASH,
    ACTIONS(125), 23,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [557] = 2,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(129), 23,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [586] = 5,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(121), 18,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [621] = 2,
    ACTIONS(141), 1,
      anon_sym_SLASH,
    ACTIONS(139), 23,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [650] = 7,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(143), 1,
      anon_sym_PIPE,
    ACTIONS(145), 1,
      anon_sym_CARET,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(121), 16,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [689] = 2,
    ACTIONS(149), 1,
      anon_sym_SLASH,
    ACTIONS(147), 23,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [718] = 6,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(145), 1,
      anon_sym_CARET,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(121), 17,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [755] = 2,
    ACTIONS(153), 1,
      anon_sym_SLASH,
    ACTIONS(151), 23,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [784] = 11,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(109), 1,
      sym_number,
    STATE(19), 1,
      sym__maybe_global_identifier,
    STATE(36), 1,
      sym_global_identifier,
    STATE(108), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(126), 2,
      sym_array_type,
      sym__type,
    STATE(41), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [831] = 8,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(143), 1,
      anon_sym_PIPE,
    ACTIONS(145), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_AMP,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(121), 15,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [872] = 4,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(121), 20,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [905] = 2,
    ACTIONS(159), 1,
      anon_sym_SLASH,
    ACTIONS(157), 23,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [934] = 11,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(105), 1,
      sym_number,
    STATE(19), 1,
      sym__maybe_global_identifier,
    STATE(36), 1,
      sym_global_identifier,
    STATE(140), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(126), 2,
      sym_array_type,
      sym__type,
    STATE(52), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [981] = 10,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(143), 1,
      anon_sym_PIPE,
    ACTIONS(145), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(161), 5,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
    ACTIONS(163), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1023] = 4,
    ACTIONS(169), 1,
      sym_identifier,
    ACTIONS(171), 1,
      anon_sym_SLASH,
    ACTIONS(173), 1,
      anon_sym_LBRACK,
    ACTIONS(167), 18,
      anon_sym_COMMA,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [1053] = 8,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(176), 1,
      sym_identifier,
    ACTIONS(178), 1,
      sym_number,
    STATE(149), 1,
      sym_range,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(93), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1090] = 3,
    ACTIONS(184), 1,
      anon_sym_else,
    ACTIONS(180), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(182), 12,
      sym_number,
      anon_sym_COLON_COLON,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
  [1117] = 2,
    ACTIONS(186), 8,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_else,
      anon_sym_for,
    ACTIONS(188), 12,
      sym_number,
      anon_sym_COLON_COLON,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
  [1142] = 8,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(176), 1,
      sym_identifier,
    ACTIONS(190), 1,
      sym_number,
    ACTIONS(192), 1,
      anon_sym_RPAREN,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(69), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1179] = 12,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(143), 1,
      anon_sym_PIPE,
    ACTIONS(145), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(194), 1,
      anon_sym_COMMA,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(117), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(196), 2,
      anon_sym_LBRACE,
      anon_sym_EQ,
    ACTIONS(163), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1224] = 13,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(143), 1,
      anon_sym_PIPE,
    ACTIONS(145), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(194), 1,
      anon_sym_COMMA,
    ACTIONS(198), 1,
      anon_sym_EQ,
    ACTIONS(200), 1,
      anon_sym_SEMI,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(109), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(163), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1271] = 2,
    ACTIONS(202), 8,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_else,
      anon_sym_for,
    ACTIONS(204), 12,
      sym_number,
      anon_sym_COLON_COLON,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
  [1296] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      sym_identifier,
    ACTIONS(208), 1,
      sym_number,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(20), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(212), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1330] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      sym_identifier,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    ACTIONS(214), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(84), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(212), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1364] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(176), 1,
      sym_identifier,
    ACTIONS(216), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(82), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1398] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(176), 1,
      sym_identifier,
    ACTIONS(218), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(92), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1432] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(176), 1,
      sym_identifier,
    ACTIONS(220), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(80), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1466] = 4,
    ACTIONS(89), 1,
      anon_sym_SLASH,
    ACTIONS(222), 1,
      anon_sym_COLON_COLON,
    STATE(50), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(85), 16,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_in,
  [1494] = 4,
    ACTIONS(83), 1,
      anon_sym_SLASH,
    ACTIONS(224), 1,
      anon_sym_COLON_COLON,
    STATE(50), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(78), 16,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_in,
  [1522] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(176), 1,
      sym_identifier,
    ACTIONS(227), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(91), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1556] = 10,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(143), 1,
      anon_sym_PIPE,
    ACTIONS(145), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(229), 3,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
    ACTIONS(163), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1596] = 2,
    ACTIONS(231), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(233), 12,
      sym_number,
      anon_sym_COLON_COLON,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
  [1620] = 12,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(143), 1,
      anon_sym_PIPE,
    ACTIONS(145), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(194), 1,
      anon_sym_COMMA,
    ACTIONS(198), 1,
      anon_sym_LBRACE,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(109), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(163), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1664] = 4,
    ACTIONS(93), 1,
      anon_sym_SLASH,
    ACTIONS(222), 1,
      anon_sym_COLON_COLON,
    STATE(49), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(91), 16,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_in,
  [1692] = 4,
    ACTIONS(93), 1,
      anon_sym_SLASH,
    ACTIONS(222), 1,
      anon_sym_COLON_COLON,
    STATE(50), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(91), 16,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_in,
  [1720] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(176), 1,
      sym_identifier,
    ACTIONS(235), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(87), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1754] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(176), 1,
      sym_identifier,
    ACTIONS(237), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(32), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1788] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(176), 1,
      sym_identifier,
    ACTIONS(239), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(21), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1822] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(176), 1,
      sym_identifier,
    ACTIONS(241), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(28), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1856] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(176), 1,
      sym_identifier,
    ACTIONS(243), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(26), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1890] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(176), 1,
      sym_identifier,
    ACTIONS(245), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(24), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1924] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(176), 1,
      sym_identifier,
    ACTIONS(247), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(31), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1958] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(176), 1,
      sym_identifier,
    ACTIONS(249), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(83), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1992] = 2,
    ACTIONS(251), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(253), 12,
      sym_number,
      anon_sym_COLON_COLON,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
  [2016] = 2,
    ACTIONS(255), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(257), 12,
      sym_number,
      anon_sym_COLON_COLON,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
  [2040] = 10,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(143), 1,
      anon_sym_PIPE,
    ACTIONS(145), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(259), 3,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
    ACTIONS(163), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2080] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(176), 1,
      sym_identifier,
    ACTIONS(208), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(20), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2114] = 12,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(143), 1,
      anon_sym_PIPE,
    ACTIONS(145), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(261), 1,
      anon_sym_COMMA,
    ACTIONS(263), 1,
      anon_sym_RPAREN,
    STATE(25), 1,
      sym_array_bracket_expression,
    STATE(139), 1,
      aux_sym_func_call_repeat1,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(163), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2158] = 4,
    ACTIONS(103), 1,
      anon_sym_SLASH,
    ACTIONS(222), 1,
      anon_sym_COLON_COLON,
    STATE(56), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(101), 16,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_in,
  [2186] = 2,
    ACTIONS(202), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(204), 12,
      sym_number,
      anon_sym_COLON_COLON,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
  [2210] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      sym_identifier,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    ACTIONS(265), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(90), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(212), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2244] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(176), 1,
      sym_identifier,
    ACTIONS(267), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(35), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2278] = 2,
    ACTIONS(186), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(188), 12,
      sym_number,
      anon_sym_COLON_COLON,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
  [2302] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      sym_identifier,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    ACTIONS(239), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(21), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(212), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2336] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      sym_identifier,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    ACTIONS(269), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(89), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(212), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2370] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      sym_identifier,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    ACTIONS(271), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(88), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(212), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2404] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      sym_identifier,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    ACTIONS(273), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(86), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(212), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2438] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      sym_identifier,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    ACTIONS(275), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(85), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(212), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2472] = 10,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(143), 1,
      anon_sym_PIPE,
    ACTIONS(145), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(277), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
    ACTIONS(163), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2511] = 2,
    ACTIONS(83), 1,
      anon_sym_SLASH,
    ACTIONS(78), 17,
      anon_sym_COLON_COLON,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_in,
  [2534] = 10,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(143), 1,
      anon_sym_PIPE,
    ACTIONS(145), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(279), 1,
      anon_sym_RBRACK,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(163), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2572] = 10,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(143), 1,
      anon_sym_PIPE,
    ACTIONS(145), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(281), 1,
      anon_sym_SEMI,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(163), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2610] = 4,
    ACTIONS(285), 1,
      anon_sym_SLASH,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(283), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(121), 13,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_LBRACK,
      anon_sym_in,
  [2636] = 6,
    ACTIONS(285), 1,
      anon_sym_SLASH,
    ACTIONS(289), 1,
      anon_sym_CARET,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(283), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(287), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(121), 10,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_LBRACK,
      anon_sym_in,
  [2666] = 7,
    ACTIONS(285), 1,
      anon_sym_SLASH,
    ACTIONS(289), 1,
      anon_sym_CARET,
    ACTIONS(291), 1,
      anon_sym_PIPE,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(283), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(287), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(121), 9,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_LBRACK,
      anon_sym_in,
  [2698] = 10,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(143), 1,
      anon_sym_PIPE,
    ACTIONS(145), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(293), 1,
      anon_sym_LBRACE,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(163), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2736] = 5,
    ACTIONS(285), 1,
      anon_sym_SLASH,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(283), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(287), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(121), 11,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_LBRACK,
      anon_sym_in,
  [2764] = 8,
    ACTIONS(285), 1,
      anon_sym_SLASH,
    ACTIONS(289), 1,
      anon_sym_CARET,
    ACTIONS(291), 1,
      anon_sym_PIPE,
    ACTIONS(295), 1,
      anon_sym_AMP,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(283), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(287), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(121), 8,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_LBRACK,
      anon_sym_in,
  [2798] = 10,
    ACTIONS(161), 1,
      anon_sym_in,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(285), 1,
      anon_sym_SLASH,
    ACTIONS(289), 1,
      anon_sym_CARET,
    ACTIONS(291), 1,
      anon_sym_PIPE,
    ACTIONS(295), 1,
      anon_sym_AMP,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(283), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(287), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(297), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2836] = 10,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(143), 1,
      anon_sym_PIPE,
    ACTIONS(145), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(299), 1,
      anon_sym_RPAREN,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(163), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2874] = 10,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(143), 1,
      anon_sym_PIPE,
    ACTIONS(145), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(301), 1,
      anon_sym_RBRACK,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(163), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2912] = 10,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(143), 1,
      anon_sym_PIPE,
    ACTIONS(145), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(303), 1,
      anon_sym_COLON,
    STATE(25), 1,
      sym_array_bracket_expression,
    ACTIONS(133), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(135), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(163), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2950] = 4,
    ACTIONS(309), 1,
      anon_sym_reg,
    STATE(94), 1,
      aux_sym__assign_left_side_repeat1,
    ACTIONS(305), 3,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
    ACTIONS(307), 10,
      sym_number,
      anon_sym_COLON_COLON,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_LPAREN,
  [2974] = 7,
    ACTIONS(312), 1,
      anon_sym_DASH_GT,
    ACTIONS(314), 1,
      sym_identifier,
    ACTIONS(316), 1,
      anon_sym_COLON_COLON,
    ACTIONS(318), 1,
      anon_sym_LBRACE,
    STATE(119), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(126), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [2999] = 6,
    ACTIONS(314), 1,
      sym_identifier,
    ACTIONS(316), 1,
      anon_sym_COLON_COLON,
    ACTIONS(320), 1,
      anon_sym_LBRACE,
    STATE(134), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(126), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3021] = 6,
    ACTIONS(314), 1,
      sym_identifier,
    ACTIONS(316), 1,
      anon_sym_COLON_COLON,
    ACTIONS(322), 1,
      anon_sym_LBRACE,
    STATE(133), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(126), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3043] = 6,
    ACTIONS(314), 1,
      sym_identifier,
    ACTIONS(316), 1,
      anon_sym_COLON_COLON,
    ACTIONS(324), 1,
      anon_sym_LBRACE,
    STATE(129), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(126), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3065] = 5,
    ACTIONS(314), 1,
      sym_identifier,
    ACTIONS(316), 1,
      anon_sym_COLON_COLON,
    STATE(124), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(126), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3084] = 5,
    ACTIONS(314), 1,
      sym_identifier,
    ACTIONS(316), 1,
      anon_sym_COLON_COLON,
    STATE(152), 1,
      sym_declaration,
    ACTIONS(326), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(131), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3103] = 3,
    ACTIONS(330), 1,
      anon_sym_SQUOTE,
    STATE(103), 1,
      sym_latency_specifier,
    ACTIONS(328), 5,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3117] = 3,
    ACTIONS(330), 1,
      anon_sym_SQUOTE,
    STATE(104), 1,
      sym_latency_specifier,
    ACTIONS(332), 5,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3131] = 1,
    ACTIONS(334), 6,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [3140] = 1,
    ACTIONS(336), 6,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [3149] = 3,
    ACTIONS(316), 1,
      anon_sym_COLON_COLON,
    ACTIONS(338), 1,
      sym_identifier,
    STATE(130), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3161] = 3,
    ACTIONS(316), 1,
      anon_sym_COLON_COLON,
    ACTIONS(338), 1,
      sym_identifier,
    STATE(127), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3173] = 3,
    ACTIONS(19), 1,
      anon_sym_LBRACE,
    ACTIONS(340), 1,
      anon_sym_if,
    STATE(53), 2,
      sym_block,
      sym_if_statement,
  [3184] = 3,
    ACTIONS(342), 1,
      anon_sym_COMMA,
    STATE(117), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(344), 2,
      anon_sym_LBRACE,
      anon_sym_EQ,
  [3195] = 3,
    ACTIONS(342), 1,
      anon_sym_COMMA,
    STATE(114), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(344), 2,
      anon_sym_LBRACE,
      anon_sym_EQ,
  [3206] = 3,
    ACTIONS(346), 1,
      anon_sym_COLON_COLON,
    STATE(110), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(83), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3217] = 3,
    ACTIONS(349), 1,
      anon_sym_COLON_COLON,
    STATE(110), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(89), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3228] = 3,
    ACTIONS(5), 1,
      anon_sym_module,
    ACTIONS(351), 1,
      ts_builtin_sym_end,
    STATE(122), 2,
      sym_module,
      aux_sym_source_file_repeat1,
  [3239] = 4,
    ACTIONS(353), 1,
      anon_sym_COMMA,
    ACTIONS(355), 1,
      anon_sym_DASH_GT,
    ACTIONS(357), 1,
      anon_sym_LBRACE,
    STATE(121), 1,
      aux_sym_interface_ports_repeat1,
  [3252] = 3,
    ACTIONS(359), 1,
      anon_sym_COMMA,
    STATE(114), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(362), 2,
      anon_sym_LBRACE,
      anon_sym_EQ,
  [3263] = 4,
    ACTIONS(342), 1,
      anon_sym_COMMA,
    ACTIONS(364), 1,
      anon_sym_EQ,
    ACTIONS(366), 1,
      anon_sym_SEMI,
    STATE(109), 1,
      aux_sym__assign_left_side_repeat2,
  [3276] = 3,
    ACTIONS(349), 1,
      anon_sym_COLON_COLON,
    STATE(118), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(103), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3287] = 3,
    ACTIONS(342), 1,
      anon_sym_COMMA,
    STATE(114), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(368), 2,
      anon_sym_LBRACE,
      anon_sym_EQ,
  [3298] = 3,
    ACTIONS(349), 1,
      anon_sym_COLON_COLON,
    STATE(110), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(93), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3309] = 4,
    ACTIONS(353), 1,
      anon_sym_COMMA,
    ACTIONS(370), 1,
      anon_sym_DASH_GT,
    ACTIONS(372), 1,
      anon_sym_LBRACE,
    STATE(113), 1,
      aux_sym_interface_ports_repeat1,
  [3322] = 3,
    ACTIONS(349), 1,
      anon_sym_COLON_COLON,
    STATE(111), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(93), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3333] = 3,
    ACTIONS(374), 1,
      anon_sym_COMMA,
    STATE(121), 1,
      aux_sym_interface_ports_repeat1,
    ACTIONS(377), 2,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
  [3344] = 3,
    ACTIONS(379), 1,
      ts_builtin_sym_end,
    ACTIONS(381), 1,
      anon_sym_module,
    STATE(122), 2,
      sym_module,
      aux_sym_source_file_repeat1,
  [3355] = 4,
    ACTIONS(384), 1,
      anon_sym_COLON,
    ACTIONS(386), 1,
      anon_sym_LBRACE,
    STATE(144), 1,
      sym_interface_ports,
    STATE(145), 1,
      sym_block,
  [3368] = 1,
    ACTIONS(377), 3,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
  [3374] = 3,
    ACTIONS(332), 1,
      anon_sym_in,
    ACTIONS(388), 1,
      anon_sym_SQUOTE,
    STATE(104), 1,
      sym_latency_specifier,
  [3384] = 3,
    ACTIONS(390), 1,
      sym_identifier,
    ACTIONS(392), 1,
      anon_sym_LBRACK,
    STATE(147), 1,
      sym_array_bracket_expression,
  [3394] = 3,
    ACTIONS(392), 1,
      anon_sym_LBRACK,
    ACTIONS(394), 1,
      sym_identifier,
    STATE(147), 1,
      sym_array_bracket_expression,
  [3404] = 3,
    ACTIONS(353), 1,
      anon_sym_COMMA,
    ACTIONS(396), 1,
      anon_sym_LBRACE,
    STATE(121), 1,
      aux_sym_interface_ports_repeat1,
  [3414] = 3,
    ACTIONS(353), 1,
      anon_sym_COMMA,
    ACTIONS(398), 1,
      anon_sym_LBRACE,
    STATE(128), 1,
      aux_sym_interface_ports_repeat1,
  [3424] = 3,
    ACTIONS(392), 1,
      anon_sym_LBRACK,
    ACTIONS(400), 1,
      sym_identifier,
    STATE(147), 1,
      sym_array_bracket_expression,
  [3434] = 3,
    ACTIONS(392), 1,
      anon_sym_LBRACK,
    ACTIONS(402), 1,
      sym_identifier,
    STATE(147), 1,
      sym_array_bracket_expression,
  [3444] = 3,
    ACTIONS(404), 1,
      anon_sym_COMMA,
    ACTIONS(407), 1,
      anon_sym_RPAREN,
    STATE(132), 1,
      aux_sym_func_call_repeat1,
  [3454] = 3,
    ACTIONS(353), 1,
      anon_sym_COMMA,
    ACTIONS(409), 1,
      anon_sym_LBRACE,
    STATE(142), 1,
      aux_sym_interface_ports_repeat1,
  [3464] = 3,
    ACTIONS(353), 1,
      anon_sym_COMMA,
    ACTIONS(411), 1,
      anon_sym_LBRACE,
    STATE(138), 1,
      aux_sym_interface_ports_repeat1,
  [3474] = 3,
    ACTIONS(328), 1,
      anon_sym_in,
    ACTIONS(388), 1,
      anon_sym_SQUOTE,
    STATE(103), 1,
      sym_latency_specifier,
  [3484] = 1,
    ACTIONS(362), 3,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
  [3490] = 1,
    ACTIONS(83), 3,
      sym_identifier,
      anon_sym_COLON_COLON,
      anon_sym_LBRACK,
  [3496] = 3,
    ACTIONS(353), 1,
      anon_sym_COMMA,
    ACTIONS(413), 1,
      anon_sym_LBRACE,
    STATE(121), 1,
      aux_sym_interface_ports_repeat1,
  [3506] = 3,
    ACTIONS(415), 1,
      anon_sym_COMMA,
    ACTIONS(417), 1,
      anon_sym_RPAREN,
    STATE(132), 1,
      aux_sym_func_call_repeat1,
  [3516] = 1,
    ACTIONS(419), 3,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
  [3522] = 3,
    ACTIONS(342), 1,
      anon_sym_COMMA,
    ACTIONS(364), 1,
      anon_sym_LBRACE,
    STATE(109), 1,
      aux_sym__assign_left_side_repeat2,
  [3532] = 3,
    ACTIONS(353), 1,
      anon_sym_COMMA,
    ACTIONS(421), 1,
      anon_sym_LBRACE,
    STATE(121), 1,
      aux_sym_interface_ports_repeat1,
  [3542] = 2,
    ACTIONS(423), 1,
      anon_sym_LBRACE,
    STATE(38), 1,
      sym_block,
  [3549] = 2,
    ACTIONS(386), 1,
      anon_sym_LBRACE,
    STATE(148), 1,
      sym_block,
  [3556] = 1,
    ACTIONS(425), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3561] = 1,
    ACTIONS(188), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3566] = 1,
    ACTIONS(427), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3571] = 1,
    ACTIONS(429), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3576] = 2,
    ACTIONS(19), 1,
      anon_sym_LBRACE,
    STATE(66), 1,
      sym_block,
  [3583] = 1,
    ACTIONS(204), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3588] = 1,
    ACTIONS(159), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3593] = 1,
    ACTIONS(431), 1,
      anon_sym_in,
  [3597] = 1,
    ACTIONS(433), 1,
      anon_sym_EQ,
  [3601] = 1,
    ACTIONS(435), 1,
      sym_identifier,
  [3605] = 1,
    ACTIONS(437), 1,
      sym_identifier,
  [3609] = 1,
    ACTIONS(439), 1,
      anon_sym_SEMI,
  [3613] = 1,
    ACTIONS(441), 1,
      sym_identifier,
  [3617] = 1,
    ACTIONS(443), 1,
      sym_identifier,
  [3621] = 1,
    ACTIONS(445), 1,
      ts_builtin_sym_end,
  [3625] = 1,
    ACTIONS(447), 1,
      sym_identifier,
  [3629] = 1,
    ACTIONS(449), 1,
      sym_identifier,
  [3633] = 1,
    ACTIONS(451), 1,
      sym_identifier,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(9)] = 0,
  [SMALL_STATE(10)] = 59,
  [SMALL_STATE(11)] = 95,
  [SMALL_STATE(12)] = 131,
  [SMALL_STATE(13)] = 167,
  [SMALL_STATE(14)] = 203,
  [SMALL_STATE(15)] = 259,
  [SMALL_STATE(16)] = 295,
  [SMALL_STATE(17)] = 326,
  [SMALL_STATE(18)] = 379,
  [SMALL_STATE(19)] = 432,
  [SMALL_STATE(20)] = 464,
  [SMALL_STATE(21)] = 496,
  [SMALL_STATE(22)] = 528,
  [SMALL_STATE(23)] = 557,
  [SMALL_STATE(24)] = 586,
  [SMALL_STATE(25)] = 621,
  [SMALL_STATE(26)] = 650,
  [SMALL_STATE(27)] = 689,
  [SMALL_STATE(28)] = 718,
  [SMALL_STATE(29)] = 755,
  [SMALL_STATE(30)] = 784,
  [SMALL_STATE(31)] = 831,
  [SMALL_STATE(32)] = 872,
  [SMALL_STATE(33)] = 905,
  [SMALL_STATE(34)] = 934,
  [SMALL_STATE(35)] = 981,
  [SMALL_STATE(36)] = 1023,
  [SMALL_STATE(37)] = 1053,
  [SMALL_STATE(38)] = 1090,
  [SMALL_STATE(39)] = 1117,
  [SMALL_STATE(40)] = 1142,
  [SMALL_STATE(41)] = 1179,
  [SMALL_STATE(42)] = 1224,
  [SMALL_STATE(43)] = 1271,
  [SMALL_STATE(44)] = 1296,
  [SMALL_STATE(45)] = 1330,
  [SMALL_STATE(46)] = 1364,
  [SMALL_STATE(47)] = 1398,
  [SMALL_STATE(48)] = 1432,
  [SMALL_STATE(49)] = 1466,
  [SMALL_STATE(50)] = 1494,
  [SMALL_STATE(51)] = 1522,
  [SMALL_STATE(52)] = 1556,
  [SMALL_STATE(53)] = 1596,
  [SMALL_STATE(54)] = 1620,
  [SMALL_STATE(55)] = 1664,
  [SMALL_STATE(56)] = 1692,
  [SMALL_STATE(57)] = 1720,
  [SMALL_STATE(58)] = 1754,
  [SMALL_STATE(59)] = 1788,
  [SMALL_STATE(60)] = 1822,
  [SMALL_STATE(61)] = 1856,
  [SMALL_STATE(62)] = 1890,
  [SMALL_STATE(63)] = 1924,
  [SMALL_STATE(64)] = 1958,
  [SMALL_STATE(65)] = 1992,
  [SMALL_STATE(66)] = 2016,
  [SMALL_STATE(67)] = 2040,
  [SMALL_STATE(68)] = 2080,
  [SMALL_STATE(69)] = 2114,
  [SMALL_STATE(70)] = 2158,
  [SMALL_STATE(71)] = 2186,
  [SMALL_STATE(72)] = 2210,
  [SMALL_STATE(73)] = 2244,
  [SMALL_STATE(74)] = 2278,
  [SMALL_STATE(75)] = 2302,
  [SMALL_STATE(76)] = 2336,
  [SMALL_STATE(77)] = 2370,
  [SMALL_STATE(78)] = 2404,
  [SMALL_STATE(79)] = 2438,
  [SMALL_STATE(80)] = 2472,
  [SMALL_STATE(81)] = 2511,
  [SMALL_STATE(82)] = 2534,
  [SMALL_STATE(83)] = 2572,
  [SMALL_STATE(84)] = 2610,
  [SMALL_STATE(85)] = 2636,
  [SMALL_STATE(86)] = 2666,
  [SMALL_STATE(87)] = 2698,
  [SMALL_STATE(88)] = 2736,
  [SMALL_STATE(89)] = 2764,
  [SMALL_STATE(90)] = 2798,
  [SMALL_STATE(91)] = 2836,
  [SMALL_STATE(92)] = 2874,
  [SMALL_STATE(93)] = 2912,
  [SMALL_STATE(94)] = 2950,
  [SMALL_STATE(95)] = 2974,
  [SMALL_STATE(96)] = 2999,
  [SMALL_STATE(97)] = 3021,
  [SMALL_STATE(98)] = 3043,
  [SMALL_STATE(99)] = 3065,
  [SMALL_STATE(100)] = 3084,
  [SMALL_STATE(101)] = 3103,
  [SMALL_STATE(102)] = 3117,
  [SMALL_STATE(103)] = 3131,
  [SMALL_STATE(104)] = 3140,
  [SMALL_STATE(105)] = 3149,
  [SMALL_STATE(106)] = 3161,
  [SMALL_STATE(107)] = 3173,
  [SMALL_STATE(108)] = 3184,
  [SMALL_STATE(109)] = 3195,
  [SMALL_STATE(110)] = 3206,
  [SMALL_STATE(111)] = 3217,
  [SMALL_STATE(112)] = 3228,
  [SMALL_STATE(113)] = 3239,
  [SMALL_STATE(114)] = 3252,
  [SMALL_STATE(115)] = 3263,
  [SMALL_STATE(116)] = 3276,
  [SMALL_STATE(117)] = 3287,
  [SMALL_STATE(118)] = 3298,
  [SMALL_STATE(119)] = 3309,
  [SMALL_STATE(120)] = 3322,
  [SMALL_STATE(121)] = 3333,
  [SMALL_STATE(122)] = 3344,
  [SMALL_STATE(123)] = 3355,
  [SMALL_STATE(124)] = 3368,
  [SMALL_STATE(125)] = 3374,
  [SMALL_STATE(126)] = 3384,
  [SMALL_STATE(127)] = 3394,
  [SMALL_STATE(128)] = 3404,
  [SMALL_STATE(129)] = 3414,
  [SMALL_STATE(130)] = 3424,
  [SMALL_STATE(131)] = 3434,
  [SMALL_STATE(132)] = 3444,
  [SMALL_STATE(133)] = 3454,
  [SMALL_STATE(134)] = 3464,
  [SMALL_STATE(135)] = 3474,
  [SMALL_STATE(136)] = 3484,
  [SMALL_STATE(137)] = 3490,
  [SMALL_STATE(138)] = 3496,
  [SMALL_STATE(139)] = 3506,
  [SMALL_STATE(140)] = 3516,
  [SMALL_STATE(141)] = 3522,
  [SMALL_STATE(142)] = 3532,
  [SMALL_STATE(143)] = 3542,
  [SMALL_STATE(144)] = 3549,
  [SMALL_STATE(145)] = 3556,
  [SMALL_STATE(146)] = 3561,
  [SMALL_STATE(147)] = 3566,
  [SMALL_STATE(148)] = 3571,
  [SMALL_STATE(149)] = 3576,
  [SMALL_STATE(150)] = 3583,
  [SMALL_STATE(151)] = 3588,
  [SMALL_STATE(152)] = 3593,
  [SMALL_STATE(153)] = 3597,
  [SMALL_STATE(154)] = 3601,
  [SMALL_STATE(155)] = 3605,
  [SMALL_STATE(156)] = 3609,
  [SMALL_STATE(157)] = 3613,
  [SMALL_STATE(158)] = 3617,
  [SMALL_STATE(159)] = 3621,
  [SMALL_STATE(160)] = 3625,
  [SMALL_STATE(161)] = 3629,
  [SMALL_STATE(162)] = 3633,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(162),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(154),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(106),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(18),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(30),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(9),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(100),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [37] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(15),
  [40] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(42),
  [43] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(154),
  [46] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(106),
  [49] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(68),
  [52] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(51),
  [55] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(5),
  [58] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2),
  [60] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(18),
  [63] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(30),
  [66] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(9),
  [69] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(100),
  [72] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [74] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [76] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [78] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_global_identifier_repeat1, 2),
  [80] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_identifier_repeat1, 2), SHIFT_REPEAT(155),
  [83] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_global_identifier_repeat1, 2),
  [85] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 3),
  [87] = {.entry = {.count = 1, .reusable = false}}, SHIFT(155),
  [89] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 3),
  [91] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 2),
  [93] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 2),
  [95] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [97] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [99] = {.entry = {.count = 1, .reusable = false}}, SHIFT(34),
  [101] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 1),
  [103] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 1),
  [105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [107] = {.entry = {.count = 1, .reusable = false}}, SHIFT(94),
  [109] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [111] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expression, 1),
  [113] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expression, 1),
  [115] = {.entry = {.count = 1, .reusable = false}}, SHIFT(40),
  [117] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_op, 2, .production_id = 8),
  [119] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_op, 2, .production_id = 8),
  [121] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_op, 3, .production_id = 16),
  [123] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_op, 3, .production_id = 16),
  [125] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 3, .production_id = 15),
  [127] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 3, .production_id = 15),
  [129] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 13),
  [131] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 13),
  [133] = {.entry = {.count = 1, .reusable = false}}, SHIFT(58),
  [135] = {.entry = {.count = 1, .reusable = false}}, SHIFT(59),
  [137] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [139] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_op, 2, .production_id = 6),
  [141] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_op, 2, .production_id = 6),
  [143] = {.entry = {.count = 1, .reusable = false}}, SHIFT(60),
  [145] = {.entry = {.count = 1, .reusable = false}}, SHIFT(62),
  [147] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 4, .production_id = 21),
  [149] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 4, .production_id = 21),
  [151] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 5, .production_id = 26),
  [153] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 5, .production_id = 26),
  [155] = {.entry = {.count = 1, .reusable = false}}, SHIFT(61),
  [157] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 13),
  [159] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 13),
  [161] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_latency_specifier, 2, .production_id = 13),
  [163] = {.entry = {.count = 1, .reusable = false}}, SHIFT(63),
  [165] = {.entry = {.count = 1, .reusable = false}}, SHIFT(47),
  [167] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__maybe_global_identifier, 1),
  [169] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__type, 1),
  [171] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__maybe_global_identifier, 1),
  [173] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym__maybe_global_identifier, 1), REDUCE(sym__type, 1),
  [176] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [178] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [180] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if_statement, 3, .production_id = 14),
  [182] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 3, .production_id = 14),
  [184] = {.entry = {.count = 1, .reusable = false}}, SHIFT(107),
  [186] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 3),
  [188] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [190] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [192] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [194] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [196] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__assign_left_side, 2),
  [198] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__assign_left_side, 1),
  [200] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expression_statement, 1),
  [202] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 2),
  [204] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [206] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [208] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [210] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [212] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [214] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [216] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [218] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [220] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [222] = {.entry = {.count = 1, .reusable = false}}, SHIFT(160),
  [224] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_identifier_repeat1, 2), SHIFT_REPEAT(160),
  [227] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [229] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__assign_left_side_repeat2, 3),
  [231] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if_statement, 5, .production_id = 23),
  [233] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 5, .production_id = 23),
  [235] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [237] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [239] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [241] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [243] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [245] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [247] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [249] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [251] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__statement, 2),
  [253] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__statement, 2),
  [255] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_for_statement, 5, .production_id = 24),
  [257] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_statement, 5, .production_id = 24),
  [259] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__assign_left_side_repeat2, 2),
  [261] = {.entry = {.count = 1, .reusable = false}}, SHIFT(48),
  [263] = {.entry = {.count = 1, .reusable = false}}, SHIFT(27),
  [265] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [267] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [269] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [271] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [273] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [275] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [277] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_func_call_repeat1, 2, .production_id = 25),
  [279] = {.entry = {.count = 1, .reusable = false}}, SHIFT(151),
  [281] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_decl_assign_statement, 3, .production_id = 17),
  [283] = {.entry = {.count = 1, .reusable = false}}, SHIFT(75),
  [285] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [287] = {.entry = {.count = 1, .reusable = false}}, SHIFT(45),
  [289] = {.entry = {.count = 1, .reusable = false}}, SHIFT(77),
  [291] = {.entry = {.count = 1, .reusable = false}}, SHIFT(79),
  [293] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_range, 3, .production_id = 28),
  [295] = {.entry = {.count = 1, .reusable = false}}, SHIFT(78),
  [297] = {.entry = {.count = 1, .reusable = false}}, SHIFT(76),
  [299] = {.entry = {.count = 1, .reusable = false}}, SHIFT(23),
  [301] = {.entry = {.count = 1, .reusable = false}}, SHIFT(33),
  [303] = {.entry = {.count = 1, .reusable = false}}, SHIFT(57),
  [305] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__assign_left_side_repeat1, 2),
  [307] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__assign_left_side_repeat1, 2),
  [309] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__assign_left_side_repeat1, 2), SHIFT_REPEAT(94),
  [312] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [314] = {.entry = {.count = 1, .reusable = false}}, SHIFT(116),
  [316] = {.entry = {.count = 1, .reusable = true}}, SHIFT(161),
  [318] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 1),
  [320] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2),
  [322] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 4, .production_id = 7),
  [324] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 2),
  [326] = {.entry = {.count = 1, .reusable = false}}, SHIFT(105),
  [328] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 10),
  [330] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [332] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 2, .production_id = 5),
  [334] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 18),
  [336] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 11),
  [338] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [340] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [342] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [344] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__assign_left_side, 2),
  [346] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_global_identifier_repeat1, 2), SHIFT_REPEAT(157),
  [349] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [351] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [353] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [355] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [357] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 7),
  [359] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__assign_left_side_repeat2, 2), SHIFT_REPEAT(14),
  [362] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__assign_left_side_repeat2, 2),
  [364] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__assign_left_side, 1),
  [366] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_decl_statement, 1),
  [368] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__assign_left_side, 3),
  [370] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [372] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2, .production_id = 2),
  [374] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_interface_ports_repeat1, 2), SHIFT_REPEAT(99),
  [377] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_interface_ports_repeat1, 2),
  [379] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [381] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(162),
  [384] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [386] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [388] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [390] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [392] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [394] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [396] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 5, .production_id = 19),
  [398] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 4, .production_id = 12),
  [400] = {.entry = {.count = 1, .reusable = true}}, SHIFT(135),
  [402] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [404] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_func_call_repeat1, 2, .production_id = 27), SHIFT_REPEAT(48),
  [407] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_func_call_repeat1, 2, .production_id = 27),
  [409] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 5, .production_id = 20),
  [411] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 4),
  [413] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 4, .production_id = 9),
  [415] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [417] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [419] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__assign_left_side_repeat2, 3),
  [421] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 6, .production_id = 22),
  [423] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [425] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 3, .production_id = 1),
  [427] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type, 2, .production_id = 6),
  [429] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 4, .production_id = 3),
  [431] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [433] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [435] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [437] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [439] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [441] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [443] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [445] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [447] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [449] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [451] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_sus(void) {
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
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
