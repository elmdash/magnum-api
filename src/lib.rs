use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn magnum_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("{}", item.to_string());
    let item: proc_macro2::TokenStream = item.into();
    quote! {
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        #item
    }
    .into()
}
