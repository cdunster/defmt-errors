use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

#[proc_macro_derive(DefmtError)]
pub fn derive(tokens: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(tokens);
    match input.data {
        Data::Struct(_) => todo!(),
        Data::Enum(_) => derive_enum(input.ident),
        Data::Union(_) => todo!(),
    }
}

fn derive_enum(type_ident: Ident) -> TokenStream {
    let ident_string = type_ident.to_string();
    quote! {
        impl ::defmt_errors::defmt::Format for #type_ident {
            fn format(&self, fmt: ::defmt_errors::defmt::Formatter) {
                use ::defmt_errors::defmt;
                defmt::write!(fmt, "A {} error occurred", #ident_string)
            }
        }
    }
    .into()
}
