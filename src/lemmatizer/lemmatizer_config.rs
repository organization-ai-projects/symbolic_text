/// Configuration pour la lemmatisation (à étendre selon besoins).
#[derive(Debug, Clone)]
pub struct LemmatizerConfig {
    pub language: String, // langue ("fr", "en", etc.)
}

impl Default for LemmatizerConfig {
    fn default() -> Self {
        Self {
            language: "fr".to_string(),
        }
    }
}
