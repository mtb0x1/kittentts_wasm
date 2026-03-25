use std::{collections::HashMap, path::Path, str::FromStr};

use cmudict_fast::{Cmudict, Rule};
use thiserror::Error;

pub const DICT: &str = include_str!("../web/cmu.dict");

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

fn get_ipa() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("AA", "ɑ"),
        ("AA1", "ɑː"),
        ("AA2", "ɑː"),
        ("AE", "æ"),
        ("AE1", "æ"),
        ("AE2", "æ"),
        ("AH", "ə"),
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
        ("ER", "ɝ"),
        ("ER1", "ɝː"),
        ("ER2", "ɝː"),
        ("EY", "eɪ"),
        ("EY1", "eɪ"),
        ("EY2", "eɪ"),
        ("IH", "ᵻ"),
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
impl Phonemizer {
    pub fn new() -> Result<Self, PhonemizerError> {
        let dict = Cmudict::from_str(DICT).map_err(|e| PhonemizerError::DictLoad(e.to_string()))?;
        let ipa = get_ipa();
        Ok(Self { dict, ipa })
    }

    pub fn phonemize(&self, word: &str) -> Option<String> {
        let lower_case = word.to_lowercase();
        let upper_case = word.to_uppercase();

        let rules = self.dict.get(lower_case.as_str());
        let rule = if let Some(rule) = rules {
            rule[0].clone()
        } else {
            let rule_from_str = Rule::from_str(upper_case.as_str());
            match rule_from_str {
                Ok(rule) => rule,
                Err(_) => return None,
            }
        };

        let pronunciation = rule.pronunciation();
        let phonemized: String = if pronunciation.is_empty() {
            upper_case
        } else {
            pronunciation
                .iter()
                .map(|p| {
                    let key = p.to_string().replace("0", "");

                    self.ipa[key.as_str()]
                })
                .collect()
        };

        Some(phonemized)
    }
}
