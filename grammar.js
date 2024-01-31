
function commaSep1(rule) {
  return seq(rule, repeat(seq(',', rule)))
}

function commaSep(rule) {
  return optional(commaSep1(rule))
}

module.exports = grammar({
    name: 'SUS',

    rules: {
        // TODO: add the actual grammar rules
        source_file: $ => repeat($.module),

        module: $ => seq(
            'module',
            $.identifier,
            ':',
            commaSep($.decl),
            "->",
            commaSep($.decl),
            $.block
        ),
        identifier: $ => /[\p{L}_][\p{L}_\d]*/,
        number: $ => /\d[\d_]*/,

        type: $ => choice(
            $.identifier,
            seq(
                $.type,
                '[',
                $.expr,
                ']'
            )
        ),

        decl: $ => seq(
            $.type,
            $.identifier
        ),

        expr: $ => choice(
            $.identifier,
            $.number,
            seq($.expr, "[", $.expr, "]")
        ),
        
        block: $ => seq(
            "{",
            repeat($.statement),
            "}"
        ),

        statement: $ => choice(
            $.block,
            seq($.expr, "=", $.expr, ";")
        )
    },

    // extras: $ => ["\s+"]
});
