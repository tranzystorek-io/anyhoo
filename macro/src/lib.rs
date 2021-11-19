use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Block, ItemFn, ReturnType, Stmt};

#[proc_macro_attribute]
pub fn anyhoo(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        mut sig,
        block,
    } = parse_macro_input!(input);

    sig.output = match &sig.output {
        ReturnType::Default => parse_quote! { -> ::anyhoo::anyhow::Result<()> },
        ReturnType::Type(_, ty) => parse_quote! { -> ::anyhoo::anyhow::Result<#ty> },
    };

    let okified_block: Block = match block.stmts.as_slice() {
        [all @ .., Stmt::Expr(last)] => {
            parse_quote!( { #(#all)* ::anyhoo::anyhow::Result::Ok(#last) } )
        }
        all => parse_quote!( { #(#all)* ::anyhoo::anyhow::Result::Ok(()) } ),
    };

    let result = quote! { #(#attrs)* #vis #sig #okified_block };

    result.into()
}
