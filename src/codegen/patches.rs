//! This file patches various simulator bugs as shown in [KNOWN_SIMULATOR_BUGS.md](../../KNOWN_SIMULATOR_BUGS.md)

use std::fmt::Display;

use crate::{to_string::FmtWrapper, typing::concrete_type::ConcreteType, value::Value};

pub fn patch_combinatorial_write_one_bit_dont_care(
    is_state: &Option<Value>,
    name: &str,
    w_typ: &ConcreteType,
) -> impl Display {
    FmtWrapper(move |f| {
        if is_state.is_none() && w_typ.sizeof() == Some(ibig::ibig!(1)) {
            writeln!(
                f,
                "\t// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care"
            )?;

            writeln!(f, "\t{name} = {name};")?;
        }
        Ok(())
    })
}
