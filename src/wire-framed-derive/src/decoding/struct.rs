use proc_macro2::TokenStream as TokenStream2;
use syn::{DataStruct, DeriveInput};
use quote::quote;

pub fn struct_impl(input: &DeriveInput, data: DataStruct) -> TokenStream2 {
    // Common vars for building the final output
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let is_unit_struct = data.fields.is_empty();
	if is_unit_struct {
		return quote! {
			impl #impl_generics ::wire_framed::wire_framed_core::FromFrame for #name #ty_generics #where_clause {
				type Error = ::std::io::Error;

				fn parse_frame(_frame: &mut ::wire_framed::wire_framed_core::bytes::Bytes) -> ::std::result::Result<Self, Self::Error> {
					Ok(Self)
				}
			}
		};
	}

	let is_tuple_struct = data.fields.iter().next().unwrap().ident.is_none();
	if is_tuple_struct {
		let empty_iters = data.fields.iter().map(|_| quote! {}).collect::<Vec<_>>();
		return quote! {
			impl #impl_generics ::wire_framed::wire_framed_core::FromFrame for #name #ty_generics #where_clause {
				type Error = ::std::io::Error;

				fn parse_frame(frame: &mut ::wire_framed::wire_framed_core::bytes::Bytes) -> ::std::result::Result<Self, Self::Error> {
					use ::wire_framed::wire_framed_core::bytes::Buf;
					Ok(Self(#(#empty_iters ::wire_framed::wire_framed_core::FromFrame::parse_frame(frame)?),*))
				}
			}
		};
	}

	let field_names = data.fields.iter().map(|field| &field.ident).collect::<Vec<_>>();
	quote! {
		impl #impl_generics ::wire_framed::wire_framed_core::FromFrame for #name #ty_generics #where_clause {
			type Error = ::std::io::Error;

			fn parse_frame(frame: &mut ::wire_framed::wire_framed_core::bytes::Bytes) -> ::std::result::Result<Self, Self::Error> {
				use ::wire_framed::wire_framed_core::bytes::Buf;
				Ok(Self {
					#(#field_names: ::wire_framed::wire_framed_core::FromFrame::parse_frame(frame).map_err(|err| ::std::io::Error::new(::std::io::ErrorKind::InvalidInput, format!("expected '{}': {}", stringify!(#field_names), err)))?,)*
				})
			}
		}
	}
}