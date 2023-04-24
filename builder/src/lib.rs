use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _ = syn::parse_macro_input!(input as DeriveInput);

    quote! {}.into()
}
