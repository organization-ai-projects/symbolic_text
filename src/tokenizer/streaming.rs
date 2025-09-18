//! API streaming et gestion d’état pour le tokenizer
use crate::tokenizer::token::Token;
use crate::tokenizer::tokenize;

/// Structure minimale pour gérer le flux et bufferiser les chunks
#[derive(Default)]
pub struct TokenStream {
    buffer: String,
}

impl TokenStream {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    /// Ajoute un chunk au buffer
    pub fn push_chunk(&mut self, chunk: &str) {
        self.buffer.push_str(chunk);
    }

    /// Retourne les tokens du buffer courant (en utilisant le vrai tokenizer)
    pub fn flush(&mut self) -> Vec<Token> {
        let tokens = tokenize(&self.buffer);
        self.buffer.clear();
        tokens
    }
}
