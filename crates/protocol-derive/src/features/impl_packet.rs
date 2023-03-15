use quote::quote;

pub fn impl_packet(derive_input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let ident = &derive_input.ident;
    let Some(packet_id) = parse_packet_id(&derive_input) else {
        panic!("packet id missing! add it using the attribute #[packet_id = 0x00]");
    };

    quote! {
        impl protocol_core::packet::Packet for #ident {
            const ID: i32 = #packet_id;
        }
    }
}

fn parse_packet_id(derive_input: &syn::DeriveInput) -> Option<i32> {
    let Some(attr) = derive_input.attrs.iter()
    .find(|attr| {
        attr.parse_meta().is_ok_and(|meta| {
            meta.path()
                .segments
                .last()
                .is_some_and(|segment| segment.ident.to_string() == "packet_id")
        })
    }) else {
        return None;
    };

    let Ok(meta) = attr.parse_meta() else {
        return None;
    };

    match meta {
        syn::Meta::Path(_) => panic!("packets need to be set in the format \"packet_id = 0x00\""),
        syn::Meta::List(_) => panic!("packets need to be set in the format \"packet_id = 0x00\""),
        syn::Meta::NameValue(name_value) => match &name_value.lit {
            syn::Lit::Int(int) => {
                let string = int.to_string();
                let without_prefix = string.trim_start_matches("0x");
                let int = i32::from_str_radix(without_prefix, 16);
                return if let Ok(int) = int { Some(int) } else { None };
            }
            _ => {
                panic!("packet ids must be a byte");
            }
        },
    };
}
