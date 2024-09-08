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
        let __m_cache_type = Self::CT::#cache #args;
        if let Some(__m_cache) = self.cache.get(__m_pos, __m_cache_type) {
            let (__m_end, __m_cache_result) = __m_cache;
            self.stream.jump(__m_end);
            return __m_cache_result.into()
        }
    };

    let store = quote! {
        let __m_result = || #rt #block();
        let __m_cache_result = Self::CR::#cache(__m_result.clone());
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

    let main = quote! {
        let __l_pos = self.stream.mark();
        let __l_cache_type = Self::CT::#cache #args;
        let mut __l_cache_result = Self::CR::#cache(None);
        let mut __l_end = __l_pos;
        loop {
            self.cache.insert(__l_pos, __l_cache_type, __l_end, __l_cache_result.clone());
            let __l_res = || #rt #block();
            if __l_end < self.stream.mark() {
                __l_cache_result = Self::CR::#cache(__l_res);
                __l_end = self.stream.mark();
                self.stream.jump(__l_pos);
            } else {
                self.stream.jump(__l_end);
                break __l_cache_result.into();
            }
        }
    };

    quote!(
        #[memoize(#cache)]
        #vis #signature {
            #main
        }
    ).into()
}

#[proc_macro_attribute]
pub fn indicator(_: TokenStream, body: TokenStream) -> TokenStream {
    let body = parse_macro_input!(body as ItemEnum);

    let ident = &body.ident;

    quote!(
        #[allow(clippy::enum_variant_names)]
        #[derive(std::fmt::Debug, std::hash::Hash, PartialEq, Eq, Clone, Copy)]
        #body
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    ).into()
}

#[proc_macro_attribute]
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
        #[allow(clippy::enum_variant_names)]
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

#[proc_macro_attribute]
pub fn ast(_: TokenStream, body: TokenStream) -> TokenStream {
    let body = proc_macro2::TokenStream::from(body);
    quote!(
        #[derive(Debug, Clone)]
        #body
    ).into()
}
