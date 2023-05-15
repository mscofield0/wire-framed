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
			impl #impl_generics ::wire_framed::wire_framed_core::IntoFrame for #name #ty_generics #where_clause {
				fn extend_frame(_frame: &mut ::wire_framed::wire_framed_core::bytes::BytesMut) {
					Ok(Self)
				}

				fn size_hint(&self) -> usize {
					0
				}
			}
		};
	}

	let is_tuple_struct = data.fields.iter().next().unwrap().ident.is_none();
	if is_tuple_struct {
		let unnamed_fields = data.fields.iter().enumerate().map(|(i, _)| quote! { self.#i }).collect::<Vec<_>>();
		return quote! {
			impl #impl_generics ::wire_framed::wire_framed_core::IntoFrame for #name #ty_generics #where_clause {
				fn extend_frame(&self, frame: &mut ::wire_framed::wire_framed_core::bytes::BytesMut) {
					use ::wire_framed::wire_framed_core::bytes::BufMut;
					#(::wire_framed::wire_framed_core::IntoFrame::extend_frame(&#unnamed_fields, frame);)*
				}

				fn size_hint(&self) -> usize {
					[#(::wire_framed::wire_framed_core::IntoFrame::size_hint(&#unnamed_fields)),*].iter().sum()
				}
			}
		};
	}

	let field_names = data.fields.iter().map(|field| &field.ident).collect::<Vec<_>>();
	quote! {
		impl #impl_generics ::wire_framed::wire_framed_core::IntoFrame for #name #ty_generics #where_clause {
			fn extend_frame(&self, frame: &mut ::wire_framed::wire_framed_core::bytes::BytesMut) {
				use ::wire_framed::wire_framed_core::bytes::BufMut;
				#(::wire_framed::wire_framed_core::IntoFrame::extend_frame(&self.#field_names, frame);)*
			}
			
			fn size_hint(&self) -> usize {
				[#(::wire_framed::wire_framed_core::IntoFrame::size_hint(&self.#field_names)),*].iter().sum()
			}
		}
	}
}