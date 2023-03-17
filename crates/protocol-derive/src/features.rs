mod impl_packet;
pub use impl_packet::impl_packet;
use syn::DeriveInput;

mod impl_readable_async;
mod impl_readable_sync;
mod impl_writeable_async;
mod impl_writeable_sync;

pub fn impl_writeable(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    #[cfg(all(feature = "sync"))]
    stream.extend_one(impl_writeable_sync::impl_writeable_sync(&derive_input));

    #[cfg(all(feature = "async"))]
    stream.extend_one(impl_writeable_async::impl_writeable_async(&derive_input));

    stream
}

pub fn impl_readable(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    #[cfg(all(feature = "sync"))]
    stream.extend_one(impl_readable_sync::impl_readable_sync(&derive_input));

    #[cfg(all(feature = "async"))]
    stream.extend_one(impl_readable_async::impl_readable_async(&derive_input));

    stream
}
