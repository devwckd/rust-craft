#![feature(extend_one)]
#![feature(is_some_and)]

mod features;
mod impls;

#[cfg(feature = "packet")]
#[proc_macro_derive(Packet, attributes(packet_id, varint, varlong))]
pub fn derive_packet(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impls::derive_packet_impl(token_stream)
}
