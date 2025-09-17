# Symbolic Text

Tokenizer, lemmatizer et normalizer universel pour NLP/IA en Rust.

## Installation

Ajoutez à votre Cargo.toml :
```toml
symbolic_text = "0.1"
```

## Usage

```rust
use symbolic_text::tokenizer::tokenize;
let tokens = tokenize("Ceci est un exemple !");
for t in tokens {
    println!("{}: {:?}", t.text, t.kind);
}
```

## Fonctionnalités
- Tokenization modulaire et typée
- Extraction URL, email, hashtag, emoji, abbr, nombre, ponctuation, groupes, tirets
- Normalisation et lemmatisation
- Extensible et prêt pour l’open source

## Documentation
- [Crates.io](https://crates.io/crates/symbolic_text)
- [GitHub](https://github.com/organization-ai-projects/symbolic_text)

## Licence
MIT
