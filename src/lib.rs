extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;

use syn::{
    parse_macro_input,
};

#[proc_macro_attribute]
pub fn requires(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(_attr as syn::Expr);
    let strct = parse_macro_input!(item as syn::Item);
    
    let strct = match strct {
        syn::Item::Struct(s) => s,
        _ => panic!("Expected a struct!")
    };
    

    let vis = strct.vis.clone();
    let name = strct.ident.clone();
    let fields = strct.fields.clone();
    let generics = strct.generics.clone();
    
    let lifetimes = generics.lifetimes().cloned().collect::<Vec<_>>();
    let typeparams = generics.type_params().cloned().collect::<Vec<_>>();
    let genericparams = generics.const_params().cloned().map(|param| param.ident).collect::<Vec<_>>();

    let (impl_gen, _, _) = generics.split_for_impl();

    quote! (
        #vis struct #name #generics
            #fields
        impl #impl_gen #name <#(#lifetimes),* #(#typeparams),* #({#genericparams}),* > {
            const __validator__: () = [()][!(#expr) as usize];

            const fn validate() -> () {
                Self::__validator__
            }
        }
    ).into()
}