mod comprehension;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn comprehension(input: TokenStream) -> TokenStream {
    let comprehension: comprehension::Comprehension = syn::parse_macro_input!(input);
    quote! {#comprehension}.into()
}
