//! API streaming et gestion d’état pour le tokenizer
use crate::tokenizer::token::Token;

pub struct Tokenizer {
    // À compléter avec les états nécessaires
}

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer {}
    }
    pub fn tokenize(&mut self, input: &str) -> Vec<Token> {
        // À compléter avec la logique de segmentation
        vec![]
    }
    pub fn tokenize_stream<'a>(
        &'a mut self,
        chunk: &'a str,
        is_last: bool,
    ) -> impl Iterator<Item = Token> + 'a {
        // À compléter pour le streaming
        std::iter::empty()
    }
}
