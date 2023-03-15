use crate::rw::{Readable, Writeable};

#[cfg(all(feature = "rw-sync", not(feature = "rw-async-tokio")))]
macro_rules! impl_number {
    ($($type:ty),+) => {$(
        impl Readable for $type {
            fn read<T>(read: &mut T) -> anyhow::Result<Self>
            where
                T: std::io::Read,
            {
                let mut buf = [0u8; (<$type>::BITS / 8) as usize];
                read.read_exact(&mut buf)?;

                Ok(<$type>::from_be_bytes(buf))
            }
        }

        impl Writeable for $type {
            fn write<T>(&self, write: &mut T) -> anyhow::Result<()>
            where
                T: std::io::Write,
            {
                write.write(&self.to_be_bytes())?;
                Ok(())
            }
        }
    )*};
}

#[cfg(all(feature = "rw-async-tokio", not(feature = "rw-sync")))]
macro_rules! impl_number {
    ($($type:ty),+) => {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        $(
            #[async_trait::async_trait]
            impl Readable for $type {
                async fn read<T>(read: &mut T) -> anyhow::Result<Self>
                where
                    T: tokio::io::AsyncRead + std::marker::Unpin + Send + Sync,
                {
                    let mut buf = [0u8; (<$type>::BITS / 8) as usize];
                    read.read_exact(&mut buf).await?;

                    Ok(<$type>::from_be_bytes(buf))
                }
            }

            #[async_trait::async_trait]
            impl Writeable for $type {
                async fn write<T>(&self, write: &mut T) -> anyhow::Result<()>
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

impl_number! {
    u8, i8,
    u16, i16,
    u32, i32,
    u64, i64,
    u128, i128
}
