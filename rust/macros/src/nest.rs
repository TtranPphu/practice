use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Token,
};

use crate::comprehension::{parse_some, IteratorFilter};

pub struct Nest {
    pub iterator_filter: IteratorFilter,
    pub more_iterator_filters: Vec<IteratorFilter>,
}

impl Parse for Nest {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            iterator_filter: input.parse()?,
            more_iterator_filters: parse_some(input),
        })
    }
}

impl ToTokens for Nest {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        todo!()
    }
}
