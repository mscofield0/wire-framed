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

	let frame_variant = data.variants.iter().enumerate().map(|(kind, variant)| {
		let kind = kind as u8;
		let variant_name = &variant.ident;
		let is_unit_struct = variant.fields.is_empty();
		if is_unit_struct {
			quote! { 
				Self::#variant_name => {
					frame.put_u8(#kind);
				}
			}
		} else {
			let is_tuple_struct = variant.fields.iter().next().unwrap().ident.is_none();
			if is_tuple_struct {
				let field_names = variant.fields.iter().enumerate().map(|(i, _)| quote::format_ident!("_{}", i)).collect::<Vec<_>>();

				quote! {
					Self::#variant_name(#(#field_names),*) => {
						frame.put_u8(#kind);
						#(#field_names.extend_frame(frame);)*
					}
				}
			} else {
				let field_names = variant.fields.iter().map(|field| &field.ident).collect::<Vec<_>>();
				quote! {
					Self::#variant_name { #(#field_names),* } => {
						frame.put_u8(#kind);
						#(#field_names.extend_frame(frame);)*
					}
				}
			}
		}
	}).collect::<Vec<_>>();

	quote! {
		impl #impl_generics ::wire_framed::wire_framed_core::IntoFrame for #name #ty_generics #where_clause {
			fn extend_frame(&self, frame: &mut ::wire_framed::wire_framed_core::bytes::BytesMut) {
				use ::wire_framed::wire_framed_core::bytes::BufMut;
				match self {
					#(#frame_variant),*
				}
			}
		}
	}
}