use unicode_normalization::{char::is_combining_mark, UnicodeNormalization};

use crate::{
    normalizer::{NormalizedToken, NormalizerOpts},
    tokenizer::TokenKind,
};

/// API pratique (options par défaut).
pub fn normalize(tokens: &[String]) -> Vec<NormalizedToken> {
    normalize_with(tokens, &NormalizerOpts::default())
}

/// API avec options.
pub fn normalize_with(tokens: &[String], opts: &NormalizerOpts) -> Vec<NormalizedToken> {
    tokens
        .iter()
        .map(|raw| {
            let mut s = raw.as_str().to_owned();

            // 1) Unicode NFKC (avant tout, pour stabiliser les formes)
            if opts.use_nfkc {
                s = s.nfkc().collect();
            }

            // 2) Case fold (minuscule)
            if opts.case_fold {
                s = s.to_lowercase();
            }

            // 3) Unifier les guillemets/apostrophes (optionnel, indépendant des espaces)
            if opts.unify_quotes {
                s = unify_quotes(&s);
            }

            // 4) Nettoyage des caractères de contrôle
            if opts.strip_control {
                // On supprime les Cc sauf TAB/LF/CR (qu’on laissera convertir en espace)
                s = s
                    .chars()
                    .filter(|&c| !(c.is_control() && c != '\t' && c != '\n' && c != '\r'))
                    .collect();
            }

            // 5) Normalisation des espaces (y compris NBSP, NNBSP, ZWSP, BOM) + compaction
            if opts.clean_spaces {
                s = normalize_spaces(&s);
            }

            // 6) (Optionnel) suppression des accents
            if opts.strip_accents {
                let no_marks: String = s.nfd().filter(|c| !is_combining_mark(*c)).collect();
                s = no_marks.nfc().collect();
            }

            // 7) Trim final pour stabiliser la canon
            s = s.trim().to_string();

            let kind = classify(&s);
            NormalizedToken {
                raw: raw.clone(),
                canon: s,
                kind,
            }
        })
        .collect()
}

fn unify_quotes(s: &str) -> String {
    // remplace les variantes typographiques par ' et "
    s.chars()
        .map(|c| match c {
            // apostrophes
            '’' | '‘' | '‚' | '`' | '´' => '\'',
            // guillemets
            '“' | '”' | '„' | '«' | '»' => '"',
            _ => c,
        })
        .collect()
}

fn normalize_spaces(s: &str) -> String {
    // convertit une grande variété d'espaces en ' ', supprime BOM, ZWSP, etc., puis compacte

    const TO_SPACE: &[char] = &[
        '\t', '\n', '\r', '\u{00A0}', // NBSP
        '\u{202F}', // NNBSP (narrow no-break space)
        '\u{2009}', // thin space
        '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}',
        '\u{200A}', // divers espaces
    ];

    const DROP: &[char] = &[
        '\u{200B}', // ZWSP
        '\u{FEFF}', // BOM
    ];

    let mut buf = String::with_capacity(s.len());
    let mut last_space = false;

    for c in s.chars() {
        if DROP.contains(&c) {
            continue; // on jette
        } else if TO_SPACE.contains(&c) || c.is_whitespace() {
            if !last_space {
                buf.push(' ');
                last_space = true;
            }
        } else {
            buf.push(c);
            last_space = false;
        }
    }

    // pas de trim ici : laissé au caller (on le fait déjà plus haut)
    buf
}

fn classify(s: &str) -> TokenKind {
    if s.is_empty() {
        return TokenKind::Other;
    }
    if is_group(s) {
        return TokenKind::Group;
    }
    if s.chars().all(|c| c.is_ascii_digit()) {
        return TokenKind::Number;
    }
    if s.len() == 1 && matches!(s.chars().next().unwrap(), '.' | ',' | ';' | ':' | '!' | '?') {
        return TokenKind::Punct;
    }
    if s.chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return TokenKind::Word;
    }
    TokenKind::Other
}

fn is_group(s: &str) -> bool {
    let mut it = s.chars();
    let first = match it.next() {
        Some(c) => c,
        None => return false,
    };
    let last = match s.chars().last() {
        Some(c) => c,
        None => return false,
    };
    matches!(
        (first, last),
        ('(', ')') | ('[', ']') | ('{', '}') | ('"', '"') | ('\'', '\'')
    )
}
