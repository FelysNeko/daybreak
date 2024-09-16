mod func;
mod ast;

use ast::*;
use func::*;

use proc_macro::TokenStream;

/// Cache the result after calling this method
///
/// Put `#[daybreak::memoize(AST)]` on top of the desired method,
/// where `AST` must match `Option<AST>` in the function signature.
/// The parser will remember the parsing result of this method at this position.
#[proc_macro_attribute]
pub fn memoize(meta: TokenStream, body: TokenStream) -> TokenStream {
    memoize_helper(meta, body)
}

/// Allow potential left recursion and cache them
/// 
/// Refer to the usage of `daybreak::memoize`.
/// It will use `#[daybreak::memoize(AST)]` to wrap the method first,
/// then manipulate the cache to support left recursion.
/// Thus, caching is required.
#[proc_macro_attribute]
pub fn lecursion(meta: TokenStream, body: TokenStream) -> TokenStream {
    lecursion_helper(meta, body)
}

/// Ensure strict mode manipulation not affecting the outer methods
/// 
/// Put `#[daybreak::strict]` before `daybreak::memoize` or `daybreak::lecursion` if exists,
/// and you can safely switch the strict mode inside this method.
/// Though not required, but strongly recommended.
#[proc_macro_attribute]
pub fn strict(meta: TokenStream, body: TokenStream) -> TokenStream {
    strict_helper(meta, body)
}

/// Implement traits for cache type
/// 
/// Put `#[daybreak::ct]` on top of the cache type registry.
/// Use `enum` if there is something to be cached,
/// or `struct` with no fields as a placeholder.
#[proc_macro_attribute]
pub fn ct(meta: TokenStream, body: TokenStream) -> TokenStream {
    cache_type_helper(meta, body)
}

/// Implement traits for cache result
///
/// Refer to the usage of `daybreak::ct`.
/// If `enum` is used, every variant must be in form of `Name(Option<AST>)`
/// where `Name` must match registered name under `daybreak::ct`,
/// and `AST` is the actual node without alias.
#[proc_macro_attribute]
pub fn cr(meta: TokenStream, body: TokenStream) -> TokenStream {
    cache_result_helper(meta, body)
}

/// Implement traits for ast
/// 
/// Put `#[daybreak::ast]` on top of the abstract syntax tree node.
/// However, you still need to manually implement the `Display` trait.
#[proc_macro_attribute]
pub fn ast(meta: TokenStream, body: TokenStream) -> TokenStream {
    ast_helper(meta, body)
}
