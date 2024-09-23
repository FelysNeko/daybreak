mod helper;

use crate::helper::*;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn memoize(meta: TokenStream, body: TokenStream) -> TokenStream {
    memoize_helper(meta, body)
}

#[proc_macro_attribute]
pub fn lecursion(meta: TokenStream, body: TokenStream) -> TokenStream {
    lecursion_helper(meta, body)
}

#[proc_macro_derive(FromCR)]
pub fn from_cr(body: TokenStream) -> TokenStream {
    from_cr_helper(body)
}
