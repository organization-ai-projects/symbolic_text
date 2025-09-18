/// Catégories de tokens communes à la normalisation et la lemmatisation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Word,
    Number,
    Group,  // bloc entre (), [], {}, "" ou ''
    Punct,  // . , ; : ! ?
    Symbol, // $, €, #, %, @, _, /, \
    Url,
    Email,
    Hashtag,
    Emoji,
    Abbr,
    Mention,
    Other,
}
