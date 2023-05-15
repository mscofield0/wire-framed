mod r#struct;
mod r#enum;
use r#struct::struct_impl;
use r#enum::enum_impl;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error, Data};

pub fn encoding_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens = match input.data {
        Data::Struct(ref data) => struct_impl(&input, data.clone()),
		Data::Enum(ref data) => enum_impl(&input, data.clone()),
        _ => return Error::new(input.ident.span(), "wire-framed does not support unions").into_compile_error().into(),
    };

	tokens.into()
}