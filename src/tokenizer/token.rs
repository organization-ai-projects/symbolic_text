//! Définition du token universel pour NLP/IA
use super::token_kind::TokenKind;

// Ajout de la variante Url à TokenKind si elle n'existe pas déjà

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub text: String,
    pub kind: TokenKind,
    pub byte_start: usize,
    pub byte_end: usize,
}
