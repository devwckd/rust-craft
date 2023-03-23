use quote::quote;
use syn::PathArguments;

pub fn impl_readable_sync(derive_input: &syn::DeriveInput) -> proc_macro2::TokenStream {
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
        impl protocol_core::rw::SyncReadable for #ident {
            fn read_sync<T>(read: &mut T) -> anyhow::Result<Self>
            where
                T: std::io::Read
            {
                use protocol_core::rw::SyncReadable;

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
        impl protocol_core::rw::SyncReadable for #ident {
            fn read_sync<T>(mut read: &mut T) -> anyhow::Result<Self>
            where
                T: std::io::Read
            {
                use protocol_core::{data::VarInt, rw::SyncReadable};

                let id = *VarInt::read_sync(&mut read)?;

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
                        let #ident = #t_ident::#args::read_sync(read)?;
                    }),
                    _ => result_token_stream.extend_one(quote! {
                        let #ident = #t_ident::read_sync(read)?;
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
