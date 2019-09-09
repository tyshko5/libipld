extern crate proc_macro;

mod from_ipld;
mod to_ipld;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Ipld)]
pub fn derive_ipld(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let from_ipld = crate::from_ipld::from_ipld(&input.ident, &input.data);
    let to_ipld = crate::to_ipld::to_ipld(&input.ident, &input.data);

    let expanded = quote! {
        impl #ident {
            #to_ipld
            #from_ipld
        }
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let t = trybuild::TestCases::new();
        t.pass("examples/struct.rs");
    }
}
