//! Extracteur pour les délimiteurs de groupe et guillemets (quotes)
use crate::tokenizer::token::Token;
use crate::tokenizer::TokenKind;

/// Extrait les délimiteurs de groupe et guillemets non capturés par les autres extracteurs
pub fn extract_group_delimiters(input: &str, offset: usize) -> Vec<(usize, usize, Token)> {
    let mut tokens = Vec::new();
    let delimiters = [
        '(', ')', '[', ']', '{', '}', '«', '»', '“', '”', '"', '\'', '’',
    ];
    for (i, ch) in input.char_indices() {
        if delimiters.contains(&ch) {
            tokens.push((
                i + offset,
                i + offset + ch.len_utf8(),
                Token {
                    text: ch.to_string(),
                    kind: TokenKind::Group,
                    byte_start: i + offset,
                    byte_end: i + offset + ch.len_utf8(),
                },
            ));
        }
    }
    tokens
}
