#![feature(extend_one)]
#![feature(is_some_and)]

mod features;
mod impls;

#[proc_macro_derive(Packet, attributes(packet_id))]
pub fn derive_packet(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impls::derive_packet_impl(token_stream)
}

#[proc_macro_derive(Readable)]
pub fn derive_readable(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impls::derive_readable_impl(token_stream)
}

#[proc_macro_derive(Writeable)]
pub fn derive_writeable(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impls::derive_writeable_impl(token_stream)
}
