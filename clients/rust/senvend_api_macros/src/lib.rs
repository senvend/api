use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Attribute macro to implement conversions between a protobuf UUID struct and `uuid::Uuid`.
/// The conversion will fail, if the UUID version is not 4 (random), with Error:
/// senvend_api_utils::UuidParseError.
///
/// NOTE: To use this macro, it is required that the `senvend_api_utils` crate is included as a
/// dependency!!
#[proc_macro_attribute]
pub fn proto_uuid4(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let code = quote! {
        #ast

        impl #impl_generics TryFrom<&#name> for uuid::Uuid #ty_generics #where_clause {
            type Error = senvend_api_utils::UuidParseError;
            fn try_from(value: &#name) -> Result<Self, Self::Error> {
                let uuid = uuid::Uuid::from_u64_pair(value.msb, value.lsb);
                match uuid.get_version() {
                    Some(uuid::Version::Random) => Ok(uuid),
                    Some(v) => Err(senvend_api_utils::UuidParseError::InvalidVersion(v)),
                    None => Err(senvend_api_utils::UuidParseError::MissingVersion),
                }
            }
        }

        impl #impl_generics TryFrom<#name> for uuid::Uuid #ty_generics #where_clause {
            type Error = senvend_api_utils::UuidParseError;
            fn try_from(value: #name) -> Result<Self, Self::Error> {
                uuid::Uuid::try_from(&value)
            }
        }

        impl #impl_generics TryFrom<&uuid::Uuid> for #name #ty_generics #where_clause {
            type Error = senvend_api_utils::UuidParseError;
            fn try_from(value: &uuid::Uuid) -> Result<Self, Self::Error> {
                match value.get_version() {
                    Some(uuid::Version::Random) => {
                    let (msb, lsb) = value.as_u64_pair();
                    Ok(Self {msb, lsb})

                    },
                    Some(v) => Err(senvend_api_utils::UuidParseError::InvalidVersion(v)),
                    None => Err(senvend_api_utils::UuidParseError::MissingVersion),
                }

            }
        }

        impl #impl_generics TryFrom<uuid::Uuid> for #name #ty_generics #where_clause {
            type Error = senvend_api_utils::UuidParseError;
            fn try_from(value: uuid::Uuid) -> Result<Self, Self::Error> {
                Self::try_from(&value)
            }
        }

    };
    code.into()
}
