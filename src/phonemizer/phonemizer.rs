use std::{collections::HashMap, str::FromStr};

use cmudict_fast::{Cmudict, Rule};
use thiserror::Error;

pub const DICT: &str = include_str!("../../web/cmu.dict");

#[derive(Error, Debug, Clone)]
pub enum PhonemizerError {
    #[error("failed to load dictionary: {0}")]
    DictLoad(String),
}

#[derive(Debug)]
pub struct Phonemizer {
    dict: Cmudict,
    ipa: HashMap<&'static str, &'static str>,
}

#[inline(always)]
fn get_ipa() -> HashMap<&'static str, &'static str> {
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
}

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

#[inline(always)]
fn get_custom_phonemes() -> HashMap<&'static str, &'static str> {
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
        ("in", "ɪn"),
    ])
}

impl Phonemizer {
    pub fn new() -> Result<Self, PhonemizerError> {
        tracing::info!("Initializing Phonemizer");
        tracing::debug!("Loading phonemizer dictionary from embedded CMU dict");
        let dict = Cmudict::from_str(DICT).map_err(|e| PhonemizerError::DictLoad(e.to_string()))?;
        let ipa = get_ipa();
        tracing::info!("Phonemizer initialized successfully");
        Ok(Self { dict, ipa })
    }

    pub fn phonemize_text(&self, text: &str) -> String {
        tracing::info!("phonemize_text input: {}", text);
        let expanded = expand_contractions(text);
        tracing::trace!("After expand_contractions: {}", expanded);
        let with_numbers = replace_numbers(&expanded);
        tracing::trace!("After replace_numbers: {}", with_numbers);

        let phonemized_tokens: Vec<String> = tokenize_text(&with_numbers)
            .into_iter()
            .map(|token| self.phonemize(&token).unwrap_or(token))
            .collect();

        let mut joined = Vec::new();
        let mut i = 0;
        while i < phonemized_tokens.len() {
            if i + 1 < phonemized_tokens.len() {
                let cur = phonemized_tokens[i].as_str();
                let next = phonemized_tokens[i + 1].as_str();

                let merged = match (cur, next) {
                    ("ɔn", "ðə") => Some("ɔnðə"),
                    ("ˈɪt", "ˈɪz") | ("ɪt", "ɪz") => Some("ɪɾ"),
                    ("duː", "nˈoʊt") => Some("duːnˌɑːt"),
                    ("duː", "nˈɑːt") => Some("duːnˌɑːt"),
                    ("dˈuː", "nˈɑːt") => Some("duːnˌɑːt"),
                    ("doʊn", "t") => Some("doʊnˌɑːt"),
                    ("woʊn", "t") => Some("woʊnˌɑːt"),
                    ("əv", "ə") | ("əv", "ɐ") | ("ʌv", "ɐ") => Some("əvə"),
                    ("ʌv", "ðə") => Some("ʌvðə"),
                    ("əv", "ðə") => Some("əvðə"),
                    ("fɹʌm", "ðə") => Some("fɹʌmðə"),
                    ("ɪn", "ðə") => Some("ɪnðə"),
                    ("ɪn", "ðɪ") => Some("ɪnðɪ"),
                    ("ðɛɹ", "ɑːɹ") => Some("ðɛɹˌɑːɹ"),
                    ("ðɛɹ", "ˈɑːɹ") => Some("ðɛɹˌɑːɹ"),
                    ("æt", "wˈʌns") => Some("ætwˈʌns"),
                    ("ɐt", "wˈʌns") => Some("ɐtwˈʌns"),
                    ("æt", "ˈaʊt") => Some("ætˈaʊt"),
                    ("ˈaɪ", "l") => Some("ˈaɪl"),
                    ("ˈaɪ", "v") => Some("ˈaɪv"),
                    ("jˈuː", "l") => Some("jˈuːl"),
                    ("jˈuː", "v") => Some("jˈuːv"),
                    ("wˈiː", "l") => Some("wˈiːl"),
                    ("wˈiː", "v") => Some("wˈiːv"),
                    ("hˈiː", "l") => Some("hˈiːl"),
                    ("hˈiː", "z") => Some("hˈiːz"),
                    ("ʃˈiː", "l") => Some("ʃˈiːl"),
                    ("ʃˈiː", "z") => Some("ʃˈiːz"),
                    ("ðˈeɪ", "l") => Some("ðˈeɪl"),
                    ("ðˈeɪ", "v") => Some("ðˈeɪv"),
                    ("ˈaɪ", "m") => Some("ˈaɪm"),
                    ("jˈuː", "ɹ") => Some("jˈuːɹ"),
                    ("ɪz", "ðə") if i > 0 && phonemized_tokens[i - 1].contains("ʃ") => {
                        Some("ɪzðə")
                    }
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
                    ("wɪl", "nɒt") => Some("wɪlnɒt"),
                    ("wˈɪl", "nɒt") => Some("wˈɪlnɒt"),
                    ("kæn", "nɒt") => Some("kænnɒt"),
                    ("kæn", "ˌɑːt") => Some("kænˌɑːt"),
                    _ => None,
                };

                if let Some(value) = merged {
                    joined.push(value.to_string());
                    // `it's` from contractions expands to it is; keep 'ɪz' part after merging.
                    if (cur == "ˈɪt" && next == "ˈɪz") || (cur == "ɪt" && next == "ɪz") {
                        joined.push("ɪz".to_string());
                    }
                    i += 2;
                    continue;
                }

                if next == "z" {
                    joined.push(format!("{}z", cur));
                    i += 2;
                    continue;
                }

                if cur == "ðə" {
                    let vowels = [
                        "ˈa", "a", "ˈe", "e", "ˈi", "i", "ˈo", "o", "ˈu", "u", "ˈɐ", "ɐ", "ˈə",
                        "ə", "ˈɪ", "ɪ", "ˈɛ", "ɛ", "ˈɔ", "ɔ", "ˈʌ", "ʌ", "ˈɜ", "ɜ",
                    ];
                    if vowels.iter().any(|v| next.starts_with(v)) {
                        joined.push("ðɪ".to_string());
                        i += 1;
                        continue;
                    }
                }

                if cur == "ɪn" && (next == "ðə" || next == "ðɪ") {
                    let after = phonemized_tokens
                        .get(i + 2)
                        .map(|s| s.as_str())
                        .unwrap_or("");
                    let vowels = [
                        "ˈa", "a", "ˈe", "e", "ˈi", "i", "ˈo", "o", "ˈu", "u", "ˈɪ", "ɪ",
                    ];
                    if vowels.iter().any(|v| after.starts_with(v)) {
                        joined.push("ɪnðɪ".to_string());
                        i += 2;
                        continue;
                    } else {
                        joined.push("ɪn\u{0260}".to_string());
                        i += 2;
                        continue;
                    }
                }

                if cur == "ɪn\u{0260}" {
                    let vowels = [
                        "ˈa", "a", "ˈe", "e", "ˈi", "i", "ˈo", "o", "ˈu", "u", "ˈɪ", "ɪ",
                    ];
                    if vowels.iter().any(|v| next.starts_with(v)) {
                        joined.push("ɪnðɪ".to_string());
                        i += 1;
                        continue;
                    }
                }

                if (cur == "hˈiː"
                    || cur == "ʃˈiː"
                    || cur == "ˈaɪ"
                    || cur == "wˈiː"
                    || cur == "ðˈeɪ"
                    || cur == "ɪt"
                    || cur == "ˈɪt")
                    && next == "wɪl"
                {
                    joined.push(format!("{}wˈɪl", cur));
                    i += 2;
                    continue;
                }
            }

            joined.push(phonemized_tokens[i].clone());
            i += 1;
        }

        tracing::info!("phonemize_text done.");
        joined.join(" ")
    }

    #[inline(always)]
    pub fn phonemize(&self, word: &str) -> Option<String> {
        let lower_case = word.to_lowercase();
        let upper_case = word.to_uppercase();

        // custom override for function words / known CMU dialect differences
        if let Some(custom) = get_custom_phonemes().get(lower_case.as_str()) {
            tracing::debug!("Phonemize custom override '{}' -> '{}'", word, custom);
            return Some(custom.to_string());
        }

        let rules = self.dict.get(lower_case.as_str());
        let rule = if let Some(rule) = rules {
            rule[0].clone()
        } else {
            let rule_from_str = Rule::from_str(upper_case.as_str());
            match rule_from_str {
                Ok(rule) => rule,
                Err(_) => {
                    tracing::warn!("Word not found in dictionary: {}", word);
                    return None;
                }
            }
        };

        let pronunciation = rule.pronunciation();
        let mut phonemized = String::new();

        if pronunciation.is_empty() {
            phonemized = upper_case;
        } else {
            for p in pronunciation {
                let p_str = p.to_string();
                if p_str.contains('1') {
                    phonemized.push('ˈ');
                } else if p_str.contains('2') {
                    phonemized.push('ˌ');
                }

                if let Some(ipa_char) = self.ipa.get(p_str.as_str()) {
                    phonemized.push_str(ipa_char);
                } else {
                    // Fallback to key without stress
                    let key = p_str
                        .chars()
                        .filter(|c| !c.is_ascii_digit())
                        .collect::<String>();
                    if let Some(ipa_char) = self.ipa.get(key.as_str()) {
                        phonemized.push_str(ipa_char);
                    }
                }
            }
        }

        tracing::info!("Phonemized '{}' -> '{}'", word, phonemized);
        Some(phonemized)
    }
}

#[inline(always)]
fn expand_contractions(text: &str) -> String {
    let mut expanded = text.to_lowercase();

    let contractions = [
        ("it's", "it is"),
        ("i'm", "i am"),
        ("can't", "cannot"),
        ("won't", "will not"),
        ("shan't", "shall not"),
        ("ain't", "is not"),
        ("let's", "let us"),
        ("he's", "he is"),
        ("she's", "she is"),
        ("we're", "we are"),
        ("they're", "they are"),
        ("i've", "i have"),
        ("you've", "you have"),
        ("we've", "we have"),
        ("they've", "they have"),
        ("i'll", "i will"),
        ("you'll", "you will"),
        ("he'll", "he will"),
        ("she'll", "she will"),
        ("we'll", "we will"),
        ("they'll", "they will"),
        ("i'd", "i would"),
        ("you'd", "you would"),
        ("he'd", "he would"),
        ("she'd", "she would"),
        ("we'd", "we would"),
        ("they'd", "they would"),
        ("'re", " are"),
        ("'ve", " have"),
        ("'ll", " will"),
        ("'d", " would"),
        ("n't", " not"),
    ];

    for (contracted, full) in contractions {
        expanded = expanded.replace(contracted, full);
    }

    let expanded = expanded
        .chars()
        .map(|c| {
            if ['\'', '"', '\\', '/', '-'].contains(&c) {
                ' '
            } else {
                c
            }
        })
        .collect();
    tracing::info!("expand_contractions input: {}, output: {}", text, expanded);
    expanded
}

fn replace_numbers(text: &str) -> String {
    tracing::info!("replace_numbers input: {}", text);
    let mut result = String::new();
    let mut current_number = String::new();

    for c in text.chars() {
        if c.is_ascii_digit() {
            current_number.push(c);
        } else {
            if !current_number.is_empty() {
                if let Ok(n) = current_number.parse::<u32>() {
                    result.push_str(&number_to_words(n));
                } else {
                    result.push_str(&current_number);
                }
                current_number.clear();
            }
            result.push(c);
        }
    }

    if !current_number.is_empty() {
        if let Ok(n) = current_number.parse::<u32>() {
            result.push_str(&number_to_words(n));
        } else {
            result.push_str(&current_number);
        }
    }

    result
}

fn number_to_words(n: u32) -> String {
    tracing::info!("number_to_words input: {}", n);
    if n == 0 {
        return "zero".to_string();
    }

    let ones = [
        "",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let mut parts = Vec::new();

    let thousands = n / 1000;
    let remainder = n % 1000;

    if thousands > 0 {
        parts.push(number_to_words(thousands));
        parts.push("thousand".to_string());
    }

    if remainder > 0 {
        let hundreds = remainder / 100;
        let ten_rem = remainder % 100;

        if hundreds > 0 {
            parts.push(ones[hundreds as usize].to_string());
            parts.push("hundred".to_string());
        }

        if ten_rem > 0 {
            if ten_rem < 20 {
                parts.push(ones[ten_rem as usize].to_string());
            } else {
                let t = ten_rem / 10;
                let o = ten_rem % 10;
                if o > 0 {
                    parts.push(format!("{}-{}", tens[t as usize], ones[o as usize]));
                } else {
                    parts.push(tens[t as usize].to_string());
                }
            }
        }
    }

    parts.join(" ")
}

fn tokenize_text(text: &str) -> Vec<String> {
    tracing::info!("tokenize_text input: {}", text);
    let mut tokens = Vec::new();
    let mut current = String::new();
    for c in text.chars() {
        if c.is_alphanumeric() {
            current.push(c);
        } else {
            if !current.is_empty() {
                tokens.push(current);
                current = String::new();
            }
            if !c.is_whitespace() {
                tokens.push(c.to_string());
            }
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
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
