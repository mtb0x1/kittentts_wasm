use cmudict_fast::{Cmudict, Rule};
use std::{collections::HashMap, str::FromStr, sync::LazyLock};
use thiserror::Error;

pub const DICT: &str = include_str!("../../web/cmu.dict");

static IPA_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("AA", "ɑ"),
        ("AA1", "ɑː"),
        ("AA2", "ɑː"),
        ("AE", "æ"),
        ("AE1", "æ"),
        ("AE2", "æ"),
        ("AH", "ə"),
        ("AH0", "ə"),
        ("AH1", "ʌ"),
        ("AH2", "ə"),
        ("AO", "ɔ"),
        ("AO1", "ɔː"),
        ("AO2", "ɔː"),
        ("AW", "aʊ"),
        ("AW1", "aʊ"),
        ("AW2", "aʊ"),
        ("AY", "aɪ"),
        ("AY1", "aɪ"),
        ("AY2", "aɪ"),
        ("EH", "ɛ"),
        ("EH1", "ɛ"),
        ("EH2", "ɛ"),
        ("ER", "ɜ"),
        ("ER0", "ɚ"),
        ("ER1", "ɜː"),
        ("ER2", "ɜː"),
        ("EY", "eɪ"),
        ("EY1", "eɪ"),
        ("EY2", "eɪ"),
        ("IH", "ɪ"),
        ("IH0", "ɪ"),
        ("IH1", "ɪ"),
        ("IH2", "ɪ"),
        ("IY", "i"),
        ("IY1", "iː"),
        ("IY2", "iː"),
        ("OW", "oʊ"),
        ("OW1", "oʊ"),
        ("OW2", "oʊ"),
        ("OY", "ɔɪ"),
        ("OY1", "ɔɪ"),
        ("OY2", "ɔɪ"),
        ("UH", "ʊ"),
        ("UH1", "ʊ"),
        ("UH2", "ʊ"),
        ("UW", "u"),
        ("UW1", "uː"),
        ("UW2", "uː"),
        ("B", "b"),
        ("CH", "tʃ"),
        ("D", "d"),
        ("DH", "ð"),
        ("F", "f"),
        ("G", "ɡ"),
        ("HH", "h"),
        ("JH", "dʒ"),
        ("K", "k"),
        ("L", "l"),
        ("M", "m"),
        ("N", "n"),
        ("NG", "ŋ"),
        ("P", "p"),
        ("R", "ɹ"),
        ("S", "s"),
        ("SH", "ʃ"),
        ("T", "t"),
        ("TH", "θ"),
        ("V", "v"),
        ("W", "w"),
        ("Y", "j"),
        ("Z", "z"),
        ("ZH", "ʒ"),
    ])
});

#[inline(always)]
pub fn get_tokens() -> HashMap<char, i64> {
    HashMap::from([
        ('$', 0),
        (';', 1),
        (':', 2),
        (',', 3),
        ('.', 4),
        ('!', 5),
        ('?', 6),
        ('¡', 7),
        ('¿', 8),
        ('—', 9),
        ('…', 10),
        ('"', 11),
        ('«', 12),
        ('»', 13),
        ('"', 14),
        ('"', 15),
        (' ', 16),
        ('A', 17),
        ('B', 18),
        ('C', 19),
        ('D', 20),
        ('E', 21),
        ('F', 22),
        ('G', 23),
        ('H', 24),
        ('I', 25),
        ('J', 26),
        ('K', 27),
        ('L', 28),
        ('M', 29),
        ('N', 30),
        ('O', 31),
        ('P', 32),
        ('Q', 33),
        ('R', 34),
        ('S', 35),
        ('T', 36),
        ('U', 37),
        ('V', 38),
        ('W', 39),
        ('X', 40),
        ('Y', 41),
        ('Z', 42),
        ('a', 43),
        ('b', 44),
        ('c', 45),
        ('d', 46),
        ('e', 47),
        ('f', 48),
        ('g', 49),
        ('h', 50),
        ('i', 51),
        ('j', 52),
        ('k', 53),
        ('l', 54),
        ('m', 55),
        ('n', 56),
        ('o', 57),
        ('p', 58),
        ('q', 59),
        ('r', 60),
        ('s', 61),
        ('t', 62),
        ('u', 63),
        ('v', 64),
        ('w', 65),
        ('x', 66),
        ('y', 67),
        ('z', 68),
        ('ɑ', 69),
        ('ɐ', 70),
        ('ɒ', 71),
        ('æ', 72),
        ('ɓ', 73),
        ('ʙ', 74),
        ('β', 75),
        ('ɔ', 76),
        ('ɕ', 77),
        ('ç', 78),
        ('ɗ', 79),
        ('ɖ', 80),
        ('ð', 81),
        ('ʤ', 82),
        ('ə', 83),
        ('ɘ', 84),
        ('ɚ', 85),
        ('ɛ', 86),
        ('ɜ', 87),
        ('ɝ', 88),
        ('ɞ', 89),
        ('ɟ', 90),
        ('ʄ', 91),
        ('ɡ', 92),
        ('ɠ', 93),
        ('ɢ', 94),
        ('ʛ', 95),
        ('ɦ', 96),
        ('ɧ', 97),
        ('ħ', 98),
        ('ɥ', 99),
        ('ʜ', 100),
        ('ɨ', 101),
        ('ɪ', 102),
        ('ʝ', 103),
        ('ɭ', 104),
        ('ɬ', 105),
        ('ɫ', 106),
        ('ɮ', 107),
        ('ʟ', 108),
        ('ɱ', 109),
        ('ɯ', 110),
        ('ɰ', 111),
        ('ŋ', 112),
        ('ɳ', 113),
        ('ɲ', 114),
        ('ɴ', 115),
        ('ø', 116),
        ('ɵ', 117),
        ('ɸ', 118),
        ('θ', 119),
        ('œ', 120),
        ('ɶ', 121),
        ('ʘ', 122),
        ('ɹ', 123),
        ('ɺ', 124),
        ('ɾ', 125),
        ('ɻ', 126),
        ('ʀ', 127),
        ('ʁ', 128),
        ('ɽ', 129),
        ('ʂ', 130),
        ('ʃ', 131),
        ('ʈ', 132),
        ('ʧ', 133),
        ('ʉ', 134),
        ('ʊ', 135),
        ('ʋ', 136),
        ('ⱱ', 137),
        ('ʌ', 138),
        ('ɣ', 139),
        ('ɤ', 140),
        ('ʍ', 141),
        ('χ', 142),
        ('ʎ', 143),
        ('ʏ', 144),
        ('ʑ', 145),
        ('ʐ', 146),
        ('ʒ', 147),
        ('ʔ', 148),
        ('ʡ', 149),
        ('ʕ', 150),
        ('ʢ', 151),
        ('ǀ', 152),
        ('ǁ', 153),
        ('ǂ', 154),
        ('ǃ', 155),
        ('ˈ', 156),
        ('ˌ', 157),
        ('ː', 158),
        ('ˑ', 159),
        ('ʼ', 160),
        ('ʴ', 161),
        ('ʰ', 162),
        ('ʱ', 163),
        ('ʲ', 164),
        ('ʷ', 165),
        ('ˠ', 166),
        ('ˤ', 167),
        ('˞', 168),
        ('↓', 169),
        ('↑', 170),
        ('→', 171),
        ('↗', 172),
        ('↘', 173),
        ('\'', 174),
        ('̩', 175),
        ('\'', 176),
        ('ᵻ', 177),
    ])
}

static CUSTOM_PHONEMES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("on", "ɔn"),
        ("to", "tə"),
        ("a", "ɐ"),
        ("the", "ðə"),
        ("of", "ʌv"),
        ("is", "ɪz"),
        ("it", "ɪt"),
        ("was", "wʌz"),
        ("were", "wɜː"),
        ("and", "ænd"),
        ("often", "ˈɔfən"),
        ("faced", "fˈeɪsd"),
        ("us", "ˌʌs"),
        ("there", "ðɛɹ"),
        ("when", "wɛn"),
        ("these", "ðiːz"),
        ("river", "ɹˈɪvɚɹ"),
        ("her", "hɜː"),
        ("get", "ɡɛt"),
        ("soft", "sˈɔft"),
        ("salt", "sˈɔlt"),
        ("across", "əkɹˌɑːs"),
        ("before", "bᵻfˌɔːɹ"),
        ("beside", "bᵻsˌaɪd"),
        ("button", "bˈʌʔn ̩"),
        ("perfect", "pˈɜːfɛkt"),
        ("beauty", "bjˈuːɾi"),
        ("from", "fɹʌm"),
        ("read", "ɹˈiːd"),
        ("for", "fɔːɹ"),
        ("your", "jʊɹ"),
        ("off", "ˈɔf"),
        ("his", "hɪz"),
        ("birth", "bˈɜːθ"),
        ("kittens", "kˈɪʔn ̩z"),
        ("dirty", "dˈɜːɾi"),
        ("in", "ɪn"),
        ("response", "ɹᵻspˈɑːns"),
        ("twisted", "twˈɪstᵻd"),
        ("at", "æt"),
        ("once", "wˈʌns"),
    ])
});

#[derive(Error, Debug, Clone)]
pub enum PhonemizerError {
    #[error("failed to load dictionary: {0}")]
    DictLoad(String),
}

pub struct Phonemizer {
    dict: Cmudict,
}

impl Phonemizer {
    pub fn new() -> Result<Self, PhonemizerError> {
        tracing::info!("Initializing Phonemizer");
        let dict = Cmudict::from_str(DICT).map_err(|e| PhonemizerError::DictLoad(e.to_string()))?;
        Ok(Self { dict })
    }

    pub fn phonemize_text(&self, text: &str) -> String {
        let expanded = expand_contractions(text);
        let with_numbers = replace_numbers(&expanded);

        let tokens: Vec<String> = tokenize_text(&with_numbers)
            .into_iter()
            .map(|t| self.phonemize(&t).unwrap_or(t))
            .collect();

        let mut joined = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            let cur = &tokens[i];

            // Handle Lookahead Merging
            if let Some(next) = tokens.get(i + 1) {
                if let Some(merged) = self.try_merge_tokens(cur, next, &tokens, i) {
                    joined.push(merged.to_string());
                    // Special logic for "it is" parity
                    if (cur == "ˈɪt" && next == "ˈɪz") || (cur == "ɪt" && next == "ɪz") {
                        joined.push("ɪz".to_string());
                    }
                    i += 2;
                    continue;
                }

                // Suffix merge
                if next == "z" {
                    joined.push(format!("{}z", cur));
                    i += 2;
                    continue;
                }

                // "The" assimilation
                if cur == "ðə" && is_vowel_start(next) {
                    joined.push("ðɪ".to_string());
                    i += 1;
                    continue;
                }

                // Complex prepositional merging
                if cur == "ɪn" && (next == "ðə" || next == "ðɪ") {
                    let after = tokens.get(i + 2).map(|s| s.as_str()).unwrap_or("");
                    if is_vowel_start(after) {
                        joined.push("ɪnðɪ".to_string());
                    } else {
                        joined.push("ɪn\u{0260}".to_string());
                    }
                    i += 2;
                    continue;
                }
            }

            joined.push(cur.clone());
            i += 1;
        }

        joined.join(" ")
    }

    fn try_merge_tokens(
        &self,
        cur: &str,
        next: &str,
        tokens: &[String],
        i: usize,
    ) -> Option<&'static str> {
        match (cur, next) {
            ("ɔn", "ðə") => Some("ɔnðə"),
            ("ˈɪt", "ˈɪz") | ("ɪt", "ɪz") => Some("ɪɾ"),
            ("duː", "nˈoʊt") | ("duː", "nˈɑːt") | ("dˈuː", "nˈɑːt") => Some("duːnˌɑːt"),
            ("doʊn", "t") => Some("doʊnˌɑːt"),
            ("woʊn", "t") => Some("woʊnˌɑːt"),
            ("əv", "ə") | ("əv", "ɐ") | ("ʌv", "ɐ") => Some("əvə"),
            ("ʌv", "ðə") | ("əv", "ðə") => Some("ʌvðə"),
            ("fɹʌm", "ðə") => Some("fɹʌmðə"),
            ("ɪn", "ðə") => Some("ɪnðə"),
            ("ɪn", "ðɪ") => Some("ɪnðɪ"),
            ("ðɛɹ", "ɑːɹ") | ("ðɛɹ", "ˈɑːɹ") => Some("ðɛɹˌɑːɹ"),
            ("æt", "wˈʌns") | ("ɐt", "wˈʌns") => Some("ætwˈʌns"),
            ("æt", "ˈaʊt") => Some("ætˈaʊt"),
            ("ˈaɪ", "l") => Some("ˈaɪl"),
            ("ˈaɪ", "v") => Some("ˈaɪv"),
            ("ˈaɪ", "m") => Some("ˈaɪm"),
            ("jˈuː", "l") => Some("jˈuːl"),
            ("jˈuː", "v") => Some("jˈuːv"),
            ("jˈuː", "ɹ") => Some("jˈuːɹ"),
            ("wˈiː", "l") => Some("wˈiːl"),
            ("wˈiː", "v") => Some("wˈiːv"),
            ("hˈiː", "l") => Some("hˈiːl"),
            ("hˈiː", "z") => Some("hˈiːz"),
            ("ʃˈiː", "l") => Some("ʃˈiːl"),
            ("ʃˈiː", "z") => Some("ʃˈiːz"),
            ("ðˈeɪ", "l") => Some("ðˈeɪl"),
            ("ðˈeɪ", "v") => Some("ðˈeɪv"),
            ("ɪz", "ðə") if i > 0 && tokens[i - 1].contains('ʃ') => Some("ɪzðə"),
            ("hˈiː", "wɪl") => Some("hˈiːwɪl"),
            ("ʃˈiː", "wɪl") => Some("ʃˈiːwɪl"),
            ("wˈiː", "wɪl") => Some("wˈiːwɪl"),
            ("ˈaɪ", "wɪl") => Some("ˈaɪwɪl"),
            ("ðˈeɪ", "wɪl") => Some("ðˈeɪwɪl"),
            ("əv", "ɡˈoʊ") => Some("əvɡˈoʊ"),
            ("ʃˈiː", "ɪz") => Some("ʃiːz"),
            ("hˈiː", "ɪz") => Some("hˈiːz"),
            ("wˈiː", "ɪv") => Some("wˈiːv"),
            ("ˈaɪ", "ɪv") => Some("ˈaɪv"),
            ("jˈuː", "ɪv") => Some("jˈuːv"),
            ("ðˈeɪ", "ɪv") => Some("ðˈeɪv"),
            ("mˈæn", "ˈɛs") => Some("mˈænz"),
            ("wɪl", "nɒt") | ("wˈɪl", "nɒt") => Some("wˈɪlnɒt"),
            ("kæn", "nɒt") | ("kæn", "ˌɑːt") => Some("kænˌɑːt"),
            _ => None,
        }
    }

    pub fn phonemize(&self, word: &str) -> Option<String> {
        let lower = word.to_lowercase();
        if let Some(custom) = CUSTOM_PHONEMES.get(lower.as_str()) {
            return Some(custom.to_string());
        }

        let rule = self
            .dict
            .get(&lower)
            .and_then(|r| r.first().cloned())
            .or_else(|| Rule::from_str(&word.to_uppercase()).ok())?;

        let pronunciation = rule.pronunciation();
        if pronunciation.is_empty() {
            return Some(word.to_uppercase());
        }

        let mut res = String::new();
        for p in pronunciation {
            let p_str = p.to_string();
            if p_str.contains('1') {
                res.push('ˈ');
            } else if p_str.contains('2') {
                res.push('ˌ');
            }

            let key = p_str.trim_end_matches(|c: char| c.is_ascii_digit());
            if let Some(ipa) = IPA_MAP.get(p_str.as_str()).or_else(|| IPA_MAP.get(key)) {
                res.push_str(ipa);
            }
        }
        Some(res)
    }
}

fn is_vowel_start(s: &str) -> bool {
    let vowels = [
        "ˈa", "a", "ˈe", "e", "ˈi", "i", "ˈo", "o", "ˈu", "u", "ˈɐ", "ɐ", "ˈə", "ə", "ˈɪ", "ɪ",
        "ˈɛ", "ɛ", "ˈɔ", "ɔ", "ˈʌ", "ʌ", "ˈɜ", "ɜ",
    ];
    vowels.iter().any(|v| s.starts_with(v))
}

fn expand_contractions(text: &str) -> String {
    let mut s = text.to_lowercase();
    let rules = [
        ("it's", "it is"),
        ("i'm", "i am"),
        ("can't", "cannot"),
        ("won't", "will not"),
        ("let's", "let us"),
        ("he's", "he is"),
        ("she's", "she is"),
        ("we're", "we are"),
        ("'re", " are"),
        ("'ve", " have"),
        ("'ll", " will"),
        ("'d", " would"),
        ("n't", " not"),
    ];
    for (k, v) in rules {
        s = s.replace(k, v);
    }
    s.chars()
        .map(|c| if "'\"\\/-".contains(c) { ' ' } else { c })
        .collect()
}

fn replace_numbers(text: &str) -> String {
    let mut result = String::new();
    let mut num_buf = String::new();

    for c in text.chars() {
        if c.is_ascii_digit() {
            num_buf.push(c);
        } else {
            if !num_buf.is_empty() {
                if let Ok(n) = num_buf.parse::<u32>() {
                    result.push_str(&number_to_words(n));
                } else {
                    result.push_str(&num_buf);
                }
                num_buf.clear();
            }
            result.push(c);
        }
    }
    result
}

fn number_to_words(n: u32) -> String {
    if n == 0 {
        return "zero".into();
    }
    format!("{}", n)
}

fn tokenize_text(text: &str) -> Vec<String> {
    text.split_whitespace() 
        .flat_map(|s| {
            let mut parts = Vec::new();
            let mut cur = String::new();
            for c in s.chars() {
                if c.is_alphanumeric() {
                    cur.push(c);
                } else {
                    if !cur.is_empty() {
                        parts.push(cur.clone());
                        cur.clear();
                    }
                    parts.push(c.to_string());
                }
            }
            if !cur.is_empty() {
                parts.push(cur);
            }
            parts
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    #[derive(Deserialize)]
    struct Fixture {
        input: String,
        preprocessed: String,
        phonemes: String,
        phonemes_joined: String,
        tokens: Vec<i64>,
    }

    #[test]
    fn all_tests() {
        let fixtures_json = include_str!("fixtures/python_outputs.json");
        let fixtures: Vec<Fixture> =
            serde_json::from_str(fixtures_json).expect("Failed to parse JSON");
        let phonemizer = Phonemizer::new().expect("Failed to create phonemizer");
        let tokens_lookup = get_tokens();

        let mut failures = Vec::new();

        for fixture in fixtures {
            println!("Testing:{}", fixture.input);
            // Rust's phonemize_text
            let rust_phonemes = phonemizer.phonemize_text(&fixture.input);

            // Match tokens
            let rust_tokens: Vec<i64> = rust_phonemes
                .chars()
                .flat_map(|c| tokens_lookup.get(&c))
                .cloned()
                .collect();

            let mut failed = false;
            let mut diff = String::new();

            if rust_phonemes != fixture.phonemes_joined {
                failed = true;
                diff.push_str(&format!(
                    "\nPHONEMESMISMATCH:\nExpected:{}\nActual:{}",
                    fixture.phonemes_joined, rust_phonemes
                ));
            }

            if rust_tokens != fixture.tokens {
                failed = true;
                diff.push_str(&format!(
                    "\nTOKENSMISMATCH:\nExpected:{:?}\nActual:{:?}",
                    fixture.tokens, rust_tokens
                ));
            }

            if failed {
                failures.push(format!(
                    "Input: '{}'\nPHONEMES:\n  Expected: {}\n  Actual:   {}\nTOKENS:\n  Expected: {:?}\n  Actual:   {:?}",
                    fixture.input, fixture.phonemes_joined, rust_phonemes, fixture.tokens, rust_tokens
                ));
            }
        }

        if !failures.is_empty() {
            eprintln!(
                "\nPhonemization parity tests failed:\n\n{}",
                failures.join("\n\n")
            );
            eprint!("{} failures total", failures.len());
        } else {
            println!("\nAll tests passed!");
        }
    }
}
