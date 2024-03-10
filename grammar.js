
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

        module: $ => seq(
            'module',
            field('name', $.identifier),
            optional(seq(':',
            field('inputs', sepSeq($.declaration, ',')),
            optional(seq('->',
            field('outputs', sepSeq($.declaration, ','))))))
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
                operator,
                field('right', $._expression)
            ))));
        },

        array_op: $ => seq(
            field('arr', $._expression),
            '[',
            field('arr_idx', $._expression),
            ']'
        ),

        func_call: $ => seq(
            field('module_name', $._maybe_global_identifier),
            '(',
            sepSeq(field('argument', $._expression), ','),
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
