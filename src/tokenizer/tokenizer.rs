//! Module principal du tokenizer : orchestre tous les extracteurs et la segmentation

use crate::tokenizer::extractors::*;
use crate::tokenizer::token::Token;

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
    // Segmentation du reste (à compléter avec une logique classique ou Unicode)
    // ...
    // Retourne les tokens extraits
    packed.into_iter().map(|(_, _, tok)| tok).collect()
}
