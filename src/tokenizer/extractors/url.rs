//! Extracteur pour les URLs
use once_cell::sync::Lazy;
use regex::Regex;

use crate::tokenizer::token::Token;
use crate::tokenizer::TokenKind;

pub fn extract_urls(input: &str) -> Vec<(usize, usize, Token)> {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"https?://[\w\-\.~:/?#\[\]@!$&'()*+,;=%]+").unwrap());
    RE.find_iter(input)
        .map(|m| {
            let token = Token {
                text: m.as_str().to_string(),
                kind: TokenKind::Other, // À spécialiser si besoin
                byte_start: m.start(),
                byte_end: m.end(),
            };
            (m.start(), m.end(), token)
        })
        .collect()
}
