use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Ident};

#[proc_macro_derive(DefmtError)]
pub fn derive(tokens: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(tokens);
    match input.data {
        Data::Struct(_) => todo!(),
        Data::Enum(enum_data) => derive_enum(input.ident, enum_data),
        Data::Union(_) => todo!(),
    }
}

fn derive_enum(type_ident: Ident, enum_data: DataEnum) -> TokenStream {
    let body = if enum_data.variants.is_empty() {
        let ident_string = type_ident.to_string();
        quote! {
            ::defmt_errors::defmt::write!(fmt, "A {} error occurred", #ident_string)
        }
    } else {
        let arms = enum_data.variants.iter().map(|variant| {
            let ident = &variant.ident;
            let ident_string = format!("{type_ident}::{ident}");
            quote! {
                #type_ident::#ident => ::defmt_errors::defmt::write!(fmt, "A {} error occurred", #ident_string),
            }
        });
        quote! {
            match self {
                #(#arms)*
            }
        }
    };
    quote! {
        impl ::defmt_errors::defmt::Format for #type_ident {
            fn format(&self, fmt: ::defmt_errors::defmt::Formatter) {
                use ::defmt_errors::defmt;
                #body
            }
        }
    }
    .into()
}
