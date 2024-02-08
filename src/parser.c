#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 160
#define LARGE_STATE_COUNT 9
#define SYMBOL_COUNT 71
#define ALIAS_COUNT 0
#define TOKEN_COUNT 43
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 5
#define MAX_ALIAS_SEQUENCE_LENGTH 9
#define PRODUCTION_ID_COUNT 14

enum {
  anon_sym_module = 1,
  anon_sym_COLON = 2,
  anon_sym_COMMA = 3,
  anon_sym_DASH_GT = 4,
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
  anon_sym_DOT_DOT = 30,
  anon_sym_LBRACE = 31,
  anon_sym_RBRACE = 32,
  anon_sym_reg = 33,
  anon_sym_initial = 34,
  anon_sym_EQ = 35,
  anon_sym_if = 36,
  anon_sym_else = 37,
  anon_sym_for = 38,
  anon_sym_in = 39,
  anon_sym_SEMI = 40,
  sym_single_line_comment = 41,
  sym_multi_line_comment = 42,
  sym_source_file = 43,
  sym_module = 44,
  sym_global_identifier = 45,
  sym__maybe_global_identifier = 46,
  sym_array_type = 47,
  sym__type = 48,
  sym_declaration = 49,
  sym_unary_op = 50,
  sym_binary_op = 51,
  sym_array_op = 52,
  sym_func_call = 53,
  sym__expression = 54,
  sym_range = 55,
  sym_block = 56,
  sym__assign_left_side = 57,
  sym_decl_assign_statement = 58,
  sym_decl_statement = 59,
  sym_expression_statement = 60,
  sym_if_statement = 61,
  sym_for_statement = 62,
  sym__statement = 63,
  aux_sym_source_file_repeat1 = 64,
  aux_sym_module_repeat1 = 65,
  aux_sym_global_identifier_repeat1 = 66,
  aux_sym_func_call_repeat1 = 67,
  aux_sym_block_repeat1 = 68,
  aux_sym__assign_left_side_repeat1 = 69,
  aux_sym__assign_left_side_repeat2 = 70,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_module] = "module",
  [anon_sym_COLON] = ":",
  [anon_sym_COMMA] = ",",
  [anon_sym_DASH_GT] = "->",
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
  [anon_sym_DOT_DOT] = "..",
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
  [sym_single_line_comment] = "single_line_comment",
  [sym_multi_line_comment] = "multi_line_comment",
  [sym_source_file] = "source_file",
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
  [aux_sym_module_repeat1] = "module_repeat1",
  [aux_sym_global_identifier_repeat1] = "global_identifier_repeat1",
  [aux_sym_func_call_repeat1] = "func_call_repeat1",
  [aux_sym_block_repeat1] = "block_repeat1",
  [aux_sym__assign_left_side_repeat1] = "_assign_left_side_repeat1",
  [aux_sym__assign_left_side_repeat2] = "_assign_left_side_repeat2",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_module] = anon_sym_module,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_DASH_GT] = anon_sym_DASH_GT,
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
  [anon_sym_DOT_DOT] = anon_sym_DOT_DOT,
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
  [sym_single_line_comment] = sym_single_line_comment,
  [sym_multi_line_comment] = sym_multi_line_comment,
  [sym_source_file] = sym_source_file,
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
  [aux_sym_module_repeat1] = aux_sym_module_repeat1,
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
  [anon_sym_module] = {
    .visible = true,
    .named = false,
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
  [anon_sym_DOT_DOT] = {
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
  [aux_sym_module_repeat1] = {
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
  field_inputs = 1,
  field_latency_spec = 2,
  field_name = 3,
  field_outputs = 4,
  field_type = 5,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_inputs] = "inputs",
  [field_latency_spec] = "latency_spec",
  [field_name] = "name",
  [field_outputs] = "outputs",
  [field_type] = "type",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 2},
  [3] = {.index = 3, .length = 2},
  [4] = {.index = 5, .length = 2},
  [5] = {.index = 7, .length = 2},
  [6] = {.index = 9, .length = 3},
  [7] = {.index = 12, .length = 3},
  [8] = {.index = 15, .length = 3},
  [9] = {.index = 18, .length = 3},
  [10] = {.index = 21, .length = 3},
  [11] = {.index = 24, .length = 4},
  [12] = {.index = 28, .length = 4},
  [13] = {.index = 32, .length = 5},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_name, 1},
  [1] =
    {field_name, 1},
    {field_type, 0},
  [3] =
    {field_name, 1},
    {field_outputs, 4},
  [5] =
    {field_name, 2},
    {field_type, 1},
  [7] =
    {field_inputs, 3},
    {field_name, 1},
  [9] =
    {field_name, 1},
    {field_outputs, 4},
    {field_outputs, 5},
  [12] =
    {field_latency_spec, 3},
    {field_name, 1},
    {field_type, 0},
  [15] =
    {field_inputs, 3},
    {field_name, 1},
    {field_outputs, 5},
  [18] =
    {field_inputs, 3},
    {field_inputs, 4},
    {field_name, 1},
  [21] =
    {field_latency_spec, 4},
    {field_name, 2},
    {field_type, 1},
  [24] =
    {field_inputs, 3},
    {field_name, 1},
    {field_outputs, 5},
    {field_outputs, 6},
  [28] =
    {field_inputs, 3},
    {field_inputs, 4},
    {field_name, 1},
    {field_outputs, 6},
  [32] =
    {field_inputs, 3},
    {field_inputs, 4},
    {field_name, 1},
    {field_outputs, 6},
    {field_outputs, 7},
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
  [42] = 9,
  [43] = 13,
  [44] = 12,
  [45] = 45,
  [46] = 34,
  [47] = 11,
  [48] = 33,
  [49] = 49,
  [50] = 10,
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
  [66] = 54,
  [67] = 58,
  [68] = 61,
  [69] = 62,
  [70] = 70,
  [71] = 53,
  [72] = 59,
  [73] = 64,
  [74] = 74,
  [75] = 75,
  [76] = 75,
  [77] = 52,
  [78] = 78,
  [79] = 15,
  [80] = 80,
  [81] = 81,
  [82] = 82,
  [83] = 83,
  [84] = 35,
  [85] = 85,
  [86] = 37,
  [87] = 32,
  [88] = 30,
  [89] = 29,
  [90] = 28,
  [91] = 26,
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
  [107] = 106,
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
  [130] = 104,
  [131] = 131,
  [132] = 132,
  [133] = 133,
  [134] = 34,
  [135] = 135,
  [136] = 136,
  [137] = 137,
  [138] = 131,
  [139] = 136,
  [140] = 140,
  [141] = 141,
  [142] = 33,
  [143] = 143,
  [144] = 144,
  [145] = 145,
  [146] = 146,
  [147] = 147,
  [148] = 148,
  [149] = 105,
  [150] = 150,
  [151] = 151,
  [152] = 152,
  [153] = 153,
  [154] = 154,
  [155] = 151,
  [156] = 156,
  [157] = 157,
  [158] = 158,
  [159] = 152,
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
      if (eof) ADVANCE(36);
      if (lookahead == '!') ADVANCE(76);
      if (lookahead == '%') ADVANCE(87);
      if (lookahead == '&') ADVANCE(78);
      if (lookahead == '\'') ADVANCE(70);
      if (lookahead == '(') ADVANCE(88);
      if (lookahead == ')') ADVANCE(89);
      if (lookahead == '*') ADVANCE(74);
      if (lookahead == '+') ADVANCE(71);
      if (lookahead == ',') ADVANCE(39);
      if (lookahead == '-') ADVANCE(73);
      if (lookahead == '.') ADVANCE(13);
      if (lookahead == '/') ADVANCE(86);
      if (lookahead == ':') ADVANCE(38);
      if (lookahead == ';') ADVANCE(104);
      if (lookahead == '<') ADVANCE(82);
      if (lookahead == '=') ADVANCE(96);
      if (lookahead == '>') ADVANCE(84);
      if (lookahead == '[') ADVANCE(64);
      if (lookahead == ']') ADVANCE(65);
      if (lookahead == '^') ADVANCE(79);
      if (lookahead == 'e') ADVANCE(25);
      if (lookahead == 'f') ADVANCE(29);
      if (lookahead == 'g') ADVANCE(18);
      if (lookahead == 'i') ADVANCE(23);
      if (lookahead == 'm') ADVANCE(30);
      if (lookahead == 'r') ADVANCE(19);
      if (lookahead == 's') ADVANCE(33);
      if (lookahead == '{') ADVANCE(91);
      if (lookahead == '|') ADVANCE(77);
      if (lookahead == '}') ADVANCE(92);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(62);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(105);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      if (lookahead == '!') ADVANCE(75);
      if (lookahead == '&') ADVANCE(78);
      if (lookahead == '(') ADVANCE(88);
      if (lookahead == ')') ADVANCE(89);
      if (lookahead == '*') ADVANCE(74);
      if (lookahead == '+') ADVANCE(71);
      if (lookahead == '-') ADVANCE(72);
      if (lookahead == '/') ADVANCE(10);
      if (lookahead == ':') ADVANCE(14);
      if (lookahead == '^') ADVANCE(79);
      if (lookahead == '|') ADVANCE(77);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(2)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(62);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(61);
      END_STATE();
    case 3:
      if (lookahead == '!') ADVANCE(75);
      if (lookahead == '&') ADVANCE(78);
      if (lookahead == '(') ADVANCE(88);
      if (lookahead == '*') ADVANCE(74);
      if (lookahead == '+') ADVANCE(71);
      if (lookahead == '-') ADVANCE(73);
      if (lookahead == '/') ADVANCE(10);
      if (lookahead == ':') ADVANCE(14);
      if (lookahead == '^') ADVANCE(79);
      if (lookahead == 'g') ADVANCE(43);
      if (lookahead == 's') ADVANCE(58);
      if (lookahead == '{') ADVANCE(91);
      if (lookahead == '|') ADVANCE(77);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(3)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(62);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(61);
      END_STATE();
    case 4:
      if (lookahead == '!') ADVANCE(75);
      if (lookahead == '&') ADVANCE(78);
      if (lookahead == '(') ADVANCE(88);
      if (lookahead == '*') ADVANCE(74);
      if (lookahead == '+') ADVANCE(71);
      if (lookahead == '-') ADVANCE(72);
      if (lookahead == '/') ADVANCE(10);
      if (lookahead == ':') ADVANCE(14);
      if (lookahead == '^') ADVANCE(79);
      if (lookahead == 'e') ADVANCE(52);
      if (lookahead == 'f') ADVANCE(55);
      if (lookahead == 'g') ADVANCE(43);
      if (lookahead == 'i') ADVANCE(47);
      if (lookahead == 'r') ADVANCE(44);
      if (lookahead == 's') ADVANCE(58);
      if (lookahead == '{') ADVANCE(91);
      if (lookahead == '|') ADVANCE(77);
      if (lookahead == '}') ADVANCE(92);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(4)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(62);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(61);
      END_STATE();
    case 5:
      if (lookahead == '!') ADVANCE(75);
      if (lookahead == '&') ADVANCE(78);
      if (lookahead == '(') ADVANCE(88);
      if (lookahead == '*') ADVANCE(74);
      if (lookahead == '+') ADVANCE(71);
      if (lookahead == '-') ADVANCE(72);
      if (lookahead == '/') ADVANCE(10);
      if (lookahead == ':') ADVANCE(14);
      if (lookahead == '^') ADVANCE(79);
      if (lookahead == 'f') ADVANCE(55);
      if (lookahead == 'g') ADVANCE(43);
      if (lookahead == 'i') ADVANCE(47);
      if (lookahead == 'r') ADVANCE(44);
      if (lookahead == 's') ADVANCE(58);
      if (lookahead == '{') ADVANCE(91);
      if (lookahead == '|') ADVANCE(77);
      if (lookahead == '}') ADVANCE(92);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(5)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(62);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(61);
      END_STATE();
    case 6:
      if (lookahead == '!') ADVANCE(75);
      if (lookahead == '&') ADVANCE(78);
      if (lookahead == '(') ADVANCE(88);
      if (lookahead == '*') ADVANCE(74);
      if (lookahead == '+') ADVANCE(71);
      if (lookahead == '-') ADVANCE(72);
      if (lookahead == '/') ADVANCE(10);
      if (lookahead == ':') ADVANCE(14);
      if (lookahead == '^') ADVANCE(79);
      if (lookahead == 'g') ADVANCE(43);
      if (lookahead == 'i') ADVANCE(53);
      if (lookahead == 'r') ADVANCE(44);
      if (lookahead == 's') ADVANCE(58);
      if (lookahead == '|') ADVANCE(77);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(6)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(62);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(61);
      END_STATE();
    case 7:
      if (lookahead == '!') ADVANCE(75);
      if (lookahead == '&') ADVANCE(78);
      if (lookahead == '(') ADVANCE(88);
      if (lookahead == '*') ADVANCE(74);
      if (lookahead == '+') ADVANCE(71);
      if (lookahead == '-') ADVANCE(72);
      if (lookahead == '/') ADVANCE(10);
      if (lookahead == ':') ADVANCE(14);
      if (lookahead == '^') ADVANCE(79);
      if (lookahead == 'g') ADVANCE(43);
      if (lookahead == 'r') ADVANCE(44);
      if (lookahead == 's') ADVANCE(58);
      if (lookahead == '|') ADVANCE(77);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(7)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(62);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(61);
      END_STATE();
    case 8:
      if (lookahead == '!') ADVANCE(15);
      if (lookahead == '%') ADVANCE(87);
      if (lookahead == '&') ADVANCE(78);
      if (lookahead == '(') ADVANCE(88);
      if (lookahead == ')') ADVANCE(89);
      if (lookahead == '*') ADVANCE(74);
      if (lookahead == '+') ADVANCE(71);
      if (lookahead == ',') ADVANCE(39);
      if (lookahead == '-') ADVANCE(73);
      if (lookahead == '.') ADVANCE(13);
      if (lookahead == '/') ADVANCE(86);
      if (lookahead == ':') ADVANCE(14);
      if (lookahead == ';') ADVANCE(104);
      if (lookahead == '<') ADVANCE(82);
      if (lookahead == '=') ADVANCE(96);
      if (lookahead == '>') ADVANCE(84);
      if (lookahead == '[') ADVANCE(64);
      if (lookahead == ']') ADVANCE(65);
      if (lookahead == '^') ADVANCE(79);
      if (lookahead == 'i') ADVANCE(27);
      if (lookahead == '{') ADVANCE(91);
      if (lookahead == '|') ADVANCE(77);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(8)
      END_STATE();
    case 9:
      if (lookahead == '!') ADVANCE(15);
      if (lookahead == '%') ADVANCE(87);
      if (lookahead == '&') ADVANCE(78);
      if (lookahead == '(') ADVANCE(88);
      if (lookahead == ')') ADVANCE(89);
      if (lookahead == '*') ADVANCE(74);
      if (lookahead == '+') ADVANCE(71);
      if (lookahead == ',') ADVANCE(39);
      if (lookahead == '-') ADVANCE(73);
      if (lookahead == '.') ADVANCE(13);
      if (lookahead == '/') ADVANCE(86);
      if (lookahead == ':') ADVANCE(14);
      if (lookahead == ';') ADVANCE(104);
      if (lookahead == '<') ADVANCE(82);
      if (lookahead == '=') ADVANCE(96);
      if (lookahead == '>') ADVANCE(84);
      if (lookahead == '[') ADVANCE(64);
      if (lookahead == ']') ADVANCE(65);
      if (lookahead == '^') ADVANCE(79);
      if (lookahead == '{') ADVANCE(91);
      if (lookahead == '|') ADVANCE(77);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(9)
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(61);
      END_STATE();
    case 10:
      if (lookahead == '*') ADVANCE(12);
      if (lookahead == '/') ADVANCE(1);
      END_STATE();
    case 11:
      if (lookahead == '*') ADVANCE(11);
      if (lookahead == '/') ADVANCE(106);
      if (lookahead != 0) ADVANCE(12);
      END_STATE();
    case 12:
      if (lookahead == '*') ADVANCE(11);
      if (lookahead != 0) ADVANCE(12);
      END_STATE();
    case 13:
      if (lookahead == '.') ADVANCE(90);
      END_STATE();
    case 14:
      if (lookahead == ':') ADVANCE(63);
      END_STATE();
    case 15:
      if (lookahead == '=') ADVANCE(81);
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(34);
      END_STATE();
    case 17:
      if (lookahead == 'd') ADVANCE(35);
      END_STATE();
    case 18:
      if (lookahead == 'e') ADVANCE(28);
      END_STATE();
    case 19:
      if (lookahead == 'e') ADVANCE(24);
      END_STATE();
    case 20:
      if (lookahead == 'e') ADVANCE(99);
      END_STATE();
    case 21:
      if (lookahead == 'e') ADVANCE(66);
      END_STATE();
    case 22:
      if (lookahead == 'e') ADVANCE(37);
      END_STATE();
    case 23:
      if (lookahead == 'f') ADVANCE(97);
      if (lookahead == 'n') ADVANCE(103);
      END_STATE();
    case 24:
      if (lookahead == 'g') ADVANCE(93);
      END_STATE();
    case 25:
      if (lookahead == 'l') ADVANCE(32);
      END_STATE();
    case 26:
      if (lookahead == 'l') ADVANCE(22);
      END_STATE();
    case 27:
      if (lookahead == 'n') ADVANCE(103);
      END_STATE();
    case 28:
      if (lookahead == 'n') ADVANCE(68);
      END_STATE();
    case 29:
      if (lookahead == 'o') ADVANCE(31);
      END_STATE();
    case 30:
      if (lookahead == 'o') ADVANCE(17);
      END_STATE();
    case 31:
      if (lookahead == 'r') ADVANCE(101);
      END_STATE();
    case 32:
      if (lookahead == 's') ADVANCE(20);
      END_STATE();
    case 33:
      if (lookahead == 't') ADVANCE(16);
      END_STATE();
    case 34:
      if (lookahead == 't') ADVANCE(21);
      END_STATE();
    case 35:
      if (lookahead == 'u') ADVANCE(26);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_module);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(51);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(61);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(60);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(61);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(54);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(48);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(67);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(100);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'f') ADVANCE(98);
      if (lookahead == 'n') ADVANCE(49);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'g') ADVANCE(94);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(59);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(41);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(95);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(57);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(49);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(69);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(56);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(102);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(46);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(42);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(50);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(45);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(62);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(anon_sym_COLON_COLON);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(anon_sym_state);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(anon_sym_state);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(anon_sym_gen);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(anon_sym_gen);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '>') ADVANCE(40);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(anon_sym_BANG);
      if (lookahead == '=') ADVANCE(81);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(anon_sym_AMP);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(83);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(85);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(anon_sym_GT_EQ);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '*') ADVANCE(12);
      if (lookahead == '/') ADVANCE(1);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(anon_sym_DOT_DOT);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(anon_sym_reg);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(anon_sym_reg);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(anon_sym_initial);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead == '=') ADVANCE(80);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 98:
      ACCEPT_TOKEN(anon_sym_if);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 99:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(anon_sym_else);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 101:
      ACCEPT_TOKEN(anon_sym_for);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(anon_sym_for);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(61);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(anon_sym_in);
      END_STATE();
    case 104:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(sym_single_line_comment);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(sym_multi_line_comment);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 5},
  [3] = {.lex_state = 5},
  [4] = {.lex_state = 5},
  [5] = {.lex_state = 5},
  [6] = {.lex_state = 5},
  [7] = {.lex_state = 5},
  [8] = {.lex_state = 5},
  [9] = {.lex_state = 9},
  [10] = {.lex_state = 9},
  [11] = {.lex_state = 9},
  [12] = {.lex_state = 9},
  [13] = {.lex_state = 9},
  [14] = {.lex_state = 6},
  [15] = {.lex_state = 9},
  [16] = {.lex_state = 7},
  [17] = {.lex_state = 7},
  [18] = {.lex_state = 8},
  [19] = {.lex_state = 8},
  [20] = {.lex_state = 8},
  [21] = {.lex_state = 8},
  [22] = {.lex_state = 8},
  [23] = {.lex_state = 8},
  [24] = {.lex_state = 8},
  [25] = {.lex_state = 8},
  [26] = {.lex_state = 9},
  [27] = {.lex_state = 3},
  [28] = {.lex_state = 9},
  [29] = {.lex_state = 9},
  [30] = {.lex_state = 9},
  [31] = {.lex_state = 3},
  [32] = {.lex_state = 9},
  [33] = {.lex_state = 4},
  [34] = {.lex_state = 4},
  [35] = {.lex_state = 9},
  [36] = {.lex_state = 4},
  [37] = {.lex_state = 9},
  [38] = {.lex_state = 9},
  [39] = {.lex_state = 9},
  [40] = {.lex_state = 2},
  [41] = {.lex_state = 2},
  [42] = {.lex_state = 8},
  [43] = {.lex_state = 8},
  [44] = {.lex_state = 8},
  [45] = {.lex_state = 5},
  [46] = {.lex_state = 5},
  [47] = {.lex_state = 8},
  [48] = {.lex_state = 5},
  [49] = {.lex_state = 5},
  [50] = {.lex_state = 8},
  [51] = {.lex_state = 5},
  [52] = {.lex_state = 2},
  [53] = {.lex_state = 2},
  [54] = {.lex_state = 2},
  [55] = {.lex_state = 2},
  [56] = {.lex_state = 2},
  [57] = {.lex_state = 9},
  [58] = {.lex_state = 2},
  [59] = {.lex_state = 2},
  [60] = {.lex_state = 2},
  [61] = {.lex_state = 2},
  [62] = {.lex_state = 2},
  [63] = {.lex_state = 2},
  [64] = {.lex_state = 2},
  [65] = {.lex_state = 2},
  [66] = {.lex_state = 2},
  [67] = {.lex_state = 2},
  [68] = {.lex_state = 2},
  [69] = {.lex_state = 2},
  [70] = {.lex_state = 2},
  [71] = {.lex_state = 2},
  [72] = {.lex_state = 2},
  [73] = {.lex_state = 2},
  [74] = {.lex_state = 9},
  [75] = {.lex_state = 2},
  [76] = {.lex_state = 2},
  [77] = {.lex_state = 2},
  [78] = {.lex_state = 2},
  [79] = {.lex_state = 8},
  [80] = {.lex_state = 9},
  [81] = {.lex_state = 9},
  [82] = {.lex_state = 9},
  [83] = {.lex_state = 9},
  [84] = {.lex_state = 8},
  [85] = {.lex_state = 9},
  [86] = {.lex_state = 8},
  [87] = {.lex_state = 8},
  [88] = {.lex_state = 8},
  [89] = {.lex_state = 8},
  [90] = {.lex_state = 8},
  [91] = {.lex_state = 8},
  [92] = {.lex_state = 9},
  [93] = {.lex_state = 9},
  [94] = {.lex_state = 9},
  [95] = {.lex_state = 9},
  [96] = {.lex_state = 9},
  [97] = {.lex_state = 7},
  [98] = {.lex_state = 3},
  [99] = {.lex_state = 3},
  [100] = {.lex_state = 3},
  [101] = {.lex_state = 3},
  [102] = {.lex_state = 3},
  [103] = {.lex_state = 3},
  [104] = {.lex_state = 0},
  [105] = {.lex_state = 0},
  [106] = {.lex_state = 9},
  [107] = {.lex_state = 9},
  [108] = {.lex_state = 9},
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
  [131] = {.lex_state = 9},
  [132] = {.lex_state = 0},
  [133] = {.lex_state = 0},
  [134] = {.lex_state = 0},
  [135] = {.lex_state = 0},
  [136] = {.lex_state = 9},
  [137] = {.lex_state = 0},
  [138] = {.lex_state = 9},
  [139] = {.lex_state = 9},
  [140] = {.lex_state = 0},
  [141] = {.lex_state = 0},
  [142] = {.lex_state = 0},
  [143] = {.lex_state = 0},
  [144] = {.lex_state = 0},
  [145] = {.lex_state = 9},
  [146] = {.lex_state = 0},
  [147] = {.lex_state = 0},
  [148] = {.lex_state = 0},
  [149] = {.lex_state = 0},
  [150] = {.lex_state = 0},
  [151] = {.lex_state = 9},
  [152] = {.lex_state = 9},
  [153] = {.lex_state = 0},
  [154] = {.lex_state = 9},
  [155] = {.lex_state = 9},
  [156] = {.lex_state = 0},
  [157] = {.lex_state = 0},
  [158] = {.lex_state = 0},
  [159] = {.lex_state = 9},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_module] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_DASH_GT] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
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
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_PERCENT] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_DOT_DOT] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_reg] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [anon_sym_for] = ACTIONS(1),
    [anon_sym_in] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(156),
    [sym_module] = STATE(111),
    [aux_sym_source_file_repeat1] = STATE(111),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_module] = ACTIONS(7),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [2] = {
    [sym_global_identifier] = STATE(38),
    [sym__maybe_global_identifier] = STATE(18),
    [sym_array_type] = STATE(136),
    [sym__type] = STATE(136),
    [sym_declaration] = STATE(117),
    [sym_unary_op] = STATE(39),
    [sym_binary_op] = STATE(39),
    [sym_array_op] = STATE(39),
    [sym_func_call] = STATE(39),
    [sym__expression] = STATE(39),
    [sym_block] = STATE(6),
    [sym__assign_left_side] = STATE(158),
    [sym_decl_assign_statement] = STATE(157),
    [sym_decl_statement] = STATE(157),
    [sym_expression_statement] = STATE(157),
    [sym_if_statement] = STATE(6),
    [sym_for_statement] = STATE(6),
    [sym__statement] = STATE(6),
    [aux_sym_block_repeat1] = STATE(6),
    [aux_sym__assign_left_side_repeat1] = STATE(16),
    [sym_identifier] = ACTIONS(9),
    [sym_number] = ACTIONS(11),
    [anon_sym_COLON_COLON] = ACTIONS(13),
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
    [anon_sym_LBRACE] = ACTIONS(21),
    [anon_sym_RBRACE] = ACTIONS(23),
    [anon_sym_reg] = ACTIONS(25),
    [anon_sym_initial] = ACTIONS(27),
    [anon_sym_if] = ACTIONS(29),
    [anon_sym_for] = ACTIONS(31),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [3] = {
    [sym_global_identifier] = STATE(38),
    [sym__maybe_global_identifier] = STATE(18),
    [sym_array_type] = STATE(136),
    [sym__type] = STATE(136),
    [sym_declaration] = STATE(117),
    [sym_unary_op] = STATE(39),
    [sym_binary_op] = STATE(39),
    [sym_array_op] = STATE(39),
    [sym_func_call] = STATE(39),
    [sym__expression] = STATE(39),
    [sym_block] = STATE(2),
    [sym__assign_left_side] = STATE(158),
    [sym_decl_assign_statement] = STATE(157),
    [sym_decl_statement] = STATE(157),
    [sym_expression_statement] = STATE(157),
    [sym_if_statement] = STATE(2),
    [sym_for_statement] = STATE(2),
    [sym__statement] = STATE(2),
    [aux_sym_block_repeat1] = STATE(2),
    [aux_sym__assign_left_side_repeat1] = STATE(16),
    [sym_identifier] = ACTIONS(9),
    [sym_number] = ACTIONS(11),
    [anon_sym_COLON_COLON] = ACTIONS(13),
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
    [anon_sym_LBRACE] = ACTIONS(21),
    [anon_sym_RBRACE] = ACTIONS(33),
    [anon_sym_reg] = ACTIONS(25),
    [anon_sym_initial] = ACTIONS(27),
    [anon_sym_if] = ACTIONS(29),
    [anon_sym_for] = ACTIONS(31),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [4] = {
    [sym_global_identifier] = STATE(38),
    [sym__maybe_global_identifier] = STATE(18),
    [sym_array_type] = STATE(136),
    [sym__type] = STATE(136),
    [sym_declaration] = STATE(117),
    [sym_unary_op] = STATE(39),
    [sym_binary_op] = STATE(39),
    [sym_array_op] = STATE(39),
    [sym_func_call] = STATE(39),
    [sym__expression] = STATE(39),
    [sym_block] = STATE(6),
    [sym__assign_left_side] = STATE(158),
    [sym_decl_assign_statement] = STATE(157),
    [sym_decl_statement] = STATE(157),
    [sym_expression_statement] = STATE(157),
    [sym_if_statement] = STATE(6),
    [sym_for_statement] = STATE(6),
    [sym__statement] = STATE(6),
    [aux_sym_block_repeat1] = STATE(6),
    [aux_sym__assign_left_side_repeat1] = STATE(16),
    [sym_identifier] = ACTIONS(9),
    [sym_number] = ACTIONS(11),
    [anon_sym_COLON_COLON] = ACTIONS(13),
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
    [anon_sym_LBRACE] = ACTIONS(21),
    [anon_sym_RBRACE] = ACTIONS(35),
    [anon_sym_reg] = ACTIONS(25),
    [anon_sym_initial] = ACTIONS(27),
    [anon_sym_if] = ACTIONS(29),
    [anon_sym_for] = ACTIONS(31),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [5] = {
    [sym_global_identifier] = STATE(38),
    [sym__maybe_global_identifier] = STATE(18),
    [sym_array_type] = STATE(136),
    [sym__type] = STATE(136),
    [sym_declaration] = STATE(117),
    [sym_unary_op] = STATE(39),
    [sym_binary_op] = STATE(39),
    [sym_array_op] = STATE(39),
    [sym_func_call] = STATE(39),
    [sym__expression] = STATE(39),
    [sym_block] = STATE(4),
    [sym__assign_left_side] = STATE(158),
    [sym_decl_assign_statement] = STATE(157),
    [sym_decl_statement] = STATE(157),
    [sym_expression_statement] = STATE(157),
    [sym_if_statement] = STATE(4),
    [sym_for_statement] = STATE(4),
    [sym__statement] = STATE(4),
    [aux_sym_block_repeat1] = STATE(4),
    [aux_sym__assign_left_side_repeat1] = STATE(16),
    [sym_identifier] = ACTIONS(9),
    [sym_number] = ACTIONS(11),
    [anon_sym_COLON_COLON] = ACTIONS(13),
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
    [anon_sym_LBRACE] = ACTIONS(21),
    [anon_sym_RBRACE] = ACTIONS(37),
    [anon_sym_reg] = ACTIONS(25),
    [anon_sym_initial] = ACTIONS(27),
    [anon_sym_if] = ACTIONS(29),
    [anon_sym_for] = ACTIONS(31),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [6] = {
    [sym_global_identifier] = STATE(38),
    [sym__maybe_global_identifier] = STATE(18),
    [sym_array_type] = STATE(136),
    [sym__type] = STATE(136),
    [sym_declaration] = STATE(117),
    [sym_unary_op] = STATE(39),
    [sym_binary_op] = STATE(39),
    [sym_array_op] = STATE(39),
    [sym_func_call] = STATE(39),
    [sym__expression] = STATE(39),
    [sym_block] = STATE(6),
    [sym__assign_left_side] = STATE(158),
    [sym_decl_assign_statement] = STATE(157),
    [sym_decl_statement] = STATE(157),
    [sym_expression_statement] = STATE(157),
    [sym_if_statement] = STATE(6),
    [sym_for_statement] = STATE(6),
    [sym__statement] = STATE(6),
    [aux_sym_block_repeat1] = STATE(6),
    [aux_sym__assign_left_side_repeat1] = STATE(16),
    [sym_identifier] = ACTIONS(39),
    [sym_number] = ACTIONS(42),
    [anon_sym_COLON_COLON] = ACTIONS(45),
    [anon_sym_state] = ACTIONS(48),
    [anon_sym_gen] = ACTIONS(48),
    [anon_sym_PLUS] = ACTIONS(51),
    [anon_sym_DASH] = ACTIONS(51),
    [anon_sym_STAR] = ACTIONS(51),
    [anon_sym_BANG] = ACTIONS(51),
    [anon_sym_PIPE] = ACTIONS(51),
    [anon_sym_AMP] = ACTIONS(51),
    [anon_sym_CARET] = ACTIONS(51),
    [anon_sym_LPAREN] = ACTIONS(54),
    [anon_sym_LBRACE] = ACTIONS(57),
    [anon_sym_RBRACE] = ACTIONS(60),
    [anon_sym_reg] = ACTIONS(62),
    [anon_sym_initial] = ACTIONS(65),
    [anon_sym_if] = ACTIONS(68),
    [anon_sym_for] = ACTIONS(71),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [7] = {
    [sym_global_identifier] = STATE(38),
    [sym__maybe_global_identifier] = STATE(18),
    [sym_array_type] = STATE(136),
    [sym__type] = STATE(136),
    [sym_declaration] = STATE(117),
    [sym_unary_op] = STATE(39),
    [sym_binary_op] = STATE(39),
    [sym_array_op] = STATE(39),
    [sym_func_call] = STATE(39),
    [sym__expression] = STATE(39),
    [sym_block] = STATE(6),
    [sym__assign_left_side] = STATE(158),
    [sym_decl_assign_statement] = STATE(157),
    [sym_decl_statement] = STATE(157),
    [sym_expression_statement] = STATE(157),
    [sym_if_statement] = STATE(6),
    [sym_for_statement] = STATE(6),
    [sym__statement] = STATE(6),
    [aux_sym_block_repeat1] = STATE(6),
    [aux_sym__assign_left_side_repeat1] = STATE(16),
    [sym_identifier] = ACTIONS(9),
    [sym_number] = ACTIONS(11),
    [anon_sym_COLON_COLON] = ACTIONS(13),
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
    [anon_sym_LBRACE] = ACTIONS(21),
    [anon_sym_RBRACE] = ACTIONS(74),
    [anon_sym_reg] = ACTIONS(25),
    [anon_sym_initial] = ACTIONS(27),
    [anon_sym_if] = ACTIONS(29),
    [anon_sym_for] = ACTIONS(31),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
  [8] = {
    [sym_global_identifier] = STATE(38),
    [sym__maybe_global_identifier] = STATE(18),
    [sym_array_type] = STATE(136),
    [sym__type] = STATE(136),
    [sym_declaration] = STATE(117),
    [sym_unary_op] = STATE(39),
    [sym_binary_op] = STATE(39),
    [sym_array_op] = STATE(39),
    [sym_func_call] = STATE(39),
    [sym__expression] = STATE(39),
    [sym_block] = STATE(7),
    [sym__assign_left_side] = STATE(158),
    [sym_decl_assign_statement] = STATE(157),
    [sym_decl_statement] = STATE(157),
    [sym_expression_statement] = STATE(157),
    [sym_if_statement] = STATE(7),
    [sym_for_statement] = STATE(7),
    [sym__statement] = STATE(7),
    [aux_sym_block_repeat1] = STATE(7),
    [aux_sym__assign_left_side_repeat1] = STATE(16),
    [sym_identifier] = ACTIONS(9),
    [sym_number] = ACTIONS(11),
    [anon_sym_COLON_COLON] = ACTIONS(13),
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
    [anon_sym_LBRACE] = ACTIONS(21),
    [anon_sym_RBRACE] = ACTIONS(76),
    [anon_sym_reg] = ACTIONS(25),
    [anon_sym_initial] = ACTIONS(27),
    [anon_sym_if] = ACTIONS(29),
    [anon_sym_for] = ACTIONS(31),
    [sym_single_line_comment] = ACTIONS(3),
    [sym_multi_line_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 5,
    ACTIONS(80), 1,
      anon_sym_COLON_COLON,
    STATE(11), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(82), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(78), 20,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_SEMI,
  [40] = 5,
    ACTIONS(80), 1,
      anon_sym_COLON_COLON,
    STATE(13), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(84), 20,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_SEMI,
  [80] = 5,
    ACTIONS(80), 1,
      anon_sym_COLON_COLON,
    STATE(12), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(90), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(88), 20,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_SEMI,
  [120] = 5,
    ACTIONS(94), 1,
      anon_sym_COLON_COLON,
    STATE(12), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(97), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(92), 20,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_SEMI,
  [160] = 5,
    ACTIONS(80), 1,
      anon_sym_COLON_COLON,
    STATE(12), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(82), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(78), 20,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_SEMI,
  [200] = 15,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(99), 1,
      sym_number,
    ACTIONS(101), 1,
      anon_sym_reg,
    ACTIONS(103), 1,
      anon_sym_initial,
    STATE(17), 1,
      aux_sym__assign_left_side_repeat1,
    STATE(18), 1,
      sym__maybe_global_identifier,
    STATE(38), 1,
      sym_global_identifier,
    STATE(143), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(136), 2,
      sym_array_type,
      sym__type,
    STATE(81), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [259] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(97), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(92), 21,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      sym_identifier,
      anon_sym_COLON_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_SEMI,
  [294] = 14,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(105), 1,
      sym_number,
    ACTIONS(107), 1,
      anon_sym_reg,
    STATE(18), 1,
      sym__maybe_global_identifier,
    STATE(38), 1,
      sym_global_identifier,
    STATE(97), 1,
      aux_sym__assign_left_side_repeat1,
    STATE(128), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(136), 2,
      sym_array_type,
      sym__type,
    STATE(57), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [350] = 14,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(107), 1,
      anon_sym_reg,
    ACTIONS(109), 1,
      sym_number,
    STATE(18), 1,
      sym__maybe_global_identifier,
    STATE(38), 1,
      sym_global_identifier,
    STATE(97), 1,
      aux_sym__assign_left_side_repeat1,
    STATE(148), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(136), 2,
      sym_array_type,
      sym__type,
    STATE(82), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [406] = 4,
    ACTIONS(115), 1,
      anon_sym_LPAREN,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(113), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(111), 19,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_in,
      anon_sym_SEMI,
  [442] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(119), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(117), 19,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_in,
      anon_sym_SEMI,
  [475] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(123), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(121), 19,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_in,
      anon_sym_SEMI,
  [508] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(127), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(125), 19,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_in,
      anon_sym_SEMI,
  [541] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(131), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(129), 19,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_in,
      anon_sym_SEMI,
  [574] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(135), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(133), 19,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_in,
      anon_sym_SEMI,
  [607] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(139), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(137), 19,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_in,
      anon_sym_SEMI,
  [640] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(143), 5,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(141), 19,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
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
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_in,
      anon_sym_SEMI,
  [673] = 5,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(143), 4,
      anon_sym_DASH,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(141), 16,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_SEMI,
  [709] = 12,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(109), 1,
      sym_number,
    STATE(18), 1,
      sym__maybe_global_identifier,
    STATE(38), 1,
      sym_global_identifier,
    STATE(148), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(136), 2,
      sym_array_type,
      sym__type,
    STATE(82), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [759] = 8,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(149), 1,
      anon_sym_PLUS,
    ACTIONS(151), 1,
      anon_sym_DASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(143), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(141), 14,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_SEMI,
  [801] = 9,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(149), 1,
      anon_sym_PLUS,
    ACTIONS(151), 1,
      anon_sym_DASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(143), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(141), 13,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_SEMI,
  [845] = 7,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(149), 1,
      anon_sym_PLUS,
    ACTIONS(151), 1,
      anon_sym_DASH,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(143), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(141), 15,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_SEMI,
  [885] = 12,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(105), 1,
      sym_number,
    STATE(18), 1,
      sym__maybe_global_identifier,
    STATE(38), 1,
      sym_global_identifier,
    STATE(128), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(136), 2,
      sym_array_type,
      sym__type,
    STATE(57), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [935] = 10,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(149), 1,
      anon_sym_PLUS,
    ACTIONS(151), 1,
      anon_sym_DASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_AMP,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(143), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(141), 12,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_RPAREN,
      anon_sym_DOT_DOT,
      anon_sym_LBRACE,
      anon_sym_SEMI,
  [981] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(159), 8,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_else,
      anon_sym_for,
    ACTIONS(161), 12,
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
  [1010] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(163), 8,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_else,
      anon_sym_for,
    ACTIONS(165), 12,
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
  [1039] = 13,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(149), 1,
      anon_sym_PLUS,
    ACTIONS(151), 1,
      anon_sym_DASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_AMP,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(175), 1,
      anon_sym_EQ,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(173), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(167), 4,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_SEMI,
    ACTIONS(171), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [1088] = 4,
    ACTIONS(181), 1,
      anon_sym_else,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(177), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(179), 12,
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
  [1119] = 13,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(149), 1,
      anon_sym_PLUS,
    ACTIONS(151), 1,
      anon_sym_DASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_AMP,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(185), 1,
      anon_sym_EQ,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(173), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(171), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
    ACTIONS(183), 4,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_SEMI,
  [1168] = 5,
    ACTIONS(189), 1,
      sym_identifier,
    ACTIONS(191), 1,
      anon_sym_LBRACK,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(194), 4,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
      anon_sym_EQ,
    ACTIONS(187), 14,
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
      anon_sym_SEMI,
  [1201] = 14,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_AMP,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(196), 1,
      anon_sym_COMMA,
    ACTIONS(198), 1,
      anon_sym_EQ,
    ACTIONS(200), 1,
      anon_sym_SEMI,
    STATE(124), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(149), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(173), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(171), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [1251] = 9,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(204), 1,
      sym_number,
    ACTIONS(206), 1,
      anon_sym_RPAREN,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(74), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1291] = 9,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(208), 1,
      sym_number,
    STATE(133), 1,
      sym_range,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(94), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1331] = 5,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    STATE(47), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(82), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(78), 14,
      anon_sym_LBRACK,
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
      anon_sym_in,
  [1363] = 5,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    STATE(44), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(82), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(78), 14,
      anon_sym_LBRACK,
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
      anon_sym_in,
  [1395] = 5,
    ACTIONS(212), 1,
      anon_sym_COLON_COLON,
    STATE(44), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(97), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(92), 14,
      anon_sym_LBRACK,
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
      anon_sym_in,
  [1427] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(215), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(217), 12,
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
  [1455] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(163), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(165), 12,
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
  [1483] = 5,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    STATE(44), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(90), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(88), 14,
      anon_sym_LBRACK,
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
      anon_sym_in,
  [1515] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(159), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(161), 12,
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
  [1543] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(219), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(221), 12,
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
  [1571] = 5,
    ACTIONS(210), 1,
      anon_sym_COLON_COLON,
    STATE(43), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(86), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(84), 14,
      anon_sym_LBRACK,
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
      anon_sym_in,
  [1603] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(223), 7,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
      anon_sym_reg,
      anon_sym_initial,
      anon_sym_if,
      anon_sym_for,
    ACTIONS(225), 12,
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
  [1631] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(227), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(35), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1668] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(229), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(25), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1705] = 8,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(231), 1,
      sym_identifier,
    ACTIONS(233), 1,
      sym_number,
    ACTIONS(235), 1,
      anon_sym_COLON_COLON,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(90), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(237), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1742] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(239), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(92), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1779] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(241), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(85), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1816] = 13,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_AMP,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(196), 1,
      anon_sym_COMMA,
    ACTIONS(243), 1,
      anon_sym_EQ,
    STATE(121), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(149), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(173), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(171), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [1863] = 8,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(231), 1,
      sym_identifier,
    ACTIONS(235), 1,
      anon_sym_COLON_COLON,
    ACTIONS(245), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(89), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(237), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1900] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(247), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(24), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1937] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(249), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(83), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [1974] = 8,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(231), 1,
      sym_identifier,
    ACTIONS(235), 1,
      anon_sym_COLON_COLON,
    ACTIONS(251), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(88), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(237), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2011] = 8,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(231), 1,
      sym_identifier,
    ACTIONS(235), 1,
      anon_sym_COLON_COLON,
    ACTIONS(253), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(87), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(237), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2048] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(255), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(93), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2085] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(257), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(26), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2122] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(259), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(80), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2159] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(261), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(28), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2196] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(263), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(29), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2233] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(265), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(30), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2270] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(267), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(32), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2307] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(269), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(96), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2344] = 8,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(229), 1,
      sym_number,
    ACTIONS(231), 1,
      sym_identifier,
    ACTIONS(235), 1,
      anon_sym_COLON_COLON,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(25), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(237), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2381] = 8,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(231), 1,
      sym_identifier,
    ACTIONS(235), 1,
      anon_sym_COLON_COLON,
    ACTIONS(247), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(24), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(237), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2418] = 8,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(231), 1,
      sym_identifier,
    ACTIONS(235), 1,
      anon_sym_COLON_COLON,
    ACTIONS(271), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(91), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(237), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2455] = 13,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_AMP,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(273), 1,
      anon_sym_COMMA,
    ACTIONS(275), 1,
      anon_sym_RPAREN,
    STATE(120), 1,
      aux_sym_func_call_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(149), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(173), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(171), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2502] = 8,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(231), 1,
      sym_identifier,
    ACTIONS(235), 1,
      anon_sym_COLON_COLON,
    ACTIONS(277), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(86), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(237), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2539] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(279), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(37), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2576] = 8,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(231), 1,
      sym_identifier,
    ACTIONS(235), 1,
      anon_sym_COLON_COLON,
    ACTIONS(281), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(84), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(237), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2613] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(283), 1,
      sym_number,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(18), 2,
      sym_global_identifier,
      sym__maybe_global_identifier,
    STATE(95), 5,
      sym_unary_op,
      sym_binary_op,
      sym_array_op,
      sym_func_call,
      sym__expression,
    ACTIONS(17), 7,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_BANG,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
  [2650] = 3,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(97), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(92), 15,
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
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_PERCENT,
      anon_sym_LPAREN,
      anon_sym_in,
  [2677] = 11,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_AMP,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(149), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(173), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(285), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
    ACTIONS(171), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2719] = 12,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_AMP,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(287), 1,
      anon_sym_COMMA,
    ACTIONS(289), 1,
      anon_sym_EQ,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(149), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(173), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(171), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2763] = 12,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_AMP,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(291), 1,
      anon_sym_COMMA,
    ACTIONS(293), 1,
      anon_sym_EQ,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(149), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(173), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(171), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2807] = 12,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_AMP,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(295), 1,
      anon_sym_LBRACE,
    STATE(36), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(149), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(173), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(171), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2851] = 11,
    ACTIONS(167), 1,
      anon_sym_in,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(301), 1,
      anon_sym_PIPE,
    ACTIONS(303), 1,
      anon_sym_AMP,
    ACTIONS(305), 1,
      anon_sym_CARET,
    ACTIONS(311), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(297), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(299), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(309), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(307), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2892] = 11,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_AMP,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(313), 1,
      anon_sym_RPAREN,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(149), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(173), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(171), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2933] = 11,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(183), 1,
      anon_sym_in,
    ACTIONS(301), 1,
      anon_sym_PIPE,
    ACTIONS(303), 1,
      anon_sym_AMP,
    ACTIONS(305), 1,
      anon_sym_CARET,
    ACTIONS(311), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(297), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(299), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(309), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(307), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2974] = 9,
    ACTIONS(301), 1,
      anon_sym_PIPE,
    ACTIONS(303), 1,
      anon_sym_AMP,
    ACTIONS(305), 1,
      anon_sym_CARET,
    ACTIONS(311), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(143), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(297), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(299), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(141), 6,
      anon_sym_LBRACK,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3011] = 6,
    ACTIONS(311), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(143), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(297), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(299), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(141), 9,
      anon_sym_LBRACK,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3042] = 8,
    ACTIONS(301), 1,
      anon_sym_PIPE,
    ACTIONS(305), 1,
      anon_sym_CARET,
    ACTIONS(311), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(143), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(297), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(299), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(141), 7,
      anon_sym_LBRACK,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3077] = 7,
    ACTIONS(305), 1,
      anon_sym_CARET,
    ACTIONS(311), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(143), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(297), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(299), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(141), 8,
      anon_sym_LBRACK,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3110] = 5,
    ACTIONS(311), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(143), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(299), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(141), 11,
      anon_sym_LBRACK,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_PIPE,
      anon_sym_AMP,
      anon_sym_CARET,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3139] = 11,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_AMP,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(315), 1,
      anon_sym_RBRACK,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(149), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(173), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(171), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3180] = 11,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_AMP,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(317), 1,
      anon_sym_RBRACK,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(149), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(173), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(171), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3221] = 11,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_AMP,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(319), 1,
      anon_sym_DOT_DOT,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(149), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(173), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(171), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3262] = 11,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_AMP,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(321), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(149), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(173), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(171), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3303] = 11,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    ACTIONS(153), 1,
      anon_sym_CARET,
    ACTIONS(155), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_AMP,
    ACTIONS(169), 1,
      anon_sym_LBRACK,
    ACTIONS(323), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(145), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(149), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(173), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(171), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3344] = 5,
    ACTIONS(329), 1,
      anon_sym_reg,
    STATE(97), 1,
      aux_sym__assign_left_side_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(325), 3,
      sym_identifier,
      anon_sym_state,
      anon_sym_gen,
    ACTIONS(327), 10,
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
  [3372] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(332), 1,
      sym_identifier,
    ACTIONS(334), 1,
      anon_sym_LBRACE,
    STATE(113), 1,
      sym_declaration,
    STATE(141), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(136), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3401] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(332), 1,
      sym_identifier,
    ACTIONS(334), 1,
      anon_sym_LBRACE,
    STATE(118), 1,
      sym_declaration,
    STATE(129), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(136), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3430] = 8,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(332), 1,
      sym_identifier,
    ACTIONS(334), 1,
      anon_sym_LBRACE,
    STATE(116), 1,
      sym_declaration,
    STATE(140), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(136), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3459] = 7,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(332), 1,
      sym_identifier,
    ACTIONS(336), 1,
      anon_sym_DASH_GT,
    STATE(126), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(136), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3485] = 6,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(332), 1,
      sym_identifier,
    STATE(122), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(15), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(136), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3508] = 6,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(332), 1,
      sym_identifier,
    STATE(150), 1,
      sym_declaration,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(338), 2,
      anon_sym_state,
      anon_sym_gen,
    STATE(139), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3531] = 3,
    ACTIONS(342), 1,
      anon_sym_SQUOTE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(340), 5,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3546] = 3,
    ACTIONS(346), 1,
      anon_sym_SQUOTE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(344), 5,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
      anon_sym_EQ,
      anon_sym_SEMI,
  [3561] = 4,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(348), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(131), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3577] = 4,
    ACTIONS(13), 1,
      anon_sym_COLON_COLON,
    ACTIONS(348), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(138), 3,
      sym_global_identifier,
      sym_array_type,
      sym__type,
  [3593] = 4,
    ACTIONS(80), 1,
      anon_sym_COLON_COLON,
    STATE(13), 1,
      aux_sym_global_identifier_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(84), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [3608] = 4,
    ACTIONS(350), 1,
      anon_sym_COMMA,
    STATE(109), 1,
      aux_sym_module_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(353), 2,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
  [3623] = 4,
    ACTIONS(21), 1,
      anon_sym_LBRACE,
    ACTIONS(355), 1,
      anon_sym_if,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(49), 2,
      sym_block,
      sym_if_statement,
  [3638] = 4,
    ACTIONS(7), 1,
      anon_sym_module,
    ACTIONS(357), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(112), 2,
      sym_module,
      aux_sym_source_file_repeat1,
  [3653] = 4,
    ACTIONS(359), 1,
      ts_builtin_sym_end,
    ACTIONS(361), 1,
      anon_sym_module,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    STATE(112), 2,
      sym_module,
      aux_sym_source_file_repeat1,
  [3668] = 5,
    ACTIONS(334), 1,
      anon_sym_LBRACE,
    ACTIONS(364), 1,
      anon_sym_COMMA,
    STATE(114), 1,
      aux_sym_module_repeat1,
    STATE(132), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3685] = 5,
    ACTIONS(334), 1,
      anon_sym_LBRACE,
    ACTIONS(364), 1,
      anon_sym_COMMA,
    STATE(109), 1,
      aux_sym_module_repeat1,
    STATE(137), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3702] = 5,
    ACTIONS(334), 1,
      anon_sym_LBRACE,
    ACTIONS(364), 1,
      anon_sym_COMMA,
    STATE(109), 1,
      aux_sym_module_repeat1,
    STATE(144), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3719] = 5,
    ACTIONS(334), 1,
      anon_sym_LBRACE,
    ACTIONS(364), 1,
      anon_sym_COMMA,
    STATE(119), 1,
      aux_sym_module_repeat1,
    STATE(135), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3736] = 5,
    ACTIONS(196), 1,
      anon_sym_COMMA,
    ACTIONS(366), 1,
      anon_sym_EQ,
    ACTIONS(368), 1,
      anon_sym_SEMI,
    STATE(124), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3753] = 5,
    ACTIONS(334), 1,
      anon_sym_LBRACE,
    ACTIONS(364), 1,
      anon_sym_COMMA,
    STATE(115), 1,
      aux_sym_module_repeat1,
    STATE(146), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3770] = 5,
    ACTIONS(334), 1,
      anon_sym_LBRACE,
    ACTIONS(364), 1,
      anon_sym_COMMA,
    STATE(109), 1,
      aux_sym_module_repeat1,
    STATE(147), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3787] = 4,
    ACTIONS(273), 1,
      anon_sym_COMMA,
    ACTIONS(370), 1,
      anon_sym_RPAREN,
    STATE(125), 1,
      aux_sym_func_call_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3801] = 4,
    ACTIONS(196), 1,
      anon_sym_COMMA,
    ACTIONS(372), 1,
      anon_sym_EQ,
    STATE(123), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3815] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(353), 3,
      anon_sym_COMMA,
      anon_sym_DASH_GT,
      anon_sym_LBRACE,
  [3825] = 4,
    ACTIONS(287), 1,
      anon_sym_EQ,
    ACTIONS(374), 1,
      anon_sym_COMMA,
    STATE(123), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3839] = 4,
    ACTIONS(196), 1,
      anon_sym_COMMA,
    ACTIONS(377), 1,
      anon_sym_EQ,
    STATE(123), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3853] = 4,
    ACTIONS(285), 1,
      anon_sym_RPAREN,
    ACTIONS(379), 1,
      anon_sym_COMMA,
    STATE(125), 1,
      aux_sym_func_call_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3867] = 4,
    ACTIONS(364), 1,
      anon_sym_COMMA,
    ACTIONS(382), 1,
      anon_sym_DASH_GT,
    STATE(127), 1,
      aux_sym_module_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3881] = 4,
    ACTIONS(364), 1,
      anon_sym_COMMA,
    ACTIONS(384), 1,
      anon_sym_DASH_GT,
    STATE(109), 1,
      aux_sym_module_repeat1,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3895] = 4,
    ACTIONS(196), 1,
      anon_sym_COMMA,
    ACTIONS(377), 1,
      anon_sym_EQ,
    STATE(121), 1,
      aux_sym__assign_left_side_repeat2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3909] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(386), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3918] = 3,
    ACTIONS(340), 1,
      anon_sym_in,
    ACTIONS(388), 1,
      anon_sym_SQUOTE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3929] = 3,
    ACTIONS(390), 1,
      sym_identifier,
    ACTIONS(392), 1,
      anon_sym_LBRACK,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3940] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(394), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3949] = 3,
    ACTIONS(21), 1,
      anon_sym_LBRACE,
    STATE(45), 1,
      sym_block,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3960] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(165), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3969] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(396), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3978] = 3,
    ACTIONS(392), 1,
      anon_sym_LBRACK,
    ACTIONS(398), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [3989] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(400), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [3998] = 3,
    ACTIONS(392), 1,
      anon_sym_LBRACK,
    ACTIONS(402), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4009] = 3,
    ACTIONS(392), 1,
      anon_sym_LBRACK,
    ACTIONS(404), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4020] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(406), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [4029] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(408), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [4038] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(161), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [4047] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(287), 2,
      anon_sym_COMMA,
      anon_sym_EQ,
  [4056] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(410), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [4065] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(412), 2,
      sym_identifier,
      anon_sym_LBRACK,
  [4074] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(414), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [4083] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(416), 2,
      ts_builtin_sym_end,
      anon_sym_module,
  [4092] = 2,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
    ACTIONS(291), 2,
      anon_sym_COMMA,
      anon_sym_EQ,
  [4101] = 3,
    ACTIONS(344), 1,
      anon_sym_in,
    ACTIONS(418), 1,
      anon_sym_SQUOTE,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4112] = 2,
    ACTIONS(420), 1,
      anon_sym_in,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4120] = 2,
    ACTIONS(422), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4128] = 2,
    ACTIONS(424), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4136] = 2,
    ACTIONS(426), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4144] = 2,
    ACTIONS(428), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4152] = 2,
    ACTIONS(430), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4160] = 2,
    ACTIONS(432), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4168] = 2,
    ACTIONS(434), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4176] = 2,
    ACTIONS(436), 1,
      anon_sym_EQ,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
  [4184] = 2,
    ACTIONS(438), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_single_line_comment,
      sym_multi_line_comment,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(9)] = 0,
  [SMALL_STATE(10)] = 40,
  [SMALL_STATE(11)] = 80,
  [SMALL_STATE(12)] = 120,
  [SMALL_STATE(13)] = 160,
  [SMALL_STATE(14)] = 200,
  [SMALL_STATE(15)] = 259,
  [SMALL_STATE(16)] = 294,
  [SMALL_STATE(17)] = 350,
  [SMALL_STATE(18)] = 406,
  [SMALL_STATE(19)] = 442,
  [SMALL_STATE(20)] = 475,
  [SMALL_STATE(21)] = 508,
  [SMALL_STATE(22)] = 541,
  [SMALL_STATE(23)] = 574,
  [SMALL_STATE(24)] = 607,
  [SMALL_STATE(25)] = 640,
  [SMALL_STATE(26)] = 673,
  [SMALL_STATE(27)] = 709,
  [SMALL_STATE(28)] = 759,
  [SMALL_STATE(29)] = 801,
  [SMALL_STATE(30)] = 845,
  [SMALL_STATE(31)] = 885,
  [SMALL_STATE(32)] = 935,
  [SMALL_STATE(33)] = 981,
  [SMALL_STATE(34)] = 1010,
  [SMALL_STATE(35)] = 1039,
  [SMALL_STATE(36)] = 1088,
  [SMALL_STATE(37)] = 1119,
  [SMALL_STATE(38)] = 1168,
  [SMALL_STATE(39)] = 1201,
  [SMALL_STATE(40)] = 1251,
  [SMALL_STATE(41)] = 1291,
  [SMALL_STATE(42)] = 1331,
  [SMALL_STATE(43)] = 1363,
  [SMALL_STATE(44)] = 1395,
  [SMALL_STATE(45)] = 1427,
  [SMALL_STATE(46)] = 1455,
  [SMALL_STATE(47)] = 1483,
  [SMALL_STATE(48)] = 1515,
  [SMALL_STATE(49)] = 1543,
  [SMALL_STATE(50)] = 1571,
  [SMALL_STATE(51)] = 1603,
  [SMALL_STATE(52)] = 1631,
  [SMALL_STATE(53)] = 1668,
  [SMALL_STATE(54)] = 1705,
  [SMALL_STATE(55)] = 1742,
  [SMALL_STATE(56)] = 1779,
  [SMALL_STATE(57)] = 1816,
  [SMALL_STATE(58)] = 1863,
  [SMALL_STATE(59)] = 1900,
  [SMALL_STATE(60)] = 1937,
  [SMALL_STATE(61)] = 1974,
  [SMALL_STATE(62)] = 2011,
  [SMALL_STATE(63)] = 2048,
  [SMALL_STATE(64)] = 2085,
  [SMALL_STATE(65)] = 2122,
  [SMALL_STATE(66)] = 2159,
  [SMALL_STATE(67)] = 2196,
  [SMALL_STATE(68)] = 2233,
  [SMALL_STATE(69)] = 2270,
  [SMALL_STATE(70)] = 2307,
  [SMALL_STATE(71)] = 2344,
  [SMALL_STATE(72)] = 2381,
  [SMALL_STATE(73)] = 2418,
  [SMALL_STATE(74)] = 2455,
  [SMALL_STATE(75)] = 2502,
  [SMALL_STATE(76)] = 2539,
  [SMALL_STATE(77)] = 2576,
  [SMALL_STATE(78)] = 2613,
  [SMALL_STATE(79)] = 2650,
  [SMALL_STATE(80)] = 2677,
  [SMALL_STATE(81)] = 2719,
  [SMALL_STATE(82)] = 2763,
  [SMALL_STATE(83)] = 2807,
  [SMALL_STATE(84)] = 2851,
  [SMALL_STATE(85)] = 2892,
  [SMALL_STATE(86)] = 2933,
  [SMALL_STATE(87)] = 2974,
  [SMALL_STATE(88)] = 3011,
  [SMALL_STATE(89)] = 3042,
  [SMALL_STATE(90)] = 3077,
  [SMALL_STATE(91)] = 3110,
  [SMALL_STATE(92)] = 3139,
  [SMALL_STATE(93)] = 3180,
  [SMALL_STATE(94)] = 3221,
  [SMALL_STATE(95)] = 3262,
  [SMALL_STATE(96)] = 3303,
  [SMALL_STATE(97)] = 3344,
  [SMALL_STATE(98)] = 3372,
  [SMALL_STATE(99)] = 3401,
  [SMALL_STATE(100)] = 3430,
  [SMALL_STATE(101)] = 3459,
  [SMALL_STATE(102)] = 3485,
  [SMALL_STATE(103)] = 3508,
  [SMALL_STATE(104)] = 3531,
  [SMALL_STATE(105)] = 3546,
  [SMALL_STATE(106)] = 3561,
  [SMALL_STATE(107)] = 3577,
  [SMALL_STATE(108)] = 3593,
  [SMALL_STATE(109)] = 3608,
  [SMALL_STATE(110)] = 3623,
  [SMALL_STATE(111)] = 3638,
  [SMALL_STATE(112)] = 3653,
  [SMALL_STATE(113)] = 3668,
  [SMALL_STATE(114)] = 3685,
  [SMALL_STATE(115)] = 3702,
  [SMALL_STATE(116)] = 3719,
  [SMALL_STATE(117)] = 3736,
  [SMALL_STATE(118)] = 3753,
  [SMALL_STATE(119)] = 3770,
  [SMALL_STATE(120)] = 3787,
  [SMALL_STATE(121)] = 3801,
  [SMALL_STATE(122)] = 3815,
  [SMALL_STATE(123)] = 3825,
  [SMALL_STATE(124)] = 3839,
  [SMALL_STATE(125)] = 3853,
  [SMALL_STATE(126)] = 3867,
  [SMALL_STATE(127)] = 3881,
  [SMALL_STATE(128)] = 3895,
  [SMALL_STATE(129)] = 3909,
  [SMALL_STATE(130)] = 3918,
  [SMALL_STATE(131)] = 3929,
  [SMALL_STATE(132)] = 3940,
  [SMALL_STATE(133)] = 3949,
  [SMALL_STATE(134)] = 3960,
  [SMALL_STATE(135)] = 3969,
  [SMALL_STATE(136)] = 3978,
  [SMALL_STATE(137)] = 3989,
  [SMALL_STATE(138)] = 3998,
  [SMALL_STATE(139)] = 4009,
  [SMALL_STATE(140)] = 4020,
  [SMALL_STATE(141)] = 4029,
  [SMALL_STATE(142)] = 4038,
  [SMALL_STATE(143)] = 4047,
  [SMALL_STATE(144)] = 4056,
  [SMALL_STATE(145)] = 4065,
  [SMALL_STATE(146)] = 4074,
  [SMALL_STATE(147)] = 4083,
  [SMALL_STATE(148)] = 4092,
  [SMALL_STATE(149)] = 4101,
  [SMALL_STATE(150)] = 4112,
  [SMALL_STATE(151)] = 4120,
  [SMALL_STATE(152)] = 4128,
  [SMALL_STATE(153)] = 4136,
  [SMALL_STATE(154)] = 4144,
  [SMALL_STATE(155)] = 4152,
  [SMALL_STATE(156)] = 4160,
  [SMALL_STATE(157)] = 4168,
  [SMALL_STATE(158)] = 4176,
  [SMALL_STATE(159)] = 4184,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(154),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(159),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(106),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(31),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(60),
  [31] = {.entry = {.count = 1, .reusable = false}}, SHIFT(103),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [39] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(10),
  [42] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(39),
  [45] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(159),
  [48] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(106),
  [51] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(59),
  [54] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(56),
  [57] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(5),
  [60] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2),
  [62] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(16),
  [65] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(31),
  [68] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(60),
  [71] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(103),
  [74] = {.entry = {.count = 1, .reusable = true}}, SHIFT(142),
  [76] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
  [78] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 2),
  [80] = {.entry = {.count = 1, .reusable = true}}, SHIFT(155),
  [82] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 2),
  [84] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 1),
  [86] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 1),
  [88] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_identifier, 3),
  [90] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_identifier, 3),
  [92] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_global_identifier_repeat1, 2),
  [94] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_global_identifier_repeat1, 2), SHIFT_REPEAT(155),
  [97] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_global_identifier_repeat1, 2),
  [99] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [101] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [103] = {.entry = {.count = 1, .reusable = false}}, SHIFT(27),
  [105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [107] = {.entry = {.count = 1, .reusable = false}}, SHIFT(97),
  [109] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [111] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expression, 1),
  [113] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expression, 1),
  [115] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [117] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expression, 3),
  [119] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expression, 3),
  [121] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 3),
  [123] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 3),
  [125] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 5),
  [127] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 5),
  [129] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_op, 4),
  [131] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_op, 4),
  [133] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call, 4),
  [135] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func_call, 4),
  [137] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_op, 2),
  [139] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_op, 2),
  [141] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_op, 3),
  [143] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_op, 3),
  [145] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [147] = {.entry = {.count = 1, .reusable = false}}, SHIFT(53),
  [149] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [151] = {.entry = {.count = 1, .reusable = false}}, SHIFT(64),
  [153] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [155] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [157] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [159] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 3),
  [161] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [163] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 2),
  [165] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [167] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 7),
  [169] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [171] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [173] = {.entry = {.count = 1, .reusable = false}}, SHIFT(69),
  [175] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_declaration, 4, .production_id = 7),
  [177] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if_statement, 3),
  [179] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 3),
  [181] = {.entry = {.count = 1, .reusable = false}}, SHIFT(110),
  [183] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 5, .production_id = 10),
  [185] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_declaration, 5, .production_id = 10),
  [187] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__maybe_global_identifier, 1),
  [189] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__type, 1),
  [191] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym__maybe_global_identifier, 1), REDUCE(sym__type, 1),
  [194] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__maybe_global_identifier, 1),
  [196] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [198] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__assign_left_side, 1),
  [200] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression_statement, 1),
  [202] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [204] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [206] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [208] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [210] = {.entry = {.count = 1, .reusable = true}}, SHIFT(151),
  [212] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_global_identifier_repeat1, 2), SHIFT_REPEAT(151),
  [215] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_for_statement, 5),
  [217] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_statement, 5),
  [219] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if_statement, 5),
  [221] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 5),
  [223] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__statement, 2),
  [225] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__statement, 2),
  [227] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [229] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [231] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [233] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [235] = {.entry = {.count = 1, .reusable = true}}, SHIFT(152),
  [237] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [239] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [241] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [243] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__assign_left_side, 2),
  [245] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [247] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [249] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [251] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [253] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [255] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [257] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [259] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [261] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [263] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [265] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [267] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [269] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [271] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [273] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [275] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [277] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [279] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [281] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [283] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [285] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_func_call_repeat1, 2),
  [287] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__assign_left_side_repeat2, 2),
  [289] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__assign_left_side_repeat2, 2),
  [291] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__assign_left_side_repeat2, 3),
  [293] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__assign_left_side_repeat2, 3),
  [295] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [297] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [299] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [301] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [303] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [305] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [307] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [309] = {.entry = {.count = 1, .reusable = false}}, SHIFT(62),
  [311] = {.entry = {.count = 1, .reusable = false}}, SHIFT(71),
  [313] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [315] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [317] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [319] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [321] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_range, 3),
  [323] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_decl_assign_statement, 3),
  [325] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__assign_left_side_repeat1, 2),
  [327] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__assign_left_side_repeat1, 2),
  [329] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__assign_left_side_repeat1, 2), SHIFT_REPEAT(97),
  [332] = {.entry = {.count = 1, .reusable = false}}, SHIFT(108),
  [334] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [336] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [338] = {.entry = {.count = 1, .reusable = false}}, SHIFT(107),
  [340] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 3, .production_id = 4),
  [342] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [344] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 2, .production_id = 2),
  [346] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [348] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [350] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_module_repeat1, 2), SHIFT_REPEAT(102),
  [353] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_module_repeat1, 2),
  [355] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [357] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [359] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [361] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(154),
  [364] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [366] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__assign_left_side, 1),
  [368] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_decl_statement, 1),
  [370] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [372] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__assign_left_side, 3),
  [374] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__assign_left_side_repeat2, 2), SHIFT_REPEAT(14),
  [377] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__assign_left_side, 2),
  [379] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_func_call_repeat1, 2), SHIFT_REPEAT(65),
  [382] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [384] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [386] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 5, .production_id = 1),
  [388] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [390] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [392] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [394] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 7, .production_id = 8),
  [396] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 8, .production_id = 12),
  [398] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [400] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 8, .production_id = 11),
  [402] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [404] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [406] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 7, .production_id = 9),
  [408] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 6, .production_id = 5),
  [410] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 7, .production_id = 6),
  [412] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type, 4),
  [414] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 6, .production_id = 3),
  [416] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module, 9, .production_id = 13),
  [418] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [420] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [422] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [424] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [426] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [428] = {.entry = {.count = 1, .reusable = true}}, SHIFT(153),
  [430] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [432] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [434] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [436] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [438] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
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
