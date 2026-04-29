use std::{fs::File, io::Read};

use proc_macro::TokenStream;

use quote::{quote, quote_spanned};
use regex::Regex;
use syn::{LitStr, parse_macro_input};

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

fn get_standard_library_text() -> String {
    /// These must be in the same order as [sus-compiler/src/compiler_top.rs]
    const BUILTIN_FILES: &[&str] = &[
        "std/core.sus",
        "std/array.sus",
        "std/math.sus",
        "std/conversion.sus",
    ];

    let mut total_text = String::new();
    for path in BUILTIN_FILES {
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut total_text).unwrap();
    }
    total_text
}

#[proc_macro]
pub fn get_builtin_type(token_stream: TokenStream) -> TokenStream {
    let string_literal: LitStr = parse_macro_input!(token_stream);

    let object_name = string_literal.value();

    let stl_text = get_standard_library_text();

    let re = Regex::new(r"__builtin__\s+struct\s+([a-zA-Z0-9_]+)\s*(?:#\(.*\))?\s*\{").unwrap();

    if let Some(idx) = re.captures_iter(&stl_text).position(|c| {
        let (_full, [found_name]) = c.extract();
        found_name == object_name
    }) {
        quote! {
            crate::alloc::UUID::<crate::prelude::TypeUUIDMarker>(#idx, std::marker::PhantomData)
        }
        .into()
    } else {
        quote_spanned!(
            string_literal.span() =>
            compile_error!("Unknown builtin struct was not found in the standard library")
        )
        .into()
    }
}

#[proc_macro]
pub fn get_builtin_const(token_stream: TokenStream) -> TokenStream {
    let string_literal: LitStr = parse_macro_input!(token_stream);

    let object_name = string_literal.value();

    let stl_text = get_standard_library_text();

    let re = Regex::new(r"__builtin__\s+const\s+.+\s+([a-zA-Z0-9_]+)\s*(?:#\(.*\))?\s*\{").unwrap();

    if let Some(idx) = re.captures_iter(&stl_text).position(|c| {
        let (_full, [found_name]) = c.extract();
        found_name == object_name
    }) {
        quote! {
            crate::alloc::UUID::<crate::prelude::ConstantUUIDMarker>(#idx, std::marker::PhantomData)
        }
        .into()
    } else {
        quote_spanned!(
            string_literal.span() =>
            compile_error!("Unknown builtin const was not found in the standard library")
        )
        .into()
    }
}

/// This could be a macro_rules!, but then rust insists on binding the line number to the macro.
/// By wrapping it in a proc_macro rust can only assign the `__debug_breakpoint!` usage location for the lldb breakpoint
#[proc_macro]
pub fn __debug_breakpoint(_input: TokenStream) -> TokenStream {
    quote! {
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
    }
    .into()
}

/// This could be a macro_rules!, but then rust insists on binding the line number to the macro.
/// By wrapping it in a proc_macro rust can only assign the `__debug_breakpoint_if!` usage location for the lldb breakpoint
#[proc_macro]
pub fn __debug_breakpoint_if(input: TokenStream) -> TokenStream {
    let expr: syn::Expr = syn::parse_macro_input!(input as syn::Expr);
    quote! {
        if crate::debug::debugging_enabled() && (#expr) {
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
    }
    .into()
}
