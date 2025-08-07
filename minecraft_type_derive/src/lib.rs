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
    // We only support structs with named fields, like `struct Foo { bar: u32 }`.
    // This code extracts those fields.
    let fields = match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => &f.named,
            // You could add support for tuple structs or unit structs here if needed.
            _ => panic!("MinecraftType can only be derived for structs with named fields."),
        },
        // We don't support enums or unions with this derive.
        _ => panic!("MinecraftType can only be derived on structs."),
    };

    // --- Generate `PacketWritable` implementation ---
    // Iterate over the fields to generate a `write` call for each one.
    // e.g., `self.registry_id.write(stream); self.entries.write(stream);`
    let write_calls = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        quote! {
            self.#field_name.write(stream);
        }
    });

    // --- Generate `PacketReadable` implementation ---
    // Iterate over the fields to generate a `read` call for each one.
    // Then, use the results to construct a new `Self`.
    // e.g., `registry_id: types::Identifier::read(stream), entries: ...`
    let read_fields = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_type = &f.ty;
        quote! {
            #field_name: <#field_type as PacketReadable>::read(stream)
        }
    });

    // --- Assemble the final code using `quote!` ---
    // The `quote!` macro lets us write Rust code and splice in variables.
    // The `#(...)*` syntax means "repeat the enclosed code for every item in the iterator".
    quote! {
        // Implement the marker trait.
        impl MinecraftType for #name {}

        // Implement the PacketWritable trait.
        impl PacketWritable for #name {
            fn write(&self, stream: &mut impl std::io::Write) {
                #( #write_calls )*
            }
        }

        // Implement the PacketReadable trait.
        impl PacketReadable for #name {
            fn read(stream: &mut impl std::io::Read) -> Self {
                Self {
                    #( #read_fields ),*
                }
            }
        }
    }
}