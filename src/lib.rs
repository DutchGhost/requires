extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::quote;

use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn requires(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut expr = parse_macro_input!(_attr as syn::Expr);
    let strct = parse_macro_input!(item as syn::Item);

    let mut require_name = syn::Ident::new("__VALIDATE__blank", Span::call_site());
    if let syn::Expr::Assign(ref assign_expr) = expr {
        if let syn::Expr::Path(ref path) = *assign_expr.left {
            let ident = &path.path.segments[0].ident;
            require_name = syn::Ident::new(&format!("__VALIDATE__{}", ident), Span::call_site());
        }

        if let syn::Expr::Binary(_) = *assign_expr.right {
            expr = *assign_expr.right.clone();
        }
    }

    let strct = match strct {
        syn::Item::Struct(s) => s,
        _ => panic!("Expected a struct!"),
    };

    let attrs = strct.attrs.clone();
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

    dbg!(quote! (
        #(#attrs)*
        #vis struct #name #generics
            #fields
        impl #impl_gen #name <#(#lifetimes),* #(#typeparams),* #({#genericparams}),* > {
            pub const #require_name: () = assert!(#expr);
        }
    )
    .into())
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

fn validate_function(f: syn::ItemFn, require_name: syn::Ident) -> TokenStream {
    let vis = f.vis;
    let sig = f.sig;
    let block = f.block;

    let require_name = syn::Ident::new(&format!("__VALIDATE__{}", require_name), Span::call_site());
    dbg!(quote!(#vis #sig {
        Self::#require_name;
        #block
    })
    .into())
}

fn validate_impl_block(mut impl_block: syn::ItemImpl, require_name: syn::Ident) -> TokenStream {
    for item in impl_block.items.iter_mut() {
        let method = match item {
            syn::ImplItem::Method(method) => method,
            _ => continue,
        };

        let require_name =
            syn::Ident::new(&format!("__VALIDATE__{}", require_name), Span::call_site());
        let insert = quote!(Self::#require_name;).into();
        let insert_validate = parse_macro_input!(insert as syn::Stmt);
        method.block.stmts.insert(0, insert_validate);
    }
    quote!(#impl_block).into()
}

#[proc_macro_attribute]
pub fn validate(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let require_name = if _attr.is_empty() {
        syn::Ident::new("blank", Span::call_site())
    } else {
        parse_macro_input!(_attr as syn::Ident)
    };
    dbg!(&require_name);
    let validator = parse_macro_input!(item as ValidatorApply);
    match validator {
        ValidatorApply::Function(f) => validate_function(f, require_name),
        ValidatorApply::ItemImpl(item_impl) => validate_impl_block(item_impl, require_name),
    }
}
