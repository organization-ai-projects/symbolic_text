use crate::tokenizer::TokenKind;

/// Résultat de la normalisation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedToken {
    pub raw: String,   // forme brute
    pub canon: String, // forme canonique (selon options)
    pub kind: TokenKind,
}
