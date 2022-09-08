use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use std::ops::Add;
use std::str::Split;
use syn::{Attribute, Variant};

struct StrFromEnumAttrs {
    optional_csv_file_path: Option<String>,
}

impl StrFromEnumAttrs {
    pub fn new(attrs: Vec<Attribute>) -> Self {
        let mut a: Vec<&Attribute> = attrs.iter()
            .filter(|attr: &&Attribute| attr.path.get_ident().unwrap().to_string() == "optional_csv_file_path")
            .collect();
        let optional_csv_file_path: Option<String> = a.pop().map(|it: &Attribute| it.tokens.to_string()
            .replace("(", "").replace(")", "").replace("\"", ""));
        Self { optional_csv_file_path }
    }
    pub fn optional_csv_file_path(&self) -> &Option<String> {
        &self.optional_csv_file_path
    }
}

/// It takes a CSV file and generates a `ToString` implementation for an enum
///
/// Arguments:
///
/// * `derive_input`: syn::DeriveInput
///
/// Returns:
///
/// A TokenStream.
pub fn impl_to_string(derive_input: syn::DeriveInput) -> TokenStream {
    let syn::DeriveInput { ident, attrs, data, .. } = derive_input;
    let variants_as_str: Vec<String> = if let syn::Data::Enum(data_enum) = data {
        data_enum.variants.iter().map(|v: &Variant| {
            v.ident.to_string()
        }).collect()
    } else {
        panic!("No Enum data.")
    };
    let attr_struct: StrFromEnumAttrs = StrFromEnumAttrs::new(attrs.clone());

    let data: String = match attr_struct.optional_csv_file_path() {
        Some(p) => {
            let csv_file_path: &String = p;
            // dbg!(&csv_file_path);
             String::from_utf8_lossy(&fs::read(csv_file_path.as_str())
                 .expect("Failed to read CSV file.")).to_string()
        },
        None => { // We'll mimic a csv file in a string here.
            let mut result: String = String::new();
            for variant in variants_as_str.as_slice() {
                result = result.add(
                    &format!("{};{}\n", variant, variant)
                );
            }
            result
        }
    };
    // dbg!(&data);
    let data_as_str_newline_split: Split<char> = data.split('\n');
    let mut keys: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut vals: Vec<String> = Vec::new();
    for line in data_as_str_newline_split {
        let line_separator_split: Option<(&str, &str)> = line.split_once(';');
        let _ = line_separator_split.map(|(key, val): (&str, &str)| {
            let mut key: String = String::from(key);
            let k_first_char: char = key.chars().next().expect("Key lacks first char");
            if k_first_char.is_numeric() {
                let num_in_english: &str = crate::english_numerical::match_numeric_to_english(k_first_char);
                key = key.replacen(k_first_char, num_in_english, 1);
            }
            key = key.replace('.', "").to_uppercase();
            if variants_as_str.contains(&key) {
                keys.push(key.parse().unwrap());
                vals.push(val.replace('\n', "")
                    .replace('\r', ""));
            }
        });
    }
    let quote: proc_macro2::TokenStream = quote! {
        impl ToString for #ident {
            fn to_string(&self) -> String {
                match self {
                    #(#ident::#keys => {
                        String::from( #vals )
                    },)*_ => {
                        panic!("to_string failed.")
                    }
                }
            }
        }
    };
    println!("Generated match stmt:\r\n{}", &quote.to_string());
    TokenStream::from(quote)
}