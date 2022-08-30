extern crate proc_macro;

use proc_macro::TokenStream;

use crate::implementations::{enum_from_str, str_from_enum};
use crate::misc::english_numerical;

// TODO: Generify implementations.

mod implementations {
    pub mod enum_from_str;
    pub mod str_from_enum;
}

mod misc {
    pub mod english_numerical;
}

#[proc_macro_derive(MimeTypeFromEnum, attributes(csv_file_path, ident_based))]
pub fn mime_type_from_enum_derive(input: TokenStream) -> TokenStream {
    str_from_enum::impl_mime_type(syn::parse_macro_input!(input))
}

/// It takes a Rust `TokenStream` as input, and returns a Rust `TokenStream` as output
///
/// english_number_prefix_to_numerical(bool)
/// # Example
/// ## (if english_number_prefix_to_numerical is false) // Default
/// ```
/// "SEVENZ" => { StaticFileExt::SEVENZ }
/// ```
/// ## (if english_number_prefix_to_numerical is true)
/// ```
/// "7Z" => { StaticFileExt::SEVENZ }
/// ```
/// Arguments:
///
/// * `input`: TokenStream - The input to the macro.
///
/// Returns:
///
/// A TokenStream.
#[proc_macro_derive(EnumFromStr, attributes(english_number_prefix_to_numerical))]
pub fn string_enum_derive(input: TokenStream) -> TokenStream {
    enum_from_str::impl_enum_from_str(syn::parse_macro_input!(input))
}