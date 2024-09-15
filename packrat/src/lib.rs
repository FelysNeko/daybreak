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
pub fn ct(meta: TokenStream, body: TokenStream) -> TokenStream {
    cache_type_helper(meta, body)
}

#[proc_macro_attribute]
pub fn cr(meta: TokenStream, body: TokenStream) -> TokenStream {
    cache_result_helper(meta, body)
}

#[proc_macro_attribute]
pub fn ast(meta: TokenStream, body: TokenStream) -> TokenStream {
    ast_helper(meta, body)
}
