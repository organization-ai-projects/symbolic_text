//! Extracteur pour les hashtags
use once_cell::sync::Lazy;
use regex::Regex;

use crate::tokenizer::TokenKind;
use crate::tokenizer::token::Token;

pub fn extract_hashtags(input: &str) -> Vec<(usize, usize, Token)> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"#[\w_]+").unwrap());
    RE.find_iter(input)
        .map(|m| {
            let token = Token {
                text: m.as_str().to_string(),
                kind: TokenKind::Hashtag,
                byte_start: m.start(),
                byte_end: m.end(),
            };
            (m.start(), m.end(), token)
        })
        .collect()
}
