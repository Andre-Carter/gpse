pub mod errors;
pub mod normalization;
pub mod tokenization;
pub mod validation;

pub fn capitalize_first_letter(input: &str) -> String {
    let mut chars = input.chars();

    match chars.next() {
        Some(first_char) => {
            first_char.to_uppercase().collect::<String>() + chars.as_str()
        }
        None => String::new(),
    }
}