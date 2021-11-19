mod helpers;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn anyhoo(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        mut sig,
        block,
    } = parse_macro_input!(input);

    helpers::change_output(&mut sig.output);

    let okified_block = helpers::okified_block(&block.stmts);

    let result = quote! { #(#attrs)* #vis #sig #okified_block };

    result.into()
}
