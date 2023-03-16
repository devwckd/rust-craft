#[cfg(feature = "sync")]
macro_rules! impl_number_sync {
    ($($type:ty),+) => {$(
        impl crate::rw::SyncReadable for $type {
            fn read_sync<T>(read: &mut T) -> anyhow::Result<Self>
            where
                T: std::io::Read,
            {
                let mut buf = [0u8; (<$type>::BITS / 8) as usize];
                read.read_exact(&mut buf)?;

                Ok(<$type>::from_be_bytes(buf))
            }
        }

        impl crate::rw::SyncWriteable for $type {
            fn write_sync<T>(&self, write: &mut T) -> anyhow::Result<()>
            where
                T: std::io::Write,
            {
                write.write(&self.to_be_bytes())?;
                Ok(())
            }
        }
    )*};
}

#[cfg(feature = "sync")]
impl_number_sync! {
    u8, i8,
    u16, i16,
    u32, i32,
    u64, i64,
    u128, i128
}

#[cfg(feature = "async")]
macro_rules! impl_number_async {
    ($($type:ty),+) => {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        $(
            #[async_trait::async_trait]
            impl crate::rw::AsyncReadable for $type {
                async fn read_async<T>(read: &mut T) -> anyhow::Result<Self>
                where
                    T: tokio::io::AsyncRead + std::marker::Unpin + Send + Sync,
                {
                    let mut buf = [0u8; (<$type>::BITS / 8) as usize];
                    read.read_exact(&mut buf).await?;

                    Ok(<$type>::from_be_bytes(buf))
                }
            }

            #[async_trait::async_trait]
            impl crate::rw::AsyncWriteable for $type {
                async fn write_async<T>(&self, write: &mut T) -> anyhow::Result<()>
                where
                    T: tokio::io::AsyncWrite + std::marker::Unpin + Send + Sync,
                {
                    write.write(&self.to_be_bytes()).await?;
                    Ok(())
                }
            }
        )*
    };
}

#[cfg(feature = "async")]
impl_number_async! {
    u8, i8,
    u16, i16,
    u32, i32,
    u64, i64,
    u128, i128
}
