extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;

use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn requires(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(_attr as syn::Expr);
    let strct = parse_macro_input!(item as syn::Item);

    let strct = match strct {
        syn::Item::Struct(s) => s,
        _ => panic!("Expected a struct!"),
    };

    let vis = strct.vis.clone();
    let name = strct.ident.clone();
    let fields = strct.fields.clone();
    let generics = strct.generics;

    let lifetimes = generics.lifetimes().cloned().collect::<Vec<_>>();
    let typeparams = generics.type_params().cloned().collect::<Vec<_>>();
    let genericparams = generics
        .const_params()
        .cloned()
        .map(|param| param.ident)
        .collect::<Vec<_>>();

    let (impl_gen, _, _) = generics.split_for_impl();

    quote! (
        #vis struct #name #generics
            #fields
        impl #impl_gen #name <#(#lifetimes),* #(#typeparams),* #({#genericparams}),* > {
            const __VALIDATE: () = assert!(#expr);
        }
    )
    .into()
}

enum ValidatorApply {
    Function(syn::ItemFn),
    ItemImpl(syn::ItemImpl),
}

impl syn::parse::Parse for ValidatorApply {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let try_parse_fn = input.parse::<syn::ItemFn>();

        if let Ok(function) = try_parse_fn {
            return Ok(Self::Function(function));
        }

        let try_parse_item_impl = input.parse::<syn::ItemImpl>()?;
        Ok(Self::ItemImpl(try_parse_item_impl))
    }
}

fn validate_function(f: syn::ItemFn) -> TokenStream {
    let vis = f.vis;
    let sig = f.sig;
    let block = f.block;

    quote!(#vis #sig {
        Self::__VALIDATE;
        #block
    })
    .into()
}

fn validate_impl_block(mut impl_block: syn::ItemImpl) -> TokenStream {
    for item in impl_block.items.iter_mut() {
        let method = match item {
            syn::ImplItem::Method(method) => method,
            _ => continue,
        };

        let insert = quote!(Self::__VALIDATE;).into();
        let insert_validate = parse_macro_input!(insert as syn::Stmt);
        method.block.stmts.insert(0, insert_validate);
    }
    quote!(#impl_block).into()
}

#[proc_macro_attribute]
pub fn validate(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let validator = parse_macro_input!(item as ValidatorApply);
    match validator {
        ValidatorApply::Function(f) => validate_function(f),
        ValidatorApply::ItemImpl(item_impl) => validate_impl_block(item_impl),
    }
}
