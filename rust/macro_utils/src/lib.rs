mod comprehension;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn comprehension(input: TokenStream) -> TokenStream {
    use comprehension::comprehension::Comprehension;
    let comprehension = syn::parse_macro_input!(input as Comprehension);
    quote! {#comprehension}.into()
}
