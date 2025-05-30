use proc_macro::TokenStream;

use quote::{quote, quote_spanned};
use regex::Regex;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn kind(token_stream: TokenStream) -> TokenStream {
    let string_literal: LitStr = parse_macro_input!(token_stream);

    let requested_kind = string_literal.value();

    let language = tree_sitter_sus::language();
    let found_id = language.id_for_node_kind(&requested_kind, true);

    if found_id != 0 {
        quote! {
            #found_id
        }
    } else {
        quote_spanned!(
            string_literal.span() =>
            compile_error!("This is not a valid node kind in the SUS language")
        )
    }
    .into()
}

#[proc_macro]
pub fn kw(token_stream: TokenStream) -> TokenStream {
    let string_literal: LitStr = parse_macro_input!(token_stream);

    let requested_keyword = string_literal.value();

    let language = tree_sitter_sus::language();
    let found_id = language.id_for_node_kind(&requested_keyword, false);

    if found_id != 0 {
        quote! {
            #found_id
        }
    } else {
        quote_spanned!(
            string_literal.span() =>
            compile_error!("This is not a valid keyword in the SUS language")
        )
    }
    .into()
}

#[proc_macro]
pub fn field(token_stream: TokenStream) -> TokenStream {
    let string_literal: LitStr = parse_macro_input!(token_stream);

    let requested_keyword = string_literal.value();

    let language = tree_sitter_sus::language();
    let found_id = language.field_id_for_name(&requested_keyword);

    if let Some(found_id) = found_id {
        let id_number: u16 = found_id.into();
        quote! {
            std::num::NonZeroU16::new(#id_number).unwrap()
        }
    } else {
        quote_spanned!(
            string_literal.span() =>
            compile_error!("This is not a valid field in the SUS language")
        )
    }
    .into()
}

#[proc_macro]
pub fn get_builtin_type(token_stream: TokenStream) -> TokenStream {
    let string_literal: LitStr = parse_macro_input!(token_stream);

    let object_name = string_literal.value();

    let core_file_text = std::fs::read_to_string("std/core.sus").unwrap();

    let re = Regex::new(r"__builtin__\s+struct\s+([a-zA-Z0-9_]+)\s*(?:#\(.*\))?\s*\{").unwrap();

    for (idx, c) in re.captures_iter(&core_file_text).enumerate() {
        let (_full, [found_name]) = c.extract();
        if found_name == object_name {
            return quote! {
                crate::alloc::UUID::<crate::prelude::TypeUUIDMarker>(#idx, std::marker::PhantomData)
            }
            .into();
        }
    }

    quote_spanned!(
        string_literal.span() =>
        compile_error!("Unknown builtin type was not found in std/core.sus")
    )
    .into()
}

#[proc_macro]
pub fn get_builtin_const(token_stream: TokenStream) -> TokenStream {
    let string_literal: LitStr = parse_macro_input!(token_stream);

    let object_name = string_literal.value();

    let core_file_text = std::fs::read_to_string("std/core.sus").unwrap();

    let re = Regex::new(r"__builtin__\s+const\s+.+\s+([a-zA-Z0-9_]+)\s*(?:#\(.*\))?\s*\{").unwrap();

    for (idx, c) in re.captures_iter(&core_file_text).enumerate() {
        let (_full, [found_name]) = c.extract();
        if found_name == object_name {
            return quote! {
                crate::alloc::UUID::<crate::prelude::ConstantUUIDMarker>(#idx, std::marker::PhantomData)
            }
            .into();
        }
    }

    quote_spanned!(
        string_literal.span() =>
        compile_error!("Unknown builtin const was not found in std/core.sus")
    )
    .into()
}

#[proc_macro]
pub fn __debug_breakpoint(_input: TokenStream) -> TokenStream {
    let expanded = quote! {
        if crate::debug::debugging_enabled() {
            #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
            unsafe {
                core::arch::asm!("int3");
            }

            #[cfg(all(target_arch = "x86_64", target_os = "windows"))]
            unsafe {
                core::arch::asm!("int3");
            }

            #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
            unsafe {
                core::arch::asm!("brk #0");
            }

            #[cfg(all(target_arch = "aarch64", target_os = "windows"))]
            unsafe {
                core::arch::asm!("brk #0");
            }
        }
    };
    TokenStream::from(expanded)
}
