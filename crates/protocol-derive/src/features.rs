mod impl_packet;
pub use impl_packet::impl_packet;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "sync", not(feature = "async-tokio")))] {
        mod impl_readable_sync;
        pub use impl_readable_sync::impl_readable;
        mod impl_writeable_sync;
        pub use impl_writeable_sync::impl_writeable;
    } else if #[cfg(all(feature = "async-tokio", not(feature = "sync")))] {
        mod impl_readable_async_tokio;
        pub use impl_readable_async_tokio::impl_readable;
        mod impl_writeable_async_tokio;
        pub use impl_writeable_async_tokio::impl_writeable;
    }
}
