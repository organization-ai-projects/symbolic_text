//! Extracteur pour la ponctuation officielle (ISO)
use crate::tokenizer::TokenKind;
use crate::tokenizer::token::Token;

/// Extrait la ponctuation officielle non capturée par les autres extracteurs
pub fn extract_punctuation(input: &str, offset: usize) -> Vec<(usize, usize, Token)> {
    let mut tokens = Vec::new();
    for (i, ch) in input.char_indices() {
        if matches!(ch, '.' | ',' | ';' | ':' | '!' | '?' | '…') {
            tokens.push((
                i + offset,
                i + offset + ch.len_utf8(),
                Token {
                    text: ch.to_string(),
                    kind: TokenKind::Punct,
                    byte_start: i + offset,
                    byte_end: i + offset + ch.len_utf8(),
                },
            ));
        }
    }
    tokens
}
