use proc_macro2::{TokenStream as TokenStream2};
use syn::{DeriveInput, DataEnum, Error};
use quote::quote;

pub fn enum_impl(input: &DeriveInput, data: DataEnum) -> TokenStream2 {
    // Common vars for building the final output
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	if data.variants.len() > 255 {
		return Error::new(name.span(), "Cannot derive `Encoding` for enum with more than 255 variants").into_compile_error().into();
	}

	let kind_values = data.variants.iter().enumerate().map(|(i, _)| i as u8).collect::<Vec<_>>();

	let frame_variant = data.variants.iter().map(|variant| {
		let variant_name = &variant.ident;
		let is_unit_struct = variant.fields.is_empty();
		if is_unit_struct {
			quote! { Self::#variant_name }
		} else {
			let is_tuple_struct = variant.fields.iter().next().unwrap().ident.is_none();
			if is_tuple_struct {
				let empty_iters = variant.fields.iter().map(|_| quote! {}).collect::<Vec<_>>();
				quote! { Self::#variant_name(#(#empty_iters ::wire_framed::wire_framed_core::FromFrame::parse_frame(frame)?,)*) }
			} else {
				let field_names = variant.fields.iter().map(|field| &field.ident).collect::<Vec<_>>();
				quote! {
					Self::#variant_name {
						#(#field_names: ::wire_framed::wire_framed_core::FromFrame::parse_frame(frame).map_err(|err| ::std::io::Error::new(::std::io::ErrorKind::InvalidInput, format!("expected '{}': {}", stringify!(#field_names), err)))?,)*
					}
				}
			}
		}
	}).collect::<Vec<_>>();

	quote! {
		impl #impl_generics ::wire_framed::wire_framed_core::FromFrame for #name #ty_generics #where_clause {
			type Error = ::std::io::Error;

			fn parse_frame(frame: &mut ::wire_framed::wire_framed_core::bytes::Bytes) -> ::std::result::Result<Self, Self::Error> {
				use ::wire_framed::wire_framed_core::bytes::Buf;
				let kind: u8 = ::wire_framed::wire_framed_core::FromFrame::parse_frame(frame).map_err(|_| ::std::io::Error::new(::std::io::ErrorKind::InvalidInput, format!("expected '{}' kind", stringify!(#name))))?;
				let value = match kind {
					#(#kind_values => #frame_variant,)*
					_ => return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidInput, format!("invalid '{}' kind", stringify!(#name)))),
				};

				Ok(value)
			}
		}
	}
}