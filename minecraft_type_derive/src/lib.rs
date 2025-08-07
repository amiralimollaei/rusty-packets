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
    // This part is the main change. We'll handle different kinds of structs.
    let (write_impl, read_impl) = match &ast.data {
        // We are matching on a struct.
        Data::Struct(s) => match &s.fields {
            // Case 1: Struct with named fields, e.g., `struct Foo { bar: u32 }`
            Fields::Named(fields) => {
                let write_calls = fields.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    quote! { self.#field_name.write(stream); }
                });

                let read_fields = fields.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let field_type = &f.ty;
                    quote! { #field_name: <#field_type as PacketReadable>::read(stream) }
                });

                // Generate code for writing and reading fields.
                (
                    quote! { #( #write_calls )* },
                    quote! { Self { #( #read_fields ),* } }
                )
            }
            // Case 2: Struct with unnamed fields (tuple struct), e.g., `struct Foo(u32);`
            Fields::Unnamed(fields) => {
                let write_calls = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    // Access tuple fields by index: `self.0`, `self.1`, ...
                    let index = syn::Index::from(i);
                    quote! { self.#index.write(stream); }
                });

                let read_fields = fields.unnamed.iter().map(|f| {
                    let field_type = &f.ty;
                    quote! { <#field_type as PacketReadable>::read(stream) }
                });

                // Generate code for writing and reading tuple fields.
                (
                    quote! { #( #write_calls )* },
                    quote! { Self( #( #read_fields ),* ) }
                )
            }
            // Case 3: Unit struct, e.g., `struct Foo;`
            Fields::Unit => {
                // For a unit struct, write does nothing and read constructs `Self`.
                (
                    quote! { /* No fields to write */ },
                    quote! { Self }
                )
            }
        },
        // We still don't support enums or unions.
        _ => panic!("MinecraftType can only be derived on structs."),
    };

    // --- Assemble the final code ---
    // This part is now simpler because we prepared `write_impl` and `read_impl` above.
    quote! {
        impl MinecraftType for #name {}

        impl PacketWritable for #name {
            fn write(&self, stream: &mut impl std::io::Write) {
                #write_impl
            }
        }

        impl PacketReadable for #name {
            fn read(stream: &mut impl std::io::Read) -> Self {
                #read_impl
            }
        }
    }
}