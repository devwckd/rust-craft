pub fn derive_writeable_impl(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = syn::parse_macro_input!(token_stream as syn::DeriveInput);
    let mut result_token_stream = proc_macro2::TokenStream::new();

    if cfg!(feature = "write") {
        result_token_stream.extend_one(crate::features::impl_writeable(&derive_input));
    }

    result_token_stream.into()
}
