//! Extracteur pour les emails
use once_cell::sync::Lazy;
use regex::Regex;

use crate::tokenizer::token::Token;
use crate::tokenizer::TokenKind;

pub fn extract_emails(input: &str) -> Vec<(usize, usize, Token)> {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+").unwrap());
    RE.find_iter(input)
        .map(|m| {
            let token = Token {
                text: m.as_str().to_string(),
                kind: TokenKind::Email,
                byte_start: m.start(),
                byte_end: m.end(),
            };
            (m.start(), m.end(), token)
        })
        .collect()
}
