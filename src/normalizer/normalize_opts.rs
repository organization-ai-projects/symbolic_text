/// Options de normalisation.
#[derive(Debug, Clone)]
pub struct NormalizerOpts {
    pub use_nfkc: bool,      // normalisation Unicode NFKC
    pub case_fold: bool,     // minuscule / case-fold
    pub strip_accents: bool, // suppression des diacritiques (NFD + filtre + NFC)
    pub clean_spaces: bool,  // normalisation/compaction des espaces
    pub unify_quotes: bool,  // uniformisation des apostrophes/guillemets
    pub strip_control: bool, // suppression des caractères de contrôle (hors tab/lf/cr)
}

impl Default for NormalizerOpts {
    fn default() -> Self {
        Self {
            use_nfkc: true,
            case_fold: true,
            strip_accents: false,
            clean_spaces: true,
            unify_quotes: true,
            strip_control: true,
        }
    }
}
