//! Extracteur pour les tirets et traits d’union
use crate::tokenizer::TokenKind;
use crate::tokenizer::token::Token;

/// Extrait les tirets isolés ou non capturés
pub fn extract_hyphens(input: &str, offset: usize) -> Vec<(usize, usize, Token)> {
    let mut tokens = Vec::new();
    for (i, ch) in input.char_indices() {
        if matches!(ch, '-' | '‐' | '‑' | '‒' | '–' | '—' | '―') {
            tokens.push((
                i + offset,
                i + offset + ch.len_utf8(),
                Token {
                    text: ch.to_string(),
                    kind: TokenKind::Symbol,
                    byte_start: i + offset,
                    byte_end: i + offset + ch.len_utf8(),
                },
            ));
        }
    }
    tokens
}
