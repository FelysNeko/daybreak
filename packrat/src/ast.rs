use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::{parse_macro_input, Field, Fields, ItemEnum, ItemStruct};

pub fn cache_type_helper(_: TokenStream, body: TokenStream) -> TokenStream {
    // if let Ok(body) = syn::parse::<ItemStruct>(body.clone()) {
    //     return cache_type_struct(body);
    // }

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

fn cache_type_struct(body: ItemStruct) -> TokenStream {
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

pub fn cache_result_helper(_: TokenStream, body: TokenStream) -> TokenStream {
    if let Ok(body) = syn::parse::<ItemStruct>(body.clone()) {
        return cache_result_struct(body);
    }

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

pub fn ast_helper(_: TokenStream, body: TokenStream) -> TokenStream {
    let body = proc_macro2::TokenStream::from(body);
    quote!(
        #[derive(Debug, Clone)]
        #body
    ).into()
}

fn cache_result_struct(body: ItemStruct) -> TokenStream {
    let ident = &body.ident;
    quote!(
        #[derive(Debug, Clone)]
        #body
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    ).into()
}