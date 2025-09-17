//! Extracteur par défaut : gère uniquement les segments non capturés ou flous
use crate::token_kind::TokenKind;
use crate::tokenizer::token::Token;

/// Découpe grossière pour les cas non capturés par les extracteurs spécialisés
/// Peut intégrer des heuristiques/statistiques pour typage
pub fn extract_default(input: &str, offset: usize) -> Vec<Token> {
    let mut tokens = Vec::new();
    // Découpage naïf par espace, à remplacer par heuristique/statistique si besoin
    let mut byte_pos = offset;
    for word in input.split_whitespace() {
        let kind = if word.chars().all(|c| c.is_alphanumeric()) {
            TokenKind::Word
        } else {
            TokenKind::Other
        };
        tokens.push(Token {
            text: word.to_string(),
            kind,
            byte_start: byte_pos,
            byte_end: byte_pos + word.len(),
        });
        byte_pos += word.len() + 1; // +1 pour l'espace
    }
    tokens
}
