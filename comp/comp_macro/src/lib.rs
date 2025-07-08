// comp: mapping for_if_clause
//
// mapping: expression
//
// for_if_clause:
//   | 'for' pattern 'in' expression ('if' expression)*
//
// pattern: name (, name)*

use quote::{ToTokens, quote};
use syn::{
    Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

use proc_macro2::TokenStream as TokenStream2;

impl Parse for ForIfClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![for] = input.parse::<Token![for]>()?;
        let pattern = Pattern::parse(input)?;
        let _: Token![in] = input.parse::<Token![in]>()?;
        let expression = input.parse()?;
        let conditions = parse_zero_or_more(input);
        Ok(Self {
            pattern,
            sequence: expression,
            conditions,
        })
    }
}

fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut result = Vec::new();
    while let Ok(item) = input.parse::<T>() {
        result.push(item);
    }
    result
}

struct Comp {
    mapping: Mapping,
    for_if_clause: ForIfClause,
}

impl Parse for Comp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapping: input.parse()?,
            for_if_clause: input.parse()?,
        })
    }
}

struct Mapping(syn::Expr);

struct ForIfClause {
    pattern: Pattern,
    sequence: syn::Expr,
    conditions: Vec<Condition>,
}

struct Condition(syn::Expr);

struct Pattern(syn::Pat);

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        syn::Pat::parse_single(input).map(Self)
    }
}

impl ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

impl ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

impl ToTokens for Mapping {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![if] = input.parse::<Token![if]>()?;
        input.parse::<syn::Expr>().map(Self)
    }
}

impl Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Expr>().map(Self)
    }
}

impl quote::ToTokens for Comp {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        // core::Iter::IntoIterator::into_iter(sequence).filter_map(|pattern| {
        //   (true && ...).then(|| mapping)
        // })

        let Mapping(mapping) = &self.mapping;
        let ForIfClause {
            pattern,
            sequence,
            conditions,
        } = &self.for_if_clause;

        tokens.extend(quote! {
            core::iter::IntoIterator::into_iter(#sequence)
                .filter_map(move |#pattern| {
                    (true #(&& (#conditions))*).then(|| #mapping)
                })
        });
    }
}

#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let c = parse_macro_input!(input as Comp);
    quote! { #c }.into()
}
