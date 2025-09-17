// src/lemmatizer.rs
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::format_converter::ensure_bin_up_to_date;
use crate::tokenizer::TokenKind;
use crate::lemmatizer::LemmaEntries;
use crate::lemmatizer::LemmatizerConfig;
use crate::lemmatizer::LemmatizedToken;

/// Chargement générique des lemmes pour n'importe quelle langue
fn load_lemmatization_forms(lang: &str) -> HashMap<String, String> {
    let mut forms_map = HashMap::new();
    let dir_str = format!("crates/symbolic_text/data/{}/lemmatization/ron", lang);
    let dir = Path::new(&dir_str);
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "ron") {
                let bin_path = path.with_extension("bin");
                let _ = ensure_bin_up_to_date(&path, &bin_path);
                if let Ok(bin_content) = fs::read(&bin_path) {
                    if let Ok((lemma_entries, _)) = bincode::decode_from_slice::<LemmaEntries, _>(
                        &bin_content,
                        bincode::config::standard(),
                    ) {
                        for entry in lemma_entries.entries {
                            for f in entry.forms {
                                forms_map.insert(f, entry.lemma.clone());
                            }
                        }
                    } else {
                        eprintln!("Erreur de conversion bincode pour {:?}", bin_path);
                    }
                } else {
                    eprintln!("Impossible de lire le fichier bin {:?}", bin_path);
                }
            }
        }
    }
    forms_map
}

/// API principale adaptée au format groupé .ron
pub fn lemmatize(tokens: &[String], config: &LemmatizerConfig) -> Vec<LemmatizedToken> {
    let stemmer = match config.language.as_str() {
        "fr" => Some(Stemmer::create(Algorithm::French)),
        "en" => Some(Stemmer::create(Algorithm::English)),
        _ => None,
    };

    let forms_map = load_lemmatization_forms(&config.language);

    tokens
        .iter()
        .map(|raw| {
            let stem = stemmer.as_ref().map(|s| s.stem(raw).to_string());
            let lemma = forms_map
                .get(raw)
                .cloned()
                .unwrap_or_else(|| stem.clone().unwrap_or_else(|| raw.clone()));
            LemmatizedToken {
                raw: raw.clone(),
                lemma,
                canon: raw.clone(),
                kind: TokenKind::Word,
            }
        })
        .collect()
}