use syn::{parse_quote, Block, ReturnType, Stmt};

#[cfg(feature = "reexport")]
pub fn change_output(output: &mut ReturnType) {
    *output = match &output {
        ReturnType::Default => parse_quote! { -> ::anyhoo::anyhow::Result<()> },
        ReturnType::Type(_, ty) => parse_quote! { -> ::anyhoo::anyhow::Result<#ty> },
    };
}

#[cfg(not(feature = "reexport"))]
pub fn change_output(output: &mut ReturnType) {
    *output = match &output {
        ReturnType::Default => parse_quote! { -> anyhow::Result<()> },
        ReturnType::Type(_, ty) => parse_quote! { -> anyhow::Result<#ty> },
    };
}

#[cfg(feature = "reexport")]
pub fn okified_block(stmts: &[Stmt]) -> Block {
    match stmts {
        [all @ .., Stmt::Expr(last)] => {
            parse_quote!( { #(#all)* ::anyhoo::anyhow::Result::Ok(#last) } )
        }
        all => parse_quote!( { #(#all)* ::anyhoo::anyhow::Result::Ok(()) } ),
    }
}

#[cfg(not(feature = "reexport"))]
pub fn okified_block(stmts: &[Stmt]) -> Block {
    match stmts {
        [all @ .., Stmt::Expr(last)] => {
            parse_quote!( { #(#all)* anyhow::Result::Ok(#last) } )
        }
        all => parse_quote!( { #(#all)* anyhow::Result::Ok(()) } ),
    }
}
