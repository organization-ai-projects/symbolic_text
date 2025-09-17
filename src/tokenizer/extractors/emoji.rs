//! Extracteur pour les emojis
use unicode_segmentation::UnicodeSegmentation;

use crate::tokenizer::TokenKind;
use crate::tokenizer::token::Token;

pub fn extract_emojis(input: &str) -> Vec<(usize, usize, Token)> {
    let mut result = Vec::new();
    for (i, g) in input.grapheme_indices(true) {
        if let Some(c) = g.chars().next() {
            if (c >= '\u{1F600}' && c <= '\u{1F64F}')
                || (c >= '\u{1F300}' && c <= '\u{1F5FF}')
                || (c >= '\u{1F680}' && c <= '\u{1F6FF}')
                || (c >= '\u{2600}' && c <= '\u{26FF}')
                || (c >= '\u{2700}' && c <= '\u{27BF}')
                || (c >= '\u{1F900}' && c <= '\u{1F9FF}')
                || (c >= '\u{1FA70}' && c <= '\u{1FAFF}')
            {
                result.push((
                    i,
                    i + g.len(),
                    Token {
                        text: g.to_string(),
                        kind: TokenKind::Emoji,
                        byte_start: i,
                        byte_end: i + g.len(),
                    },
                ));
            }
        }
    }
    result
}
