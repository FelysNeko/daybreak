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
        let _m_pos = self.stream.mark();
        let _m_strict = self.stream.mode();
        let _m_cache_type = Self::CT::#cache #args;
        if let Some(_m_cache) = self.cache.get(_m_pos, _m_strict, _m_cache_type) {
            let (_m_end, _m_cache_result) = _m_cache;
            self.stream.jump(_m_end);
            return _m_cache_result.into()
        }
    };

    let store = quote! {
        let _m_result = || #rt #block();
        let _m_cache_result = Self::CR::#cache(_m_result.clone());
        let _m_end = self.stream.mark();
        let _m_strict = self.stream.mode();
        self.cache.insert(_m_pos, _m_strict, _m_cache_type, _m_end, _m_cache_result);
        _m_result
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
        let _l_pos = self.stream.mark();
        let _l_cache_type = Self::CT::#cache #args;
        let mut _l_cache_result = Self::CR::#cache(None);
        let mut _l_end = _l_pos;
        loop {
            let _l_strict = self.stream.mode();
            self.cache.insert(_l_pos, _l_strict, _l_cache_type, _l_end, _l_cache_result.clone());
            let _l_res = || #rt #block();
            if _l_end < self.stream.mark() {
                _l_cache_result = Self::CR::#cache(_l_res);
                _l_end = self.stream.mark();
                self.stream.jump(_l_pos);
            } else {
                self.stream.jump(_l_end);
                break _l_cache_result.into();
            }
        }
    };

    quote!(
        #(#attrs)*
        #[::helper::memoize(#cache)]
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
            let _s_res = || #rt #block();
            self.stream.strict(false);
            _s_res
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
            let _s_res = || #rt #block();
            self.stream.strict(true);
            _s_res
        }
    ).into()
}
