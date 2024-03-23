#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 163
#define LARGE_STATE_COUNT 9
#define SYMBOL_COUNT 70
#define ALIAS_COUNT 0
#define TOKEN_COUNT 40
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 25
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 30

enum {
  anon_sym_COLON = 1,
  anon_sym_COMMA = 2,
  anon_sym_DASH_GT = 3,
  anon_sym_module = 4,
  sym_identifier = 5,
  sym_number = 6,
  anon_sym_COLON_COLON = 7,
  anon_sym_LBRACK = 8,
  anon_sym_RBRACK = 9,
  anon_sym_state = 10,
  anon_sym_gen = 11,
  anon_sym_SQUOTE = 12,
  anon_sym_PLUS = 13,
  anon_sym_DASH = 14,
  anon_sym_STAR = 15,
  anon_sym_BANG = 16,
  anon_sym_PIPE = 17,
  anon_sym_AMP = 18,
  anon_sym_CARET = 19,
  anon_sym_EQ_EQ = 20,
  anon_sym_BANG_EQ = 21,
  anon_sym_LT = 22,
  anon_sym_LT_EQ = 23,
  anon_sym_GT = 24,
  anon_sym_GT_EQ = 25,
  anon_sym_SLASH = 26,
  anon_sym_PERCENT = 27,
  anon_sym_LPAREN = 28,
  anon_sym_RPAREN = 29,
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
  sym_declaration = 47,
  sym_unary_op = 48,
  sym_binary_op = 49,
  sym_array_op = 50,
  sym_func_call = 51,
  sym_parenthesis_expression = 52,
  sym__expression = 53,
  sym_range = 54,
  sym_block = 55,
  sym__assign_left_side = 56,
  sym_decl_assign_statement = 57,
  sym_decl_statement = 58,
  sym_expression_statement = 59,
  sym_if_statement = 60,
  sym_for_statement = 61,
  sym__statement = 62,
  aux_sym_source_file_repeat1 = 63,
  aux_sym_interface_ports_repeat1 = 64,
  aux_sym_global_identifier_repeat1 = 65,
  aux_sym_func_call_repeat1 = 66,
  aux_sym_block_repeat1 = 67,
  aux_sym__assign_left_side_repeat1 = 68,
  aux_sym__assign_left_side_repeat2 = 69,
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
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
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
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
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
  [sym_declaration] = "declaration",
  [sym_unary_op] = "unary_op",
  [sym_binary_op] = "binary_op",
  [sym_array_op] = "array_op",
  [sym_func_call] = "func_call",
  [sym_parenthesis_expression] = "parenthesis_expression",
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
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
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
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
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
  [sym_declaration] = sym_declaration,
  [sym_unary_op] = sym_unary_op,
  [sym_binary_op] = sym_binary_op,
  [sym_array_op] = sym_array_op,
  [sym_func_call] = sym_func_call,
  [sym_parenthesis_expression] = sym_parenthesis_expression,
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
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
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
  [anon_sym_LPAREN] = {
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
  field_array_element_type = 4,
  field_array_size = 5,
  field_assign_to = 6,
  field_assign_value = 7,
  field_block = 8,
  field_condition = 9,
  field_declaration_modifiers = 10,
  field_else_block = 11,
  field_for_decl = 12,
  field_for_range = 13,
  field_from = 14,
  field_inputs = 15,
  field_interface_ports = 16,
  field_latency_spec = 17,
  field_left = 18,
  field_name = 19,
  field_operator = 20,
  field_outputs = 21,
  field_right = 22,
  field_then_block = 23,
  field_to = 24,
  field_type = 25,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_argument] = "argument",
  [field_arr] = "arr",
  [field_arr_idx] = "arr_idx",
  [field_array_element_type] = "array_element_type",
  [field_array_size] = "array_size",
  [field_assign_to] = "assign_to",
  [field_assign_value] = "assign_value",
  [field_block] = "block",
  [field_condition] = "condition",
  [field_declaration_modifiers] = "declaration_modifiers",
  [field_else_block] = "else_block",
  [field_for_decl] = "for_decl",
  [field_for_range] = "for_range",
  [field_from] = "from",
  [field_inputs] = "inputs",
  [field_interface_ports] = "interface_ports",
  [field_latency_spec] = "latency_spec",
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
  [9] = {.index = 15, .length = 3},
  [10] = {.index = 18, .length = 2},
  [11] = {.index = 20, .length = 1},
  [12] = {.index = 21, .length = 2},
  [13] = {.index = 23, .length = 1},
  [14] = {.index = 24, .length = 3},
  [15] = {.index = 27, .length = 2},
  [16] = {.index = 29, .length = 3},
  [17] = {.index = 32, .length = 2},
  [18] = {.index = 34, .length = 3},
  [19] = {.index = 37, .length = 3},
  [20] = {.index = 40, .length = 2},
  [21] = {.index = 42, .length = 2},
  [22] = {.index = 44, .length = 4},
  [23] = {.index = 48, .length = 4},
  [24] = {.index = 52, .length = 3},
  [25] = {.index = 55, .length = 3},
  [26] = {.index = 58, .length = 1},
  [27] = {.index = 59, .length = 3},
  [28] = {.index = 62, .length = 2},
  [29] = {.index = 64, .length = 2},
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
    {field_inputs, 1},
    {field_inputs, 2},
  [11] =
    {field_operator, 0},
    {field_right, 1},
  [13] =
    {field_outputs, 2},
    {field_outputs, 3},
  [15] =
    {field_declaration_modifiers, 0},
    {field_name, 2},
    {field_type, 1},
  [18] =
    {field_inputs, 1},
    {field_outputs, 3},
  [20] =
    {field_right, 1},
  [21] =
    {field_condition, 1},
    {field_then_block, 2},
  [23] =
    {field_name, 0},
  [24] =
    {field_left, 0},
    {field_operator, 1},
    {field_right, 2},
  [27] =
    {field_assign_to, 0},
    {field_assign_value, 2},
  [29] =
    {field_latency_spec, 3},
    {field_name, 1},
    {field_type, 0},
  [32] =
    {field_array_element_type, 0},
    {field_array_size, 2},
  [34] =
    {field_inputs, 1},
    {field_outputs, 3},
    {field_outputs, 4},
  [37] =
    {field_inputs, 1},
    {field_inputs, 2},
    {field_outputs, 4},
  [40] =
    {field_argument, 2},
    {field_name, 0},
  [42] =
    {field_arr, 0},
    {field_arr_idx, 2},
  [44] =
    {field_declaration_modifiers, 0},
    {field_latency_spec, 4},
    {field_name, 2},
    {field_type, 1},
  [48] =
    {field_inputs, 1},
    {field_inputs, 2},
    {field_outputs, 4},
    {field_outputs, 5},
  [52] =
    {field_condition, 1},
    {field_else_block, 4},
    {field_then_block, 2},
  [55] =
    {field_block, 4},
    {field_for_decl, 1},
    {field_for_range, 3},
  [58] =
    {field_argument, 1},
  [59] =
    {field_argument, 2},
    {field_argument, 3, .inherited = true},
    {field_name, 0},
  [62] =
    {field_argument, 0, .inherited = true},
    {field_argument, 1, .inherited = true},
  [64] =
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
  [44] = 11,
  [45] = 45,
  [46] = 10,
  [47] = 47,
  [48] = 13,
  [49] = 14,
  [50] = 47,
  [51] = 51,
  [52] = 43,
  [53] = 53,
  [54] = 41,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 56,
  [63] = 55,
  [64] = 64,
  [65] = 65,
  [66] = 61,
  [67] = 60,
  [68] = 59,
  [69] = 58,
  [70] = 39,
  [71] = 15,
  [72] = 72,
  [73] = 73,
  [74] = 74,
  [75] = 75,
  [76] = 76,
  [77] = 42,
  [78] = 78,
  [79] = 79,
  [80] = 80,
  [81] = 81,
  [82] = 16,
  [83] = 83,
  [84] = 84,
  [85] = 30,
  [86] = 86,
  [87] = 87,
  [88] = 33,
  [89] = 32,
  [90] = 90,
  [91] = 91,
  [92] = 35,
  [93] = 37,
  [94] = 29,
  [95] = 31,
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
  [107] = 106,
  [108] = 10,
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
  [122] = 11,
  [123] = 14,
  [124] = 13,
  [125] = 125,
  [126] = 126,
  [127] = 127,
  [128] = 16,
  [129] = 129,
  [130] = 130,
  [131] = 131,
  [132] = 132,
  [133] = 133,
  [134] = 134,
  [135] = 135,
  [136] = 136,
  [137] = 137,
  [138] = 39,
  [139] = 139,
  [140] = 140,
  [141] = 141,
  [142] = 41,
  [143] = 139,
  [144] = 144,
  [145] = 145,
  [146] = 146,
  [147] = 147,
  [148] = 148,
  [149] = 105,
  [150] = 104,
  [151] = 146,
  [152] = 152,
  [153] = 153,
  [154] = 154,
  [155] = 155,
  [156] = 156,
  [157] = 152,
  [158] = 158,
  [159] = 153,
  [160] = 153,
  [161] = 152,
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
      if (lookahead == '!') ADVANCE(107);
      if (lookahead == '%') ADVANCE(118);
      if (lookahead == '&') ADVANCE(109);
      if (lookahead == '\'') ADVANCE(101);
      if (lookahead == '(') ADVANCE(119);
      if (lookahead == ')') ADVANCE(120);
      if (lookahead == '*') ADVANCE(105);
      if (lookahead == '+') ADVANCE(102);
      if (lookahead == ',') ADVANCE(69);
      if (lookahead == '-') ADVANCE(104);
      if (lookahead == '/') SKIP(63)
      if (lookahead == ':') ADVANCE(68);
      if (lookahead == ';') ADVANCE(134);
      if (lookahead == '<') ADVANCE(113);
      if (lookahead == '=') ADVANCE(126);
      if (lookahead == '>') ADVANCE(115);
      if (lookahead == '[') ADVANCE(95);
      if (lookahead == ']') ADVANCE(96);
      if (lookahead == '^') ADVANCE(110);
      if (lookahead == 'e') ADVANCE(51);
      if (lookahead == 'f') ADVANCE(55);
      if (lookahead == 'g') ADVANCE(44);
      if (lookahead == 'i') ADVANCE(49);
      if (lookahead == 'm') ADVANCE(56);
      if (lookahead == 'r') ADVANCE(45);
      if (lookahead == 's') ADVANCE(59);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(108);
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
      if (lookahead == '!') ADVANCE(106);
      if (lookahead == '&') ADVANCE(109);
      if (lookahead == '(') ADVANCE(119);
      if (lookahead == ')') ADVANCE(120);
      if (lookahead == '*') ADVANCE(105);
      if (lookahead == '+') ADVANCE(102);
      if (lookahead == '-') ADVANCE(103);
      if (lookahead == '/') SKIP(35)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '[') ADVANCE(95);
      if (lookahead == '^') ADVANCE(110);
      if (lookahead == '|') ADVANCE(108);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(8)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(93);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(92);
      END_STATE();
    case 9:
      if (lookahead == '!') ADVANCE(106);
      if (lookahead == '&') ADVANCE(109);
      if (lookahead == '(') ADVANCE(119);
      if (lookahead == '*') ADVANCE(105);
      if (lookahead == '+') ADVANCE(102);
      if (lookahead == '-') ADVANCE(104);
      if (lookahead == '/') SKIP(34)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '^') ADVANCE(110);
      if (lookahead == 'g') ADVANCE(74);
      if (lookahead == 's') ADVANCE(89);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(108);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(9)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(93);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(92);
      END_STATE();
    case 10:
      if (lookahead == '!') ADVANCE(106);
      if (lookahead == '&') ADVANCE(109);
      if (lookahead == '(') ADVANCE(119);
      if (lookahead == '*') ADVANCE(105);
      if (lookahead == '+') ADVANCE(102);
      if (lookahead == '-') ADVANCE(103);
      if (lookahead == '/') SKIP(17)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '^') ADVANCE(110);
      if (lookahead == 'f') ADVANCE(86);
      if (lookahead == 'g') ADVANCE(74);
      if (lookahead == 'i') ADVANCE(78);
      if (lookahead == 'r') ADVANCE(75);
      if (lookahead == 's') ADVANCE(89);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(108);
      if (lookahead == '}') ADVANCE(122);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(10)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(93);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(92);
      END_STATE();
    case 11:
      if (lookahead == '!') ADVANCE(106);
      if (lookahead == '&') ADVANCE(109);
      if (lookahead == '(') ADVANCE(119);
      if (lookahead == '*') ADVANCE(105);
      if (lookahead == '+') ADVANCE(102);
      if (lookahead == '-') ADVANCE(103);
      if (lookahead == '/') SKIP(32)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '^') ADVANCE(110);
      if (lookahead == 'g') ADVANCE(74);
      if (lookahead == 'i') ADVANCE(84);
      if (lookahead == 'r') ADVANCE(75);
      if (lookahead == 's') ADVANCE(89);
      if (lookahead == '|') ADVANCE(108);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(11)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(93);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(92);
      END_STATE();
    case 12:
      if (lookahead == '!') ADVANCE(106);
      if (lookahead == '&') ADVANCE(109);
      if (lookahead == '(') ADVANCE(119);
      if (lookahead == '*') ADVANCE(105);
      if (lookahead == '+') ADVANCE(102);
      if (lookahead == '-') ADVANCE(103);
      if (lookahead == '/') SKIP(33)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '^') ADVANCE(110);
      if (lookahead == 'g') ADVANCE(74);
      if (lookahead == 'r') ADVANCE(75);
      if (lookahead == 's') ADVANCE(89);
      if (lookahead == '|') ADVANCE(108);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(12)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(93);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(92);
      END_STATE();
    case 13:
      if (lookahead == '!') ADVANCE(106);
      if (lookahead == '&') ADVANCE(109);
      if (lookahead == '(') ADVANCE(119);
      if (lookahead == '*') ADVANCE(105);
      if (lookahead == '+') ADVANCE(102);
      if (lookahead == '-') ADVANCE(103);
      if (lookahead == '/') SKIP(36)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '^') ADVANCE(110);
      if (lookahead == 'e') ADVANCE(83);
      if (lookahead == 'f') ADVANCE(86);
      if (lookahead == 'g') ADVANCE(74);
      if (lookahead == 'i') ADVANCE(78);
      if (lookahead == 'r') ADVANCE(75);
      if (lookahead == 's') ADVANCE(89);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(108);
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
      if (lookahead == '%') ADVANCE(118);
      if (lookahead == '&') ADVANCE(109);
      if (lookahead == '(') ADVANCE(119);
      if (lookahead == ')') ADVANCE(120);
      if (lookahead == '*') ADVANCE(105);
      if (lookahead == '+') ADVANCE(102);
      if (lookahead == ',') ADVANCE(69);
      if (lookahead == '-') ADVANCE(104);
      if (lookahead == '/') ADVANCE(117);
      if (lookahead == ':') ADVANCE(68);
      if (lookahead == ';') ADVANCE(134);
      if (lookahead == '<') ADVANCE(113);
      if (lookahead == '=') ADVANCE(126);
      if (lookahead == '>') ADVANCE(115);
      if (lookahead == '[') ADVANCE(95);
      if (lookahead == ']') ADVANCE(96);
      if (lookahead == '^') ADVANCE(110);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(108);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(14)
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(92);
      END_STATE();
    case 15:
      if (lookahead == '!') ADVANCE(40);
      if (lookahead == '%') ADVANCE(118);
      if (lookahead == '&') ADVANCE(109);
      if (lookahead == '(') ADVANCE(119);
      if (lookahead == ')') ADVANCE(120);
      if (lookahead == '*') ADVANCE(105);
      if (lookahead == '+') ADVANCE(102);
      if (lookahead == ',') ADVANCE(69);
      if (lookahead == '-') ADVANCE(104);
      if (lookahead == '/') ADVANCE(117);
      if (lookahead == ':') ADVANCE(67);
      if (lookahead == ';') ADVANCE(134);
      if (lookahead == '<') ADVANCE(113);
      if (lookahead == '=') ADVANCE(126);
      if (lookahead == '>') ADVANCE(115);
      if (lookahead == '[') ADVANCE(95);
      if (lookahead == ']') ADVANCE(96);
      if (lookahead == '^') ADVANCE(110);
      if (lookahead == 'i') ADVANCE(53);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(108);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(15)
      END_STATE();
    case 16:
      if (lookahead == '!') ADVANCE(40);
      if (lookahead == '%') ADVANCE(118);
      if (lookahead == '&') ADVANCE(109);
      if (lookahead == '(') ADVANCE(119);
      if (lookahead == '*') ADVANCE(105);
      if (lookahead == '+') ADVANCE(102);
      if (lookahead == '-') ADVANCE(103);
      if (lookahead == '/') ADVANCE(117);
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '<') ADVANCE(113);
      if (lookahead == '=') ADVANCE(41);
      if (lookahead == '>') ADVANCE(115);
      if (lookahead == '[') ADVANCE(95);
      if (lookahead == '^') ADVANCE(110);
      if (lookahead == 'i') ADVANCE(53);
      if (lookahead == '|') ADVANCE(108);
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
      if (lookahead == '=') ADVANCE(112);
      END_STATE();
    case 41:
      if (lookahead == '=') ADVANCE(111);
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
      if (lookahead == 'e') ADVANCE(97);
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
      if (lookahead == 'n') ADVANCE(99);
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
      if (lookahead == 'e') ADVANCE(98);
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
      if (lookahead == 'n') ADVANCE(100);
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
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(anon_sym_state);
      END_STATE();
    case 98:
      ACCEPT_TOKEN(anon_sym_state);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 99:
      ACCEPT_TOKEN(anon_sym_gen);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(anon_sym_gen);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(92);
      END_STATE();
    case 101:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 104:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '>') ADVANCE(70);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 107:
      ACCEPT_TOKEN(anon_sym_BANG);
      if (lookahead == '=') ADVANCE(112);
      END_STATE();
    case 108:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 109:
      ACCEPT_TOKEN(anon_sym_AMP);
      END_STATE();
    case 110:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 111:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 112:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 113:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(114);
      END_STATE();
    case 114:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 115:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(116);
      END_STATE();
    case 116:
      ACCEPT_TOKEN(anon_sym_GT_EQ);
      END_STATE();
    case 117:
      ACCEPT_TOKEN(anon_sym_SLASH);
      END_STATE();
    case 118:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 119:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 120:
      ACCEPT_TOKEN(anon_sym_RPAREN);
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
      if (lookahead == '=') ADVANCE(111);
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
  [12] = {.lex_state = 11},
  [13] = {.lex_state = 14},
  [14] = {.lex_state = 14},
  [15] = {.lex_state = 14},
  [16] = {.lex_state = 14},
  [17] = {.lex_state = 12},
  [18] = {.lex_state = 12},
  [19] = {.lex_state = 15},
  [20] = {.lex_state = 15},
  [21] = {.lex_state = 15},
  [22] = {.lex_state = 15},
  [23] = {.lex_state = 9},
  [24] = {.lex_state = 15},
  [25] = {.lex_state = 15},
  [26] = {.lex_state = 15},
  [27] = {.lex_state = 15},
  [28] = {.lex_state = 9},
  [29] = {.lex_state = 15},
  [30] = {.lex_state = 15},
  [31] = {.lex_state = 15},
  [32] = {.lex_state = 15},
  [33] = {.lex_state = 15},
  [34] = {.lex_state = 14},
  [35] = {.lex_state = 14},
  [36] = {.lex_state = 8},
  [37] = {.lex_state = 14},
  [38] = {.lex_state = 13},
  [39] = {.lex_state = 13},
  [40] = {.lex_state = 8},
  [41] = {.lex_state = 13},
  [42] = {.lex_state = 8},
  [43] = {.lex_state = 8},
  [44] = {.lex_state = 16},
  [45] = {.lex_state = 8},
  [46] = {.lex_state = 16},
  [47] = {.lex_state = 8},
  [48] = {.lex_state = 16},
  [49] = {.lex_state = 16},
  [50] = {.lex_state = 8},
  [51] = {.lex_state = 14},
  [52] = {.lex_state = 8},
  [53] = {.lex_state = 8},
  [54] = {.lex_state = 10},
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
  [65] = {.lex_state = 10},
  [66] = {.lex_state = 8},
  [67] = {.lex_state = 8},
  [68] = {.lex_state = 8},
  [69] = {.lex_state = 8},
  [70] = {.lex_state = 10},
  [71] = {.lex_state = 16},
  [72] = {.lex_state = 8},
  [73] = {.lex_state = 14},
  [74] = {.lex_state = 10},
  [75] = {.lex_state = 8},
  [76] = {.lex_state = 10},
  [77] = {.lex_state = 8},
  [78] = {.lex_state = 14},
  [79] = {.lex_state = 14},
  [80] = {.lex_state = 14},
  [81] = {.lex_state = 14},
  [82] = {.lex_state = 16},
  [83] = {.lex_state = 14},
  [84] = {.lex_state = 15},
  [85] = {.lex_state = 15},
  [86] = {.lex_state = 14},
  [87] = {.lex_state = 14},
  [88] = {.lex_state = 15},
  [89] = {.lex_state = 15},
  [90] = {.lex_state = 14},
  [91] = {.lex_state = 14},
  [92] = {.lex_state = 15},
  [93] = {.lex_state = 15},
  [94] = {.lex_state = 15},
  [95] = {.lex_state = 15},
  [96] = {.lex_state = 14},
  [97] = {.lex_state = 12},
  [98] = {.lex_state = 9},
  [99] = {.lex_state = 9},
  [100] = {.lex_state = 9},
  [101] = {.lex_state = 9},
  [102] = {.lex_state = 9},
  [103] = {.lex_state = 9},
  [104] = {.lex_state = 0},
  [105] = {.lex_state = 0},
  [106] = {.lex_state = 8},
  [107] = {.lex_state = 8},
  [108] = {.lex_state = 8},
  [109] = {.lex_state = 8},
  [110] = {.lex_state = 0},
  [111] = {.lex_state = 38},
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
  [122] = {.lex_state = 8},
  [123] = {.lex_state = 8},
  [124] = {.lex_state = 8},
  [125] = {.lex_state = 0},
  [126] = {.lex_state = 0},
  [127] = {.lex_state = 0},
  [128] = {.lex_state = 8},
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
  [139] = {.lex_state = 8},
  [140] = {.lex_state = 0},
  [141] = {.lex_state = 8},
  [142] = {.lex_state = 0},
  [143] = {.lex_state = 8},
  [144] = {.lex_state = 0},
  [145] = {.lex_state = 0},
  [146] = {.lex_state = 8},
  [147] = {.lex_state = 0},
  [148] = {.lex_state = 0},
  [149] = {.lex_state = 0},
  [150] = {.lex_state = 0},
  [151] = {.lex_state = 8},
  [152] = {.lex_state = 8},
  [153] = {.lex_state = 8},
  [154] = {.lex_state = 0},
  [155] = {.lex_state = 8},
  [156] = {.lex_state = 0},
  [157] = {.lex_state = 8},
  [158] = {.lex_state = 0},
  [159] = {.lex_state = 8},
  [160] = {.lex_state = 8},
  [161] = {.lex_state = 8},
  [162] = {.lex_state = 0},
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
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
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
    [anon_sym_PERCENT] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
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
    [sym_source_file] = STATE(158),
    [sym_module] = STATE(110),
    [aux_sym_source_file_repeat1] = STATE(110),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_module] = ACTIONS(5),
  },
  [2] = {
    [sym_global_identifier] = STATE(34),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(151),
    [sym__type] = STATE(151),
    [sym_declaration] = STATE(118),
    [sym_unary_op] = STATE(73),
    [sym_binary_op] = STATE(73),
    [sym_array_op] = STATE(73),
    [sym_func_call] = STATE(73),
    [sym_parenthesis_expression] = STATE(73),
    [sym__expression] = STATE(73),
    [sym_block] = STATE(6),
    [sym__assign_left_side] = STATE(154),
    [sym_decl_assign_statement] = STATE(162),
    [sym_decl_statement] = STATE(162),
    [sym_expression_statement] = STATE(162),
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
    [sym_global_identifier] = STATE(34),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(151),
    [sym__type] = STATE(151),
    [sym_declaration] = STATE(118),
    [sym_unary_op] = STATE(73),
    [sym_binary_op] = STATE(73),
    [sym_array_op] = STATE(73),
    [sym_func_call] = STATE(73),
    [sym_parenthesis_expression] = STATE(73),
    [sym__expression] = STATE(73),
    [sym_block] = STATE(2),
    [sym__assign_left_side] = STATE(154),
    [sym_decl_assign_statement] = STATE(162),
    [sym_decl_statement] = STATE(162),
    [sym_expression_statement] = STATE(162),
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
    [sym_global_identifier] = STATE(34),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(151),
    [sym__type] = STATE(151),
    [sym_declaration] = STATE(118),
    [sym_unary_op] = STATE(73),
    [sym_binary_op] = STATE(73),
    [sym_array_op] = STATE(73),
    [sym_func_call] = STATE(73),
    [sym_parenthesis_expression] = STATE(73),
    [sym__expression] = STATE(73),
    [sym_block] = STATE(6),
    [sym__assign_left_side] = STATE(154),
    [sym_decl_assign_statement] = STATE(162),
    [sym_decl_statement] = STATE(162),
    [sym_expression_statement] = STATE(162),
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
    [sym_global_identifier] = STATE(34),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(151),
    [sym__type] = STATE(151),
    [sym_declaration] = STATE(118),
    [sym_unary_op] = STATE(73),
    [sym_binary_op] = STATE(73),
    [sym_array_op] = STATE(73),
    [sym_func_call] = STATE(73),
    [sym_parenthesis_expression] = STATE(73),
    [sym__expression] = STATE(73),
    [sym_block] = STATE(4),
    [sym__assign_left_side] = STATE(154),
    [sym_decl_assign_statement] = STATE(162),
    [sym_decl_statement] = STATE(162),
    [sym_expression_statement] = STATE(162),
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
    [sym_global_identifier] = STATE(34),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(151),
    [sym__type] = STATE(151),
    [sym_declaration] = STATE(118),
    [sym_unary_op] = STATE(73),
    [sym_binary_op] = STATE(73),
    [sym_array_op] = STATE(73),
    [sym_func_call] = STATE(73),
    [sym_parenthesis_expression] = STATE(73),
    [sym__expression] = STATE(73),
    [sym_block] = STATE(6),
    [sym__assign_left_side] = STATE(154),
    [sym_decl_assign_statement] = STATE(162),
    [sym_decl_statement] = STATE(162),
    [sym_expression_statement] = STATE(162),
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
    [sym_global_identifier] = STATE(34),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(151),
    [sym__type] = STATE(151),
    [sym_declaration] = STATE(118),
    [sym_unary_op] = STATE(73),
    [sym_binary_op] = STATE(73),
    [sym_array_op] = STATE(73),
    [sym_func_call] = STATE(73),
    [sym_parenthesis_expression] = STATE(73),
    [sym__expression] = STATE(73),
    [sym_block] = STATE(6),
    [sym__assign_left_side] = STATE(154),
    [sym_decl_assign_statement] = STATE(162),
    [sym_decl_statement] = STATE(162),
    [sym_expression_statement] = STATE(162),
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
    [sym_global_identifier] = STATE(34),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(151),
    [sym__type] = STATE(151),
    [sym_declaration] = STATE(118),
    [sym_unary_op] = STATE(73),
    [sym_binary_op] = STATE(73),
    [sym_array_op] = STATE(73),
    [sym_func_call] = STATE(73),
    [sym_parenthesis_expression] = STATE(73),
    [sym__expression] = STATE(73),
    [sym_block] = STATE(7),
    [sym__assign_left_side] = STATE(154),
    [sym_decl_assign_statement] = STATE(162),
    [sym_decl_statement] = STATE(162),
    [sym_expression_statement] = STATE(162),
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
    STATE(34), 1,
      sym_global_identifier,
    STATE(136), 1,
      sym_declaration,
    STATE(147), 1,
      sym__assign_left_side,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(151), 2,
      sym_array_type,
      sym__type,
    STATE(79), 6,
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
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [131] = 14,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(91), 1,
      sym_number,
    ACTIONS(93), 1,
      anon_sym_reg,
    ACTIONS(95), 1,
      anon_sym_initial,
    STATE(17), 1,
      aux_sym__assign_left_side_repeat1,
    STATE(19), 1,
      sym__maybe_global_identifier,
    STATE(34), 1,
      sym_global_identifier,
    STATE(127), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(151), 2,
      sym_array_type,
      sym__type,
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
  [187] = 4,
    ACTIONS(87), 1,
      anon_sym_COLON_COLON,
    ACTIONS(99), 1,
      anon_sym_SLASH,
    STATE(11), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(97), 24,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [223] = 4,
    ACTIONS(87), 1,
      anon_sym_COLON_COLON,
    ACTIONS(99), 1,
      anon_sym_SLASH,
    STATE(10), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(97), 24,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [259] = 4,
    ACTIONS(87), 1,
      anon_sym_COLON_COLON,
    ACTIONS(103), 1,
      anon_sym_SLASH,
    STATE(14), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(101), 24,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
    STATE(34), 1,
      sym_global_identifier,
    STATE(97), 1,
      aux_sym__assign_left_side_repeat1,
    STATE(131), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(151), 2,
      sym_array_type,
      sym__type,
    STATE(78), 6,
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
    STATE(34), 1,
      sym_global_identifier,
    STATE(97), 1,
      aux_sym__assign_left_side_repeat1,
    STATE(117), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(151), 2,
      sym_array_type,
      sym__type,
    STATE(51), 6,
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
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [464] = 2,
    ACTIONS(119), 1,
      anon_sym_SLASH,
    ACTIONS(117), 23,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [493] = 2,
    ACTIONS(123), 1,
      anon_sym_SLASH,
    ACTIONS(121), 23,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [522] = 2,
    ACTIONS(127), 1,
      anon_sym_SLASH,
    ACTIONS(125), 23,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [551] = 11,
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
    STATE(34), 1,
      sym_global_identifier,
    STATE(117), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(151), 2,
      sym_array_type,
      sym__type,
    STATE(51), 6,
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
  [598] = 2,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(129), 23,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [627] = 2,
    ACTIONS(135), 1,
      anon_sym_SLASH,
    ACTIONS(133), 23,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [656] = 2,
    ACTIONS(139), 1,
      anon_sym_SLASH,
    ACTIONS(137), 23,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [685] = 2,
    ACTIONS(143), 1,
      anon_sym_SLASH,
    ACTIONS(141), 23,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [714] = 11,
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
    STATE(34), 1,
      sym_global_identifier,
    STATE(131), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(151), 2,
      sym_array_type,
      sym__type,
    STATE(78), 6,
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
  [761] = 7,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(151), 1,
      anon_sym_AMP,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(133), 15,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [799] = 4,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(133), 18,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [831] = 6,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(133), 16,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [867] = 5,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(133), 17,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [901] = 3,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(133), 20,
      anon_sym_COLON,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [931] = 4,
    ACTIONS(159), 1,
      sym_identifier,
    ACTIONS(161), 1,
      anon_sym_LBRACK,
    ACTIONS(164), 1,
      anon_sym_SLASH,
    ACTIONS(157), 18,
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
  [961] = 9,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(151), 1,
      anon_sym_AMP,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(166), 5,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
    ACTIONS(170), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1000] = 8,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(174), 1,
      sym_number,
    STATE(140), 1,
      sym_range,
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
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1037] = 9,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(151), 1,
      anon_sym_AMP,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(176), 5,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
    ACTIONS(170), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1076] = 3,
    ACTIONS(182), 1,
      anon_sym_else,
    ACTIONS(178), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(180), 12,
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
  [1103] = 2,
    ACTIONS(184), 8,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_else,
      anon_sym_for,
    ACTIONS(186), 12,
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
  [1128] = 8,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(188), 1,
      sym_number,
    ACTIONS(190), 1,
      anon_sym_RPAREN,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(81), 6,
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
  [1165] = 2,
    ACTIONS(192), 8,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_else,
      anon_sym_for,
    ACTIONS(194), 12,
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
  [1190] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(196), 1,
      sym_identifier,
    ACTIONS(198), 1,
      sym_number,
    ACTIONS(200), 1,
      anon_sym_COLON_COLON,
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
    ACTIONS(202), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1224] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(204), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(37), 6,
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
  [1258] = 4,
    ACTIONS(89), 1,
      anon_sym_SLASH,
    ACTIONS(206), 1,
      anon_sym_COLON_COLON,
    STATE(46), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(85), 16,
      anon_sym_LBRACK,
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
      anon_sym_in,
  [1286] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(208), 1,
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
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1320] = 4,
    ACTIONS(83), 1,
      anon_sym_SLASH,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    STATE(46), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(78), 16,
      anon_sym_LBRACK,
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
      anon_sym_in,
  [1348] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(213), 1,
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
  [1382] = 4,
    ACTIONS(99), 1,
      anon_sym_SLASH,
    ACTIONS(206), 1,
      anon_sym_COLON_COLON,
    STATE(44), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(97), 16,
      anon_sym_LBRACK,
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
      anon_sym_in,
  [1410] = 4,
    ACTIONS(99), 1,
      anon_sym_SLASH,
    ACTIONS(206), 1,
      anon_sym_COLON_COLON,
    STATE(46), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(97), 16,
      anon_sym_LBRACK,
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
      anon_sym_in,
  [1438] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(196), 1,
      sym_identifier,
    ACTIONS(200), 1,
      anon_sym_COLON_COLON,
    ACTIONS(215), 1,
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
    ACTIONS(202), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1472] = 11,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(151), 1,
      anon_sym_AMP,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(217), 1,
      anon_sym_COMMA,
    STATE(114), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(219), 2,
      anon_sym_LBRACE,
      anon_sym_EQ,
    ACTIONS(170), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1514] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(196), 1,
      sym_identifier,
    ACTIONS(200), 1,
      anon_sym_COLON_COLON,
    ACTIONS(221), 1,
      sym_number,
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
    ACTIONS(202), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1548] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(223), 1,
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
  [1582] = 2,
    ACTIONS(192), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(194), 12,
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
  [1606] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(196), 1,
      sym_identifier,
    ACTIONS(200), 1,
      anon_sym_COLON_COLON,
    ACTIONS(225), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(94), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(202), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1640] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(196), 1,
      sym_identifier,
    ACTIONS(200), 1,
      anon_sym_COLON_COLON,
    ACTIONS(227), 1,
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
    ACTIONS(202), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1674] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(229), 1,
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
    ACTIONS(15), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1708] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(231), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(33), 6,
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
  [1742] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(233), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(25), 6,
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
  [1776] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(235), 1,
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
  [1810] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(237), 1,
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
  [1844] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(239), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(30), 6,
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
  [1878] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(241), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(29), 6,
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
  [1912] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(243), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(96), 6,
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
  [1946] = 2,
    ACTIONS(245), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(247), 12,
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
  [1970] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(196), 1,
      sym_identifier,
    ACTIONS(200), 1,
      anon_sym_COLON_COLON,
    ACTIONS(249), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(95), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(202), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2004] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(196), 1,
      sym_identifier,
    ACTIONS(200), 1,
      anon_sym_COLON_COLON,
    ACTIONS(251), 1,
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
    ACTIONS(202), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2038] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(196), 1,
      sym_identifier,
    ACTIONS(200), 1,
      anon_sym_COLON_COLON,
    ACTIONS(233), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(25), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(202), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2072] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(196), 1,
      sym_identifier,
    ACTIONS(200), 1,
      anon_sym_COLON_COLON,
    ACTIONS(253), 1,
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
    ACTIONS(202), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2106] = 2,
    ACTIONS(184), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(186), 12,
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
  [2130] = 4,
    ACTIONS(103), 1,
      anon_sym_SLASH,
    ACTIONS(206), 1,
      anon_sym_COLON_COLON,
    STATE(49), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(101), 16,
      anon_sym_LBRACK,
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
      anon_sym_in,
  [2158] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(255), 1,
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
  [2192] = 12,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(151), 1,
      anon_sym_AMP,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(217), 1,
      anon_sym_COMMA,
    ACTIONS(257), 1,
      anon_sym_EQ,
    ACTIONS(259), 1,
      anon_sym_SEMI,
    STATE(121), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(170), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2236] = 2,
    ACTIONS(261), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(263), 12,
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
  [2260] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(265), 1,
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
  [2294] = 2,
    ACTIONS(267), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(269), 12,
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
  [2318] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(172), 1,
      sym_identifier,
    ACTIONS(198), 1,
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
  [2352] = 9,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(151), 1,
      anon_sym_AMP,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(271), 3,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
    ACTIONS(170), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2389] = 11,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(151), 1,
      anon_sym_AMP,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(217), 1,
      anon_sym_COMMA,
    ACTIONS(257), 1,
      anon_sym_LBRACE,
    STATE(121), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(170), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2430] = 9,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(151), 1,
      anon_sym_AMP,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(273), 3,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
    ACTIONS(170), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2467] = 11,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(151), 1,
      anon_sym_AMP,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(275), 1,
      anon_sym_COMMA,
    ACTIONS(277), 1,
      anon_sym_RPAREN,
    STATE(130), 1,
      aux_sym_func_call_repeat1,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(170), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2508] = 2,
    ACTIONS(83), 1,
      anon_sym_SLASH,
    ACTIONS(78), 17,
      anon_sym_COLON_COLON,
      anon_sym_LBRACK,
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
      anon_sym_in,
  [2531] = 9,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(151), 1,
      anon_sym_AMP,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(279), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
    ACTIONS(170), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2567] = 9,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(151), 1,
      anon_sym_AMP,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(281), 1,
      anon_sym_COLON,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(170), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2602] = 4,
    ACTIONS(287), 1,
      anon_sym_SLASH,
    ACTIONS(283), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(285), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(133), 11,
      anon_sym_LBRACK,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_in,
  [2627] = 9,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(151), 1,
      anon_sym_AMP,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(289), 1,
      anon_sym_RBRACK,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(170), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2662] = 9,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(151), 1,
      anon_sym_AMP,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(291), 1,
      anon_sym_LBRACE,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(170), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2697] = 3,
    ACTIONS(287), 1,
      anon_sym_SLASH,
    ACTIONS(285), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(133), 13,
      anon_sym_LBRACK,
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
      anon_sym_in,
  [2720] = 5,
    ACTIONS(287), 1,
      anon_sym_SLASH,
    ACTIONS(293), 1,
      anon_sym_CARET,
    ACTIONS(283), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(285), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(133), 10,
      anon_sym_LBRACK,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_in,
  [2747] = 9,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(151), 1,
      anon_sym_AMP,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(295), 1,
      anon_sym_RBRACK,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(170), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2782] = 9,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(151), 1,
      anon_sym_AMP,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(297), 1,
      anon_sym_RPAREN,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(170), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2817] = 9,
    ACTIONS(166), 1,
      anon_sym_in,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(287), 1,
      anon_sym_SLASH,
    ACTIONS(293), 1,
      anon_sym_CARET,
    ACTIONS(299), 1,
      anon_sym_PIPE,
    ACTIONS(301), 1,
      anon_sym_AMP,
    ACTIONS(283), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(285), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(303), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2852] = 9,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(176), 1,
      anon_sym_in,
    ACTIONS(287), 1,
      anon_sym_SLASH,
    ACTIONS(293), 1,
      anon_sym_CARET,
    ACTIONS(299), 1,
      anon_sym_PIPE,
    ACTIONS(301), 1,
      anon_sym_AMP,
    ACTIONS(283), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(285), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(303), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2887] = 7,
    ACTIONS(287), 1,
      anon_sym_SLASH,
    ACTIONS(293), 1,
      anon_sym_CARET,
    ACTIONS(299), 1,
      anon_sym_PIPE,
    ACTIONS(301), 1,
      anon_sym_AMP,
    ACTIONS(283), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(285), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(133), 8,
      anon_sym_LBRACK,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_in,
  [2918] = 6,
    ACTIONS(287), 1,
      anon_sym_SLASH,
    ACTIONS(293), 1,
      anon_sym_CARET,
    ACTIONS(299), 1,
      anon_sym_PIPE,
    ACTIONS(283), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(285), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(133), 9,
      anon_sym_LBRACK,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_in,
  [2947] = 9,
    ACTIONS(149), 1,
      anon_sym_PIPE,
    ACTIONS(151), 1,
      anon_sym_AMP,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    ACTIONS(168), 1,
      anon_sym_LBRACK,
    ACTIONS(305), 1,
      anon_sym_SEMI,
    ACTIONS(145), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(147), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(170), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2982] = 4,
    ACTIONS(311), 1,
      anon_sym_reg,
    STATE(97), 1,
      aux_sym__assign_left_side_repeat1,
    ACTIONS(307), 3,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
    ACTIONS(309), 10,
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
  [3006] = 7,
    ACTIONS(314), 1,
      anon_sym_DASH_GT,
    ACTIONS(316), 1,
      sym_identifier,
    ACTIONS(318), 1,
      anon_sym_COLON_COLON,
    ACTIONS(320), 1,
      anon_sym_LBRACE,
    STATE(119), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(151), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3031] = 6,
    ACTIONS(316), 1,
      sym_identifier,
    ACTIONS(318), 1,
      anon_sym_COLON_COLON,
    ACTIONS(322), 1,
      anon_sym_LBRACE,
    STATE(133), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(151), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3053] = 6,
    ACTIONS(316), 1,
      sym_identifier,
    ACTIONS(318), 1,
      anon_sym_COLON_COLON,
    ACTIONS(324), 1,
      anon_sym_LBRACE,
    STATE(134), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(151), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3075] = 6,
    ACTIONS(316), 1,
      sym_identifier,
    ACTIONS(318), 1,
      anon_sym_COLON_COLON,
    ACTIONS(326), 1,
      anon_sym_LBRACE,
    STATE(125), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(151), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3097] = 5,
    ACTIONS(316), 1,
      sym_identifier,
    ACTIONS(318), 1,
      anon_sym_COLON_COLON,
    STATE(156), 1,
      sym_declaration,
    ACTIONS(328), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(146), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3116] = 5,
    ACTIONS(316), 1,
      sym_identifier,
    ACTIONS(318), 1,
      anon_sym_COLON_COLON,
    STATE(126), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(151), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3135] = 2,
    ACTIONS(332), 1,
      anon_sym_SQUOTE,
    ACTIONS(330), 5,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3146] = 2,
    ACTIONS(336), 1,
      anon_sym_SQUOTE,
    ACTIONS(334), 5,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3157] = 3,
    ACTIONS(318), 1,
      anon_sym_COLON_COLON,
    ACTIONS(338), 1,
      sym_identifier,
    STATE(139), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3169] = 3,
    ACTIONS(318), 1,
      anon_sym_COLON_COLON,
    ACTIONS(338), 1,
      sym_identifier,
    STATE(143), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3181] = 3,
    ACTIONS(340), 1,
      anon_sym_COLON_COLON,
    STATE(108), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(83), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3192] = 3,
    ACTIONS(343), 1,
      anon_sym_COLON_COLON,
    STATE(123), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(103), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3203] = 3,
    ACTIONS(5), 1,
      anon_sym_module,
    ACTIONS(345), 1,
      ts_builtin_sym_end,
    STATE(112), 2,
      sym_module,
      aux_sym_source_file_repeat1,
  [3214] = 4,
    ACTIONS(347), 1,
      anon_sym_COLON,
    ACTIONS(349), 1,
      anon_sym_LBRACE,
    STATE(144), 1,
      sym_interface_ports,
    STATE(148), 1,
      sym_block,
  [3227] = 3,
    ACTIONS(351), 1,
      ts_builtin_sym_end,
    ACTIONS(353), 1,
      anon_sym_module,
    STATE(112), 2,
      sym_module,
      aux_sym_source_file_repeat1,
  [3238] = 3,
    ACTIONS(356), 1,
      anon_sym_COMMA,
    STATE(113), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(359), 2,
      anon_sym_LBRACE,
      anon_sym_EQ,
  [3249] = 3,
    ACTIONS(361), 1,
      anon_sym_COMMA,
    STATE(113), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(363), 2,
      anon_sym_LBRACE,
      anon_sym_EQ,
  [3260] = 3,
    ACTIONS(365), 1,
      anon_sym_COMMA,
    STATE(115), 1,
      aux_sym_interface_ports_repeat1,
    ACTIONS(368), 2,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
  [3271] = 4,
    ACTIONS(370), 1,
      anon_sym_COMMA,
    ACTIONS(372), 1,
      anon_sym_DASH_GT,
    ACTIONS(374), 1,
      anon_sym_LBRACE,
    STATE(115), 1,
      aux_sym_interface_ports_repeat1,
  [3284] = 3,
    ACTIONS(361), 1,
      anon_sym_COMMA,
    STATE(114), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(376), 2,
      anon_sym_LBRACE,
      anon_sym_EQ,
  [3295] = 4,
    ACTIONS(361), 1,
      anon_sym_COMMA,
    ACTIONS(378), 1,
      anon_sym_EQ,
    ACTIONS(380), 1,
      anon_sym_SEMI,
    STATE(121), 1,
      aux_sym__assign_left_side_repeat2,
  [3308] = 4,
    ACTIONS(370), 1,
      anon_sym_COMMA,
    ACTIONS(382), 1,
      anon_sym_DASH_GT,
    ACTIONS(384), 1,
      anon_sym_LBRACE,
    STATE(116), 1,
      aux_sym_interface_ports_repeat1,
  [3321] = 3,
    ACTIONS(19), 1,
      anon_sym_LBRACE,
    ACTIONS(386), 1,
      anon_sym_if,
    STATE(76), 2,
      sym_block,
      sym_if_statement,
  [3332] = 3,
    ACTIONS(361), 1,
      anon_sym_COMMA,
    STATE(113), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(376), 2,
      anon_sym_LBRACE,
      anon_sym_EQ,
  [3343] = 3,
    ACTIONS(343), 1,
      anon_sym_COLON_COLON,
    STATE(108), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(89), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3354] = 3,
    ACTIONS(343), 1,
      anon_sym_COLON_COLON,
    STATE(108), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(99), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3365] = 3,
    ACTIONS(343), 1,
      anon_sym_COLON_COLON,
    STATE(122), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(99), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3376] = 3,
    ACTIONS(370), 1,
      anon_sym_COMMA,
    ACTIONS(388), 1,
      anon_sym_LBRACE,
    STATE(132), 1,
      aux_sym_interface_ports_repeat1,
  [3386] = 1,
    ACTIONS(368), 3,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
  [3392] = 1,
    ACTIONS(359), 3,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
  [3398] = 1,
    ACTIONS(83), 3,
      sym_identifier,
      anon_sym_COLON_COLON,
      anon_sym_LBRACK,
  [3404] = 3,
    ACTIONS(370), 1,
      anon_sym_COMMA,
    ACTIONS(390), 1,
      anon_sym_LBRACE,
    STATE(115), 1,
      aux_sym_interface_ports_repeat1,
  [3414] = 3,
    ACTIONS(392), 1,
      anon_sym_COMMA,
    ACTIONS(394), 1,
      anon_sym_RPAREN,
    STATE(135), 1,
      aux_sym_func_call_repeat1,
  [3424] = 1,
    ACTIONS(396), 3,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
  [3430] = 3,
    ACTIONS(370), 1,
      anon_sym_COMMA,
    ACTIONS(398), 1,
      anon_sym_LBRACE,
    STATE(115), 1,
      aux_sym_interface_ports_repeat1,
  [3440] = 3,
    ACTIONS(370), 1,
      anon_sym_COMMA,
    ACTIONS(400), 1,
      anon_sym_LBRACE,
    STATE(137), 1,
      aux_sym_interface_ports_repeat1,
  [3450] = 3,
    ACTIONS(370), 1,
      anon_sym_COMMA,
    ACTIONS(402), 1,
      anon_sym_LBRACE,
    STATE(129), 1,
      aux_sym_interface_ports_repeat1,
  [3460] = 3,
    ACTIONS(404), 1,
      anon_sym_COMMA,
    ACTIONS(407), 1,
      anon_sym_RPAREN,
    STATE(135), 1,
      aux_sym_func_call_repeat1,
  [3470] = 3,
    ACTIONS(361), 1,
      anon_sym_COMMA,
    ACTIONS(378), 1,
      anon_sym_LBRACE,
    STATE(121), 1,
      aux_sym__assign_left_side_repeat2,
  [3480] = 3,
    ACTIONS(370), 1,
      anon_sym_COMMA,
    ACTIONS(409), 1,
      anon_sym_LBRACE,
    STATE(115), 1,
      aux_sym_interface_ports_repeat1,
  [3490] = 1,
    ACTIONS(186), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3495] = 2,
    ACTIONS(411), 1,
      sym_identifier,
    ACTIONS(413), 1,
      anon_sym_LBRACK,
  [3502] = 2,
    ACTIONS(19), 1,
      anon_sym_LBRACE,
    STATE(74), 1,
      sym_block,
  [3509] = 1,
    ACTIONS(415), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3514] = 1,
    ACTIONS(194), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3519] = 2,
    ACTIONS(413), 1,
      anon_sym_LBRACK,
    ACTIONS(417), 1,
      sym_identifier,
  [3526] = 2,
    ACTIONS(349), 1,
      anon_sym_LBRACE,
    STATE(145), 1,
      sym_block,
  [3533] = 1,
    ACTIONS(419), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3538] = 2,
    ACTIONS(413), 1,
      anon_sym_LBRACK,
    ACTIONS(421), 1,
      sym_identifier,
  [3545] = 2,
    ACTIONS(423), 1,
      anon_sym_LBRACE,
    STATE(38), 1,
      sym_block,
  [3552] = 1,
    ACTIONS(425), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3557] = 2,
    ACTIONS(334), 1,
      anon_sym_in,
    ACTIONS(427), 1,
      anon_sym_SQUOTE,
  [3564] = 2,
    ACTIONS(330), 1,
      anon_sym_in,
    ACTIONS(429), 1,
      anon_sym_SQUOTE,
  [3571] = 2,
    ACTIONS(413), 1,
      anon_sym_LBRACK,
    ACTIONS(431), 1,
      sym_identifier,
  [3578] = 1,
    ACTIONS(433), 1,
      sym_identifier,
  [3582] = 1,
    ACTIONS(435), 1,
      sym_identifier,
  [3586] = 1,
    ACTIONS(437), 1,
      anon_sym_EQ,
  [3590] = 1,
    ACTIONS(439), 1,
      sym_identifier,
  [3594] = 1,
    ACTIONS(441), 1,
      anon_sym_in,
  [3598] = 1,
    ACTIONS(443), 1,
      sym_identifier,
  [3602] = 1,
    ACTIONS(445), 1,
      ts_builtin_sym_end,
  [3606] = 1,
    ACTIONS(447), 1,
      sym_identifier,
  [3610] = 1,
    ACTIONS(449), 1,
      sym_identifier,
  [3614] = 1,
    ACTIONS(451), 1,
      sym_identifier,
  [3618] = 1,
    ACTIONS(453), 1,
      anon_sym_SEMI,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(9)] = 0,
  [SMALL_STATE(10)] = 59,
  [SMALL_STATE(11)] = 95,
  [SMALL_STATE(12)] = 131,
  [SMALL_STATE(13)] = 187,
  [SMALL_STATE(14)] = 223,
  [SMALL_STATE(15)] = 259,
  [SMALL_STATE(16)] = 295,
  [SMALL_STATE(17)] = 326,
  [SMALL_STATE(18)] = 379,
  [SMALL_STATE(19)] = 432,
  [SMALL_STATE(20)] = 464,
  [SMALL_STATE(21)] = 493,
  [SMALL_STATE(22)] = 522,
  [SMALL_STATE(23)] = 551,
  [SMALL_STATE(24)] = 598,
  [SMALL_STATE(25)] = 627,
  [SMALL_STATE(26)] = 656,
  [SMALL_STATE(27)] = 685,
  [SMALL_STATE(28)] = 714,
  [SMALL_STATE(29)] = 761,
  [SMALL_STATE(30)] = 799,
  [SMALL_STATE(31)] = 831,
  [SMALL_STATE(32)] = 867,
  [SMALL_STATE(33)] = 901,
  [SMALL_STATE(34)] = 931,
  [SMALL_STATE(35)] = 961,
  [SMALL_STATE(36)] = 1000,
  [SMALL_STATE(37)] = 1037,
  [SMALL_STATE(38)] = 1076,
  [SMALL_STATE(39)] = 1103,
  [SMALL_STATE(40)] = 1128,
  [SMALL_STATE(41)] = 1165,
  [SMALL_STATE(42)] = 1190,
  [SMALL_STATE(43)] = 1224,
  [SMALL_STATE(44)] = 1258,
  [SMALL_STATE(45)] = 1286,
  [SMALL_STATE(46)] = 1320,
  [SMALL_STATE(47)] = 1348,
  [SMALL_STATE(48)] = 1382,
  [SMALL_STATE(49)] = 1410,
  [SMALL_STATE(50)] = 1438,
  [SMALL_STATE(51)] = 1472,
  [SMALL_STATE(52)] = 1514,
  [SMALL_STATE(53)] = 1548,
  [SMALL_STATE(54)] = 1582,
  [SMALL_STATE(55)] = 1606,
  [SMALL_STATE(56)] = 1640,
  [SMALL_STATE(57)] = 1674,
  [SMALL_STATE(58)] = 1708,
  [SMALL_STATE(59)] = 1742,
  [SMALL_STATE(60)] = 1776,
  [SMALL_STATE(61)] = 1810,
  [SMALL_STATE(62)] = 1844,
  [SMALL_STATE(63)] = 1878,
  [SMALL_STATE(64)] = 1912,
  [SMALL_STATE(65)] = 1946,
  [SMALL_STATE(66)] = 1970,
  [SMALL_STATE(67)] = 2004,
  [SMALL_STATE(68)] = 2038,
  [SMALL_STATE(69)] = 2072,
  [SMALL_STATE(70)] = 2106,
  [SMALL_STATE(71)] = 2130,
  [SMALL_STATE(72)] = 2158,
  [SMALL_STATE(73)] = 2192,
  [SMALL_STATE(74)] = 2236,
  [SMALL_STATE(75)] = 2260,
  [SMALL_STATE(76)] = 2294,
  [SMALL_STATE(77)] = 2318,
  [SMALL_STATE(78)] = 2352,
  [SMALL_STATE(79)] = 2389,
  [SMALL_STATE(80)] = 2430,
  [SMALL_STATE(81)] = 2467,
  [SMALL_STATE(82)] = 2508,
  [SMALL_STATE(83)] = 2531,
  [SMALL_STATE(84)] = 2567,
  [SMALL_STATE(85)] = 2602,
  [SMALL_STATE(86)] = 2627,
  [SMALL_STATE(87)] = 2662,
  [SMALL_STATE(88)] = 2697,
  [SMALL_STATE(89)] = 2720,
  [SMALL_STATE(90)] = 2747,
  [SMALL_STATE(91)] = 2782,
  [SMALL_STATE(92)] = 2817,
  [SMALL_STATE(93)] = 2852,
  [SMALL_STATE(94)] = 2887,
  [SMALL_STATE(95)] = 2918,
  [SMALL_STATE(96)] = 2947,
  [SMALL_STATE(97)] = 2982,
  [SMALL_STATE(98)] = 3006,
  [SMALL_STATE(99)] = 3031,
  [SMALL_STATE(100)] = 3053,
  [SMALL_STATE(101)] = 3075,
  [SMALL_STATE(102)] = 3097,
  [SMALL_STATE(103)] = 3116,
  [SMALL_STATE(104)] = 3135,
  [SMALL_STATE(105)] = 3146,
  [SMALL_STATE(106)] = 3157,
  [SMALL_STATE(107)] = 3169,
  [SMALL_STATE(108)] = 3181,
  [SMALL_STATE(109)] = 3192,
  [SMALL_STATE(110)] = 3203,
  [SMALL_STATE(111)] = 3214,
  [SMALL_STATE(112)] = 3227,
  [SMALL_STATE(113)] = 3238,
  [SMALL_STATE(114)] = 3249,
  [SMALL_STATE(115)] = 3260,
  [SMALL_STATE(116)] = 3271,
  [SMALL_STATE(117)] = 3284,
  [SMALL_STATE(118)] = 3295,
  [SMALL_STATE(119)] = 3308,
  [SMALL_STATE(120)] = 3321,
  [SMALL_STATE(121)] = 3332,
  [SMALL_STATE(122)] = 3343,
  [SMALL_STATE(123)] = 3354,
  [SMALL_STATE(124)] = 3365,
  [SMALL_STATE(125)] = 3376,
  [SMALL_STATE(126)] = 3386,
  [SMALL_STATE(127)] = 3392,
  [SMALL_STATE(128)] = 3398,
  [SMALL_STATE(129)] = 3404,
  [SMALL_STATE(130)] = 3414,
  [SMALL_STATE(131)] = 3424,
  [SMALL_STATE(132)] = 3430,
  [SMALL_STATE(133)] = 3440,
  [SMALL_STATE(134)] = 3450,
  [SMALL_STATE(135)] = 3460,
  [SMALL_STATE(136)] = 3470,
  [SMALL_STATE(137)] = 3480,
  [SMALL_STATE(138)] = 3490,
  [SMALL_STATE(139)] = 3495,
  [SMALL_STATE(140)] = 3502,
  [SMALL_STATE(141)] = 3509,
  [SMALL_STATE(142)] = 3514,
  [SMALL_STATE(143)] = 3519,
  [SMALL_STATE(144)] = 3526,
  [SMALL_STATE(145)] = 3533,
  [SMALL_STATE(146)] = 3538,
  [SMALL_STATE(147)] = 3545,
  [SMALL_STATE(148)] = 3552,
  [SMALL_STATE(149)] = 3557,
  [SMALL_STATE(150)] = 3564,
  [SMALL_STATE(151)] = 3571,
  [SMALL_STATE(152)] = 3578,
  [SMALL_STATE(153)] = 3582,
  [SMALL_STATE(154)] = 3586,
  [SMALL_STATE(155)] = 3590,
  [SMALL_STATE(156)] = 3594,
  [SMALL_STATE(157)] = 3598,
  [SMALL_STATE(158)] = 3602,
  [SMALL_STATE(159)] = 3606,
  [SMALL_STATE(160)] = 3610,
  [SMALL_STATE(161)] = 3614,
  [SMALL_STATE(162)] = 3618,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(155),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(152),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(106),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(18),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(23),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(9),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(102),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [37] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(15),
  [40] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(73),
  [43] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(152),
  [46] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(106),
  [49] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(77),
  [52] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(72),
  [55] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(5),
  [58] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2),
  [60] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(18),
  [63] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(23),
  [66] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(9),
  [69] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(102),
  [72] = {.entry = {.count = 1, .reusable = true}}, SHIFT(142),
  [74] = {.entry = {.count = 1, .reusable = true}}, SHIFT(138),
  [76] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [78] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_global_identifier_repeat1, 2),
  [80] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_identifier_repeat1, 2), SHIFT_REPEAT(153),
  [83] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_global_identifier_repeat1, 2),
  [85] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 3),
  [87] = {.entry = {.count = 1, .reusable = false}}, SHIFT(153),
  [89] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 3),
  [91] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [93] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [95] = {.entry = {.count = 1, .reusable = false}}, SHIFT(28),
  [97] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 2),
  [99] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 2),
  [101] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 1),
  [103] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 1),
  [105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [107] = {.entry = {.count = 1, .reusable = false}}, SHIFT(97),
  [109] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [111] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expression, 1),
  [113] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expression, 1),
  [115] = {.entry = {.count = 1, .reusable = false}}, SHIFT(40),
  [117] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 3, .production_id = 13),
  [119] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 3, .production_id = 13),
  [121] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 4, .production_id = 20),
  [123] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 4, .production_id = 20),
  [125] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 5, .production_id = 27),
  [127] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 5, .production_id = 27),
  [129] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_op, 4, .production_id = 21),
  [131] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_op, 4, .production_id = 21),
  [133] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_op, 3, .production_id = 14),
  [135] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_op, 3, .production_id = 14),
  [137] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_op, 2, .production_id = 7),
  [139] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_op, 2, .production_id = 7),
  [141] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 11),
  [143] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 11),
  [145] = {.entry = {.count = 1, .reusable = false}}, SHIFT(58),
  [147] = {.entry = {.count = 1, .reusable = false}}, SHIFT(59),
  [149] = {.entry = {.count = 1, .reusable = false}}, SHIFT(60),
  [151] = {.entry = {.count = 1, .reusable = false}}, SHIFT(61),
  [153] = {.entry = {.count = 1, .reusable = false}}, SHIFT(62),
  [155] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [157] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__maybe_global_identifier, 1),
  [159] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__type, 1),
  [161] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym__maybe_global_identifier, 1), REDUCE(sym__type, 1),
  [164] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__maybe_global_identifier, 1),
  [166] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_declaration, 5, .production_id = 22),
  [168] = {.entry = {.count = 1, .reusable = false}}, SHIFT(57),
  [170] = {.entry = {.count = 1, .reusable = false}}, SHIFT(63),
  [172] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [174] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [176] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_declaration, 4, .production_id = 16),
  [178] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if_statement, 3, .production_id = 12),
  [180] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 3, .production_id = 12),
  [182] = {.entry = {.count = 1, .reusable = false}}, SHIFT(120),
  [184] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 2),
  [186] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [188] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [190] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [192] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 3),
  [194] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [196] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [198] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [200] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [202] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [204] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [206] = {.entry = {.count = 1, .reusable = false}}, SHIFT(159),
  [208] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [210] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_identifier_repeat1, 2), SHIFT_REPEAT(159),
  [213] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [215] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [217] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [219] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__assign_left_side, 2),
  [221] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [223] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [225] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [227] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [229] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [231] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [233] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [235] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [237] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [239] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [241] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [243] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [245] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__statement, 2),
  [247] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__statement, 2),
  [249] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [251] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [253] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [255] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [257] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__assign_left_side, 1),
  [259] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expression_statement, 1),
  [261] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_for_statement, 5, .production_id = 25),
  [263] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_statement, 5, .production_id = 25),
  [265] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [267] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if_statement, 5, .production_id = 24),
  [269] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 5, .production_id = 24),
  [271] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__assign_left_side_repeat2, 3),
  [273] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__assign_left_side_repeat2, 2),
  [275] = {.entry = {.count = 1, .reusable = false}}, SHIFT(53),
  [277] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [279] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_func_call_repeat1, 2, .production_id = 26),
  [281] = {.entry = {.count = 1, .reusable = false}}, SHIFT(75),
  [283] = {.entry = {.count = 1, .reusable = false}}, SHIFT(69),
  [285] = {.entry = {.count = 1, .reusable = false}}, SHIFT(68),
  [287] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [289] = {.entry = {.count = 1, .reusable = false}}, SHIFT(141),
  [291] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_range, 3, .production_id = 29),
  [293] = {.entry = {.count = 1, .reusable = false}}, SHIFT(56),
  [295] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [297] = {.entry = {.count = 1, .reusable = false}}, SHIFT(27),
  [299] = {.entry = {.count = 1, .reusable = false}}, SHIFT(67),
  [301] = {.entry = {.count = 1, .reusable = false}}, SHIFT(66),
  [303] = {.entry = {.count = 1, .reusable = false}}, SHIFT(55),
  [305] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_decl_assign_statement, 3, .production_id = 15),
  [307] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__assign_left_side_repeat1, 2),
  [309] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__assign_left_side_repeat1, 2),
  [311] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__assign_left_side_repeat1, 2), SHIFT_REPEAT(97),
  [314] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [316] = {.entry = {.count = 1, .reusable = false}}, SHIFT(109),
  [318] = {.entry = {.count = 1, .reusable = true}}, SHIFT(161),
  [320] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 1),
  [322] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 2),
  [324] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2),
  [326] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 4, .production_id = 6),
  [328] = {.entry = {.count = 1, .reusable = false}}, SHIFT(107),
  [330] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 2, .production_id = 5),
  [332] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [334] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 9),
  [336] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [338] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [340] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_global_identifier_repeat1, 2), SHIFT_REPEAT(160),
  [343] = {.entry = {.count = 1, .reusable = true}}, SHIFT(160),
  [345] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [347] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [349] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [351] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [353] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(155),
  [356] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__assign_left_side_repeat2, 2), SHIFT_REPEAT(12),
  [359] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__assign_left_side_repeat2, 2),
  [361] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [363] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__assign_left_side, 3),
  [365] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_interface_ports_repeat1, 2), SHIFT_REPEAT(103),
  [368] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_interface_ports_repeat1, 2),
  [370] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [372] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [374] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 6),
  [376] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__assign_left_side, 2),
  [378] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__assign_left_side, 1),
  [380] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_decl_statement, 1),
  [382] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [384] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2, .production_id = 2),
  [386] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [388] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 5, .production_id = 19),
  [390] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 4, .production_id = 8),
  [392] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [394] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [396] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__assign_left_side_repeat2, 3),
  [398] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 6, .production_id = 23),
  [400] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 4, .production_id = 10),
  [402] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 3, .production_id = 4),
  [404] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_func_call_repeat1, 2, .production_id = 28), SHIFT_REPEAT(53),
  [407] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_func_call_repeat1, 2, .production_id = 28),
  [409] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 5, .production_id = 18),
  [411] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [413] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [415] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type, 4, .production_id = 17),
  [417] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [419] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 4, .production_id = 3),
  [421] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [423] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [425] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 3, .production_id = 1),
  [427] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [429] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [431] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [433] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [435] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [437] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [439] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [441] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [443] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [445] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [447] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [449] = {.entry = {.count = 1, .reusable = true}}, SHIFT(128),
  [451] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [453] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
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
