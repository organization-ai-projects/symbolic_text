//! Extracteur pour les nombres
use once_cell::sync::Lazy;
use regex::Regex;

use crate::tokenizer::token::Token;
use crate::tokenizer::TokenKind;

pub fn extract_numbers(input: &str) -> Vec<(usize, usize, Token)> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[+-]?\d+(?:[.,]\d+)?").unwrap());
    RE.find_iter(input)
        .map(|m| {
            let token = Token {
                text: m.as_str().to_string(),
                kind: TokenKind::Number,
                byte_start: m.start(),
                byte_end: m.end(),
            };
            (m.start(), m.end(), token)
        })
        .collect()
}
