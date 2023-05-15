mod encoding;
mod decoding;

use proc_macro::TokenStream;

/// Implements the `IntoFrame` traits for the type.
#[proc_macro_derive(Encoding)]
pub fn encoding(input: TokenStream) -> TokenStream {
    encoding::encoding_impl(input)
}

/// Implements the `FromFrame` traits for the type.
#[proc_macro_derive(Decoding)]
pub fn decoding(input: TokenStream) -> TokenStream {
    decoding::decoding_impl(input)
}
