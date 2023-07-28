extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    input
}
