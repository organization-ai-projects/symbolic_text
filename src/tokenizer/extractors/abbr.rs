//! Extracteur pour les abréviations
use regex::Regex;

use crate::tokenizer::token::Token;
use crate::tokenizer::TokenKind;

pub fn extract_abbr(input: &str) -> Vec<(usize, usize, Token)> {
    let re = Regex::new(r"\b([A-Za-z]{1,10}\.)\b").unwrap();
    re.find_iter(input)
        .map(|m| {
            let token = Token {
                text: m.as_str().to_string(),
                kind: TokenKind::Abbr,
                byte_start: m.start(),
                byte_end: m.end(),
            };
            (m.start(), m.end(), token)
        })
        .collect()
}
