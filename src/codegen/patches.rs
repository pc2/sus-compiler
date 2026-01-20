//! This file patches various simulator bugs as shown in [KNOWN_SIMULATOR_BUGS.md](../../KNOWN_SIMULATOR_BUGS.md)

use std::fmt::Display;

use crate::{to_string::FmtWrapper, typing::concrete_type::ConcreteType, value::Value};

pub fn patch_combinatorial_write_one_bit_dont_care(
    is_state: &Option<Value>,
    name: impl Display,
    w_typ: &ConcreteType,
) -> impl Display {
    FmtWrapper(move |f| {
        if is_state.is_none() && w_typ.sizeof() == ibig::ubig!(1) {
            writeln!(
                f,
                "\t// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care"
            )?;

            writeln!(f, "\t{name} = {name};")?;
        }
        Ok(())
    })
}

pub fn patch_empty_modules_should_have_content(module_content: &mut String) {
    if module_content.lines().all(|l| {
        let l = l.trim();
        l.starts_with("//") || l.is_empty()
    }) {
        use std::fmt::Write;
        writeln!(
            module_content,
            "// PATCH XRT 2.16 over-zealous empty module DRC\ninitial begin end"
        )
        .unwrap();
    }
}
