use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{parse_macro_input, Field, Fields, FieldsUnnamed, GenericArgument, Ident, ItemEnum, ItemFn, Meta, PathArguments, Type, TypePath};

#[proc_macro_attribute]
pub fn memoize(meta: TokenStream, body: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(meta as Meta);
    let body = parse_macro_input!(body as ItemFn);
    todo!()
}


#[proc_macro_attribute]
pub fn lecursion(meta: TokenStream, body: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(meta as Meta);
    let body = parse_macro_input!(body as ItemFn);
    todo!()
}

/// Derive all the traits that are needed for the cache system, and the field
/// must match the function signature when it has `memoize` or `lecursion` attribute.
/// 
/// For example:
/// ```ignore
/// impl Parser {
///     #[memoize(Expect)]
///     fn expect(&mut self, s: &'static str) -> Option<&'static str> {
///         todo!()
///     }
/// }
/// 
/// #[indicator]
/// enum CacheType {
///     Expect(Option<&'static str>),
/// }
/// ```
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

/// You must implement `Display` trait for all `Value`, since it is required for 
/// effective cache logging. (abstract syntax tree is usually extremely verbose)
/// 
/// Each variant must be in this format `Variant(Option<Value>)`, where `Value` 
/// must be a name without nested names, and even `&'static str` is invalid due 
/// to lifetime specifier. Please do not bring lifetime into cache result, except 
/// for `&'static`.
///
/// To bypass the restriction:
///
/// ```ignore
/// enum CacheResult {
///     Elysia(Option<StaticStr>),
///     Me(Option<StaticStr>),
/// }
///
/// type StaticStr = &'static str;
/// ```
#[proc_macro_attribute]
pub fn output(_: TokenStream, body: TokenStream) -> TokenStream {
    let body = parse_macro_input!(body as ItemEnum);
    let variants = &body.variants;
    let ident = &body.ident;

    let mut dict = HashMap::<Ident, Vec<Ident>>::new();
    for each in variants {
        let id = each.ident.clone();
        let FieldsUnnamed {
            unnamed, ..
        } = match &each.fields {
            Fields::Unnamed(fu) => fu,
            _ => panic!("Fields::Unnamed")
        };
        let Field { ty, .. } = unnamed.first().expect("unnamed.first()");
        let TypePath { path, .. } = match ty {
            Type::Path(tp) => tp,
            _ => panic!("Type::Path")
        };
        let arg = &path.segments
            .first().expect("path.segments")
            .arguments;
        let ab = match arg {
            PathArguments::AngleBracketed(ab) => ab,
            _ => panic!("PathArguments::AngleBracketed")
        };
        let ga = ab.args.first().expect("args.first()");
        let gt = match ga {
            GenericArgument::Type(gt) => gt,
            _ => panic!("GenericArgument::Type")
        };
        let path = match gt {
            Type::Path(TypePath { path, .. }) => path,
            _ => panic!("Type::Path(TypePath)")
        };
        let ps = path.segments.first().expect("segments.first()");
        let inner = ps.ident.clone();
        if let Some(x) = dict.get_mut(&inner) {
            x.push(id)
        } else {
            dict.insert(inner, vec![id]);
        }
    }

    let from = dict.iter().map(|(k, v)| {
        let inner = v.iter().map(|x| {
            quote!(#ident::#x(inner) => inner,)
        });
        quote! {
            impl From<#ident> for Option<#k> {
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
