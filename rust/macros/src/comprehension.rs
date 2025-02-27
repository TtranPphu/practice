use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Token,
};

pub struct Comprehension {
    mapper: Mapper,
    iterator_filter: IteratorFilter,
    more_iterator_filters: Vec<IteratorFilter>,
}

struct Mapper(syn::Expr);

pub struct IteratorFilter {
    iterator: Iterator,
    iterable: syn::Expr,
    filters: Vec<Filter>,
}

struct Iterator(syn::Pat);
struct Filter(syn::Expr);

pub fn parse_some<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut r = vec![];
    while let Ok(item) = input.parse() {
        r.push(item);
    }
    r
}

impl Parse for Comprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapper: input.parse()?,
            iterator_filter: input.parse()?,
            more_iterator_filters: parse_some(input),
        })
    }
}

impl ToTokens for Comprehension {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut iterator_filters = std::iter::once(&self.iterator_filter)
            .chain(&self.more_iterator_filters)
            .rev();

        let mut output = {
            let Mapper(mapper) = &self.mapper;

            let IteratorFilter {
                iterator,
                iterable,
                filters,
            } = iterator_filters.next().unwrap();

            quote! {
                ::core::iter::IntoIterator::into_iter(#iterable).filter_map(move |#iterator| {
                    (true #(&& (#filters))*).then(|| #mapper)
                })
            }
        };

        output = iterator_filters.fold(output, |current_output, next_iterator_filter| {
            let IteratorFilter {
                iterator,
                iterable,
                filters,
            } = next_iterator_filter;
            quote! {
                core::iter::IntoIterator::into_iter(#iterable).filter_map(move |#iterator| {
                    (true #(&& (#filters))*).then(|| #current_output)
                }).flatten()
            }
        });

        tokens.extend(output);
    }
}

impl Parse for Mapper {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

impl ToTokens for Mapper {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}

impl Parse for IteratorFilter {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![for] = input.parse()?;
        let iterator = input.parse()?;
        let _: Token![in] = input.parse()?;
        Ok(Self {
            iterator,
            iterable: input.parse()?,
            filters: parse_some(input),
        })
    }
}

impl Parse for Iterator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        syn::Pat::parse_single(input).map(Self)
    }
}

impl ToTokens for Iterator {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}

impl Parse for Filter {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![if] = input.parse()?;
        input.parse().map(Self)
    }
}

impl ToTokens for Filter {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}
