pub mod lemmatizer_config;
pub mod lemma_entry;
pub mod lemma_entries;
pub mod lemmatizer;
pub mod lemmatized_token;

pub use lemmatizer_config::LemmatizerConfig;
pub use lemma_entry::LemmaEntry;
pub use lemma_entries::LemmaEntries;
pub use lemmatizer::lemmatize;
pub use lemmatized_token::LemmatizedToken;