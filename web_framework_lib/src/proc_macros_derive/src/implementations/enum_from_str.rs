use proc_macro::TokenStream;
use quote::quote;
use std::iter::Map;
use syn::Attribute;

struct EnumFromStrAttrs {
    english_number_prefix_to_numerical: Option<bool>
}

impl EnumFromStrAttrs {
    pub fn new(attrs: Vec<Attribute>) -> Self {
        let mut a: Vec<&Attribute> = attrs.iter()
            .filter(|attr: &&Attribute| attr.path.get_ident().unwrap().to_string() == "english_number_prefix_to_numerical")
            .collect();
        let english_number_prefix_to_numerical: Option<bool> = a.pop().map(|it: &Attribute| {
            let str: String = it.tokens.to_string()
                .replace("(", "").replace(")", "").replace("\"", "");
            str.parse().expect("Failed parse")
        });
        Self { english_number_prefix_to_numerical }
    }
    pub fn english_number_prefix_to_numerical(&self) -> bool {
        self.english_number_prefix_to_numerical
            .unwrap_or_else(|| false)
    }
}

/// It takes in a `syn::DeriveInput` and returns a `TokenStream` that implements the `FromStr` trait for
/// the enum
///
/// Arguments:
///
/// * `derive_input`: syn::DeriveInput
///
/// Returns:
///
/// A TokenStream.
pub fn impl_enum_from_str(derive_input: syn::DeriveInput) -> TokenStream {
    let syn::DeriveInput { ident, data, attrs, .. } = derive_input;

    let attr_struct: &EnumFromStrAttrs = &EnumFromStrAttrs::new(attrs);

    let output_token_stream = match data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let variant_token_identity_iterator: Map<syn::punctuated::Iter<syn::Variant>, fn(&syn::Variant) -> &syn::Ident> =
                variants.iter().map(|v| &v.ident);
            let mut variant_token_identity_string_vec: Vec<String> =
                variants.iter().map(|v| {
                    let s: String = v.ident.to_string();
                    println!(":{}\n", s);
                    s
                }).collect();
            // If #[english_number_prefix_to_numerical(bool)] is set.
            if attr_struct.english_number_prefix_to_numerical() {
                variant_token_identity_string_vec = variant_token_identity_string_vec.iter().map(|s| {
                    let num_res: Result<&str, ()> = crate::english_numerical::starts_with_numeric_english(&s);
                    println!("{}", s);
                    if num_res.is_ok() {
                        let num_res: &str = num_res.unwrap();
                        let new_s: String = s.replace(num_res,
                                      &crate::english_numerical::match_english_to_numeric(&num_res).to_string());
                        new_s
                    } else {
                        s.to_string()
                    }
                }).collect();
            }
            let variant_token_identity_string_iterator: std::slice::Iter<String> =
                variant_token_identity_string_vec.iter();
            quote! {
                impl std::str::FromStr for #ident {
                    type Err = ();
                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        match s.to_uppercase().as_str() {
                            #(#variant_token_identity_string_iterator => {
                                Ok( #ident::#variant_token_identity_iterator )
                            },)*
                            _ => {
                                Err(())
                            }
                        }
                    }
                }
            }
        },
        _ => {
            panic!("NOT ENUM")
        }
    };
    println!("Generated match stmt:\r\n{}", &output_token_stream.to_string());
    output_token_stream.into()
}