//! This file patches various simulator bugs as shown in [KNOWN_SIMULATOR_BUGS.md](../../KNOWN_SIMULATOR_BUGS.md)

use std::fmt::Display;
use std::fmt::from_fn;

use crate::{typing::concrete_type::ConcreteType, value::Value};

pub fn patch_combinatorial_write_one_bit_dont_care(
    is_state: &Option<Value>,
    name: impl Display,
    w_typ: &ConcreteType,
) -> impl Display {
    from_fn(move |f| {
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

/// Not a simulator bug, but Vivado-specific: Vivado synthesis turns chains of registers into SRL shift-register LUTs,
/// which are slow and can't be retimed or spread across SLRs. Latency registers the user explicitly asked for with `reg`
/// are marked so they stay real registers. Balancing registers inserted by latency counting are left alone.
pub fn patch_explicit_reg_no_srl(is_explicit_reg: bool) -> &'static str {
    if is_explicit_reg {
        "(* shreg_extract = \"no\" *) "
    } else {
        ""
    }
}
