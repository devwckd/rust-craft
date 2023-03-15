pub fn derive_packet_impl(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = syn::parse_macro_input!(token_stream as syn::DeriveInput);
    let mut result_token_stream = proc_macro2::TokenStream::new();

    result_token_stream.extend_one(crate::features::impl_packet(&derive_input));
    if cfg!(feature = "packet-write") {
        result_token_stream.extend_one(crate::features::impl_writeable(&derive_input));
    }

    if cfg!(feature = "packet-read") {
        result_token_stream.extend_one(crate::features::impl_readable(&derive_input));
    }

    result_token_stream.into()
}
