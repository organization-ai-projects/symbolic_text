# Documentation française

Bienvenue dans la documentation de Symbolic Text.

## Présentation
Symbolic Text est un tokenizer, lemmatizer et normalizer universel pour le NLP et l’IA en Rust.

## Installation
Ajoutez à votre fichier Cargo.toml :
```toml
symbolic_text = "0.1"
```

## Utilisation
```rust
use symbolic_text::tokenizer::tokenize;
let tokens = tokenize("Ceci est un exemple !");
for t in tokens {
    println!("{}: {:?}", t.text, t.kind);
}
```

## Fonctionnalités
- Tokenisation modulaire et typée
- Extraction d’URL, email, hashtag, emoji, abréviation, nombre, ponctuation, groupes, tirets
- Normalisation et lemmatisation
- Extensible et prêt pour l’open source

## Licence
MIT
