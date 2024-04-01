#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 118
#define LARGE_STATE_COUNT 5
#define SYMBOL_COUNT 69
#define ALIAS_COUNT 0
#define TOKEN_COUNT 40
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 26
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 33

enum {
  sym_identifier = 1,
  anon_sym_COLON = 2,
  anon_sym_DASH_GT = 3,
  anon_sym_module = 4,
  sym_number = 5,
  anon_sym_COLON_COLON = 6,
  anon_sym_SQUOTE = 7,
  anon_sym_state = 8,
  anon_sym_gen = 9,
  anon_sym_PLUS = 10,
  anon_sym_DASH = 11,
  anon_sym_STAR = 12,
  anon_sym_BANG = 13,
  anon_sym_PIPE = 14,
  anon_sym_AMP = 15,
  anon_sym_CARET = 16,
  anon_sym_EQ_EQ = 17,
  anon_sym_BANG_EQ = 18,
  anon_sym_LT = 19,
  anon_sym_LT_EQ = 20,
  anon_sym_GT = 21,
  anon_sym_GT_EQ = 22,
  anon_sym_SLASH = 23,
  anon_sym_PERCENT = 24,
  anon_sym_LPAREN = 25,
  anon_sym_COMMA = 26,
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
  sym_assign_left_side = 58,
  sym_decl_assign_statement = 59,
  sym_if_statement = 60,
  sym_for_statement = 61,
  sym__statement = 62,
  aux_sym_source_file_repeat1 = 63,
  aux_sym_global_identifier_repeat1 = 64,
  aux_sym_func_call_repeat1 = 65,
  aux_sym_block_repeat1 = 66,
  aux_sym_assign_left_side_repeat1 = 67,
  aux_sym_assign_left_side_repeat2 = 68,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
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
  [anon_sym_COMMA] = ",",
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
  [sym_assign_left_side] = "assign_left_side",
  [sym_decl_assign_statement] = "decl_assign_statement",
  [sym_if_statement] = "if_statement",
  [sym_for_statement] = "for_statement",
  [sym__statement] = "_statement",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_global_identifier_repeat1] = "global_identifier_repeat1",
  [aux_sym_func_call_repeat1] = "func_call_repeat1",
  [aux_sym_block_repeat1] = "block_repeat1",
  [aux_sym_assign_left_side_repeat1] = "assign_left_side_repeat1",
  [aux_sym_assign_left_side_repeat2] = "assign_left_side_repeat2",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_identifier] = sym_identifier,
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
  [anon_sym_COMMA] = anon_sym_COMMA,
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
  [sym_assign_left_side] = sym_assign_left_side,
  [sym_decl_assign_statement] = sym_decl_assign_statement,
  [sym_if_statement] = sym_if_statement,
  [sym_for_statement] = sym_for_statement,
  [sym__statement] = sym__statement,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_global_identifier_repeat1] = aux_sym_global_identifier_repeat1,
  [aux_sym_func_call_repeat1] = aux_sym_func_call_repeat1,
  [aux_sym_block_repeat1] = aux_sym_block_repeat1,
  [aux_sym_assign_left_side_repeat1] = aux_sym_assign_left_side_repeat1,
  [aux_sym_assign_left_side_repeat2] = aux_sym_assign_left_side_repeat2,
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
  [anon_sym_COMMA] = {
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
  [sym__statement] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
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
  [aux_sym_assign_left_side_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_assign_left_side_repeat2] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_argument = 1,
  field_arr = 2,
  field_arr_idx = 3,
  field_assign_left = 4,
  field_assign_to = 5,
  field_assign_value = 6,
  field_block = 7,
  field_block_statement = 8,
  field_condition = 9,
  field_content = 10,
  field_declaration_modifiers = 11,
  field_else_block = 12,
  field_for_decl = 13,
  field_for_range = 14,
  field_from = 15,
  field_inputs = 16,
  field_interface_ports = 17,
  field_latency_specifier = 18,
  field_left = 19,
  field_name = 20,
  field_operator = 21,
  field_outputs = 22,
  field_right = 23,
  field_then_block = 24,
  field_to = 25,
  field_type = 26,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_argument] = "argument",
  [field_arr] = "arr",
  [field_arr_idx] = "arr_idx",
  [field_assign_left] = "assign_left",
  [field_assign_to] = "assign_to",
  [field_assign_value] = "assign_value",
  [field_block] = "block",
  [field_block_statement] = "block_statement",
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
  [3] = {.index = 3, .length = 1},
  [4] = {.index = 4, .length = 1},
  [5] = {.index = 5, .length = 3},
  [6] = {.index = 8, .length = 2},
  [7] = {.index = 10, .length = 2},
  [8] = {.index = 12, .length = 2},
  [9] = {.index = 14, .length = 2},
  [10] = {.index = 16, .length = 2},
  [11] = {.index = 18, .length = 1},
  [12] = {.index = 19, .length = 2},
  [13] = {.index = 21, .length = 3},
  [14] = {.index = 24, .length = 1},
  [15] = {.index = 25, .length = 3},
  [16] = {.index = 28, .length = 1},
  [17] = {.index = 29, .length = 3},
  [18] = {.index = 32, .length = 1},
  [19] = {.index = 33, .length = 2},
  [20] = {.index = 35, .length = 3},
  [21] = {.index = 38, .length = 2},
  [22] = {.index = 40, .length = 2},
  [23] = {.index = 42, .length = 2},
  [24] = {.index = 44, .length = 4},
  [25] = {.index = 48, .length = 2},
  [26] = {.index = 50, .length = 2},
  [27] = {.index = 52, .length = 1},
  [28] = {.index = 53, .length = 3},
  [29] = {.index = 56, .length = 2},
  [30] = {.index = 58, .length = 3},
  [31] = {.index = 61, .length = 3},
  [32] = {.index = 64, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_block, 2},
    {field_name, 1},
  [2] =
    {field_assign_to, 0},
  [3] =
    {field_inputs, 1},
  [4] =
    {field_block_statement, 0},
  [5] =
    {field_block, 3},
    {field_interface_ports, 2},
    {field_name, 1},
  [8] =
    {field_operator, 0},
    {field_right, 1},
  [10] =
    {field_assign_to, 0},
    {field_assign_to, 1},
  [12] =
    {field_name, 1},
    {field_type, 0},
  [14] =
    {field_arr, 0},
    {field_arr_idx, 1},
  [16] =
    {field_assign_to, 0},
    {field_assign_to, 1, .inherited = true},
  [18] =
    {field_block_statement, 1, .inherited = true},
  [19] =
    {field_block_statement, 0, .inherited = true},
    {field_block_statement, 1, .inherited = true},
  [21] =
    {field_declaration_modifiers, 0},
    {field_name, 2},
    {field_type, 1},
  [24] =
    {field_content, 1},
  [25] =
    {field_assign_to, 0},
    {field_assign_to, 1},
    {field_assign_to, 2, .inherited = true},
  [28] =
    {field_name, 0},
  [29] =
    {field_latency_specifier, 2},
    {field_name, 1},
    {field_type, 0},
  [32] =
    {field_assign_to, 1},
  [33] =
    {field_assign_to, 0, .inherited = true},
    {field_assign_to, 1, .inherited = true},
  [35] =
    {field_left, 0},
    {field_operator, 1},
    {field_right, 2},
  [38] =
    {field_inputs, 1},
    {field_outputs, 3},
  [40] =
    {field_condition, 1},
    {field_then_block, 2},
  [42] =
    {field_assign_left, 0},
    {field_assign_value, 2},
  [44] =
    {field_declaration_modifiers, 0},
    {field_latency_specifier, 3},
    {field_name, 2},
    {field_type, 1},
  [48] =
    {field_argument, 2},
    {field_name, 0},
  [50] =
    {field_assign_to, 1},
    {field_assign_to, 2},
  [52] =
    {field_argument, 1},
  [53] =
    {field_argument, 2},
    {field_argument, 3, .inherited = true},
    {field_name, 0},
  [56] =
    {field_argument, 0, .inherited = true},
    {field_argument, 1, .inherited = true},
  [58] =
    {field_condition, 1},
    {field_else_block, 4},
    {field_then_block, 2},
  [61] =
    {field_block, 4},
    {field_for_decl, 1},
    {field_for_range, 3},
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
  [47] = 46,
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
  [70] = 67,
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
  [88] = 6,
  [89] = 10,
  [90] = 9,
  [91] = 7,
  [92] = 92,
  [93] = 93,
  [94] = 94,
  [95] = 95,
  [96] = 96,
  [97] = 97,
  [98] = 98,
  [99] = 99,
  [100] = 13,
  [101] = 101,
  [102] = 102,
  [103] = 28,
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
  [114] = 112,
  [115] = 110,
  [116] = 116,
  [117] = 117,
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
      if (eof) ADVANCE(19);
      if (lookahead == '!') ADVANCE(32);
      if (lookahead == '%') ADVANCE(43);
      if (lookahead == '&') ADVANCE(34);
      if (lookahead == '\'') ADVANCE(26);
      if (lookahead == '(') ADVANCE(44);
      if (lookahead == ')') ADVANCE(46);
      if (lookahead == '*') ADVANCE(30);
      if (lookahead == '+') ADVANCE(27);
      if (lookahead == ',') ADVANCE(45);
      if (lookahead == '-') ADVANCE(29);
      if (lookahead == '/') SKIP(13)
      if (lookahead == ':') ADVANCE(21);
      if (lookahead == ';') ADVANCE(52);
      if (lookahead == '<') ADVANCE(38);
      if (lookahead == '=') ADVANCE(51);
      if (lookahead == '>') ADVANCE(40);
      if (lookahead == '[') ADVANCE(47);
      if (lookahead == ']') ADVANCE(48);
      if (lookahead == '^') ADVANCE(35);
      if (lookahead == '{') ADVANCE(49);
      if (lookahead == '|') ADVANCE(33);
      if (lookahead == '}') ADVANCE(50);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(24);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(23);
      END_STATE();
    case 1:
      if (lookahead == '\n') SKIP(7)
      if (lookahead != 0) SKIP(1)
      END_STATE();
    case 2:
      if (lookahead == '!') ADVANCE(9);
      if (lookahead == '%') ADVANCE(43);
      if (lookahead == '&') ADVANCE(34);
      if (lookahead == '(') ADVANCE(44);
      if (lookahead == ')') ADVANCE(46);
      if (lookahead == '*') ADVANCE(30);
      if (lookahead == '+') ADVANCE(27);
      if (lookahead == ',') ADVANCE(45);
      if (lookahead == '-') ADVANCE(29);
      if (lookahead == '/') ADVANCE(42);
      if (lookahead == ':') ADVANCE(21);
      if (lookahead == ';') ADVANCE(52);
      if (lookahead == '<') ADVANCE(38);
      if (lookahead == '=') ADVANCE(51);
      if (lookahead == '>') ADVANCE(40);
      if (lookahead == '[') ADVANCE(47);
      if (lookahead == ']') ADVANCE(48);
      if (lookahead == '^') ADVANCE(35);
      if (lookahead == '{') ADVANCE(49);
      if (lookahead == '|') ADVANCE(33);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(2)
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(23);
      END_STATE();
    case 3:
      if (lookahead == '!') ADVANCE(9);
      if (lookahead == '%') ADVANCE(43);
      if (lookahead == '&') ADVANCE(34);
      if (lookahead == '(') ADVANCE(44);
      if (lookahead == ')') ADVANCE(46);
      if (lookahead == '*') ADVANCE(30);
      if (lookahead == '+') ADVANCE(27);
      if (lookahead == ',') ADVANCE(45);
      if (lookahead == '-') ADVANCE(29);
      if (lookahead == '/') ADVANCE(42);
      if (lookahead == ':') ADVANCE(20);
      if (lookahead == ';') ADVANCE(52);
      if (lookahead == '<') ADVANCE(38);
      if (lookahead == '=') ADVANCE(51);
      if (lookahead == '>') ADVANCE(40);
      if (lookahead == '[') ADVANCE(47);
      if (lookahead == ']') ADVANCE(48);
      if (lookahead == '^') ADVANCE(35);
      if (lookahead == '{') ADVANCE(49);
      if (lookahead == '|') ADVANCE(33);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(3)
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(23);
      END_STATE();
    case 4:
      if (lookahead == '*') SKIP(6)
      if (lookahead == '/') SKIP(1)
      END_STATE();
    case 5:
      if (lookahead == '*') SKIP(5)
      if (lookahead == '/') SKIP(7)
      if (lookahead != 0) SKIP(6)
      END_STATE();
    case 6:
      if (lookahead == '*') SKIP(5)
      if (lookahead != 0) SKIP(6)
      END_STATE();
    case 7:
      if (lookahead == '/') SKIP(4)
      if (lookahead == ':') ADVANCE(20);
      if (lookahead == '{') ADVANCE(49);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(7)
      END_STATE();
    case 8:
      if (lookahead == ':') ADVANCE(25);
      END_STATE();
    case 9:
      if (lookahead == '=') ADVANCE(37);
      END_STATE();
    case 10:
      if (eof) ADVANCE(19);
      if (lookahead == '\n') SKIP(0)
      if (lookahead != 0) SKIP(10)
      END_STATE();
    case 11:
      if (eof) ADVANCE(19);
      if (lookahead == '\n') SKIP(12)
      if (lookahead != 0) SKIP(11)
      END_STATE();
    case 12:
      if (eof) ADVANCE(19);
      if (lookahead == '!') ADVANCE(31);
      if (lookahead == '&') ADVANCE(34);
      if (lookahead == '(') ADVANCE(44);
      if (lookahead == ')') ADVANCE(46);
      if (lookahead == '*') ADVANCE(30);
      if (lookahead == '+') ADVANCE(27);
      if (lookahead == '-') ADVANCE(28);
      if (lookahead == '/') SKIP(18)
      if (lookahead == ':') ADVANCE(8);
      if (lookahead == '^') ADVANCE(35);
      if (lookahead == '{') ADVANCE(49);
      if (lookahead == '|') ADVANCE(33);
      if (lookahead == '}') ADVANCE(50);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(12)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(24);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(23);
      END_STATE();
    case 13:
      if (eof) ADVANCE(19);
      if (lookahead == '*') SKIP(15)
      if (lookahead == '/') SKIP(10)
      END_STATE();
    case 14:
      if (eof) ADVANCE(19);
      if (lookahead == '*') SKIP(14)
      if (lookahead == '/') SKIP(0)
      if (lookahead != 0) SKIP(15)
      END_STATE();
    case 15:
      if (eof) ADVANCE(19);
      if (lookahead == '*') SKIP(14)
      if (lookahead != 0) SKIP(15)
      END_STATE();
    case 16:
      if (eof) ADVANCE(19);
      if (lookahead == '*') SKIP(16)
      if (lookahead == '/') SKIP(12)
      if (lookahead != 0) SKIP(17)
      END_STATE();
    case 17:
      if (eof) ADVANCE(19);
      if (lookahead == '*') SKIP(16)
      if (lookahead != 0) SKIP(17)
      END_STATE();
    case 18:
      if (eof) ADVANCE(19);
      if (lookahead == '*') SKIP(17)
      if (lookahead == '/') SKIP(11)
      END_STATE();
    case 19:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_COLON);
      if (lookahead == ':') ADVANCE(25);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(23);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(24);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_COLON_COLON);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '>') ADVANCE(22);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_BANG);
      if (lookahead == '=') ADVANCE(37);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_AMP);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(39);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(41);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(anon_sym_GT_EQ);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(anon_sym_SLASH);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead == '=') ADVANCE(36);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(anon_sym_SEMI);
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
      if (lookahead == '/') SKIP(1)
      if (lookahead == 'e') ADVANCE(2);
      if (lookahead == 'f') ADVANCE(3);
      if (lookahead == 'g') ADVANCE(4);
      if (lookahead == 'i') ADVANCE(5);
      if (lookahead == 'm') ADVANCE(6);
      if (lookahead == 'r') ADVANCE(7);
      if (lookahead == 's') ADVANCE(8);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == '*') SKIP(9)
      if (lookahead == '/') SKIP(10)
      END_STATE();
    case 2:
      if (lookahead == 'l') ADVANCE(11);
      END_STATE();
    case 3:
      if (lookahead == 'o') ADVANCE(12);
      END_STATE();
    case 4:
      if (lookahead == 'e') ADVANCE(13);
      END_STATE();
    case 5:
      if (lookahead == 'f') ADVANCE(14);
      if (lookahead == 'n') ADVANCE(15);
      END_STATE();
    case 6:
      if (lookahead == 'o') ADVANCE(16);
      END_STATE();
    case 7:
      if (lookahead == 'e') ADVANCE(17);
      END_STATE();
    case 8:
      if (lookahead == 't') ADVANCE(18);
      END_STATE();
    case 9:
      if (lookahead == '*') SKIP(19)
      if (lookahead != 0) SKIP(9)
      END_STATE();
    case 10:
      if (lookahead == '\n') SKIP(0)
      if (lookahead != 0) SKIP(10)
      END_STATE();
    case 11:
      if (lookahead == 's') ADVANCE(20);
      END_STATE();
    case 12:
      if (lookahead == 'r') ADVANCE(21);
      END_STATE();
    case 13:
      if (lookahead == 'n') ADVANCE(22);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_in);
      if (lookahead == 'i') ADVANCE(23);
      END_STATE();
    case 16:
      if (lookahead == 'd') ADVANCE(24);
      END_STATE();
    case 17:
      if (lookahead == 'g') ADVANCE(25);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(26);
      END_STATE();
    case 19:
      if (lookahead == '*') SKIP(19)
      if (lookahead == '/') SKIP(0)
      if (lookahead != 0) SKIP(27)
      END_STATE();
    case 20:
      if (lookahead == 'e') ADVANCE(28);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_for);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_gen);
      END_STATE();
    case 23:
      if (lookahead == 't') ADVANCE(29);
      END_STATE();
    case 24:
      if (lookahead == 'u') ADVANCE(30);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_reg);
      END_STATE();
    case 26:
      if (lookahead == 't') ADVANCE(31);
      END_STATE();
    case 27:
      if (lookahead == '*') SKIP(32)
      if (lookahead != 0) SKIP(27)
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 29:
      if (lookahead == 'i') ADVANCE(33);
      END_STATE();
    case 30:
      if (lookahead == 'l') ADVANCE(34);
      END_STATE();
    case 31:
      if (lookahead == 'e') ADVANCE(35);
      END_STATE();
    case 32:
      if (lookahead == '*') SKIP(32)
      if (lookahead == '/') SKIP(0)
      if (lookahead != 0) SKIP(27)
      END_STATE();
    case 33:
      if (lookahead == 'a') ADVANCE(36);
      END_STATE();
    case 34:
      if (lookahead == 'e') ADVANCE(37);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_state);
      END_STATE();
    case 36:
      if (lookahead == 'l') ADVANCE(38);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_module);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_initial);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 12},
  [3] = {.lex_state = 12},
  [4] = {.lex_state = 12},
  [5] = {.lex_state = 12},
  [6] = {.lex_state = 2},
  [7] = {.lex_state = 2},
  [8] = {.lex_state = 12},
  [9] = {.lex_state = 2},
  [10] = {.lex_state = 2},
  [11] = {.lex_state = 2},
  [12] = {.lex_state = 12},
  [13] = {.lex_state = 2},
  [14] = {.lex_state = 12},
  [15] = {.lex_state = 12},
  [16] = {.lex_state = 3},
  [17] = {.lex_state = 3},
  [18] = {.lex_state = 3},
  [19] = {.lex_state = 3},
  [20] = {.lex_state = 3},
  [21] = {.lex_state = 3},
  [22] = {.lex_state = 3},
  [23] = {.lex_state = 3},
  [24] = {.lex_state = 12},
  [25] = {.lex_state = 3},
  [26] = {.lex_state = 3},
  [27] = {.lex_state = 12},
  [28] = {.lex_state = 3},
  [29] = {.lex_state = 3},
  [30] = {.lex_state = 3},
  [31] = {.lex_state = 3},
  [32] = {.lex_state = 12},
  [33] = {.lex_state = 2},
  [34] = {.lex_state = 2},
  [35] = {.lex_state = 2},
  [36] = {.lex_state = 12},
  [37] = {.lex_state = 2},
  [38] = {.lex_state = 2},
  [39] = {.lex_state = 2},
  [40] = {.lex_state = 12},
  [41] = {.lex_state = 12},
  [42] = {.lex_state = 12},
  [43] = {.lex_state = 12},
  [44] = {.lex_state = 12},
  [45] = {.lex_state = 12},
  [46] = {.lex_state = 12},
  [47] = {.lex_state = 12},
  [48] = {.lex_state = 12},
  [49] = {.lex_state = 12},
  [50] = {.lex_state = 12},
  [51] = {.lex_state = 12},
  [52] = {.lex_state = 12},
  [53] = {.lex_state = 12},
  [54] = {.lex_state = 12},
  [55] = {.lex_state = 12},
  [56] = {.lex_state = 12},
  [57] = {.lex_state = 2},
  [58] = {.lex_state = 12},
  [59] = {.lex_state = 12},
  [60] = {.lex_state = 12},
  [61] = {.lex_state = 12},
  [62] = {.lex_state = 12},
  [63] = {.lex_state = 2},
  [64] = {.lex_state = 2},
  [65] = {.lex_state = 2},
  [66] = {.lex_state = 2},
  [67] = {.lex_state = 2},
  [68] = {.lex_state = 2},
  [69] = {.lex_state = 3},
  [70] = {.lex_state = 2},
  [71] = {.lex_state = 12},
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
  [87] = {.lex_state = 7},
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
  [107] = {.lex_state = 0},
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
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
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
    [anon_sym_PERCENT] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_reg] = ACTIONS(1),
    [anon_sym_initial] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [anon_sym_for] = ACTIONS(1),
    [anon_sym_in] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(113),
    [sym_module] = STATE(93),
    [aux_sym_source_file_repeat1] = STATE(93),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_module] = ACTIONS(5),
  },
  [2] = {
    [sym_global_identifier] = STATE(35),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(98),
    [sym__type] = STATE(98),
    [sym_declaration] = STATE(79),
    [sym_unary_op] = STATE(33),
    [sym_binary_op] = STATE(33),
    [sym_array_op] = STATE(33),
    [sym_func_call] = STATE(33),
    [sym_parenthesis_expression] = STATE(33),
    [sym__expression] = STATE(33),
    [sym_block] = STATE(43),
    [sym_assign_left_side] = STATE(104),
    [sym_decl_assign_statement] = STATE(109),
    [sym_if_statement] = STATE(43),
    [sym_for_statement] = STATE(43),
    [sym__statement] = STATE(43),
    [aux_sym_block_repeat1] = STATE(2),
    [aux_sym_assign_left_side_repeat1] = STATE(14),
    [sym_identifier] = ACTIONS(7),
    [sym_number] = ACTIONS(10),
    [anon_sym_COLON_COLON] = ACTIONS(13),
    [anon_sym_state] = ACTIONS(16),
    [anon_sym_gen] = ACTIONS(16),
    [anon_sym_PLUS] = ACTIONS(19),
    [anon_sym_DASH] = ACTIONS(19),
    [anon_sym_STAR] = ACTIONS(19),
    [anon_sym_BANG] = ACTIONS(19),
    [anon_sym_PIPE] = ACTIONS(19),
    [anon_sym_AMP] = ACTIONS(19),
    [anon_sym_CARET] = ACTIONS(19),
    [anon_sym_LPAREN] = ACTIONS(22),
    [anon_sym_LBRACE] = ACTIONS(25),
    [anon_sym_RBRACE] = ACTIONS(28),
    [anon_sym_reg] = ACTIONS(30),
    [anon_sym_initial] = ACTIONS(33),
    [anon_sym_if] = ACTIONS(36),
    [anon_sym_for] = ACTIONS(39),
  },
  [3] = {
    [sym_global_identifier] = STATE(35),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(98),
    [sym__type] = STATE(98),
    [sym_declaration] = STATE(79),
    [sym_unary_op] = STATE(33),
    [sym_binary_op] = STATE(33),
    [sym_array_op] = STATE(33),
    [sym_func_call] = STATE(33),
    [sym_parenthesis_expression] = STATE(33),
    [sym__expression] = STATE(33),
    [sym_block] = STATE(43),
    [sym_assign_left_side] = STATE(104),
    [sym_decl_assign_statement] = STATE(109),
    [sym_if_statement] = STATE(43),
    [sym_for_statement] = STATE(43),
    [sym__statement] = STATE(43),
    [aux_sym_block_repeat1] = STATE(4),
    [aux_sym_assign_left_side_repeat1] = STATE(14),
    [sym_identifier] = ACTIONS(42),
    [sym_number] = ACTIONS(44),
    [anon_sym_COLON_COLON] = ACTIONS(46),
    [anon_sym_state] = ACTIONS(48),
    [anon_sym_gen] = ACTIONS(48),
    [anon_sym_PLUS] = ACTIONS(50),
    [anon_sym_DASH] = ACTIONS(50),
    [anon_sym_STAR] = ACTIONS(50),
    [anon_sym_BANG] = ACTIONS(50),
    [anon_sym_PIPE] = ACTIONS(50),
    [anon_sym_AMP] = ACTIONS(50),
    [anon_sym_CARET] = ACTIONS(50),
    [anon_sym_LPAREN] = ACTIONS(52),
    [anon_sym_LBRACE] = ACTIONS(54),
    [anon_sym_RBRACE] = ACTIONS(56),
    [anon_sym_reg] = ACTIONS(58),
    [anon_sym_initial] = ACTIONS(60),
    [anon_sym_if] = ACTIONS(62),
    [anon_sym_for] = ACTIONS(64),
  },
  [4] = {
    [sym_global_identifier] = STATE(35),
    [sym__maybe_global_identifier] = STATE(19),
    [sym_array_type] = STATE(98),
    [sym__type] = STATE(98),
    [sym_declaration] = STATE(79),
    [sym_unary_op] = STATE(33),
    [sym_binary_op] = STATE(33),
    [sym_array_op] = STATE(33),
    [sym_func_call] = STATE(33),
    [sym_parenthesis_expression] = STATE(33),
    [sym__expression] = STATE(33),
    [sym_block] = STATE(43),
    [sym_assign_left_side] = STATE(104),
    [sym_decl_assign_statement] = STATE(109),
    [sym_if_statement] = STATE(43),
    [sym_for_statement] = STATE(43),
    [sym__statement] = STATE(43),
    [aux_sym_block_repeat1] = STATE(2),
    [aux_sym_assign_left_side_repeat1] = STATE(14),
    [sym_identifier] = ACTIONS(42),
    [sym_number] = ACTIONS(44),
    [anon_sym_COLON_COLON] = ACTIONS(46),
    [anon_sym_state] = ACTIONS(48),
    [anon_sym_gen] = ACTIONS(48),
    [anon_sym_PLUS] = ACTIONS(50),
    [anon_sym_DASH] = ACTIONS(50),
    [anon_sym_STAR] = ACTIONS(50),
    [anon_sym_BANG] = ACTIONS(50),
    [anon_sym_PIPE] = ACTIONS(50),
    [anon_sym_AMP] = ACTIONS(50),
    [anon_sym_CARET] = ACTIONS(50),
    [anon_sym_LPAREN] = ACTIONS(52),
    [anon_sym_LBRACE] = ACTIONS(54),
    [anon_sym_RBRACE] = ACTIONS(66),
    [anon_sym_reg] = ACTIONS(58),
    [anon_sym_initial] = ACTIONS(60),
    [anon_sym_if] = ACTIONS(62),
    [anon_sym_for] = ACTIONS(64),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 15,
    ACTIONS(42), 1,
      sym_identifier,
    ACTIONS(44), 1,
      sym_number,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(58), 1,
      anon_sym_reg,
    ACTIONS(60), 1,
      anon_sym_initial,
    STATE(14), 1,
      aux_sym_assign_left_side_repeat1,
    STATE(19), 1,
      sym__maybe_global_identifier,
    STATE(35), 1,
      sym_global_identifier,
    STATE(79), 1,
      sym_declaration,
    STATE(111), 1,
      sym_assign_left_side,
    ACTIONS(48), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(98), 2,
      sym_array_type,
      sym__type,
    STATE(33), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [59] = 4,
    ACTIONS(70), 1,
      anon_sym_COLON_COLON,
    ACTIONS(72), 1,
      anon_sym_SLASH,
    STATE(10), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(68), 25,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [96] = 4,
    ACTIONS(70), 1,
      anon_sym_COLON_COLON,
    ACTIONS(72), 1,
      anon_sym_SLASH,
    STATE(9), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(68), 25,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [133] = 15,
    ACTIONS(42), 1,
      sym_identifier,
    ACTIONS(44), 1,
      sym_number,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(58), 1,
      anon_sym_reg,
    ACTIONS(60), 1,
      anon_sym_initial,
    STATE(14), 1,
      aux_sym_assign_left_side_repeat1,
    STATE(19), 1,
      sym__maybe_global_identifier,
    STATE(35), 1,
      sym_global_identifier,
    STATE(79), 1,
      sym_declaration,
    STATE(102), 1,
      sym_assign_left_side,
    ACTIONS(48), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(98), 2,
      sym_array_type,
      sym__type,
    STATE(33), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [192] = 4,
    ACTIONS(76), 1,
      anon_sym_COLON_COLON,
    ACTIONS(79), 1,
      anon_sym_SLASH,
    STATE(9), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(74), 25,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [229] = 4,
    ACTIONS(70), 1,
      anon_sym_COLON_COLON,
    ACTIONS(83), 1,
      anon_sym_SLASH,
    STATE(9), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(81), 25,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [266] = 4,
    ACTIONS(70), 1,
      anon_sym_COLON_COLON,
    ACTIONS(87), 1,
      anon_sym_SLASH,
    STATE(7), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(85), 25,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [303] = 14,
    ACTIONS(42), 1,
      sym_identifier,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(89), 1,
      sym_number,
    ACTIONS(91), 1,
      anon_sym_reg,
    ACTIONS(93), 1,
      anon_sym_initial,
    STATE(15), 1,
      aux_sym_assign_left_side_repeat1,
    STATE(19), 1,
      sym__maybe_global_identifier,
    STATE(35), 1,
      sym_global_identifier,
    STATE(84), 1,
      sym_declaration,
    ACTIONS(48), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(98), 2,
      sym_array_type,
      sym__type,
    STATE(38), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [359] = 2,
    ACTIONS(79), 1,
      anon_sym_SLASH,
    ACTIONS(74), 26,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [391] = 13,
    ACTIONS(42), 1,
      sym_identifier,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      sym_number,
    ACTIONS(97), 1,
      anon_sym_reg,
    STATE(19), 1,
      sym__maybe_global_identifier,
    STATE(35), 1,
      sym_global_identifier,
    STATE(71), 1,
      aux_sym_assign_left_side_repeat1,
    STATE(75), 1,
      sym_declaration,
    ACTIONS(48), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(98), 2,
      sym_array_type,
      sym__type,
    STATE(37), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [444] = 13,
    ACTIONS(42), 1,
      sym_identifier,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(97), 1,
      anon_sym_reg,
    ACTIONS(99), 1,
      sym_number,
    STATE(19), 1,
      sym__maybe_global_identifier,
    STATE(35), 1,
      sym_global_identifier,
    STATE(71), 1,
      aux_sym_assign_left_side_repeat1,
    STATE(85), 1,
      sym_declaration,
    ACTIONS(48), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(98), 2,
      sym_array_type,
      sym__type,
    STATE(39), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [497] = 4,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(101), 21,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [531] = 7,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(109), 1,
      anon_sym_PIPE,
    ACTIONS(111), 1,
      anon_sym_CARET,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(101), 17,
      anon_sym_COLON,
      anon_sym_DASH_GT,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [571] = 3,
    ACTIONS(115), 1,
      anon_sym_SLASH,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(113), 23,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [603] = 3,
    ACTIONS(119), 1,
      anon_sym_SLASH,
    ACTIONS(121), 1,
      anon_sym_LPAREN,
    ACTIONS(117), 23,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [635] = 3,
    ACTIONS(123), 1,
      anon_sym_SLASH,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(101), 23,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [667] = 8,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(109), 1,
      anon_sym_PIPE,
    ACTIONS(111), 1,
      anon_sym_CARET,
    ACTIONS(125), 1,
      anon_sym_AMP,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(101), 16,
      anon_sym_COLON,
      anon_sym_DASH_GT,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [709] = 5,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(101), 19,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [745] = 6,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(111), 1,
      anon_sym_CARET,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(101), 18,
      anon_sym_COLON,
      anon_sym_DASH_GT,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [783] = 11,
    ACTIONS(42), 1,
      sym_identifier,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(99), 1,
      sym_number,
    STATE(19), 1,
      sym__maybe_global_identifier,
    STATE(35), 1,
      sym_global_identifier,
    STATE(85), 1,
      sym_declaration,
    ACTIONS(48), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(98), 2,
      sym_array_type,
      sym__type,
    STATE(39), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [830] = 2,
    ACTIONS(129), 1,
      anon_sym_SLASH,
    ACTIONS(127), 23,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [859] = 2,
    ACTIONS(133), 1,
      anon_sym_SLASH,
    ACTIONS(131), 23,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [888] = 11,
    ACTIONS(42), 1,
      sym_identifier,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      sym_number,
    STATE(19), 1,
      sym__maybe_global_identifier,
    STATE(35), 1,
      sym_global_identifier,
    STATE(75), 1,
      sym_declaration,
    ACTIONS(48), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(98), 2,
      sym_array_type,
      sym__type,
    STATE(37), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [935] = 2,
    ACTIONS(137), 1,
      anon_sym_SLASH,
    ACTIONS(135), 23,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [964] = 2,
    ACTIONS(141), 1,
      anon_sym_SLASH,
    ACTIONS(139), 23,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [993] = 2,
    ACTIONS(145), 1,
      anon_sym_SLASH,
    ACTIONS(143), 23,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [1022] = 2,
    ACTIONS(149), 1,
      anon_sym_SLASH,
    ACTIONS(147), 23,
      anon_sym_COLON,
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
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [1051] = 2,
    ACTIONS(153), 9,
      anon_sym_module,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_else,
      anon_sym_for,
    ACTIONS(151), 13,
      ts_builtin_sym_end,
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
  [1078] = 12,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(109), 1,
      anon_sym_PIPE,
    ACTIONS(111), 1,
      anon_sym_CARET,
    ACTIONS(125), 1,
      anon_sym_AMP,
    ACTIONS(159), 1,
      anon_sym_COMMA,
    ACTIONS(161), 1,
      anon_sym_LBRACK,
    STATE(30), 1,
      sym_array_bracket_expression,
    STATE(77), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(155), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
    ACTIONS(157), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1125] = 10,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(109), 1,
      anon_sym_PIPE,
    ACTIONS(111), 1,
      anon_sym_CARET,
    ACTIONS(125), 1,
      anon_sym_AMP,
    ACTIONS(161), 1,
      anon_sym_LBRACK,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(157), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
    ACTIONS(163), 6,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [1168] = 4,
    ACTIONS(165), 1,
      sym_identifier,
    ACTIONS(169), 1,
      anon_sym_SLASH,
    ACTIONS(171), 1,
      anon_sym_LBRACK,
    ACTIONS(167), 19,
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
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [1199] = 2,
    ACTIONS(176), 9,
      anon_sym_module,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_else,
      anon_sym_for,
    ACTIONS(174), 13,
      ts_builtin_sym_end,
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
  [1226] = 12,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(109), 1,
      anon_sym_PIPE,
    ACTIONS(111), 1,
      anon_sym_CARET,
    ACTIONS(125), 1,
      anon_sym_AMP,
    ACTIONS(159), 1,
      anon_sym_COMMA,
    ACTIONS(161), 1,
      anon_sym_LBRACK,
    STATE(30), 1,
      sym_array_bracket_expression,
    STATE(82), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(178), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
    ACTIONS(157), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1273] = 10,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(109), 1,
      anon_sym_PIPE,
    ACTIONS(111), 1,
      anon_sym_CARET,
    ACTIONS(125), 1,
      anon_sym_AMP,
    ACTIONS(161), 1,
      anon_sym_LBRACK,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(180), 5,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
    ACTIONS(157), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1315] = 10,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(109), 1,
      anon_sym_PIPE,
    ACTIONS(111), 1,
      anon_sym_CARET,
    ACTIONS(125), 1,
      anon_sym_AMP,
    ACTIONS(161), 1,
      anon_sym_LBRACK,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(182), 5,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
    ACTIONS(157), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1357] = 3,
    ACTIONS(188), 1,
      anon_sym_else,
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
  [1384] = 8,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(192), 1,
      sym_number,
    STATE(105), 1,
      sym_range,
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
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1421] = 8,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(194), 1,
      sym_number,
    ACTIONS(196), 1,
      anon_sym_RPAREN,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(57), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1458] = 2,
    ACTIONS(198), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(200), 12,
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
  [1482] = 7,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(202), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(66), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1516] = 7,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(204), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(18), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1550] = 7,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(206), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(70), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1584] = 7,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(208), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(67), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1618] = 7,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(210), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(65), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1652] = 7,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(212), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(16), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1686] = 7,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(214), 1,
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
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1720] = 7,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(216), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(23), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1754] = 7,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(218), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(17), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1788] = 7,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(220), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(22), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1822] = 7,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(222), 1,
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
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1856] = 7,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(224), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(64), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1890] = 7,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(226), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(34), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1924] = 12,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(109), 1,
      anon_sym_PIPE,
    ACTIONS(111), 1,
      anon_sym_CARET,
    ACTIONS(125), 1,
      anon_sym_AMP,
    ACTIONS(161), 1,
      anon_sym_LBRACK,
    ACTIONS(228), 1,
      anon_sym_COMMA,
    ACTIONS(230), 1,
      anon_sym_RPAREN,
    STATE(30), 1,
      sym_array_bracket_expression,
    STATE(96), 1,
      aux_sym_func_call_repeat1,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(157), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1968] = 2,
    ACTIONS(232), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(234), 12,
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
  [1992] = 7,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(236), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(68), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2026] = 2,
    ACTIONS(238), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(240), 12,
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
  [2050] = 7,
    ACTIONS(46), 1,
      anon_sym_COLON_COLON,
    ACTIONS(52), 1,
      anon_sym_LPAREN,
    ACTIONS(190), 1,
      sym_identifier,
    ACTIONS(242), 1,
      sym_number,
    STATE(19), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(63), 6,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym_parenthesis_expression,
      sym__expression,
    ACTIONS(50), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2084] = 2,
    ACTIONS(244), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(246), 12,
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
  [2108] = 10,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(109), 1,
      anon_sym_PIPE,
    ACTIONS(111), 1,
      anon_sym_CARET,
    ACTIONS(125), 1,
      anon_sym_AMP,
    ACTIONS(161), 1,
      anon_sym_LBRACK,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(248), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
    ACTIONS(157), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2147] = 11,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(109), 1,
      anon_sym_PIPE,
    ACTIONS(111), 1,
      anon_sym_CARET,
    ACTIONS(125), 1,
      anon_sym_AMP,
    ACTIONS(161), 1,
      anon_sym_LBRACK,
    ACTIONS(250), 1,
      anon_sym_LBRACE,
    STATE(30), 1,
      sym_array_bracket_expression,
    STATE(40), 1,
      sym_block,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(157), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2188] = 10,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(109), 1,
      anon_sym_PIPE,
    ACTIONS(111), 1,
      anon_sym_CARET,
    ACTIONS(125), 1,
      anon_sym_AMP,
    ACTIONS(161), 1,
      anon_sym_LBRACK,
    ACTIONS(252), 1,
      anon_sym_RPAREN,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(157), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2226] = 10,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(109), 1,
      anon_sym_PIPE,
    ACTIONS(111), 1,
      anon_sym_CARET,
    ACTIONS(125), 1,
      anon_sym_AMP,
    ACTIONS(161), 1,
      anon_sym_LBRACK,
    ACTIONS(254), 1,
      anon_sym_LBRACE,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(157), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2264] = 10,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(109), 1,
      anon_sym_PIPE,
    ACTIONS(111), 1,
      anon_sym_CARET,
    ACTIONS(125), 1,
      anon_sym_AMP,
    ACTIONS(161), 1,
      anon_sym_LBRACK,
    ACTIONS(256), 1,
      anon_sym_RBRACK,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(157), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2302] = 10,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(109), 1,
      anon_sym_PIPE,
    ACTIONS(111), 1,
      anon_sym_CARET,
    ACTIONS(125), 1,
      anon_sym_AMP,
    ACTIONS(161), 1,
      anon_sym_LBRACK,
    ACTIONS(258), 1,
      anon_sym_SEMI,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(157), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2340] = 10,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(109), 1,
      anon_sym_PIPE,
    ACTIONS(111), 1,
      anon_sym_CARET,
    ACTIONS(125), 1,
      anon_sym_AMP,
    ACTIONS(161), 1,
      anon_sym_LBRACK,
    ACTIONS(260), 1,
      anon_sym_COLON,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(157), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2378] = 10,
    ACTIONS(105), 1,
      anon_sym_SLASH,
    ACTIONS(109), 1,
      anon_sym_PIPE,
    ACTIONS(111), 1,
      anon_sym_CARET,
    ACTIONS(125), 1,
      anon_sym_AMP,
    ACTIONS(161), 1,
      anon_sym_LBRACK,
    ACTIONS(262), 1,
      anon_sym_RBRACK,
    STATE(30), 1,
      sym_array_bracket_expression,
    ACTIONS(103), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(107), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(157), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2416] = 4,
    ACTIONS(268), 1,
      anon_sym_reg,
    STATE(71), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(264), 3,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
    ACTIONS(266), 10,
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
  [2440] = 3,
    ACTIONS(273), 1,
      anon_sym_SQUOTE,
    STATE(83), 1,
      sym_latency_specifier,
    ACTIONS(271), 6,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [2455] = 3,
    ACTIONS(273), 1,
      anon_sym_SQUOTE,
    STATE(81), 1,
      sym_latency_specifier,
    ACTIONS(275), 6,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [2470] = 5,
    ACTIONS(277), 1,
      sym_identifier,
    ACTIONS(279), 1,
      anon_sym_COLON_COLON,
    STATE(117), 1,
      sym_declaration,
    ACTIONS(48), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(98), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [2489] = 3,
    ACTIONS(283), 1,
      anon_sym_COMMA,
    STATE(76), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(281), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [2502] = 3,
    ACTIONS(283), 1,
      anon_sym_COMMA,
    STATE(78), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(285), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [2515] = 3,
    ACTIONS(283), 1,
      anon_sym_COMMA,
    STATE(78), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(287), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [2528] = 3,
    ACTIONS(291), 1,
      anon_sym_COMMA,
    STATE(78), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(289), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [2541] = 3,
    ACTIONS(283), 1,
      anon_sym_COMMA,
    STATE(80), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(294), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [2554] = 3,
    ACTIONS(283), 1,
      anon_sym_COMMA,
    STATE(78), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(287), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [2567] = 1,
    ACTIONS(296), 6,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [2576] = 3,
    ACTIONS(283), 1,
      anon_sym_COMMA,
    STATE(78), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(285), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [2589] = 1,
    ACTIONS(298), 6,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [2598] = 1,
    ACTIONS(300), 5,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [2606] = 1,
    ACTIONS(302), 5,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [2614] = 3,
    ACTIONS(279), 1,
      anon_sym_COLON_COLON,
    ACTIONS(304), 1,
      sym_identifier,
    STATE(99), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [2626] = 4,
    ACTIONS(54), 1,
      anon_sym_LBRACE,
    ACTIONS(306), 1,
      anon_sym_COLON,
    STATE(101), 1,
      sym_block,
    STATE(108), 1,
      sym_interface_ports,
  [2639] = 3,
    ACTIONS(308), 1,
      anon_sym_COLON_COLON,
    STATE(89), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(72), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [2650] = 3,
    ACTIONS(308), 1,
      anon_sym_COLON_COLON,
    STATE(90), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(83), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [2661] = 3,
    ACTIONS(310), 1,
      anon_sym_COLON_COLON,
    STATE(90), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(79), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [2672] = 3,
    ACTIONS(308), 1,
      anon_sym_COLON_COLON,
    STATE(90), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(72), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [2683] = 3,
    ACTIONS(308), 1,
      anon_sym_COLON_COLON,
    STATE(91), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(87), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [2694] = 3,
    ACTIONS(5), 1,
      anon_sym_module,
    ACTIONS(313), 1,
      ts_builtin_sym_end,
    STATE(94), 2,
      sym_module,
      aux_sym_source_file_repeat1,
  [2705] = 3,
    ACTIONS(315), 1,
      ts_builtin_sym_end,
    ACTIONS(317), 1,
      anon_sym_module,
    STATE(94), 2,
      sym_module,
      aux_sym_source_file_repeat1,
  [2716] = 3,
    ACTIONS(54), 1,
      anon_sym_LBRACE,
    ACTIONS(320), 1,
      anon_sym_if,
    STATE(58), 2,
      sym_block,
      sym_if_statement,
  [2727] = 3,
    ACTIONS(322), 1,
      anon_sym_COMMA,
    ACTIONS(324), 1,
      anon_sym_RPAREN,
    STATE(97), 1,
      aux_sym_func_call_repeat1,
  [2737] = 3,
    ACTIONS(326), 1,
      anon_sym_COMMA,
    ACTIONS(329), 1,
      anon_sym_RPAREN,
    STATE(97), 1,
      aux_sym_func_call_repeat1,
  [2747] = 3,
    ACTIONS(331), 1,
      sym_identifier,
    ACTIONS(333), 1,
      anon_sym_LBRACK,
    STATE(106), 1,
      sym_array_bracket_expression,
  [2757] = 3,
    ACTIONS(333), 1,
      anon_sym_LBRACK,
    ACTIONS(335), 1,
      sym_identifier,
    STATE(106), 1,
      sym_array_bracket_expression,
  [2767] = 1,
    ACTIONS(79), 3,
      sym_identifier,
      anon_sym_COLON_COLON,
      anon_sym_LBRACK,
  [2773] = 1,
    ACTIONS(337), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [2778] = 2,
    ACTIONS(339), 1,
      anon_sym_DASH_GT,
    ACTIONS(341), 1,
      anon_sym_LBRACE,
  [2785] = 1,
    ACTIONS(137), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [2790] = 2,
    ACTIONS(343), 1,
      anon_sym_EQ,
    ACTIONS(345), 1,
      anon_sym_SEMI,
  [2797] = 2,
    ACTIONS(54), 1,
      anon_sym_LBRACE,
    STATE(62), 1,
      sym_block,
  [2804] = 1,
    ACTIONS(347), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [2809] = 1,
    ACTIONS(349), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [2814] = 2,
    ACTIONS(54), 1,
      anon_sym_LBRACE,
    STATE(107), 1,
      sym_block,
  [2821] = 1,
    ACTIONS(345), 1,
      anon_sym_SEMI,
  [2825] = 1,
    ACTIONS(351), 1,
      sym_identifier,
  [2829] = 1,
    ACTIONS(353), 1,
      anon_sym_LBRACE,
  [2833] = 1,
    ACTIONS(355), 1,
      sym_identifier,
  [2837] = 1,
    ACTIONS(357), 1,
      ts_builtin_sym_end,
  [2841] = 1,
    ACTIONS(359), 1,
      sym_identifier,
  [2845] = 1,
    ACTIONS(361), 1,
      sym_identifier,
  [2849] = 1,
    ACTIONS(363), 1,
      sym_identifier,
  [2853] = 1,
    ACTIONS(365), 1,
      anon_sym_in,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(5)] = 0,
  [SMALL_STATE(6)] = 59,
  [SMALL_STATE(7)] = 96,
  [SMALL_STATE(8)] = 133,
  [SMALL_STATE(9)] = 192,
  [SMALL_STATE(10)] = 229,
  [SMALL_STATE(11)] = 266,
  [SMALL_STATE(12)] = 303,
  [SMALL_STATE(13)] = 359,
  [SMALL_STATE(14)] = 391,
  [SMALL_STATE(15)] = 444,
  [SMALL_STATE(16)] = 497,
  [SMALL_STATE(17)] = 531,
  [SMALL_STATE(18)] = 571,
  [SMALL_STATE(19)] = 603,
  [SMALL_STATE(20)] = 635,
  [SMALL_STATE(21)] = 667,
  [SMALL_STATE(22)] = 709,
  [SMALL_STATE(23)] = 745,
  [SMALL_STATE(24)] = 783,
  [SMALL_STATE(25)] = 830,
  [SMALL_STATE(26)] = 859,
  [SMALL_STATE(27)] = 888,
  [SMALL_STATE(28)] = 935,
  [SMALL_STATE(29)] = 964,
  [SMALL_STATE(30)] = 993,
  [SMALL_STATE(31)] = 1022,
  [SMALL_STATE(32)] = 1051,
  [SMALL_STATE(33)] = 1078,
  [SMALL_STATE(34)] = 1125,
  [SMALL_STATE(35)] = 1168,
  [SMALL_STATE(36)] = 1199,
  [SMALL_STATE(37)] = 1226,
  [SMALL_STATE(38)] = 1273,
  [SMALL_STATE(39)] = 1315,
  [SMALL_STATE(40)] = 1357,
  [SMALL_STATE(41)] = 1384,
  [SMALL_STATE(42)] = 1421,
  [SMALL_STATE(43)] = 1458,
  [SMALL_STATE(44)] = 1482,
  [SMALL_STATE(45)] = 1516,
  [SMALL_STATE(46)] = 1550,
  [SMALL_STATE(47)] = 1584,
  [SMALL_STATE(48)] = 1618,
  [SMALL_STATE(49)] = 1652,
  [SMALL_STATE(50)] = 1686,
  [SMALL_STATE(51)] = 1720,
  [SMALL_STATE(52)] = 1754,
  [SMALL_STATE(53)] = 1788,
  [SMALL_STATE(54)] = 1822,
  [SMALL_STATE(55)] = 1856,
  [SMALL_STATE(56)] = 1890,
  [SMALL_STATE(57)] = 1924,
  [SMALL_STATE(58)] = 1968,
  [SMALL_STATE(59)] = 1992,
  [SMALL_STATE(60)] = 2026,
  [SMALL_STATE(61)] = 2050,
  [SMALL_STATE(62)] = 2084,
  [SMALL_STATE(63)] = 2108,
  [SMALL_STATE(64)] = 2147,
  [SMALL_STATE(65)] = 2188,
  [SMALL_STATE(66)] = 2226,
  [SMALL_STATE(67)] = 2264,
  [SMALL_STATE(68)] = 2302,
  [SMALL_STATE(69)] = 2340,
  [SMALL_STATE(70)] = 2378,
  [SMALL_STATE(71)] = 2416,
  [SMALL_STATE(72)] = 2440,
  [SMALL_STATE(73)] = 2455,
  [SMALL_STATE(74)] = 2470,
  [SMALL_STATE(75)] = 2489,
  [SMALL_STATE(76)] = 2502,
  [SMALL_STATE(77)] = 2515,
  [SMALL_STATE(78)] = 2528,
  [SMALL_STATE(79)] = 2541,
  [SMALL_STATE(80)] = 2554,
  [SMALL_STATE(81)] = 2567,
  [SMALL_STATE(82)] = 2576,
  [SMALL_STATE(83)] = 2589,
  [SMALL_STATE(84)] = 2598,
  [SMALL_STATE(85)] = 2606,
  [SMALL_STATE(86)] = 2614,
  [SMALL_STATE(87)] = 2626,
  [SMALL_STATE(88)] = 2639,
  [SMALL_STATE(89)] = 2650,
  [SMALL_STATE(90)] = 2661,
  [SMALL_STATE(91)] = 2672,
  [SMALL_STATE(92)] = 2683,
  [SMALL_STATE(93)] = 2694,
  [SMALL_STATE(94)] = 2705,
  [SMALL_STATE(95)] = 2716,
  [SMALL_STATE(96)] = 2727,
  [SMALL_STATE(97)] = 2737,
  [SMALL_STATE(98)] = 2747,
  [SMALL_STATE(99)] = 2757,
  [SMALL_STATE(100)] = 2767,
  [SMALL_STATE(101)] = 2773,
  [SMALL_STATE(102)] = 2778,
  [SMALL_STATE(103)] = 2785,
  [SMALL_STATE(104)] = 2790,
  [SMALL_STATE(105)] = 2797,
  [SMALL_STATE(106)] = 2804,
  [SMALL_STATE(107)] = 2809,
  [SMALL_STATE(108)] = 2814,
  [SMALL_STATE(109)] = 2821,
  [SMALL_STATE(110)] = 2825,
  [SMALL_STATE(111)] = 2829,
  [SMALL_STATE(112)] = 2833,
  [SMALL_STATE(113)] = 2837,
  [SMALL_STATE(114)] = 2841,
  [SMALL_STATE(115)] = 2845,
  [SMALL_STATE(116)] = 2849,
  [SMALL_STATE(117)] = 2853,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [7] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(11),
  [10] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(33),
  [13] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(112),
  [16] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(86),
  [19] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(45),
  [22] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(48),
  [25] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(3),
  [28] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12),
  [30] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(14),
  [33] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(27),
  [36] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(55),
  [39] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(74),
  [42] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [44] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [46] = {.entry = {.count = 1, .reusable = true}}, SHIFT(112),
  [48] = {.entry = {.count = 1, .reusable = false}}, SHIFT(86),
  [50] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [52] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [54] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [56] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [58] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [60] = {.entry = {.count = 1, .reusable = false}}, SHIFT(27),
  [62] = {.entry = {.count = 1, .reusable = false}}, SHIFT(55),
  [64] = {.entry = {.count = 1, .reusable = false}}, SHIFT(74),
  [66] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [68] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 2),
  [70] = {.entry = {.count = 1, .reusable = false}}, SHIFT(110),
  [72] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 2),
  [74] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_global_identifier_repeat1, 2),
  [76] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_identifier_repeat1, 2), SHIFT_REPEAT(110),
  [79] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_global_identifier_repeat1, 2),
  [81] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 3),
  [83] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 3),
  [85] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 1),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 1),
  [89] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [91] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [93] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [95] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [97] = {.entry = {.count = 1, .reusable = false}}, SHIFT(71),
  [99] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [101] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_op, 3, .production_id = 20),
  [103] = {.entry = {.count = 1, .reusable = false}}, SHIFT(50),
  [105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [107] = {.entry = {.count = 1, .reusable = false}}, SHIFT(49),
  [109] = {.entry = {.count = 1, .reusable = false}}, SHIFT(51),
  [111] = {.entry = {.count = 1, .reusable = false}}, SHIFT(53),
  [113] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_op, 2, .production_id = 6),
  [115] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_op, 2, .production_id = 6),
  [117] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expression, 1),
  [119] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expression, 1),
  [121] = {.entry = {.count = 1, .reusable = false}}, SHIFT(42),
  [123] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_op, 3, .production_id = 20),
  [125] = {.entry = {.count = 1, .reusable = false}}, SHIFT(52),
  [127] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 3, .production_id = 16),
  [129] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 3, .production_id = 16),
  [131] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 14),
  [133] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 14),
  [135] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 14),
  [137] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 14),
  [139] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 4, .production_id = 25),
  [141] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 4, .production_id = 25),
  [143] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_op, 2, .production_id = 9),
  [145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_op, 2, .production_id = 9),
  [147] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 5, .production_id = 28),
  [149] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 5, .production_id = 28),
  [151] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [153] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 2),
  [155] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_left_side, 1, .production_id = 2),
  [157] = {.entry = {.count = 1, .reusable = false}}, SHIFT(54),
  [159] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [161] = {.entry = {.count = 1, .reusable = false}}, SHIFT(47),
  [163] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_latency_specifier, 2, .production_id = 14),
  [165] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__type, 1),
  [167] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__maybe_global_identifier, 1),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__maybe_global_identifier, 1),
  [171] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym__maybe_global_identifier, 1), REDUCE(sym__type, 1),
  [174] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, .production_id = 11),
  [176] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 3, .production_id = 11),
  [178] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_left_side, 2, .production_id = 7),
  [180] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_assign_left_side_repeat2, 2, .production_id = 18),
  [182] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_assign_left_side_repeat2, 3, .production_id = 26),
  [184] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if_statement, 3, .production_id = 22),
  [186] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 3, .production_id = 22),
  [188] = {.entry = {.count = 1, .reusable = false}}, SHIFT(95),
  [190] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [192] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [194] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [196] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [198] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 1, .production_id = 4),
  [200] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 1, .production_id = 4),
  [202] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [204] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [206] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [208] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [210] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [212] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [214] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [216] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [218] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [220] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [222] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [224] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [226] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [228] = {.entry = {.count = 1, .reusable = false}}, SHIFT(61),
  [230] = {.entry = {.count = 1, .reusable = false}}, SHIFT(29),
  [232] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if_statement, 5, .production_id = 30),
  [234] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 5, .production_id = 30),
  [236] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [238] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__statement, 2),
  [240] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__statement, 2),
  [242] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [244] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_for_statement, 5, .production_id = 31),
  [246] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_statement, 5, .production_id = 31),
  [248] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_func_call_repeat1, 2, .production_id = 27),
  [250] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [252] = {.entry = {.count = 1, .reusable = false}}, SHIFT(26),
  [254] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_range, 3, .production_id = 32),
  [256] = {.entry = {.count = 1, .reusable = false}}, SHIFT(28),
  [258] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_decl_assign_statement, 3, .production_id = 23),
  [260] = {.entry = {.count = 1, .reusable = false}}, SHIFT(44),
  [262] = {.entry = {.count = 1, .reusable = false}}, SHIFT(103),
  [264] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_assign_left_side_repeat1, 2),
  [266] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2),
  [268] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_assign_left_side_repeat1, 2), SHIFT_REPEAT(71),
  [271] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 13),
  [273] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [275] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 2, .production_id = 8),
  [277] = {.entry = {.count = 1, .reusable = false}}, SHIFT(92),
  [279] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [281] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 2, .production_id = 7),
  [283] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [285] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 3, .production_id = 15),
  [287] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 2, .production_id = 10),
  [289] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat2, 2, .production_id = 19),
  [291] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat2, 2, .production_id = 19), SHIFT_REPEAT(12),
  [294] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 1, .production_id = 2),
  [296] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 17),
  [298] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 24),
  [300] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat2, 2, .production_id = 18),
  [302] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat2, 3, .production_id = 26),
  [304] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [306] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [308] = {.entry = {.count = 1, .reusable = true}}, SHIFT(115),
  [310] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_global_identifier_repeat1, 2), SHIFT_REPEAT(115),
  [313] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [315] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [317] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(116),
  [320] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [322] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [324] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [326] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_func_call_repeat1, 2, .production_id = 29), SHIFT_REPEAT(61),
  [329] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_func_call_repeat1, 2, .production_id = 29),
  [331] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [333] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [335] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [337] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 3, .production_id = 1),
  [339] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [341] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2, .production_id = 3),
  [343] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [345] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [347] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type, 2, .production_id = 9),
  [349] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 4, .production_id = 5),
  [351] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [353] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 4, .production_id = 21),
  [355] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [357] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [359] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [361] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [363] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [365] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
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
    .keyword_lex_fn = ts_lex_keywords,
    .keyword_capture_token = sym_identifier,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
