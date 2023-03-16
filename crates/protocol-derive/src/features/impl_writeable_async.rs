use quote::quote;

pub fn impl_writeable_async(derive_input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    if let syn::Data::Struct(s) = &derive_input.data {
        let mut result_token_stream = proc_macro2::TokenStream::new();
        for field in &s.fields {
            match &field.ty {
                syn::Type::Path(_) => {
                    let ident = &field.ident;
                    result_token_stream.extend_one(quote! {
                        self.#ident.write_async(write).await?;
                    });
                }
                _ => {
                    panic!()
                }
            };
        }

        let ident = &derive_input.ident;

        let tokens = quote! {
            #[async_trait::async_trait]
            impl protocol_core::rw::AsyncWriteable for #ident {
                async fn write_async<T>(&self, write: &mut T) -> anyhow::Result<()>
                where
                    T: tokio::io::AsyncWrite + std::marker::Unpin + Send + Sync
                {
                    use protocol_core::rw::AsyncWriteable;

                    #result_token_stream

                    Ok(())
                }
            }
        };
        return tokens.into();
    }
    proc_macro2::TokenStream::new()
}
