#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 153
#define LARGE_STATE_COUNT 9
#define SYMBOL_COUNT 69
#define ALIAS_COUNT 0
#define TOKEN_COUNT 40
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 26
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 33

enum {
  anon_sym_COLON = 1,
  anon_sym_DASH_GT = 2,
  anon_sym_module = 3,
  sym_identifier = 4,
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
  [anon_sym_COLON] = ":",
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
  [anon_sym_COLON] = anon_sym_COLON,
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
  [4] = 2,
  [5] = 3,
  [6] = 6,
  [7] = 3,
  [8] = 2,
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
  [50] = 12,
  [51] = 13,
  [52] = 49,
  [53] = 53,
  [54] = 14,
  [55] = 55,
  [56] = 47,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 11,
  [62] = 62,
  [63] = 63,
  [64] = 46,
  [65] = 65,
  [66] = 66,
  [67] = 60,
  [68] = 59,
  [69] = 58,
  [70] = 57,
  [71] = 71,
  [72] = 62,
  [73] = 55,
  [74] = 44,
  [75] = 75,
  [76] = 15,
  [77] = 77,
  [78] = 78,
  [79] = 79,
  [80] = 75,
  [81] = 81,
  [82] = 82,
  [83] = 83,
  [84] = 18,
  [85] = 23,
  [86] = 86,
  [87] = 87,
  [88] = 88,
  [89] = 40,
  [90] = 24,
  [91] = 25,
  [92] = 27,
  [93] = 29,
  [94] = 87,
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
  [112] = 111,
  [113] = 113,
  [114] = 114,
  [115] = 11,
  [116] = 14,
  [117] = 117,
  [118] = 118,
  [119] = 119,
  [120] = 120,
  [121] = 13,
  [122] = 12,
  [123] = 123,
  [124] = 99,
  [125] = 125,
  [126] = 126,
  [127] = 126,
  [128] = 128,
  [129] = 123,
  [130] = 18,
  [131] = 100,
  [132] = 132,
  [133] = 133,
  [134] = 134,
  [135] = 135,
  [136] = 136,
  [137] = 46,
  [138] = 44,
  [139] = 139,
  [140] = 140,
  [141] = 31,
  [142] = 142,
  [143] = 143,
  [144] = 144,
  [145] = 145,
  [146] = 145,
  [147] = 147,
  [148] = 144,
  [149] = 149,
  [150] = 145,
  [151] = 144,
  [152] = 152,
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
      if (lookahead == '!') ADVANCE(104);
      if (lookahead == '%') ADVANCE(115);
      if (lookahead == '&') ADVANCE(106);
      if (lookahead == '\'') ADVANCE(94);
      if (lookahead == '(') ADVANCE(116);
      if (lookahead == ')') ADVANCE(118);
      if (lookahead == '*') ADVANCE(102);
      if (lookahead == '+') ADVANCE(99);
      if (lookahead == ',') ADVANCE(117);
      if (lookahead == '-') ADVANCE(101);
      if (lookahead == '/') SKIP(63)
      if (lookahead == ':') ADVANCE(68);
      if (lookahead == ';') ADVANCE(134);
      if (lookahead == '<') ADVANCE(110);
      if (lookahead == '=') ADVANCE(126);
      if (lookahead == '>') ADVANCE(112);
      if (lookahead == '[') ADVANCE(119);
      if (lookahead == ']') ADVANCE(120);
      if (lookahead == '^') ADVANCE(107);
      if (lookahead == 'e') ADVANCE(51);
      if (lookahead == 'f') ADVANCE(55);
      if (lookahead == 'g') ADVANCE(44);
      if (lookahead == 'i') ADVANCE(49);
      if (lookahead == 'm') ADVANCE(56);
      if (lookahead == 'r') ADVANCE(45);
      if (lookahead == 's') ADVANCE(59);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(105);
      if (lookahead == '}') ADVANCE(122);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(92);
      END_STATE();
    case 1:
      if (lookahead == '\n') SKIP(9)
      if (lookahead != 0) SKIP(1)
      END_STATE();
    case 2:
      if (lookahead == '\n') SKIP(10)
      if (lookahead != 0) SKIP(2)
      END_STATE();
    case 3:
      if (lookahead == '\n') SKIP(11)
      if (lookahead != 0) SKIP(3)
      END_STATE();
    case 4:
      if (lookahead == '\n') SKIP(12)
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
      if (lookahead == '!') ADVANCE(103);
      if (lookahead == '&') ADVANCE(106);
      if (lookahead == '(') ADVANCE(116);
      if (lookahead == ')') ADVANCE(118);
      if (lookahead == '*') ADVANCE(102);
      if (lookahead == '+') ADVANCE(99);
      if (lookahead == '-') ADVANCE(100);
      if (lookahead == '/') SKIP(35)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '[') ADVANCE(119);
      if (lookahead == '^') ADVANCE(107);
      if (lookahead == '|') ADVANCE(105);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(8)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(92);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(91);
      END_STATE();
    case 9:
      if (lookahead == '!') ADVANCE(103);
      if (lookahead == '&') ADVANCE(106);
      if (lookahead == '(') ADVANCE(116);
      if (lookahead == '*') ADVANCE(102);
      if (lookahead == '+') ADVANCE(99);
      if (lookahead == '-') ADVANCE(100);
      if (lookahead == '/') SKIP(17)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '^') ADVANCE(107);
      if (lookahead == 'f') ADVANCE(85);
      if (lookahead == 'g') ADVANCE(73);
      if (lookahead == 'i') ADVANCE(77);
      if (lookahead == 'r') ADVANCE(74);
      if (lookahead == 's') ADVANCE(88);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(105);
      if (lookahead == '}') ADVANCE(122);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(9)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(92);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(91);
      END_STATE();
    case 10:
      if (lookahead == '!') ADVANCE(103);
      if (lookahead == '&') ADVANCE(106);
      if (lookahead == '(') ADVANCE(116);
      if (lookahead == '*') ADVANCE(102);
      if (lookahead == '+') ADVANCE(99);
      if (lookahead == '-') ADVANCE(100);
      if (lookahead == '/') SKIP(32)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '^') ADVANCE(107);
      if (lookahead == 'g') ADVANCE(73);
      if (lookahead == 'i') ADVANCE(83);
      if (lookahead == 'r') ADVANCE(74);
      if (lookahead == 's') ADVANCE(88);
      if (lookahead == '|') ADVANCE(105);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(10)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(92);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(91);
      END_STATE();
    case 11:
      if (lookahead == '!') ADVANCE(103);
      if (lookahead == '&') ADVANCE(106);
      if (lookahead == '(') ADVANCE(116);
      if (lookahead == '*') ADVANCE(102);
      if (lookahead == '+') ADVANCE(99);
      if (lookahead == '-') ADVANCE(100);
      if (lookahead == '/') SKIP(33)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '^') ADVANCE(107);
      if (lookahead == 'g') ADVANCE(73);
      if (lookahead == 'r') ADVANCE(74);
      if (lookahead == 's') ADVANCE(88);
      if (lookahead == '|') ADVANCE(105);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(11)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(92);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(91);
      END_STATE();
    case 12:
      if (lookahead == '!') ADVANCE(103);
      if (lookahead == '&') ADVANCE(106);
      if (lookahead == '(') ADVANCE(116);
      if (lookahead == '*') ADVANCE(102);
      if (lookahead == '+') ADVANCE(99);
      if (lookahead == '-') ADVANCE(100);
      if (lookahead == '/') SKIP(34)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '^') ADVANCE(107);
      if (lookahead == 'g') ADVANCE(73);
      if (lookahead == 's') ADVANCE(88);
      if (lookahead == '|') ADVANCE(105);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(12)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(92);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(91);
      END_STATE();
    case 13:
      if (lookahead == '!') ADVANCE(103);
      if (lookahead == '&') ADVANCE(106);
      if (lookahead == '(') ADVANCE(116);
      if (lookahead == '*') ADVANCE(102);
      if (lookahead == '+') ADVANCE(99);
      if (lookahead == '-') ADVANCE(100);
      if (lookahead == '/') SKIP(36)
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '^') ADVANCE(107);
      if (lookahead == 'e') ADVANCE(82);
      if (lookahead == 'f') ADVANCE(85);
      if (lookahead == 'g') ADVANCE(73);
      if (lookahead == 'i') ADVANCE(77);
      if (lookahead == 'r') ADVANCE(74);
      if (lookahead == 's') ADVANCE(88);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(105);
      if (lookahead == '}') ADVANCE(122);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(13)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(92);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(91);
      END_STATE();
    case 14:
      if (lookahead == '!') ADVANCE(40);
      if (lookahead == '%') ADVANCE(115);
      if (lookahead == '&') ADVANCE(106);
      if (lookahead == '(') ADVANCE(116);
      if (lookahead == ')') ADVANCE(118);
      if (lookahead == '*') ADVANCE(102);
      if (lookahead == '+') ADVANCE(99);
      if (lookahead == ',') ADVANCE(117);
      if (lookahead == '-') ADVANCE(101);
      if (lookahead == '/') ADVANCE(114);
      if (lookahead == ':') ADVANCE(68);
      if (lookahead == ';') ADVANCE(134);
      if (lookahead == '<') ADVANCE(110);
      if (lookahead == '=') ADVANCE(126);
      if (lookahead == '>') ADVANCE(112);
      if (lookahead == '[') ADVANCE(119);
      if (lookahead == ']') ADVANCE(120);
      if (lookahead == '^') ADVANCE(107);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(105);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(14)
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(91);
      END_STATE();
    case 15:
      if (lookahead == '!') ADVANCE(40);
      if (lookahead == '%') ADVANCE(115);
      if (lookahead == '&') ADVANCE(106);
      if (lookahead == '(') ADVANCE(116);
      if (lookahead == ')') ADVANCE(118);
      if (lookahead == '*') ADVANCE(102);
      if (lookahead == '+') ADVANCE(99);
      if (lookahead == ',') ADVANCE(117);
      if (lookahead == '-') ADVANCE(101);
      if (lookahead == '/') ADVANCE(114);
      if (lookahead == ':') ADVANCE(67);
      if (lookahead == ';') ADVANCE(134);
      if (lookahead == '<') ADVANCE(110);
      if (lookahead == '=') ADVANCE(126);
      if (lookahead == '>') ADVANCE(112);
      if (lookahead == '[') ADVANCE(119);
      if (lookahead == ']') ADVANCE(120);
      if (lookahead == '^') ADVANCE(107);
      if (lookahead == 'i') ADVANCE(53);
      if (lookahead == '{') ADVANCE(121);
      if (lookahead == '|') ADVANCE(105);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(15)
      END_STATE();
    case 16:
      if (lookahead == '!') ADVANCE(40);
      if (lookahead == '%') ADVANCE(115);
      if (lookahead == '&') ADVANCE(106);
      if (lookahead == '(') ADVANCE(116);
      if (lookahead == '*') ADVANCE(102);
      if (lookahead == '+') ADVANCE(99);
      if (lookahead == '-') ADVANCE(100);
      if (lookahead == '/') ADVANCE(114);
      if (lookahead == ':') ADVANCE(39);
      if (lookahead == '<') ADVANCE(110);
      if (lookahead == '=') ADVANCE(41);
      if (lookahead == '>') ADVANCE(112);
      if (lookahead == '[') ADVANCE(119);
      if (lookahead == '^') ADVANCE(107);
      if (lookahead == 'i') ADVANCE(53);
      if (lookahead == '|') ADVANCE(105);
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
      if (lookahead == '/') SKIP(9)
      if (lookahead != 0) SKIP(19)
      END_STATE();
    case 19:
      if (lookahead == '*') SKIP(18)
      if (lookahead != 0) SKIP(19)
      END_STATE();
    case 20:
      if (lookahead == '*') SKIP(20)
      if (lookahead == '/') SKIP(10)
      if (lookahead != 0) SKIP(21)
      END_STATE();
    case 21:
      if (lookahead == '*') SKIP(20)
      if (lookahead != 0) SKIP(21)
      END_STATE();
    case 22:
      if (lookahead == '*') SKIP(22)
      if (lookahead == '/') SKIP(11)
      if (lookahead != 0) SKIP(23)
      END_STATE();
    case 23:
      if (lookahead == '*') SKIP(22)
      if (lookahead != 0) SKIP(23)
      END_STATE();
    case 24:
      if (lookahead == '*') SKIP(24)
      if (lookahead == '/') SKIP(12)
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
      if (lookahead == ':') ADVANCE(93);
      END_STATE();
    case 40:
      if (lookahead == '=') ADVANCE(109);
      END_STATE();
    case 41:
      if (lookahead == '=') ADVANCE(108);
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
      if (lookahead == 'e') ADVANCE(95);
      END_STATE();
    case 48:
      if (lookahead == 'e') ADVANCE(70);
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
      if (lookahead == 'n') ADVANCE(97);
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
      if (lookahead == ':') ADVANCE(93);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(anon_sym_module);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(81);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(91);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(90);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(91);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(84);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(78);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(96);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(130);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'f') ADVANCE(128);
      if (lookahead == 'n') ADVANCE(79);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'g') ADVANCE(124);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(89);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(71);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(125);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(87);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(79);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(98);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(86);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(132);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(76);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(72);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(80);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(75);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(92);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(anon_sym_COLON_COLON);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(anon_sym_state);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(anon_sym_state);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(anon_sym_gen);
      END_STATE();
    case 98:
      ACCEPT_TOKEN(anon_sym_gen);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 99:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 101:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '>') ADVANCE(69);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 104:
      ACCEPT_TOKEN(anon_sym_BANG);
      if (lookahead == '=') ADVANCE(109);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(anon_sym_AMP);
      END_STATE();
    case 107:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 108:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 109:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 110:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(111);
      END_STATE();
    case 111:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 112:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(113);
      END_STATE();
    case 113:
      ACCEPT_TOKEN(anon_sym_GT_EQ);
      END_STATE();
    case 114:
      ACCEPT_TOKEN(anon_sym_SLASH);
      END_STATE();
    case 115:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 116:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 117:
      ACCEPT_TOKEN(anon_sym_COMMA);
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
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 125:
      ACCEPT_TOKEN(anon_sym_initial);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 126:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead == '=') ADVANCE(108);
      END_STATE();
    case 127:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 128:
      ACCEPT_TOKEN(anon_sym_if);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 129:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 130:
      ACCEPT_TOKEN(anon_sym_else);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
      END_STATE();
    case 131:
      ACCEPT_TOKEN(anon_sym_for);
      END_STATE();
    case 132:
      ACCEPT_TOKEN(anon_sym_for);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(91);
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
  [2] = {.lex_state = 9},
  [3] = {.lex_state = 9},
  [4] = {.lex_state = 9},
  [5] = {.lex_state = 9},
  [6] = {.lex_state = 9},
  [7] = {.lex_state = 9},
  [8] = {.lex_state = 9},
  [9] = {.lex_state = 10},
  [10] = {.lex_state = 10},
  [11] = {.lex_state = 14},
  [12] = {.lex_state = 14},
  [13] = {.lex_state = 14},
  [14] = {.lex_state = 14},
  [15] = {.lex_state = 14},
  [16] = {.lex_state = 10},
  [17] = {.lex_state = 11},
  [18] = {.lex_state = 14},
  [19] = {.lex_state = 11},
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
  [30] = {.lex_state = 15},
  [31] = {.lex_state = 15},
  [32] = {.lex_state = 15},
  [33] = {.lex_state = 12},
  [34] = {.lex_state = 12},
  [35] = {.lex_state = 15},
  [36] = {.lex_state = 14},
  [37] = {.lex_state = 14},
  [38] = {.lex_state = 14},
  [39] = {.lex_state = 14},
  [40] = {.lex_state = 14},
  [41] = {.lex_state = 14},
  [42] = {.lex_state = 8},
  [43] = {.lex_state = 8},
  [44] = {.lex_state = 13},
  [45] = {.lex_state = 13},
  [46] = {.lex_state = 13},
  [47] = {.lex_state = 8},
  [48] = {.lex_state = 9},
  [49] = {.lex_state = 8},
  [50] = {.lex_state = 16},
  [51] = {.lex_state = 16},
  [52] = {.lex_state = 8},
  [53] = {.lex_state = 8},
  [54] = {.lex_state = 16},
  [55] = {.lex_state = 8},
  [56] = {.lex_state = 8},
  [57] = {.lex_state = 8},
  [58] = {.lex_state = 8},
  [59] = {.lex_state = 8},
  [60] = {.lex_state = 8},
  [61] = {.lex_state = 16},
  [62] = {.lex_state = 8},
  [63] = {.lex_state = 8},
  [64] = {.lex_state = 9},
  [65] = {.lex_state = 8},
  [66] = {.lex_state = 8},
  [67] = {.lex_state = 8},
  [68] = {.lex_state = 8},
  [69] = {.lex_state = 8},
  [70] = {.lex_state = 8},
  [71] = {.lex_state = 14},
  [72] = {.lex_state = 8},
  [73] = {.lex_state = 8},
  [74] = {.lex_state = 9},
  [75] = {.lex_state = 8},
  [76] = {.lex_state = 16},
  [77] = {.lex_state = 9},
  [78] = {.lex_state = 8},
  [79] = {.lex_state = 9},
  [80] = {.lex_state = 8},
  [81] = {.lex_state = 9},
  [82] = {.lex_state = 14},
  [83] = {.lex_state = 14},
  [84] = {.lex_state = 16},
  [85] = {.lex_state = 15},
  [86] = {.lex_state = 14},
  [87] = {.lex_state = 14},
  [88] = {.lex_state = 14},
  [89] = {.lex_state = 15},
  [90] = {.lex_state = 15},
  [91] = {.lex_state = 15},
  [92] = {.lex_state = 15},
  [93] = {.lex_state = 15},
  [94] = {.lex_state = 14},
  [95] = {.lex_state = 15},
  [96] = {.lex_state = 14},
  [97] = {.lex_state = 11},
  [98] = {.lex_state = 12},
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
  [111] = {.lex_state = 8},
  [112] = {.lex_state = 8},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 8},
  [115] = {.lex_state = 8},
  [116] = {.lex_state = 8},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 38},
  [119] = {.lex_state = 0},
  [120] = {.lex_state = 0},
  [121] = {.lex_state = 8},
  [122] = {.lex_state = 8},
  [123] = {.lex_state = 8},
  [124] = {.lex_state = 0},
  [125] = {.lex_state = 0},
  [126] = {.lex_state = 8},
  [127] = {.lex_state = 8},
  [128] = {.lex_state = 0},
  [129] = {.lex_state = 8},
  [130] = {.lex_state = 8},
  [131] = {.lex_state = 0},
  [132] = {.lex_state = 0},
  [133] = {.lex_state = 0},
  [134] = {.lex_state = 0},
  [135] = {.lex_state = 8},
  [136] = {.lex_state = 0},
  [137] = {.lex_state = 0},
  [138] = {.lex_state = 0},
  [139] = {.lex_state = 0},
  [140] = {.lex_state = 0},
  [141] = {.lex_state = 8},
  [142] = {.lex_state = 0},
  [143] = {.lex_state = 8},
  [144] = {.lex_state = 8},
  [145] = {.lex_state = 8},
  [146] = {.lex_state = 8},
  [147] = {.lex_state = 0},
  [148] = {.lex_state = 8},
  [149] = {.lex_state = 0},
  [150] = {.lex_state = 8},
  [151] = {.lex_state = 8},
  [152] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
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
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [anon_sym_for] = ACTIONS(1),
    [anon_sym_in] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(149),
    [sym_module] = STATE(120),
    [aux_sym_source_file_repeat1] = STATE(120),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_module] = ACTIONS(5),
  },
  [2] = {
    [sym_global_identifier] = STATE(37),
    [sym__maybe_global_identifier] = STATE(21),
    [sym_array_type] = STATE(123),
    [sym__type] = STATE(123),
    [sym_declaration] = STATE(103),
    [sym_unary_op] = STATE(36),
    [sym_binary_op] = STATE(36),
    [sym_array_op] = STATE(36),
    [sym_func_call] = STATE(36),
    [sym_parenthesis_expression] = STATE(36),
    [sym__expression] = STATE(36),
    [sym_block] = STATE(81),
    [sym_assign_left_side] = STATE(134),
    [sym_decl_assign_statement] = STATE(152),
    [sym_if_statement] = STATE(81),
    [sym_for_statement] = STATE(81),
    [sym__statement] = STATE(81),
    [aux_sym_block_repeat1] = STATE(6),
    [aux_sym_assign_left_side_repeat1] = STATE(17),
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
    [sym_global_identifier] = STATE(37),
    [sym__maybe_global_identifier] = STATE(21),
    [sym_array_type] = STATE(123),
    [sym__type] = STATE(123),
    [sym_declaration] = STATE(103),
    [sym_unary_op] = STATE(36),
    [sym_binary_op] = STATE(36),
    [sym_array_op] = STATE(36),
    [sym_func_call] = STATE(36),
    [sym_parenthesis_expression] = STATE(36),
    [sym__expression] = STATE(36),
    [sym_block] = STATE(81),
    [sym_assign_left_side] = STATE(134),
    [sym_decl_assign_statement] = STATE(152),
    [sym_if_statement] = STATE(81),
    [sym_for_statement] = STATE(81),
    [sym__statement] = STATE(81),
    [aux_sym_block_repeat1] = STATE(2),
    [aux_sym_assign_left_side_repeat1] = STATE(17),
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
    [sym_global_identifier] = STATE(37),
    [sym__maybe_global_identifier] = STATE(21),
    [sym_array_type] = STATE(123),
    [sym__type] = STATE(123),
    [sym_declaration] = STATE(103),
    [sym_unary_op] = STATE(36),
    [sym_binary_op] = STATE(36),
    [sym_array_op] = STATE(36),
    [sym_func_call] = STATE(36),
    [sym_parenthesis_expression] = STATE(36),
    [sym__expression] = STATE(36),
    [sym_block] = STATE(81),
    [sym_assign_left_side] = STATE(134),
    [sym_decl_assign_statement] = STATE(152),
    [sym_if_statement] = STATE(81),
    [sym_for_statement] = STATE(81),
    [sym__statement] = STATE(81),
    [aux_sym_block_repeat1] = STATE(6),
    [aux_sym_assign_left_side_repeat1] = STATE(17),
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
    [sym_global_identifier] = STATE(37),
    [sym__maybe_global_identifier] = STATE(21),
    [sym_array_type] = STATE(123),
    [sym__type] = STATE(123),
    [sym_declaration] = STATE(103),
    [sym_unary_op] = STATE(36),
    [sym_binary_op] = STATE(36),
    [sym_array_op] = STATE(36),
    [sym_func_call] = STATE(36),
    [sym_parenthesis_expression] = STATE(36),
    [sym__expression] = STATE(36),
    [sym_block] = STATE(81),
    [sym_assign_left_side] = STATE(134),
    [sym_decl_assign_statement] = STATE(152),
    [sym_if_statement] = STATE(81),
    [sym_for_statement] = STATE(81),
    [sym__statement] = STATE(81),
    [aux_sym_block_repeat1] = STATE(4),
    [aux_sym_assign_left_side_repeat1] = STATE(17),
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
    [sym_global_identifier] = STATE(37),
    [sym__maybe_global_identifier] = STATE(21),
    [sym_array_type] = STATE(123),
    [sym__type] = STATE(123),
    [sym_declaration] = STATE(103),
    [sym_unary_op] = STATE(36),
    [sym_binary_op] = STATE(36),
    [sym_array_op] = STATE(36),
    [sym_func_call] = STATE(36),
    [sym_parenthesis_expression] = STATE(36),
    [sym__expression] = STATE(36),
    [sym_block] = STATE(81),
    [sym_assign_left_side] = STATE(134),
    [sym_decl_assign_statement] = STATE(152),
    [sym_if_statement] = STATE(81),
    [sym_for_statement] = STATE(81),
    [sym__statement] = STATE(81),
    [aux_sym_block_repeat1] = STATE(6),
    [aux_sym_assign_left_side_repeat1] = STATE(17),
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
    [sym_global_identifier] = STATE(37),
    [sym__maybe_global_identifier] = STATE(21),
    [sym_array_type] = STATE(123),
    [sym__type] = STATE(123),
    [sym_declaration] = STATE(103),
    [sym_unary_op] = STATE(36),
    [sym_binary_op] = STATE(36),
    [sym_array_op] = STATE(36),
    [sym_func_call] = STATE(36),
    [sym_parenthesis_expression] = STATE(36),
    [sym__expression] = STATE(36),
    [sym_block] = STATE(81),
    [sym_assign_left_side] = STATE(134),
    [sym_decl_assign_statement] = STATE(152),
    [sym_if_statement] = STATE(81),
    [sym_for_statement] = STATE(81),
    [sym__statement] = STATE(81),
    [aux_sym_block_repeat1] = STATE(8),
    [aux_sym_assign_left_side_repeat1] = STATE(17),
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
    [sym_global_identifier] = STATE(37),
    [sym__maybe_global_identifier] = STATE(21),
    [sym_array_type] = STATE(123),
    [sym__type] = STATE(123),
    [sym_declaration] = STATE(103),
    [sym_unary_op] = STATE(36),
    [sym_binary_op] = STATE(36),
    [sym_array_op] = STATE(36),
    [sym_func_call] = STATE(36),
    [sym_parenthesis_expression] = STATE(36),
    [sym__expression] = STATE(36),
    [sym_block] = STATE(81),
    [sym_assign_left_side] = STATE(134),
    [sym_decl_assign_statement] = STATE(152),
    [sym_if_statement] = STATE(81),
    [sym_for_statement] = STATE(81),
    [sym__statement] = STATE(81),
    [aux_sym_block_repeat1] = STATE(6),
    [aux_sym_assign_left_side_repeat1] = STATE(17),
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
    ACTIONS(9), 1,
      sym_number,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_reg,
    ACTIONS(25), 1,
      anon_sym_initial,
    STATE(17), 1,
      aux_sym_assign_left_side_repeat1,
    STATE(21), 1,
      sym__maybe_global_identifier,
    STATE(37), 1,
      sym_global_identifier,
    STATE(103), 1,
      sym_declaration,
    STATE(136), 1,
      sym_assign_left_side,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(123), 2,
      sym_array_type,
      sym__type,
    STATE(36), 6,
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
  [59] = 15,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(9), 1,
      sym_number,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_reg,
    ACTIONS(25), 1,
      anon_sym_initial,
    STATE(17), 1,
      aux_sym_assign_left_side_repeat1,
    STATE(21), 1,
      sym__maybe_global_identifier,
    STATE(37), 1,
      sym_global_identifier,
    STATE(103), 1,
      sym_declaration,
    STATE(142), 1,
      sym_assign_left_side,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(123), 2,
      sym_array_type,
      sym__type,
    STATE(36), 6,
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
  [118] = 4,
    ACTIONS(78), 1,
      anon_sym_COLON_COLON,
    ACTIONS(80), 1,
      anon_sym_SLASH,
    STATE(13), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(76), 24,
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
      anon_sym_SEMI,
  [154] = 4,
    ACTIONS(78), 1,
      anon_sym_COLON_COLON,
    ACTIONS(84), 1,
      anon_sym_SLASH,
    STATE(13), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(82), 24,
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
      anon_sym_SEMI,
  [190] = 4,
    ACTIONS(88), 1,
      anon_sym_COLON_COLON,
    ACTIONS(91), 1,
      anon_sym_SLASH,
    STATE(13), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(86), 24,
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
      anon_sym_SEMI,
  [226] = 4,
    ACTIONS(78), 1,
      anon_sym_COLON_COLON,
    ACTIONS(80), 1,
      anon_sym_SLASH,
    STATE(12), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(76), 24,
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
      anon_sym_SEMI,
  [262] = 4,
    ACTIONS(78), 1,
      anon_sym_COLON_COLON,
    ACTIONS(95), 1,
      anon_sym_SLASH,
    STATE(11), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(93), 24,
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
      anon_sym_SEMI,
  [298] = 14,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(97), 1,
      sym_number,
    ACTIONS(99), 1,
      anon_sym_reg,
    ACTIONS(101), 1,
      anon_sym_initial,
    STATE(19), 1,
      aux_sym_assign_left_side_repeat1,
    STATE(21), 1,
      sym__maybe_global_identifier,
    STATE(37), 1,
      sym_global_identifier,
    STATE(110), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(123), 2,
      sym_array_type,
      sym__type,
    STATE(39), 6,
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
  [354] = 13,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(103), 1,
      sym_number,
    ACTIONS(105), 1,
      anon_sym_reg,
    STATE(21), 1,
      sym__maybe_global_identifier,
    STATE(37), 1,
      sym_global_identifier,
    STATE(97), 1,
      aux_sym_assign_left_side_repeat1,
    STATE(104), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(123), 2,
      sym_array_type,
      sym__type,
    STATE(38), 6,
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
  [407] = 2,
    ACTIONS(91), 1,
      anon_sym_SLASH,
    ACTIONS(86), 25,
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
      anon_sym_SEMI,
  [438] = 13,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(105), 1,
      anon_sym_reg,
    ACTIONS(107), 1,
      sym_number,
    STATE(21), 1,
      sym__maybe_global_identifier,
    STATE(37), 1,
      sym_global_identifier,
    STATE(97), 1,
      aux_sym_assign_left_side_repeat1,
    STATE(113), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(123), 2,
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
  [491] = 3,
    ACTIONS(111), 1,
      anon_sym_SLASH,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(109), 23,
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
  [523] = 3,
    ACTIONS(115), 1,
      anon_sym_SLASH,
    ACTIONS(117), 1,
      anon_sym_LPAREN,
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
  [555] = 3,
    ACTIONS(121), 1,
      anon_sym_SLASH,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(119), 23,
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
  [587] = 7,
    ACTIONS(127), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(119), 16,
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
      anon_sym_SEMI,
  [626] = 8,
    ACTIONS(127), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(133), 1,
      anon_sym_AMP,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(119), 15,
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
      anon_sym_SEMI,
  [667] = 5,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(119), 18,
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
      anon_sym_SEMI,
  [702] = 2,
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
  [731] = 6,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(119), 17,
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
      anon_sym_SEMI,
  [768] = 2,
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
  [797] = 4,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(119), 20,
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
      anon_sym_SEMI,
  [830] = 2,
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
  [859] = 2,
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
  [888] = 2,
    ACTIONS(153), 1,
      anon_sym_SLASH,
    ACTIONS(151), 23,
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
  [917] = 11,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(107), 1,
      sym_number,
    STATE(21), 1,
      sym__maybe_global_identifier,
    STATE(37), 1,
      sym_global_identifier,
    STATE(113), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(123), 2,
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
  [964] = 11,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(103), 1,
      sym_number,
    STATE(21), 1,
      sym__maybe_global_identifier,
    STATE(37), 1,
      sym_global_identifier,
    STATE(104), 1,
      sym_declaration,
    ACTIONS(13), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(123), 2,
      sym_array_type,
      sym__type,
    STATE(38), 6,
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
  [1011] = 2,
    ACTIONS(157), 1,
      anon_sym_SLASH,
    ACTIONS(155), 23,
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
  [1040] = 12,
    ACTIONS(127), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(133), 1,
      anon_sym_AMP,
    ACTIONS(163), 1,
      anon_sym_COMMA,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    STATE(26), 1,
      sym_array_bracket_expression,
    STATE(109), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(159), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
    ACTIONS(161), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1087] = 4,
    ACTIONS(169), 1,
      sym_identifier,
    ACTIONS(171), 1,
      anon_sym_SLASH,
    ACTIONS(173), 1,
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
  [1118] = 12,
    ACTIONS(127), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(133), 1,
      anon_sym_AMP,
    ACTIONS(163), 1,
      anon_sym_COMMA,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    STATE(26), 1,
      sym_array_bracket_expression,
    STATE(106), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(176), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
    ACTIONS(161), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1165] = 10,
    ACTIONS(127), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(133), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(178), 5,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
    ACTIONS(161), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1207] = 10,
    ACTIONS(127), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(133), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(180), 5,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
    ACTIONS(161), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1249] = 10,
    ACTIONS(127), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(133), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(182), 5,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
    ACTIONS(161), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [1291] = 8,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(186), 1,
      sym_number,
    STATE(140), 1,
      sym_range,
    STATE(21), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(95), 6,
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
  [1328] = 8,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(188), 1,
      sym_number,
    ACTIONS(190), 1,
      anon_sym_RPAREN,
    STATE(21), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(71), 6,
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
  [1365] = 2,
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
  [1390] = 3,
    ACTIONS(200), 1,
      anon_sym_else,
    ACTIONS(196), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(198), 12,
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
  [1417] = 2,
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
  [1442] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      sym_identifier,
    ACTIONS(208), 1,
      sym_number,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    STATE(21), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(22), 6,
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
  [1476] = 2,
    ACTIONS(214), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(216), 12,
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
  [1500] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(218), 1,
      sym_number,
    STATE(21), 2,
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
  [1534] = 4,
    ACTIONS(84), 1,
      anon_sym_SLASH,
    ACTIONS(220), 1,
      anon_sym_COLON_COLON,
    STATE(51), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(82), 16,
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
  [1562] = 4,
    ACTIONS(91), 1,
      anon_sym_SLASH,
    ACTIONS(222), 1,
      anon_sym_COLON_COLON,
    STATE(51), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(86), 16,
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
  [1590] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(225), 1,
      sym_number,
    STATE(21), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(94), 6,
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
  [1624] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(227), 1,
      sym_number,
    STATE(21), 2,
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
  [1658] = 4,
    ACTIONS(80), 1,
      anon_sym_SLASH,
    ACTIONS(220), 1,
      anon_sym_COLON_COLON,
    STATE(50), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(76), 16,
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
  [1686] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(229), 1,
      sym_number,
    STATE(21), 2,
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
  [1720] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(208), 1,
      sym_number,
    STATE(21), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(22), 6,
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
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(231), 1,
      sym_number,
    STATE(21), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(27), 6,
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
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(233), 1,
      sym_number,
    STATE(21), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(23), 6,
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
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(235), 1,
      sym_number,
    STATE(21), 2,
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
  [1856] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(237), 1,
      sym_number,
    STATE(21), 2,
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
  [1890] = 4,
    ACTIONS(80), 1,
      anon_sym_SLASH,
    ACTIONS(220), 1,
      anon_sym_COLON_COLON,
    STATE(51), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(76), 16,
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
  [1918] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      sym_identifier,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    ACTIONS(239), 1,
      sym_number,
    STATE(21), 2,
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
  [1952] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(241), 1,
      sym_number,
    STATE(21), 2,
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
  [1986] = 2,
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
  [2010] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(243), 1,
      sym_number,
    STATE(21), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(88), 6,
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
  [2044] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(245), 1,
      sym_number,
    STATE(21), 2,
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
  [2078] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      sym_identifier,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    ACTIONS(247), 1,
      sym_number,
    STATE(21), 2,
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
  [2112] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      sym_identifier,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    ACTIONS(249), 1,
      sym_number,
    STATE(21), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(91), 6,
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
  [2146] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      sym_identifier,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    ACTIONS(251), 1,
      sym_number,
    STATE(21), 2,
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
  [2180] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      sym_identifier,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    ACTIONS(253), 1,
      sym_number,
    STATE(21), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(92), 6,
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
  [2214] = 12,
    ACTIONS(127), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(133), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(255), 1,
      anon_sym_COMMA,
    ACTIONS(257), 1,
      anon_sym_RPAREN,
    STATE(26), 1,
      sym_array_bracket_expression,
    STATE(128), 1,
      aux_sym_func_call_repeat1,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(161), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2258] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(259), 1,
      sym_number,
    STATE(21), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(40), 6,
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
  [2292] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      sym_identifier,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    ACTIONS(261), 1,
      sym_number,
    STATE(21), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(93), 6,
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
  [2326] = 2,
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
  [2350] = 7,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      sym_identifier,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    ACTIONS(263), 1,
      sym_number,
    STATE(21), 2,
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
  [2384] = 4,
    ACTIONS(95), 1,
      anon_sym_SLASH,
    ACTIONS(220), 1,
      anon_sym_COLON_COLON,
    STATE(61), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(93), 16,
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
  [2412] = 2,
    ACTIONS(265), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(267), 12,
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
  [2436] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(269), 1,
      sym_number,
    STATE(21), 2,
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
  [2470] = 2,
    ACTIONS(271), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(273), 12,
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
  [2494] = 7,
    ACTIONS(11), 1,
      anon_sym_COLON_COLON,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(184), 1,
      sym_identifier,
    ACTIONS(263), 1,
      sym_number,
    STATE(21), 2,
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
  [2528] = 2,
    ACTIONS(275), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(277), 12,
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
  [2552] = 10,
    ACTIONS(127), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(133), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(279), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
    ACTIONS(161), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2591] = 11,
    ACTIONS(127), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(133), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(281), 1,
      anon_sym_LBRACE,
    STATE(26), 1,
      sym_array_bracket_expression,
    STATE(45), 1,
      sym_block,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(161), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2632] = 2,
    ACTIONS(91), 1,
      anon_sym_SLASH,
    ACTIONS(86), 17,
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
  [2655] = 7,
    ACTIONS(287), 1,
      anon_sym_PIPE,
    ACTIONS(289), 1,
      anon_sym_CARET,
    ACTIONS(291), 1,
      anon_sym_SLASH,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(283), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(285), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(119), 9,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_LBRACK,
      anon_sym_in,
  [2687] = 10,
    ACTIONS(127), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(133), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(293), 1,
      anon_sym_RPAREN,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(161), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2725] = 10,
    ACTIONS(127), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(133), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(295), 1,
      anon_sym_RBRACK,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(161), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2763] = 10,
    ACTIONS(127), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(133), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(297), 1,
      anon_sym_SEMI,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(161), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2801] = 10,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(180), 1,
      anon_sym_in,
    ACTIONS(287), 1,
      anon_sym_PIPE,
    ACTIONS(289), 1,
      anon_sym_CARET,
    ACTIONS(291), 1,
      anon_sym_SLASH,
    ACTIONS(299), 1,
      anon_sym_AMP,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(283), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(285), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(301), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2839] = 8,
    ACTIONS(287), 1,
      anon_sym_PIPE,
    ACTIONS(289), 1,
      anon_sym_CARET,
    ACTIONS(291), 1,
      anon_sym_SLASH,
    ACTIONS(299), 1,
      anon_sym_AMP,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(283), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(285), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(119), 8,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
      anon_sym_LBRACK,
      anon_sym_in,
  [2873] = 5,
    ACTIONS(291), 1,
      anon_sym_SLASH,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(283), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(285), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(119), 11,
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
  [2901] = 6,
    ACTIONS(289), 1,
      anon_sym_CARET,
    ACTIONS(291), 1,
      anon_sym_SLASH,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(283), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(285), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(119), 10,
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
  [2931] = 4,
    ACTIONS(291), 1,
      anon_sym_SLASH,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(285), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(119), 13,
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
  [2957] = 10,
    ACTIONS(127), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(133), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(303), 1,
      anon_sym_RBRACK,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(161), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [2995] = 10,
    ACTIONS(127), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(133), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(305), 1,
      anon_sym_COLON,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(161), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [3033] = 10,
    ACTIONS(127), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_CARET,
    ACTIONS(131), 1,
      anon_sym_SLASH,
    ACTIONS(133), 1,
      anon_sym_AMP,
    ACTIONS(165), 1,
      anon_sym_LBRACK,
    ACTIONS(307), 1,
      anon_sym_LBRACE,
    STATE(26), 1,
      sym_array_bracket_expression,
    ACTIONS(123), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(125), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(161), 6,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT,
      anon_sym_LT_EQ,
      anon_sym_GT,
      anon_sym_GT_EQ,
  [3071] = 4,
    ACTIONS(313), 1,
      anon_sym_reg,
    STATE(97), 1,
      aux_sym_assign_left_side_repeat1,
    ACTIONS(309), 3,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
    ACTIONS(311), 10,
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
  [3095] = 5,
    ACTIONS(316), 1,
      sym_identifier,
    ACTIONS(318), 1,
      anon_sym_COLON_COLON,
    STATE(147), 1,
      sym_declaration,
    ACTIONS(320), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(129), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3114] = 3,
    ACTIONS(324), 1,
      anon_sym_SQUOTE,
    STATE(108), 1,
      sym_latency_specifier,
    ACTIONS(322), 5,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3128] = 3,
    ACTIONS(324), 1,
      anon_sym_SQUOTE,
    STATE(105), 1,
      sym_latency_specifier,
    ACTIONS(326), 5,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3142] = 3,
    ACTIONS(330), 1,
      anon_sym_COMMA,
    STATE(107), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(328), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3155] = 3,
    ACTIONS(330), 1,
      anon_sym_COMMA,
    STATE(107), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(332), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3168] = 3,
    ACTIONS(330), 1,
      anon_sym_COMMA,
    STATE(101), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(334), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3181] = 3,
    ACTIONS(330), 1,
      anon_sym_COMMA,
    STATE(102), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(336), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3194] = 1,
    ACTIONS(338), 6,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [3203] = 3,
    ACTIONS(330), 1,
      anon_sym_COMMA,
    STATE(107), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(332), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3216] = 3,
    ACTIONS(342), 1,
      anon_sym_COMMA,
    STATE(107), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(340), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3229] = 1,
    ACTIONS(345), 6,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_in,
      anon_sym_SEMI,
  [3238] = 3,
    ACTIONS(330), 1,
      anon_sym_COMMA,
    STATE(107), 1,
      aux_sym_assign_left_side_repeat2,
    ACTIONS(328), 4,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3251] = 1,
    ACTIONS(347), 5,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3259] = 3,
    ACTIONS(318), 1,
      anon_sym_COLON_COLON,
    ACTIONS(349), 1,
      sym_identifier,
    STATE(127), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3271] = 3,
    ACTIONS(318), 1,
      anon_sym_COLON_COLON,
    ACTIONS(349), 1,
      sym_identifier,
    STATE(126), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3283] = 1,
    ACTIONS(351), 5,
      anon_sym_DASH_GT,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3291] = 3,
    ACTIONS(353), 1,
      anon_sym_COLON_COLON,
    STATE(115), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(95), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3302] = 3,
    ACTIONS(353), 1,
      anon_sym_COLON_COLON,
    STATE(121), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(80), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3313] = 3,
    ACTIONS(353), 1,
      anon_sym_COLON_COLON,
    STATE(122), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(80), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3324] = 3,
    ACTIONS(355), 1,
      ts_builtin_sym_end,
    ACTIONS(357), 1,
      anon_sym_module,
    STATE(117), 2,
      sym_module,
      aux_sym_source_file_repeat1,
  [3335] = 4,
    ACTIONS(360), 1,
      anon_sym_COLON,
    ACTIONS(362), 1,
      anon_sym_LBRACE,
    STATE(133), 1,
      sym_interface_ports,
    STATE(139), 1,
      sym_block,
  [3348] = 3,
    ACTIONS(19), 1,
      anon_sym_LBRACE,
    ACTIONS(364), 1,
      anon_sym_if,
    STATE(79), 2,
      sym_block,
      sym_if_statement,
  [3359] = 3,
    ACTIONS(5), 1,
      anon_sym_module,
    ACTIONS(366), 1,
      ts_builtin_sym_end,
    STATE(117), 2,
      sym_module,
      aux_sym_source_file_repeat1,
  [3370] = 3,
    ACTIONS(368), 1,
      anon_sym_COLON_COLON,
    STATE(121), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(91), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3381] = 3,
    ACTIONS(353), 1,
      anon_sym_COLON_COLON,
    STATE(121), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(84), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3392] = 3,
    ACTIONS(371), 1,
      sym_identifier,
    ACTIONS(373), 1,
      anon_sym_LBRACK,
    STATE(135), 1,
      sym_array_bracket_expression,
  [3402] = 3,
    ACTIONS(322), 1,
      anon_sym_in,
    ACTIONS(375), 1,
      anon_sym_SQUOTE,
    STATE(108), 1,
      sym_latency_specifier,
  [3412] = 3,
    ACTIONS(377), 1,
      anon_sym_COMMA,
    ACTIONS(380), 1,
      anon_sym_RPAREN,
    STATE(125), 1,
      aux_sym_func_call_repeat1,
  [3422] = 3,
    ACTIONS(373), 1,
      anon_sym_LBRACK,
    ACTIONS(382), 1,
      sym_identifier,
    STATE(135), 1,
      sym_array_bracket_expression,
  [3432] = 3,
    ACTIONS(373), 1,
      anon_sym_LBRACK,
    ACTIONS(384), 1,
      sym_identifier,
    STATE(135), 1,
      sym_array_bracket_expression,
  [3442] = 3,
    ACTIONS(386), 1,
      anon_sym_COMMA,
    ACTIONS(388), 1,
      anon_sym_RPAREN,
    STATE(125), 1,
      aux_sym_func_call_repeat1,
  [3452] = 3,
    ACTIONS(373), 1,
      anon_sym_LBRACK,
    ACTIONS(390), 1,
      sym_identifier,
    STATE(135), 1,
      sym_array_bracket_expression,
  [3462] = 1,
    ACTIONS(91), 3,
      sym_identifier,
      anon_sym_COLON_COLON,
      anon_sym_LBRACK,
  [3468] = 3,
    ACTIONS(326), 1,
      anon_sym_in,
    ACTIONS(375), 1,
      anon_sym_SQUOTE,
    STATE(105), 1,
      sym_latency_specifier,
  [3478] = 1,
    ACTIONS(392), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3483] = 2,
    ACTIONS(362), 1,
      anon_sym_LBRACE,
    STATE(132), 1,
      sym_block,
  [3490] = 2,
    ACTIONS(394), 1,
      anon_sym_EQ,
    ACTIONS(396), 1,
      anon_sym_SEMI,
  [3497] = 1,
    ACTIONS(398), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3502] = 2,
    ACTIONS(400), 1,
      anon_sym_DASH_GT,
    ACTIONS(402), 1,
      anon_sym_LBRACE,
  [3509] = 1,
    ACTIONS(204), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3514] = 1,
    ACTIONS(194), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3519] = 1,
    ACTIONS(404), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3524] = 2,
    ACTIONS(19), 1,
      anon_sym_LBRACE,
    STATE(77), 1,
      sym_block,
  [3531] = 1,
    ACTIONS(149), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3536] = 1,
    ACTIONS(406), 1,
      anon_sym_LBRACE,
  [3540] = 1,
    ACTIONS(408), 1,
      sym_identifier,
  [3544] = 1,
    ACTIONS(410), 1,
      sym_identifier,
  [3548] = 1,
    ACTIONS(412), 1,
      sym_identifier,
  [3552] = 1,
    ACTIONS(414), 1,
      sym_identifier,
  [3556] = 1,
    ACTIONS(416), 1,
      anon_sym_in,
  [3560] = 1,
    ACTIONS(418), 1,
      sym_identifier,
  [3564] = 1,
    ACTIONS(420), 1,
      ts_builtin_sym_end,
  [3568] = 1,
    ACTIONS(422), 1,
      sym_identifier,
  [3572] = 1,
    ACTIONS(424), 1,
      sym_identifier,
  [3576] = 1,
    ACTIONS(396), 1,
      anon_sym_SEMI,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(9)] = 0,
  [SMALL_STATE(10)] = 59,
  [SMALL_STATE(11)] = 118,
  [SMALL_STATE(12)] = 154,
  [SMALL_STATE(13)] = 190,
  [SMALL_STATE(14)] = 226,
  [SMALL_STATE(15)] = 262,
  [SMALL_STATE(16)] = 298,
  [SMALL_STATE(17)] = 354,
  [SMALL_STATE(18)] = 407,
  [SMALL_STATE(19)] = 438,
  [SMALL_STATE(20)] = 491,
  [SMALL_STATE(21)] = 523,
  [SMALL_STATE(22)] = 555,
  [SMALL_STATE(23)] = 587,
  [SMALL_STATE(24)] = 626,
  [SMALL_STATE(25)] = 667,
  [SMALL_STATE(26)] = 702,
  [SMALL_STATE(27)] = 731,
  [SMALL_STATE(28)] = 768,
  [SMALL_STATE(29)] = 797,
  [SMALL_STATE(30)] = 830,
  [SMALL_STATE(31)] = 859,
  [SMALL_STATE(32)] = 888,
  [SMALL_STATE(33)] = 917,
  [SMALL_STATE(34)] = 964,
  [SMALL_STATE(35)] = 1011,
  [SMALL_STATE(36)] = 1040,
  [SMALL_STATE(37)] = 1087,
  [SMALL_STATE(38)] = 1118,
  [SMALL_STATE(39)] = 1165,
  [SMALL_STATE(40)] = 1207,
  [SMALL_STATE(41)] = 1249,
  [SMALL_STATE(42)] = 1291,
  [SMALL_STATE(43)] = 1328,
  [SMALL_STATE(44)] = 1365,
  [SMALL_STATE(45)] = 1390,
  [SMALL_STATE(46)] = 1417,
  [SMALL_STATE(47)] = 1442,
  [SMALL_STATE(48)] = 1476,
  [SMALL_STATE(49)] = 1500,
  [SMALL_STATE(50)] = 1534,
  [SMALL_STATE(51)] = 1562,
  [SMALL_STATE(52)] = 1590,
  [SMALL_STATE(53)] = 1624,
  [SMALL_STATE(54)] = 1658,
  [SMALL_STATE(55)] = 1686,
  [SMALL_STATE(56)] = 1720,
  [SMALL_STATE(57)] = 1754,
  [SMALL_STATE(58)] = 1788,
  [SMALL_STATE(59)] = 1822,
  [SMALL_STATE(60)] = 1856,
  [SMALL_STATE(61)] = 1890,
  [SMALL_STATE(62)] = 1918,
  [SMALL_STATE(63)] = 1952,
  [SMALL_STATE(64)] = 1986,
  [SMALL_STATE(65)] = 2010,
  [SMALL_STATE(66)] = 2044,
  [SMALL_STATE(67)] = 2078,
  [SMALL_STATE(68)] = 2112,
  [SMALL_STATE(69)] = 2146,
  [SMALL_STATE(70)] = 2180,
  [SMALL_STATE(71)] = 2214,
  [SMALL_STATE(72)] = 2258,
  [SMALL_STATE(73)] = 2292,
  [SMALL_STATE(74)] = 2326,
  [SMALL_STATE(75)] = 2350,
  [SMALL_STATE(76)] = 2384,
  [SMALL_STATE(77)] = 2412,
  [SMALL_STATE(78)] = 2436,
  [SMALL_STATE(79)] = 2470,
  [SMALL_STATE(80)] = 2494,
  [SMALL_STATE(81)] = 2528,
  [SMALL_STATE(82)] = 2552,
  [SMALL_STATE(83)] = 2591,
  [SMALL_STATE(84)] = 2632,
  [SMALL_STATE(85)] = 2655,
  [SMALL_STATE(86)] = 2687,
  [SMALL_STATE(87)] = 2725,
  [SMALL_STATE(88)] = 2763,
  [SMALL_STATE(89)] = 2801,
  [SMALL_STATE(90)] = 2839,
  [SMALL_STATE(91)] = 2873,
  [SMALL_STATE(92)] = 2901,
  [SMALL_STATE(93)] = 2931,
  [SMALL_STATE(94)] = 2957,
  [SMALL_STATE(95)] = 2995,
  [SMALL_STATE(96)] = 3033,
  [SMALL_STATE(97)] = 3071,
  [SMALL_STATE(98)] = 3095,
  [SMALL_STATE(99)] = 3114,
  [SMALL_STATE(100)] = 3128,
  [SMALL_STATE(101)] = 3142,
  [SMALL_STATE(102)] = 3155,
  [SMALL_STATE(103)] = 3168,
  [SMALL_STATE(104)] = 3181,
  [SMALL_STATE(105)] = 3194,
  [SMALL_STATE(106)] = 3203,
  [SMALL_STATE(107)] = 3216,
  [SMALL_STATE(108)] = 3229,
  [SMALL_STATE(109)] = 3238,
  [SMALL_STATE(110)] = 3251,
  [SMALL_STATE(111)] = 3259,
  [SMALL_STATE(112)] = 3271,
  [SMALL_STATE(113)] = 3283,
  [SMALL_STATE(114)] = 3291,
  [SMALL_STATE(115)] = 3302,
  [SMALL_STATE(116)] = 3313,
  [SMALL_STATE(117)] = 3324,
  [SMALL_STATE(118)] = 3335,
  [SMALL_STATE(119)] = 3348,
  [SMALL_STATE(120)] = 3359,
  [SMALL_STATE(121)] = 3370,
  [SMALL_STATE(122)] = 3381,
  [SMALL_STATE(123)] = 3392,
  [SMALL_STATE(124)] = 3402,
  [SMALL_STATE(125)] = 3412,
  [SMALL_STATE(126)] = 3422,
  [SMALL_STATE(127)] = 3432,
  [SMALL_STATE(128)] = 3442,
  [SMALL_STATE(129)] = 3452,
  [SMALL_STATE(130)] = 3462,
  [SMALL_STATE(131)] = 3468,
  [SMALL_STATE(132)] = 3478,
  [SMALL_STATE(133)] = 3483,
  [SMALL_STATE(134)] = 3490,
  [SMALL_STATE(135)] = 3497,
  [SMALL_STATE(136)] = 3502,
  [SMALL_STATE(137)] = 3509,
  [SMALL_STATE(138)] = 3514,
  [SMALL_STATE(139)] = 3519,
  [SMALL_STATE(140)] = 3524,
  [SMALL_STATE(141)] = 3531,
  [SMALL_STATE(142)] = 3536,
  [SMALL_STATE(143)] = 3540,
  [SMALL_STATE(144)] = 3544,
  [SMALL_STATE(145)] = 3548,
  [SMALL_STATE(146)] = 3552,
  [SMALL_STATE(147)] = 3556,
  [SMALL_STATE(148)] = 3560,
  [SMALL_STATE(149)] = 3564,
  [SMALL_STATE(150)] = 3568,
  [SMALL_STATE(151)] = 3572,
  [SMALL_STATE(152)] = 3576,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(151),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(111),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(34),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(66),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(98),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [37] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(15),
  [40] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(36),
  [43] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(151),
  [46] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(111),
  [49] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(80),
  [52] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(63),
  [55] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(5),
  [58] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12),
  [60] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(17),
  [63] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(34),
  [66] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(66),
  [69] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 12), SHIFT_REPEAT(98),
  [72] = {.entry = {.count = 1, .reusable = true}}, SHIFT(138),
  [74] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [76] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 2),
  [78] = {.entry = {.count = 1, .reusable = false}}, SHIFT(145),
  [80] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 2),
  [82] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 3),
  [84] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 3),
  [86] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_global_identifier_repeat1, 2),
  [88] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_identifier_repeat1, 2), SHIFT_REPEAT(145),
  [91] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_global_identifier_repeat1, 2),
  [93] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 1),
  [95] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 1),
  [97] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [99] = {.entry = {.count = 1, .reusable = false}}, SHIFT(19),
  [101] = {.entry = {.count = 1, .reusable = false}}, SHIFT(33),
  [103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [105] = {.entry = {.count = 1, .reusable = false}}, SHIFT(97),
  [107] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [109] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_op, 2, .production_id = 6),
  [111] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_op, 2, .production_id = 6),
  [113] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expression, 1),
  [115] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expression, 1),
  [117] = {.entry = {.count = 1, .reusable = false}}, SHIFT(43),
  [119] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_op, 3, .production_id = 20),
  [121] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_op, 3, .production_id = 20),
  [123] = {.entry = {.count = 1, .reusable = false}}, SHIFT(55),
  [125] = {.entry = {.count = 1, .reusable = false}}, SHIFT(56),
  [127] = {.entry = {.count = 1, .reusable = false}}, SHIFT(57),
  [129] = {.entry = {.count = 1, .reusable = false}}, SHIFT(59),
  [131] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [133] = {.entry = {.count = 1, .reusable = false}}, SHIFT(58),
  [135] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_op, 2, .production_id = 9),
  [137] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_op, 2, .production_id = 9),
  [139] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 5, .production_id = 28),
  [141] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 5, .production_id = 28),
  [143] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 14),
  [145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesis_expression, 3, .production_id = 14),
  [147] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 14),
  [149] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_bracket_expression, 3, .production_id = 14),
  [151] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 4, .production_id = 25),
  [153] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 4, .production_id = 25),
  [155] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 3, .production_id = 16),
  [157] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 3, .production_id = 16),
  [159] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_left_side, 1, .production_id = 2),
  [161] = {.entry = {.count = 1, .reusable = false}}, SHIFT(60),
  [163] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [165] = {.entry = {.count = 1, .reusable = false}}, SHIFT(49),
  [167] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__maybe_global_identifier, 1),
  [169] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__type, 1),
  [171] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__maybe_global_identifier, 1),
  [173] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym__maybe_global_identifier, 1), REDUCE(sym__type, 1),
  [176] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_left_side, 2, .production_id = 7),
  [178] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_assign_left_side_repeat2, 2, .production_id = 18),
  [180] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_latency_specifier, 2, .production_id = 14),
  [182] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_assign_left_side_repeat2, 3, .production_id = 26),
  [184] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [186] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [188] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [190] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [192] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 2),
  [194] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [196] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if_statement, 3, .production_id = 22),
  [198] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 3, .production_id = 22),
  [200] = {.entry = {.count = 1, .reusable = false}}, SHIFT(119),
  [202] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 3, .production_id = 11),
  [204] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, .production_id = 11),
  [206] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [208] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [210] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [212] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [214] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__statement, 2),
  [216] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__statement, 2),
  [218] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [220] = {.entry = {.count = 1, .reusable = false}}, SHIFT(150),
  [222] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_global_identifier_repeat1, 2), SHIFT_REPEAT(150),
  [225] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [227] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [229] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [231] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [233] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [235] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [237] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [239] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [241] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [243] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [245] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [247] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [249] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [251] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [253] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [255] = {.entry = {.count = 1, .reusable = false}}, SHIFT(53),
  [257] = {.entry = {.count = 1, .reusable = false}}, SHIFT(32),
  [259] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [261] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [263] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [265] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_for_statement, 5, .production_id = 31),
  [267] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_statement, 5, .production_id = 31),
  [269] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [271] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if_statement, 5, .production_id = 30),
  [273] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 5, .production_id = 30),
  [275] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 1, .production_id = 4),
  [277] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 1, .production_id = 4),
  [279] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_func_call_repeat1, 2, .production_id = 27),
  [281] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [283] = {.entry = {.count = 1, .reusable = false}}, SHIFT(73),
  [285] = {.entry = {.count = 1, .reusable = false}}, SHIFT(47),
  [287] = {.entry = {.count = 1, .reusable = false}}, SHIFT(70),
  [289] = {.entry = {.count = 1, .reusable = false}}, SHIFT(68),
  [291] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [293] = {.entry = {.count = 1, .reusable = false}}, SHIFT(30),
  [295] = {.entry = {.count = 1, .reusable = false}}, SHIFT(31),
  [297] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_decl_assign_statement, 3, .production_id = 23),
  [299] = {.entry = {.count = 1, .reusable = false}}, SHIFT(69),
  [301] = {.entry = {.count = 1, .reusable = false}}, SHIFT(67),
  [303] = {.entry = {.count = 1, .reusable = false}}, SHIFT(141),
  [305] = {.entry = {.count = 1, .reusable = false}}, SHIFT(78),
  [307] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_range, 3, .production_id = 32),
  [309] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_assign_left_side_repeat1, 2),
  [311] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat1, 2),
  [313] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_assign_left_side_repeat1, 2), SHIFT_REPEAT(97),
  [316] = {.entry = {.count = 1, .reusable = false}}, SHIFT(114),
  [318] = {.entry = {.count = 1, .reusable = true}}, SHIFT(144),
  [320] = {.entry = {.count = 1, .reusable = false}}, SHIFT(112),
  [322] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 2, .production_id = 8),
  [324] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [326] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 13),
  [328] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 2, .production_id = 10),
  [330] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [332] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 3, .production_id = 15),
  [334] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 1, .production_id = 2),
  [336] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_left_side, 2, .production_id = 7),
  [338] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 24),
  [340] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat2, 2, .production_id = 19),
  [342] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat2, 2, .production_id = 19), SHIFT_REPEAT(16),
  [345] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 17),
  [347] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat2, 2, .production_id = 18),
  [349] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [351] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assign_left_side_repeat2, 3, .production_id = 26),
  [353] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [355] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [357] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(143),
  [360] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [362] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [364] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [366] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [368] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_global_identifier_repeat1, 2), SHIFT_REPEAT(146),
  [371] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [373] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [375] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [377] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_func_call_repeat1, 2, .production_id = 29), SHIFT_REPEAT(53),
  [380] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_func_call_repeat1, 2, .production_id = 29),
  [382] = {.entry = {.count = 1, .reusable = true}}, SHIFT(131),
  [384] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [386] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [388] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [390] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [392] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 4, .production_id = 5),
  [394] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [396] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [398] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type, 2, .production_id = 9),
  [400] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [402] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 2, .production_id = 3),
  [404] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 3, .production_id = 1),
  [406] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_ports, 4, .production_id = 21),
  [408] = {.entry = {.count = 1, .reusable = true}}, SHIFT(118),
  [410] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [412] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [414] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [416] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [418] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [420] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [422] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [424] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
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
