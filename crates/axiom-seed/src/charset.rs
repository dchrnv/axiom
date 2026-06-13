// charset.rs — Десериализация файла графемной палитры для компилятора кристалла.
#![deny(unsafe_code)]

use serde::Deserialize;

/// Класс природы графемы — определяет θ-сектор в кристалле.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphemeClass {
    VowelCyr,
    VowelLat,
    ConsonantCyr,
    ConsonantLat,
    Digit,
    Period,
    Comma,
    Semicolon,
    Colon,
    Exclaim,
    Question,
    Dash,
    Quote,
    Ellipsis,
    BracketOpen,
    BracketClose,
    OpArith,
    OpCompare,
    OpMisc,
    Space,
    Underscore,
    At,
    Hash,
    Dollar,
    SlashBack,
    Amp,
}

impl GraphemeClass {
    /// Имя класса для тегов якоря.
    pub fn as_tag(&self) -> &'static str {
        use GraphemeClass::*;
        match self {
            VowelCyr => "vowel_cyr",
            VowelLat => "vowel_lat",
            ConsonantCyr => "consonant_cyr",
            ConsonantLat => "consonant_lat",
            Digit => "digit",
            Period => "period",
            Comma => "comma",
            Semicolon => "semicolon",
            Colon => "colon",
            Exclaim => "exclaim",
            Question => "question",
            Dash => "dash",
            Quote => "quote",
            Ellipsis => "ellipsis",
            BracketOpen => "bracket_open",
            BracketClose => "bracket_close",
            OpArith => "op_arith",
            OpCompare => "op_compare",
            OpMisc => "op_misc",
            Space => "space",
            Underscore => "underscore",
            At => "at",
            Hash => "hash",
            Dollar => "dollar",
            SlashBack => "slash_back",
            Amp => "amp",
        }
    }
}

/// Одна графема в палитре.
#[derive(Debug, Clone, Deserialize)]
pub struct Grapheme {
    #[serde(rename = "char")]
    pub ch: String,
    /// Глобальный ранг частоты (0 = наиболее частая → r ≈ 0 в ядре слоя).
    pub rank: u32,
    pub class: GraphemeClass,
    #[serde(default)]
    pub subsystem: String,
}

#[derive(Debug, Deserialize)]
struct CharsetInner {
    #[allow(dead_code)]
    pub layer: u32,
    pub total: u32,
    pub graphemes: Vec<Grapheme>,
}

/// Файл палитры графем (charsets/ru_en_base.yaml).
#[derive(Debug, Deserialize)]
pub struct CharsetFile {
    charset: CharsetInner,
}

impl CharsetFile {
    pub fn load(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let file: CharsetFile = serde_yaml::from_str(&content)?;
        Ok(file)
    }

    pub fn graphemes(&self) -> &[Grapheme] {
        &self.charset.graphemes
    }

    pub fn declared_total(&self) -> u32 {
        self.charset.total
    }
}
