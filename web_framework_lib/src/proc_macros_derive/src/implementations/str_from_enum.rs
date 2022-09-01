use proc_macro::TokenStream;
use quote::quote;
use std::str::Split;
use std::fs;
use syn::{Attribute, Variant};
use crate::english_numerical::match_numeric_to_english;

struct StrFromEnumAttrs {
    optional_csv_file_path: Option<String>,
}

impl StrFromEnumAttrs {
    pub fn new(input: Vec<Attribute>) -> Self {
        let attrs: Vec<Attribute> = input;
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

// TODO: Document.
pub fn impl_mime_type(derive_input: syn::DeriveInput) -> TokenStream {
    let syn::DeriveInput { ident, attrs, data, .. } = derive_input;

    let variants_as_str: Vec<String> = if let syn::Data::Enum(data_enum) = data {
        data_enum.variants.iter().map(|v: &Variant| {
            let mut tmp: String = v.ident.to_string();
            let first_char: char = tmp.chars().nth(0).unwrap();
            if first_char.is_numeric() {
                tmp = tmp.replace(first_char,
                                  match_numeric_to_english(first_char));
            }
            tmp
        }).collect()
    } else {
        panic!("No Enum data.")
    };

    let attr_struct: StrFromEnumAttrs = StrFromEnumAttrs::new(attrs.clone());

    let csv_file_path: &String;
    match attr_struct.optional_csv_file_path() {
        Some(p) => { csv_file_path = p; dbg!(&csv_file_path); },
        None => {
            panic!("You need to specify helper attribute #[optional_csv_file_path(path)], I love irony.")
        }
    }
    let file_as_u8_vec: Vec<u8> = fs::read(csv_file_path.as_str())
        .expect("Failed to read CSV file.");
    let file_as_str: &str = &*String::from_utf8_lossy(&file_as_u8_vec);
    let file_as_str_newline_split: Split<char> = file_as_str.split('\n');
    let mut keys: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut vals: Vec<String> = Vec::new();
    for line in file_as_str_newline_split {
        println!("MimeType: {}", line);
        let line_separator_split: (&str, &str) = line.split_once(';').unwrap();
        let mut key: String = line_separator_split.0.replace('.', "").to_uppercase();
        // Checking if first char of key is numeric.
        let first_char: char = key.chars().nth(0).unwrap();
        if first_char.is_numeric() {
            // Replacing first char with english word for numeric value.
            let num_in_lingua_franca: &str = crate::english_numerical::match_numeric_to_english(key.chars().nth(0)
                .expect("No first char WTF"));
            key = key.replacen(first_char, num_in_lingua_franca, 1);
        }
        // If key from csv file is represented in enum.
        if variants_as_str.contains(&key) {
            keys.push(key.parse().unwrap());
            vals.push(line_separator_split.1.replace("\r\n", "")
                .replace("\r", ""));
        }
    }
    let quote: proc_macro2::TokenStream = quote! {
        impl #ident {
            pub fn as_str(&self) -> Result<String, ()> {
                match self {
                    #(#ident::#keys => {
                        Ok( String::from( #vals ) )
                    },)*_ => {
                        Err(())
                    }
                }
            }
        }
    };
    println!("Generated match stmt:\r\n{}", quote);
    TokenStream::from(quote)
}