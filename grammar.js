
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
        // TODO: add the actual grammar rules
        source_file: $ => repeat($.module),

        module: $ => seq(
            'module',
            field('name', $.identifier),
            ':',
            field('inputs', sepSeq($.declaration, ',')),
            '->',
            field('outputs', sepSeq($.declaration, ',')),
            $.block
        ),
        identifier: $ => /[\p{L}_][\p{L}_\d]*/,
        number: $ => /\d[\d_]*/,

        global_identifier: $ => prec.left(PREC.namespace_path, seq(
            optional('::'),
            sepSeq1($.identifier, '::')
        )),

        _maybe_global_identifier: $ => choice(
            prec(1, $.identifier),
            prec(0, $.global_identifier)
        ),
        
        array_type: $ => seq(
            $._type,
            '[',
            $._expression,
            ']'
        ),
        _type: $ => choice(
            $.global_identifier,
            $.array_type
        ),

        declaration: $ => seq(
            optional(choice(
                'state',
                'gen'
            )),
            field('type', $._type),
            field('name', $.identifier),
            optional(seq(
                '\'',
                field('latency_spec', $._expression)
            ))
        ),

        unary_op: $ => prec(PREC.unary, seq(
            choice('+', '-', '*', '!', '|', '&', '^'),
            $._expression
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
                $._expression,
                operator,
                $._expression
            ))));
        },

        array_op: $ => seq(
            $._expression,
            '[',
            $._expression,
            ']'
        ),

        func_call: $ => seq(
            $._maybe_global_identifier,
            '(',
            sepSeq($._expression, ','),
            ')'
        ),

        _expression: $ => choice(
            $._maybe_global_identifier,
            $.array_op,
            $.number,
            seq('(', $._expression, ')'),
            $.unary_op,
            $.binary_op,
            $.func_call
        ),

        range: $ => seq(
            $._expression,
            '..',
            $._expression
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
            $._assign_left_side,
            '=',
            $._expression
        ),
        decl_statement: $ => $.declaration,
        expression_statement: $ => $._expression,

        if_statement: $ => seq(
            'if',
            $._expression,
            $.block,
            optional(seq(
                'else',
                choice(
                    $.block,
                    $.if_statement
                )
            ))
        ),
        for_statement: $ => seq(
            'for',
            $.declaration,
            'in',
            $.range,
            $.block
        ),
        _statement: $ => choice(
            $.block,
            seq($.decl_assign_statement, ';'),
            seq($.decl_statement, ';'),
            seq($.expression_statement, ';'),
            $.if_statement,
            $.for_statement
        ),


        single_line_comment: $ => /\/\/[^\n]*\n/,
        multi_line_comment: $ => /\/\*[^\*]*\*+([^\/\*][^\*]*\*+)*\//,
    },

    conflicts: $ => [
        [$._maybe_global_identifier, $._type]
    ],

    extras: $ => [
        /\s+/,
        $.single_line_comment,
        $.multi_line_comment
    ]
});
