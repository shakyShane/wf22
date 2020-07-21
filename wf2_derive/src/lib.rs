use proc_macro::TokenStream;
use quote::quote;
use syn::ItemEnum;

#[proc_macro_attribute]
pub fn task_list(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse::<ItemEnum>(item).expect("parsed");
    let ident = item.ident.clone();

    let next_impls = item.variants.iter().map(|v| {
        let ident = v.ident.clone();
        match ident.to_string().as_str() {
            "PassThru" => quote! { Self::#ident(inner) => unimplemented!() },
            _ => quote! { Self::#ident(inner) => inner.to_task_list(&ctx) },
        }
    });

    let expanded = quote! {
        #item
        impl TaskList for #ident {
            fn to_task_list(&self, ctx: &Context) -> Vec<Task> {
                match self {
                    #(#next_impls),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
