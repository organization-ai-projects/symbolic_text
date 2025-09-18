use crate::lemmatizer::LemmaEntry;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, bincode::Encode, bincode::Decode)]
pub struct LemmaEntries {
    pub entries: Vec<LemmaEntry>,
}
