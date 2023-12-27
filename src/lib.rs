use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Field, Fields, FieldsNamed};

#[proc_macro_derive(JsonSolTypes)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_json_sol_types(&ast)
}

fn impl_json_sol_types(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let struct_fields = match &ast.data {
        syn::Data::Struct(data) => Some(&data.fields),
        _ => None,
    };

    if let None = struct_fields {
        return Default::default();
    }

    let fields = struct_fields
        .unwrap()
        .into_iter()
        .map(|field| {

            let name_ident = field.ident.as_ref().expect("ONLY NAMED FIELDS SUPPORTED RIGHT NOW!");
            let name = format!("{}", name_ident);
            let ty = &field.ty;
            let statement = quote! {
                let key = #name.to_string();
                let value: ::substreams_alloy_helpers::prelude::SolidityType = self.#name_ident.clone().into();
                let value: ::substreams_alloy_helpers::prelude::SolidityJsonValue = value.into();
                let value = ::serde_json::to_string(&value).unwrap();
                output_map.insert(key, value.into());
            };
            statement
        })
        .fold(quote!(), |statement, acc| {
            quote!{
                #acc
                #statement
            }
        });

    let gen = quote! {
        impl JsonSolTypes for #name {
            fn as_json(self) -> ::serde_json::Value {
                use ::substreams_alloy_helpers::json_values::*;
                let mut output_map: ::serde_json::Map<String, ::serde_json::Value> = ::serde_json::Map::new();
                #fields
                output_map.into()
            }
        }
    };
    gen.into()
}
