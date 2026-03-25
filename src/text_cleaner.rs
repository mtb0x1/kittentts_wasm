use std::collections::HashMap;

/// Symbol table matching KittenTTS Python TextCleaner exactly.
/// Order: pad ($) → punctuation → ASCII letters → IPA symbols.
const PAD: &str = "$";
const PUNCTUATION: &str = ";:,.!?¡¿—…\"«»\"\u{201c}\u{201d} ";
const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const LETTERS_IPA: &str = "ɑɐɒæɓʙβɔɕçɗɖðʤəɘɚɛɜɝɞɟʄɡɠɢʛɦɧħɥʜɨɪʝɭɬɫɮʟɱɯɰŋɳɲɴøɵɸθœɶʘɹɺɾɻʀʁɽʂʃʈʧʉʊʋⱱʌɣɤʍχʎʏʑʐʒʔʡʕʢǀǁǂǃˈˌːˑʼʴʰʱʲʷˠˤ˞↓↑→↗↘'̩'ᵻ";

/// End-of-sequence token index (maps to '…' at position 10 in the symbol table).
const EOS_TOKEN: i64 = 10;
/// Padding token index (maps to '$' at position 0).
const PAD_TOKEN: i64 = 0;

/// Port of the KittenTTS Python `TextCleaner`.
///
/// Builds a char→index mapping from the concatenated symbol alphabet
/// and converts text into a sequence of token indices.
pub struct TextCleaner {
    word_index: HashMap<char, i64>,
}

impl TextCleaner {
    pub fn new() -> Self {
        let mut word_index = HashMap::new();
        let symbols: Vec<char> = PAD
            .chars()
            .chain(PUNCTUATION.chars())
            .chain(LETTERS.chars())
            .chain(LETTERS_IPA.chars())
            .collect();

        for (i, ch) in symbols.iter().enumerate() {
            word_index.insert(*ch, i as i64);
        }

        Self { word_index }
    }

    /// Map each character to its symbol-table index, skipping unknowns.
    /// This matches the Python `TextCleaner.__call__` behaviour.
    fn tokenize(&self, text: &str) -> Vec<i64> {
        text.chars()
            .filter_map(|ch| self.word_index.get(&ch).copied())
            .collect()
    }

    /// Tokenize text and wrap with start / end / pad tokens to match
    /// the KittenTTS `_prepare_inputs` convention:
    ///   `[0, ...tokens, 10, 0]`
    pub fn tokenize_for_model(&self, text: &str) -> Vec<i64> {
        let mut tokens = Vec::with_capacity(text.len() + 3);
        tokens.push(PAD_TOKEN); // start token
        tokens.extend(self.tokenize(text));
        tokens.push(EOS_TOKEN); // end-of-sequence token
        tokens.push(PAD_TOKEN); // trailing pad
        tokens
    }
}
