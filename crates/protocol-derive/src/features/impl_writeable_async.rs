use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_writeable_async(derive_input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return match &derive_input.data {
        syn::Data::Struct(s) => impl_writeable_async_struct(s, derive_input),
        syn::Data::Enum(e) => impl_writeable_async_enum(e, derive_input),
        _ => proc_macro2::TokenStream::new(),
    };
}

fn impl_writeable_async_struct(
    s: &syn::DataStruct,
    derive_input: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    let result_token_stream = write_inputs(&s.fields, true);
    let ident = &derive_input.ident;

    let tokens = quote! {
        #[async_trait::async_trait]
        impl protocol_core::rw::AsyncWriteable for #ident {
            async fn write_async<T>(&self, mut write: &mut T) -> anyhow::Result<()>
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

fn impl_writeable_async_enum(
    e: &syn::DataEnum,
    derive_input: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    let ident = &derive_input.ident;
    let mut match_body = proc_macro2::TokenStream::new();

    for (index, variant) in e.variants.iter().enumerate() {
        let index = index as i32;
        let variant_ident = &variant.ident;

        let mut field_idents = proc_macro2::TokenStream::new();
        for field in &variant.fields {
            let field_ident = &field.ident;
            field_idents.extend_one(quote::quote! { #field_ident, });
        }
        let result_token_stream = write_inputs(&variant.fields, false);
        match_body.extend_one(quote::quote! {
            Self::#variant_ident { #field_idents } => {
                VarInt::new(#index).write_async(&mut write).await?;
                #result_token_stream
            }
        });
    }

    quote::quote! {
        #[async_trait::async_trait]
        impl protocol_core::rw::AsyncWriteable for #ident {
            async fn write_async<T>(&self, mut write: &mut T) -> anyhow::Result<()>
            where
                T: tokio::io::AsyncWrite + std::marker::Unpin + Send + Sync
            {
                use protocol_core::rw::AsyncWriteable;

                match &self {
                    #match_body
                    _ => panic!()
                }

                Ok(())
            }
        }
    }
}

fn write_inputs(fields: &syn::Fields, has_self: bool) -> proc_macro2::TokenStream {
    let mut result_token_stream = proc_macro2::TokenStream::new();
    for field in fields {
        match &field.ty {
            syn::Type::Path(_) => {
                let ident = &field.ident;
                let accessor = if has_self {
                    quote::quote! { self. }
                } else {
                    quote::quote!()
                };

                result_token_stream.extend_one(quote! {
                    #accessor #ident.write_async(write).await?;
                });
            }
            _ => {
                panic!()
            }
        };
    }

    result_token_stream
}
