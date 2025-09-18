//! Définition du token universel pour NLP/IA
use crate::types::token_kind::TokenKind;

// Ajout de la variante Url à TokenKind si elle n'existe pas déjà

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub text: String,
    pub kinds: Vec<(TokenKind, bool)>, // type détecté + statut de validation
    pub byte_start: usize,
    pub byte_end: usize,
}
