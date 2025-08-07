extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, Data, Fields, Ident};

/// This is the function the compiler will call when it sees `#[derive(MinecraftType)]`.
#[proc_macro_derive(MinecraftType)]
pub fn minecraft_type_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    // `parse_macro_input!` is a helper from `syn` that handles errors for us.
    let ast = parse_macro_input!(input as syn::DeriveInput);

    // Get the name of the struct we are deriving the trait for.
    let name = &ast.ident;

    // Call a helper function to generate the implementation code.
    // This keeps our logic organized.
    let _gen = impl_minecraft_type_for_struct(&ast, name);

    // Return the generated code as a TokenStream.
    TokenStream::from(_gen)
}

/// Generates the `impl` blocks for MinecraftType, PacketWritable, and PacketReadable.
fn impl_minecraft_type_for_struct(ast: &syn::DeriveInput, name: &Ident) -> proc_macro2::TokenStream {
    // Get the generics from the input struct definition.
    let generics = &ast.generics;

    // `split_for_impl` is a powerful syn helper. It gives us three parts:
    // - `impl_generics`: The part that goes after `impl` -> `<'a, T>`
    // - `ty_generics`: The part that goes after the struct name -> `<'a, T>`
    // - `where_clause`: The original where clause on the struct, if any.
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Create new `where` clauses with our required trait bounds.
    // We start with the original where clause and add our bounds to it.
    let mut minecraft_type_where_clause = where_clause.cloned().unwrap_or_else(|| syn::parse_quote! { where });
    let mut writable_where_clause = where_clause.cloned().unwrap_or_else(|| syn::parse_quote! { where });
    let mut readable_where_clause = where_clause.cloned().unwrap_or_else(|| syn::parse_quote! { where });

    // For every generic type parameter on the struct (e.g., for `T` in `<T>`),
    // add the required trait bound.
    for param in generics.type_params() {
        let type_ident = &param.ident;
        minecraft_type_where_clause.predicates.push(syn::parse_quote! { #type_ident: MinecraftType });
        writable_where_clause.predicates.push(syn::parse_quote! { #type_ident: PacketWritable });
        readable_where_clause.predicates.push(syn::parse_quote! { #type_ident: PacketReadable });
    }

    // Generate the read/write logic
    let (write_impl, read_impl) = match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => {
                let write_calls = fields.named.iter().map(|f| { // ... same as before
                    let field_name = f.ident.as_ref().unwrap();
                    quote! { self.#field_name.write(stream); }
                });
                let read_fields = fields.named.iter().map(|f| { // ... same as before
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
                // ... same logic for tuple structs as before
                let write_calls = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    let index = syn::Index::from(i);
                    quote! { self.#index.write(stream); }
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
            Fields::Unit => { (quote! {}, quote! { Self }) }
        },
        _ => panic!("MinecraftType can only be derived on structs."),
    };


    // 4. Assemble the final code, now including the generic parameters and where clauses.
    quote! {
        // Note the use of #impl_generics, #ty_generics, and #where_clause
        impl #impl_generics MinecraftType for #name #ty_generics #minecraft_type_where_clause {}

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