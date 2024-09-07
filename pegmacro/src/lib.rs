use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::{parse_macro_input, Field, Fields, FnArg, Ident, ItemEnum, ItemFn, Meta, PatType};

#[proc_macro_attribute]
pub fn memoize(meta: TokenStream, body: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(meta as Meta);
    let body = parse_macro_input!(body as ItemFn);

    let signature = &body.sig;
    let rt = &signature.output;
    let block = &body.block;
    let vis = &body.vis;

    let path = &meta
        .require_path_only().expect("meta.require_path_only()")
        .segments;
    let cache = &path.first().expect("path.first()").ident;

    let mut pa = signature.inputs.iter()
        .filter_map(|x| match x {
            FnArg::Typed(PatType { pat, .. }) => Some(pat),
            _ => None
        })
        .peekable();

    let args = if pa.peek().is_some() {
        quote! { (#(#pa),*) }
    } else {
        quote! {}
    };

    let fast = quote! {
        let __m_pos = self.stream.mark();
        let __m_cache_type = ElyCacheType::#cache #args;
        if let Some(__m_cache) = self.cache.get(__m_pos, __m_cache_type) {
            let (__m_end, __m_cache_result) = __m_cache;
            self.stream.jump(__m_end);
            return __m_cache_result.into()
        }
    };

    let store = quote! {
        let __m_result = || #rt #block();
        let __m_cache_result = crate::ast::ElyCacheResult::#cache(__m_result.clone());
        let __m_end = self.stream.mark();
        self.cache.insert(__m_pos, __m_cache_type, __m_end, __m_cache_result);
        __m_result
    };

    quote!(
        #vis #signature {
            #fast
            #store
        }
    ).into()
}

#[proc_macro_attribute]
pub fn lecursion(meta: TokenStream, body: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(meta as Meta);
    let body = parse_macro_input!(body as ItemFn);
    todo!()
}

/// Derive all the traits that are needed for the cache system, and its field
/// `Option<Value>` must match the function signature args when it has `memoize`
/// or `lecursion` attribute.
#[proc_macro_attribute]
pub fn indicator(_: TokenStream, body: TokenStream) -> TokenStream {
    let body = parse_macro_input!(body as ItemEnum);

    let ident = &body.ident;

    quote!(
        #[derive(std::fmt::Debug, std::hash::Hash, PartialEq, Eq, Clone, Copy)]
        #body
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    ).into()
}

/// Each variant must be in this format `Variant(Option<Value>)`, and You
/// must implement `Display` trait for all `Value`, since it is required for
/// effective cache logging. (abstract syntax tree is usually very verbose)
#[proc_macro_attribute]
#[allow(clippy::never_loop)]
pub fn output(_: TokenStream, body: TokenStream) -> TokenStream {
    let body = parse_macro_input!(body as ItemEnum);

    let variants = &body.variants;
    let ident = &body.ident;

    let mut dict = HashMap::<String, (Field, Vec<Ident>)>::new();
    for each in variants {
        let id = each.ident.clone();
        let Fields::Unnamed(fu) = &each.fields else {
            panic!("Fields::Unnamed")
        };
        let field = fu.unnamed.first().expect("unnamed.first()").clone();
        let index = field.to_token_stream().to_string();
        if let Some((_, v)) = dict.get_mut(&index) {
            v.push(id)
        } else {
            dict.insert(index, (field, vec![id]));
        }
    }

    let from = dict.iter().map(|(_, (k, v))| {
        let inner = v.iter().map(|x| {
            quote!(#ident::#x(inner) => inner,)
        });
        quote! {
            impl From<#ident> for #k {
                fn from(value: #ident) -> Self {
                    match value {
                        #(#inner)*
                        _ => panic!("cache unmatched")
                    }
                }
            }
        }
    });

    let display = variants.iter().map(|x| {
        let cr = &x.ident;
        quote! {
            #ident::#cr(x) => match x {
                Some(i) => write!(f, "{}", i),
                None => write!(f, "None"),
            }
        }
    });

    quote!(
        #[derive(Debug, Clone)]
        #body
        #(#from)*
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display)*
                }
            }
        }
    ).into()
}
