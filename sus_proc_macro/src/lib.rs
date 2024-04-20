

use proc_macro::TokenStream;

use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn kind(token_stream : TokenStream) -> TokenStream {
    let string_literal : LitStr = parse_macro_input!(token_stream);
    
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
    }.into()
}

#[proc_macro]
pub fn kw(token_stream : TokenStream) -> TokenStream {
    let string_literal : LitStr = parse_macro_input!(token_stream);
    
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
    }.into()
}

#[proc_macro]
pub fn field(token_stream : TokenStream) -> TokenStream {
    let string_literal : LitStr = parse_macro_input!(token_stream);
    
    let requested_keyword = string_literal.value();

    let language = tree_sitter_sus::language();
    let found_id = language.field_id_for_name(&requested_keyword);

    if let Some(found_id) = found_id {
        let id_number : u16 = found_id.into();
        quote! {
            std::num::NonZeroU16::new(#id_number).unwrap()
        }
    } else {
        quote_spanned!(
            string_literal.span() => 
            compile_error!("This is not a valid field in the SUS language")
        )
    }.into()
}
