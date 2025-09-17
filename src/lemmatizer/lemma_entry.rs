#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, bincode::Encode, bincode::Decode)]
pub struct LemmaEntry {
    pub lemma: String,
    pub forms: Vec<String>,
}