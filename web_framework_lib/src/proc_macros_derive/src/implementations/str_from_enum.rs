use proc_macro::TokenStream;
use quote::quote;
use std::str::Split;
use std::fs;

pub fn impl_mime_type(derive_input: syn::DeriveInput) -> TokenStream {
    let syn::DeriveInput { ident, .. } = derive_input;
    let arr: Vec<u8> = fs::read("src/proc_macros_derive/resource/mime-types.csv")
        .expect("UwU");
    let str: &str = &*String::from_utf8_lossy(&arr);
    let newline_split: Split<char> = str.split('\n');
    let mut keys: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut vals: Vec<String> = Vec::new();
    for line in newline_split {
        println!("MimeType: {}", line);
        let separator_split: (&str, &str) = line.split_once(';').unwrap();
        let mut key: String = separator_split.0.replace('.', "").to_uppercase();
        let first_char: char = key.chars().nth(0).unwrap();
        if first_char.is_numeric() {
            let num_in_lingua_franca: &str = crate::english_numerical::match_numerica_to_lingua_franca(key.chars().nth(0)
                .expect("No first char WTF"));
            key = key.replace(first_char, num_in_lingua_franca);
        }
        keys.push(key.parse().unwrap());
        vals.push(separator_split.1.replace("\r\n", "")
            .replace("\r", ""));
    }
    let quote: proc_macro2::TokenStream = quote! {
        impl #ident {
            pub fn mime_type(&self) -> Result<&str, ()> {
                match self {
                    #(#ident::#keys => {
                        Ok( #vals )
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