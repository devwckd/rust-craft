use quote::quote;
use syn::PathArguments;

pub fn impl_readable_async(derive_input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return match &derive_input.data {
        syn::Data::Struct(s) => impl_readable_sync_struct(s, derive_input),
        syn::Data::Enum(e) => impl_readable_sync_enum(e, derive_input),
        _ => proc_macro2::TokenStream::new(),
    };
}

fn impl_readable_sync_struct(
    s: &syn::DataStruct,
    derive_input: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    let (result_token_stream, create_token_stream) = read_inputs(&s.fields);
    let ident = &derive_input.ident;

    let tokens = quote! {
        #[async_trait::async_trait]
        impl protocol_core::rw::AsyncReadable for #ident {
            async fn read_async<T>(mut read: &mut T) -> anyhow::Result<Self>
            where
                T: tokio::io::AsyncRead + std::marker::Unpin + Send + Sync
            {
                use protocol_core::rw::AsyncReadable;

                #result_token_stream

                Ok(Self {
                    #create_token_stream
                })
            }
        }
    };
    return tokens.into();
}

fn impl_readable_sync_enum(
    e: &syn::DataEnum,
    derive_input: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    let ident = &derive_input.ident;
    let mut match_body = proc_macro2::TokenStream::new();

    for (index, variant) in e.variants.iter().enumerate() {
        let index = index as i32;
        let variant_ident = &variant.ident;

        let (result_token_stream, create_token_stream) = read_inputs(&variant.fields);
        match_body.extend_one(quote::quote! {
            #index => {
                #result_token_stream

                Ok(Self::#variant_ident {
                    #create_token_stream
                })
            }
        });
    }

    quote::quote! {
        #[async_trait::async_trait]
        impl protocol_core::rw::AsyncReadable for #ident {
            async fn read_async<T>(mut read: &mut T) -> anyhow::Result<Self>
            where
                T: tokio::io::AsyncRead + std::marker::Unpin + Send + Sync
            {
                use protocol_core::rw::AsyncReadable;

                let id = *VarInt::read_async(&mut read).await?;

                match id {
                    #match_body
                    _ => panic!()
                }
            }
        }
    }
}

fn read_inputs(fields: &syn::Fields) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut result_token_stream = proc_macro2::TokenStream::new();
    let mut create_token_stream = proc_macro2::TokenStream::new();
    for field in fields {
        match &field.ty {
            syn::Type::Path(p) => {
                let ident = &field.ident;

                let r#type = p.path.segments.last().unwrap();

                let t_ident = &r#type.ident;

                match &r#type.arguments {
                    PathArguments::AngleBracketed(args) => result_token_stream.extend_one(quote! {
                        let #ident = #t_ident::#args::read_async(read).await?;
                    }),
                    _ => result_token_stream.extend_one(quote! {
                        let #ident = #t_ident::read_async(read).await?;
                    }),
                };
                create_token_stream.extend_one(quote! { #ident, })
            }
            _ => {
                panic!()
            }
        };
    }

    (result_token_stream, create_token_stream)
}
