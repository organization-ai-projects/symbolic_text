//! Extracteur pour les emojis
use unicode_segmentation::UnicodeSegmentation;

use crate::tokenizer::token::Token;
use crate::tokenizer::TokenKind;

pub fn extract_emojis(input: &str) -> Vec<(usize, usize, Token)> {
    let mut result = Vec::new();
    for (i, g) in input.grapheme_indices(true) {
        if let Some(c) = g.chars().next() {
            if ('\u{1F600}'..='\u{1F64F}').contains(&c)
                || ('\u{1F300}'..='\u{1F5FF}').contains(&c)
                || ('\u{1F680}'..='\u{1F6FF}').contains(&c)
                || ('\u{2600}'..='\u{26FF}').contains(&c)
                || ('\u{2700}'..='\u{27BF}').contains(&c)
                || ('\u{1F900}'..='\u{1F9FF}').contains(&c)
                || ('\u{1FA70}'..='\u{1FAFF}').contains(&c)
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
