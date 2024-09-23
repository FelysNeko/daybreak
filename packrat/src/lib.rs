mod helper;

use crate::helper::*;
use proc_macro::TokenStream;

/// Cache the result of this sub-expression
///
/// Put `#[daybreak::memoize(AST)]` on top of the method,
/// where `AST` is the variant name in `Self::CT` and `Self::CR`.
#[proc_macro_attribute]
pub fn memoize(meta: TokenStream, body: TokenStream) -> TokenStream {
    memoize_helper(meta, body)
}

/// Allow left-recursion in this sub-expression, caching required
///
/// Put `#[daybreak::lecursion(AST)]` on top of the method,
/// where `AST` is the variant name in `Self::CT` and `Self::CR`.
#[proc_macro_attribute]
pub fn lecursion(meta: TokenStream, body: TokenStream) -> TokenStream {
    lecursion_helper(meta, body)
}

/// Unwrap cache result into the data it stores
/// 
/// Manually implement the `From<CR>` trait is ok,
/// but not recommended unless enums are complicated.
#[proc_macro_derive(FromCR)]
pub fn from_cr(body: TokenStream) -> TokenStream {
    from_cr_helper(body)
}
