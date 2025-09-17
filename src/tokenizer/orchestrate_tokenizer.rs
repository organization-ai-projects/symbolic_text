//! Module principal du tokenizer : orchestre tous les extracteurs et la segmentation

use crate::tokenizer::token::Token;
use crate::tokenizer::{extractors::*, TokenStream};

/// Orchestrateur principal : applique tous les extracteurs, résout les overlaps, segmente le reste
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut matches: Vec<(usize, usize, Token)> = Vec::new();
    // Appel de chaque extracteur
    matches.extend(extract_urls(input));
    matches.extend(extract_emails(input));
    matches.extend(extract_hashtags(input));
    matches.extend(extract_emojis(input));
    matches.extend(extract_abbr(input));
    matches.extend(extract_numbers(input));
    // Trie et résolution des chevauchements (overlap)
    matches.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1))); // start asc, end desc
    let mut packed = Vec::new();
    let mut cur_end = 0;
    for (s, e, tok) in matches {
        if s >= cur_end {
            packed.push((s, e, tok));
            cur_end = e;
        }
    }
    // Bufferisation uniquement des gaps non extraits
    let mut tokens = packed
        .iter()
        .map(|(_, _, tok)| tok.clone())
        .collect::<Vec<_>>();
    let mut last_end = 0;
    for (start, end, _) in &packed {
        if last_end < *start {
            let gap = &input[last_end..*start];
            if !gap.trim().is_empty() {
                let mut stream = TokenStream::new();
                stream.push_chunk(gap);
                tokens.extend(stream.flush());
            }
        }
        last_end = *end;
    }
    // Traiter le gap final si besoin
    if last_end < input.len() {
        let gap = &input[last_end..];
        if !gap.trim().is_empty() {
            let mut stream = TokenStream::new();
            stream.push_chunk(gap);
            tokens.extend(stream.flush());
        }
    }
    tokens
}
