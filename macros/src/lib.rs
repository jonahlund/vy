use proc_macro::TokenStream;

mod ast;
#[cfg(feature = "well-known")]
mod well_known;

#[proc_macro]
pub fn parse_and_forward(input: TokenStream) -> TokenStream {
    todo!()
}
