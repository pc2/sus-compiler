
function sepSeq1(rule, sepChar) {
  return seq(rule, repeat(seq(sepChar, rule)))
}

function sepSeq(rule, sepChar) {
  return optional(sepSeq1(rule, sepChar))
}

const PREC = {
    compare : 2,
    and: 3,
    or: 4,
    xor: 5,
    additive: 6,
    multiplicative: 7,
    unary: 8,
    namespace_path : 9
}

module.exports = grammar({
    name: 'sus',

    rules: {
        source_file: $ => repeat($.module),

        interface_ports : $ => seq(
            ':',
            field('inputs', sepSeq($.declaration, ',')),
            optional(seq(
                '->',
                field('outputs', sepSeq($.declaration, ','))
            ))
        ),
        module: $ => seq(
            'module',
            field('name', $.identifier),
            optional(field('interface_ports', $.interface_ports)),
            field('block', $.block)
        ),
        identifier: $ => /[\p{L}_][\p{L}_\d]*/,
        number: $ => /\d[\d_]*/,

        word: $=> $.identifier,

        global_identifier: $ => prec.left(PREC.namespace_path, seq(
            optional('::'),
            sepSeq1($.identifier, '::')
        )),

        _maybe_global_identifier: $ => choice(
            prec(1, $.identifier),
            prec(0, $.global_identifier)
        ),
        
        array_type: $ => seq(
            field('arr', $._type),
            field('arr_idx', $.array_bracket_expression)
        ),
        _type: $ => choice(
            $.global_identifier,
            $.array_type
        ),

        latency_specifier : $ => seq(seq(
            '\'',
            field('content', $._expression)
        )),

        declaration: $ => seq(
            optional(field('declaration_modifiers', choice(
                'state',
                'gen'
            ))),
            field('type', $._type),
            field('name', $.identifier),
            optional(field('latency_specifier', $.latency_specifier))
        ),

        unary_op: $ => prec(PREC.unary, seq(
            field('operator', choice('+', '-', '*', '!', '|', '&', '^')),
            field('right', $._expression)
        )),

        binary_op: $ => {
            const TABLE = [
                [PREC.compare, choice('==', '!=', '<', '<=', '>', '>=')],
                [PREC.and, '&'],
                [PREC.or, '|'],
                [PREC.xor, '^'],
                [PREC.additive, choice('+', '-')],
                [PREC.multiplicative, choice('*', '/', '%')],
            ];

            return choice(...TABLE.map(([precedence, operator]) => prec.left(precedence, seq(
                field('left', $._expression),
                field('operator', operator),
                field('right', $._expression)
            ))));
        },

        array_op: $ => seq(
            field('arr', $._expression),
            field('arr_idx', $.array_bracket_expression)
        ),

        func_call: $ => seq(
            field('name', $._maybe_global_identifier),
            '(',
            sepSeq(field('argument', $._expression), ','),
            ')'
        ),

        parenthesis_expression: $ => seq(
            '(',
            field('content', $._expression),
            ')'
        ),

        array_bracket_expression: $ => seq(
            '[',
            field('content', $._expression),
            ']'
        ),

        _expression: $ => choice(
            $._maybe_global_identifier,
            $.array_op,
            $.number,
            $.parenthesis_expression,
            $.unary_op,
            $.binary_op,
            $.func_call
        ),

        range: $ => seq(
            field('from', $._expression),
            ':',
            field('to', $._expression),
        ),
        
        block: $ => seq(
            '{',
            repeat($._statement),
            '}'
        ),
        
        _assign_left_side: $ => sepSeq1(seq(
            choice(
                repeat('reg'),
                'initial'
            ),
            choice(
                $._expression,
                $.declaration
            )
        ), ','),
        decl_assign_statement: $ => seq(
            field('assign_to', $._assign_left_side),
            '=',
            field('assign_value', $._expression),
        ),
        decl_statement: $ => $.declaration,
        expression_statement: $ => $._expression,

        if_statement: $ => seq(
            'if',
            field('condition', $._assign_left_side),
            field('then_block', $.block),
            optional(seq(
                'else',
                field('else_block', choice(
                    $.block,
                    $.if_statement
                ))
            ))
        ),
        for_statement: $ => seq(
            'for',
            field('for_decl', $.declaration),
            'in',
            field('for_range', $.range),
            field('block', $.block)
        ),
        _statement: $ => choice(
            $.block,
            seq($.decl_assign_statement, ';'),
            seq($.decl_statement, ';'),
            seq($.expression_statement, ';'),
            $.if_statement,
            $.for_statement
        ),
    },

    conflicts: $ => [
        [$._maybe_global_identifier, $._type] // Just because LR(1) is too weak to resolve 'ident[] a' vs 'type_name[]'. Tree sitter resolves this itself with more expensive GLR. NOT a precedence relation. 
    ],

    extras: $ => [
        /\s+/,
        /\/\/[^\n]*\n/, // Single line comment
        /\/\*[^\*]*\*+([^\/\*][^\*]*\*+)*\// // Multi line comment
    ]
});
