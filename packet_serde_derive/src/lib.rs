extern crate proc_macro;
use std::any::Any;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{self, parse_macro_input, Data, Fields, Ident, Attribute};

#[proc_macro_derive(PacketSerde, attributes(discriminant_type))]
pub fn packet_serde_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;
    let r#gen = impl_packet_serde(&ast, name);
    TokenStream::from(r#gen)
}

fn impl_packet_serde(ast: &syn::DeriveInput, name: &Ident) -> proc_macro2::TokenStream {
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    let mut packet_serde_where_clause = where_clause.cloned().unwrap_or_else(|| syn::parse_quote! { where });
    let mut writable_where_clause = where_clause.cloned().unwrap_or_else(|| syn::parse_quote! { where });
    let mut readable_where_clause = where_clause.cloned().unwrap_or_else(|| syn::parse_quote! { where });
    
    for param in generics.type_params() {
        let type_ident = &param.ident;
        packet_serde_where_clause.predicates.push(syn::parse_quote! { #type_ident: PacketSerde });
        writable_where_clause.predicates.push(syn::parse_quote! { #type_ident: PacketWritable });
        readable_where_clause.predicates.push(syn::parse_quote! { #type_ident: PacketReadable });
    }
    
    let (write_impl, read_impl) = match &ast.data {
        Data::Struct(s) => generate_struct_impl(s),
        Data::Enum(e) => generate_enum_impl(e, &ast.attrs),
        _ => panic!("PacketSerde can only be derived on structs and enums."),
    };

    quote! {
        impl #impl_generics PacketSerde for #name #ty_generics #packet_serde_where_clause {}
        
        impl #impl_generics PacketWritable for #name #ty_generics #writable_where_clause {
            fn write(&self, stream: &mut impl std::io::Write) {
                #write_impl
            }
        }
        
        impl #impl_generics PacketReadable for #name #ty_generics #readable_where_clause {
            fn read(stream: &mut impl std::io::Read) -> Self {
                #read_impl
            }
        }
    }
}

fn generate_struct_impl(s: &syn::DataStruct) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    match &s.fields {
        Fields::Named(fields) => {
            let write_calls = fields.named.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                quote! { <_ as PacketWritable>::write(&self.#field_name, stream); }
            });
            let read_fields = fields.named.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                let field_type = &f.ty;
                quote! { #field_name: <#field_type as PacketReadable>::read(stream) }
            });
            (
                quote! { #( #write_calls )* },
                quote! { Self { #( #read_fields ),* } }
            )
        }
        Fields::Unnamed(fields) => {
            let write_calls = fields.unnamed.iter().enumerate().map(|(i, _)| {
                let index = syn::Index::from(i);
                quote! { <_ as PacketWritable>::write(&self.#index, stream); }
            });
            let read_fields = fields.unnamed.iter().map(|f| {
                let field_type = &f.ty;
                quote! { <#field_type as PacketReadable>::read(stream) }
            });
            (
                quote! { #( #write_calls )* },
                quote! { Self( #( #read_fields ),* ) }
            )
        }
        Fields::Unit => (quote! {}, quote! { Self }),
    }
}

fn get_discriminant_type(attrs: &[Attribute]) -> proc_macro2::TokenStream {
    for attr in attrs {
        if attr.path().is_ident("discriminant_type") {
            // Parse the tokens inside the attribute
            let tokens = attr.parse_args::<proc_macro2::TokenStream>().ok();
            if let Some(tokens) = tokens {
                return tokens;
            }
        }
    }
    // Default to VarInt if no discriminant_type attribute is found
    quote! { types::VarInt }
}

fn generate_enum_impl(e: &syn::DataEnum, attrs: &[Attribute]) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let discriminant_type = get_discriminant_type(attrs);
    
    let write_arms = e.variants.iter().enumerate().map(|(i, variant)| {
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
                let write_calls = field_names.iter().map(|name| {
                    quote! { <_ as PacketWritable>::write(#name, stream); }
                });
                quote! {
                    Self::#variant_name { #( #field_names ),* } => {
                        <#discriminant_type as PacketWritable>::write(&<#discriminant_type>::from(#discriminant), stream);
                        #( #write_calls )*
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let field_patterns: Vec<_> = (0..fields.unnamed.len())
                    .map(|i| quote::format_ident!("field{}", i))
                    .collect();
                let write_calls = field_patterns.iter().map(|name| {
                    quote! { <_ as PacketWritable>::write(#name, stream); }
                });
                quote! {
                    Self::#variant_name( #( #field_patterns ),* ) => {
                        <#discriminant_type as PacketWritable>::write(&<#discriminant_type>::from(#discriminant), stream);
                        #( #write_calls )*
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    Self::#variant_name => {
                        <#discriminant_type as PacketWritable>::write(&<#discriminant_type>::from(#discriminant), stream);
                    }
                }
            }
        }
    });
    
    let read_arms = e.variants.iter().enumerate().map(|(i, variant)| {
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
                let field_reads = fields.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let field_type = &f.ty;
                    quote! { #field_name: <#field_type as PacketReadable>::read(stream) }
                });
                quote! {
                    #discriminant => Self::#variant_name { #( #field_reads ),* },
                }
            }
            Fields::Unnamed(fields) => {
                let field_reads = fields.unnamed.iter().map(|f| {
                    let field_type = &f.ty;
                    quote! { <#field_type as PacketReadable>::read(stream) }
                });
                quote! {
                    #discriminant => Self::#variant_name( #( #field_reads ),* ),
                }
            }
            Fields::Unit => {
                quote! {
                    #discriminant => Self::#variant_name,
                }
            }
        }
    });
    
    let write_impl = quote! {
        match self {
            #( #write_arms )*
        }
    };
    
    let read_impl = quote! {
        let discriminant = <#discriminant_type as PacketReadable>::read(stream);
        match discriminant.into() {
            #( #read_arms )*
            _ => panic!("Invalid enum discriminant")
        }
    };
    
    (write_impl, read_impl)
}