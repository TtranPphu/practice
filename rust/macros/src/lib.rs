mod comprehension;

use proc_macro::TokenStream;
#[proc_macro]
pub fn comprehension(input: TokenStream) -> TokenStream {
    let comprehension: comprehension::Comprehension = syn::parse_macro_input!(input);
    quote::quote! {#comprehension}.into()
}
