use quote::quote;

pub fn impl_writeable(derive_input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    if let syn::Data::Struct(s) = &derive_input.data {
        let mut result_token_stream = proc_macro2::TokenStream::new();
        for field in &s.fields {
            match &field.ty {
                syn::Type::Path(_) => {
                    let ident = &field.ident;
                    result_token_stream.extend_one(quote! {
                        self.#ident.write(write)?;
                    });
                }
                _ => {
                    panic!()
                }
            };
        }

        let ident = &derive_input.ident;

        let tokens = quote! {
            impl protocol_core::rw::Writeable for #ident {
                fn write<T>(&self, write: &mut T) -> anyhow::Result<()>
                where
                    T: std::io::Write
                {
                    use protocol_core::rw::Writeable;

                    #result_token_stream

                    Ok(())
                }
            }
        };
        return tokens.into();
    }
    proc_macro2::TokenStream::new()
}
