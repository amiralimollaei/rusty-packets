extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{self, parse_macro_input, Data, Fields, Ident, Attribute};

#[proc_macro_derive(GenericPacket, attributes(discriminant_type))]
pub fn generic_packet_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;
    let r#gen = impl_generic_packet(&ast, name);
    TokenStream::from(r#gen)
}

fn impl_generic_packet(ast: &syn::DeriveInput, name: &Ident) -> proc_macro2::TokenStream {
    match &ast.data {
        Data::Enum(e) => {
            let (get_id_impl, get_name_impl) = generate_enum_extended_impl(e, &ast.attrs);
            quote! {
                impl GenericPacket for #name 
                where 
                    Self: Sized,
                {
                    fn get_id(&self) -> i32 {
                        #get_id_impl
                    }

                    fn get_name(&self) -> std::string::String {
                        #get_name_impl
                    }
                }
            }
        },
        _ => panic!("GenericPacket can only be derived on enums."),
    }

    
}

fn generate_enum_extended_impl(e: &syn::DataEnum, attrs: &[Attribute]) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let get_id_arms = e.variants.iter().enumerate().map(|(i, variant)| {
        let variant_name = &variant.ident;
        let discriminant = match &variant.discriminant {
            Some((_, exp)) => {
                exp.to_token_stream()
            },
            None => {
                let enumerator = i as i32;
                quote! { #enumerator }
            },
        };

        match &variant.fields {
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter()
                    .filter_map(|f| f.ident.as_ref())
                    .collect();
                quote! {
                    Self::#variant_name { #( #field_names ),* } => {
                        #discriminant
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let field_patterns: Vec<_> = (0..fields.unnamed.len())
                    .map(|i| quote::format_ident!("field{}", i))
                    .collect();
                quote! {
                    Self::#variant_name( #( #field_patterns ),* ) => {
                        #discriminant
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    Self::#variant_name => {
                        #discriminant
                    }
                }
            }
        }
    });

    let get_id_impl = quote! {
        match self {
            #( #get_id_arms )*
        }
    };

    let get_name_arms = e.variants.iter().enumerate().map(|(i, variant)| {
        let variant_name = &variant.ident;

        match &variant.fields {
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter()
                    .filter_map(|f| f.ident.as_ref())
                    .collect();
                quote! {
                    Self::#variant_name { #( #field_names ),* } => stringify!(#variant_name).to_string(),
                }
            }
            Fields::Unnamed(fields) => {
                let field_patterns: Vec<_> = (0..fields.unnamed.len())
                    .map(|i| quote::format_ident!("field{}", i))
                    .collect();
                quote! {
                    Self::#variant_name( #( #field_patterns ),* ) => stringify!(#variant_name).to_string(),
                }
            }
            Fields::Unit => {
                quote! {
                    Self::#variant_name => stringify!(#variant_name).to_string(),
                }
            }
        }
    });

    let get_name_impl = quote! {
        match self {
            #( #get_name_arms )*
        }
    };

    (get_id_impl, get_name_impl)
}