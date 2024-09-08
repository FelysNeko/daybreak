use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Meta, PatType};

pub fn memoize_helper(meta: TokenStream, body: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(meta as Meta);
    let body = parse_macro_input!(body as ItemFn);

    let attrs = &body.attrs;
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
        #(#attrs)*
        #vis #signature {
            #fast
            #store
        }
    ).into()
}

pub fn lecursion_helper(meta: TokenStream, body: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(meta as Meta);
    let body = parse_macro_input!(body as ItemFn);

    let attrs = &body.attrs;
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
        #(#attrs)*
        #[memoize(#cache)]
        #vis #signature {
            #main
        }
    ).into()
}

pub fn strict_helper(_: TokenStream, body: TokenStream) -> TokenStream {
    let body = parse_macro_input!(body as ItemFn);

    let attrs = &body.attrs;
    let signature = &body.sig;
    let rt = &signature.output;
    let block = &body.block;
    let vis = &body.vis;
    
    quote!(
        #(#attrs)*
        #vis #signature {
            self.stream.strict(true);
            let __s_res = || #rt #block();
            self.stream.strict(false);
            __s_res
        }
    ).into()
}

pub fn loose_helper(_: TokenStream, body: TokenStream) -> TokenStream {
    let body = parse_macro_input!(body as ItemFn);

    let attrs = &body.attrs;
    let signature = &body.sig;
    let rt = &signature.output;
    let block = &body.block;
    let vis = &body.vis;

    quote!(
        #(#attrs)*
        #vis #signature {
            self.stream.strict(false);
            let __s_res = || #rt #block();
            self.stream.strict(true);
            __s_res
        }
    ).into()
}
