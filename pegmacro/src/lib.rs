mod func;
mod ast;

use ast::*;
use func::*;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn memoize(meta: TokenStream, body: TokenStream) -> TokenStream {
    memoize_helper(meta, body)
}

#[proc_macro_attribute]
pub fn lecursion(meta: TokenStream, body: TokenStream) -> TokenStream {
    lecursion_helper(meta, body)
}

#[proc_macro_attribute]
pub fn strict(meta: TokenStream, body: TokenStream) -> TokenStream {
    strict_helper(meta, body)
}

#[proc_macro_attribute]
pub fn loose(meta: TokenStream, body: TokenStream) -> TokenStream {
    loose_helper(meta, body)
}

#[proc_macro_attribute]
pub fn index(meta: TokenStream, body: TokenStream) -> TokenStream {
    index_helper(meta, body)
}

#[proc_macro_attribute]
pub fn output(meta: TokenStream, body: TokenStream) -> TokenStream {
    output_helper(meta, body)
}

#[proc_macro_attribute]
pub fn ast(meta: TokenStream, body: TokenStream) -> TokenStream {
    ast_helper(meta, body)
}
