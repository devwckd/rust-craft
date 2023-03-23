use quote::quote;

pub fn impl_writeable_sync(derive_input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return match &derive_input.data {
        syn::Data::Struct(s) => impl_writeable_sync_struct(s, derive_input),
        syn::Data::Enum(e) => impl_writeable_sync_enum(e, derive_input),
        _ => proc_macro2::TokenStream::new(),
    };
}

fn impl_writeable_sync_struct(
    s: &syn::DataStruct,
    derive_input: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    let result_token_stream = write_inputs(&s.fields, true);
    let ident = &derive_input.ident;

    let tokens = quote! {
        impl protocol_core::rw::SyncWriteable for #ident {
            fn write_sync<T>(&self, mut write: &mut T) -> anyhow::Result<()>
            where
                T: std::io::Write
            {
                use protocol_core::rw::SyncWriteable;

                #result_token_stream

                Ok(())
            }
        }
    };
    return tokens.into();
}

fn impl_writeable_sync_enum(
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
                VarInt::new(#index).write_sync(&mut write)?;
                #result_token_stream
            }
        });
    }

    quote::quote! {
        impl protocol_core::rw::SyncWriteable for #ident {
            fn write_sync<T>(&self, mut write: &mut T) -> anyhow::Result<()>
            where
                T: std::io::Write
            {
                use protocol_core::rw::SyncWriteable;

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
                    #accessor #ident.write_sync(write)?;
                });
            }
            _ => {
                panic!()
            }
        };
    }

    result_token_stream
}
