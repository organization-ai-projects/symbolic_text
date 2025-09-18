use crate::tokenizer::TokenKind;

/// Structure pour un token lemmatisé.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LemmatizedToken {
    pub raw: String,     // forme brute
    pub lemma: String,   // lemme (forme de base)
    pub canon: String,   // forme canonique (optionnel, pour chaînage)
    pub kind: TokenKind, // type de token
}
