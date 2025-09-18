# Symbolic Text

Universal tokenizer, lemmatizer and normalizer for NLP and AI in Rust.

## Installation

Add to your Cargo.toml:
```toml
symbolic_text = "0.1"
```

## Usage

```rust
use symbolic_text::tokenizer::tokenize;
let tokens = tokenize("This is an example!");
for t in tokens {
    println!("{}: {:?}", t.text, t.kind);
}
```

## Features
- Modular and strongly-typed tokenization
- Extraction of URL, email, hashtag, emoji, abbreviation, number, punctuation, groups, hyphens
- Normalization and lemmatization
- Extensible and open-source ready




## Documentation
- [French documentation](./docs/fr/README_fr.md)
- [Crates.io](https://crates.io/crates/symbolic_text)
- [GitHub](https://github.com/organization-ai-projects/symbolic_text)

## License
MIT
