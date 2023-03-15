use quote::quote;
use syn::PathArguments;

pub fn impl_readable(derive_input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    if let syn::Data::Struct(s) = &derive_input.data {
        let mut result_token_stream = proc_macro2::TokenStream::new();
        let mut create_token_stream = proc_macro2::TokenStream::new();
        for field in &s.fields {
            match &field.ty {
                syn::Type::Path(p) => {
                    let ident = &field.ident;

                    let r#type = p.path.segments.last().unwrap();

                    let t_ident = &r#type.ident;

                    match &r#type.arguments {
                        PathArguments::AngleBracketed(args) => {
                            result_token_stream.extend_one(quote! {
                                let #ident = #t_ident::#args::read(read).await?;
                            })
                        }
                        _ => result_token_stream.extend_one(quote! {
                            let #ident = #t_ident::read(read).await?;
                        }),
                    };
                    create_token_stream.extend_one(quote! { #ident, })
                }
                _ => {
                    panic!()
                }
            };
        }

        let ident = &derive_input.ident;

        let tokens = quote! {
            #[async_trait::async_trait]
            impl protocol_core::rw::Readable for #ident {
                async fn read<T>(read: &mut T) -> anyhow::Result<Self>
                where
                    T: tokio::io::AsyncRead + std::marker::Unpin + Send + Sync
                {
                    use protocol_core::rw::Readable;

                    #result_token_stream

                    Ok(Self {
                        #create_token_stream
                    })
                }
            }
        };
        return tokens.into();
    }
    proc_macro2::TokenStream::new()
}
