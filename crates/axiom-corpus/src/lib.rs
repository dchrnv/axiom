// axiom-corpus — deterministic text generator for engine testing and Phase E corpus building.
//
// Four modes cover the full noise-to-prose spectrum:
//   noise     — random printable ASCII, no structure
//   syllables — pronounceable CV-pattern tokens, language-like without meaning
//   words     — random common English words
//   prose     — sentences with punctuation, capitalization, varied length

use serde::{Deserialize, Serialize};

/// Generation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GenerateMode {
    Noise,
    Syllables,
    Words,
    Prose,
}

impl Default for GenerateMode {
    fn default() -> Self { Self::Prose }
}

impl std::str::FromStr for GenerateMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noise"     => Ok(Self::Noise),
            "syllables" => Ok(Self::Syllables),
            "words"     => Ok(Self::Words),
            "prose"     => Ok(Self::Prose),
            other       => Err(format!("unknown mode: {other}")),
        }
    }
}

/// Generate `count` lines using the given mode and seed.
/// Seed = 0 means use a fixed default (fully deterministic for testing).
pub fn generate(mode: GenerateMode, count: usize, seed: u64) -> Vec<String> {
    let mut rng = Lcg::new(if seed == 0 { 0xdeadbeef_cafebabe } else { seed });
    (0..count).map(|_| generate_line(mode, &mut rng)).collect()
}

// ── Per-line generators ───────────────────────────────────────────────────────

fn generate_line(mode: GenerateMode, rng: &mut Lcg) -> String {
    match mode {
        GenerateMode::Noise     => gen_noise(rng),
        GenerateMode::Syllables => gen_syllables(rng),
        GenerateMode::Words     => gen_words(rng),
        GenerateMode::Prose     => gen_prose(rng),
    }
}

fn gen_noise(rng: &mut Lcg) -> String {
    let len = 20 + rng.next_usize() % 60;
    (0..len).map(|_| (32 + rng.next_usize() % 95) as u8 as char).collect()
}

fn gen_syllables(rng: &mut Lcg) -> String {
    const ONSET:  &[&str] = &["b","c","d","f","g","h","j","k","l","m","n","p","r","s","t","v","z","br","cr","dr","fl","gr","pr","st","tr"];
    const VOWELS: &[&str] = &["a","e","i","o","u","ai","au","ei","ou","ia"];
    const CODA:   &[&str] = &["","","","n","m","r","l","s","t","k"];

    let word_count = 4 + rng.next_usize() % 8;
    let mut words = Vec::with_capacity(word_count);
    for _ in 0..word_count {
        let syllables = 1 + rng.next_usize() % 3;
        let mut w = String::new();
        for _ in 0..syllables {
            w.push_str(ONSET[rng.next_usize() % ONSET.len()]);
            w.push_str(VOWELS[rng.next_usize() % VOWELS.len()]);
            w.push_str(CODA[rng.next_usize() % CODA.len()]);
        }
        words.push(w);
    }
    words.join(" ")
}

fn gen_words(rng: &mut Lcg) -> String {
    let count = 4 + rng.next_usize() % 10;
    (0..count).map(|_| WORDS[rng.next_usize() % WORDS.len()]).collect::<Vec<_>>().join(" ")
}

fn gen_prose(rng: &mut Lcg) -> String {
    let sentence_count = 1 + rng.next_usize() % 3;
    let mut out = String::new();
    for s in 0..sentence_count {
        if s > 0 { out.push(' '); }
        let word_count = 3 + rng.next_usize() % 10;
        for w in 0..word_count {
            let word = WORDS[rng.next_usize() % WORDS.len()];
            if w == 0 {
                // Capitalize first letter
                let mut chars = word.chars();
                if let Some(c) = chars.next() {
                    out.extend(c.to_uppercase());
                    out.push_str(chars.as_str());
                }
            } else {
                out.push_str(word);
            }
            if w < word_count - 1 {
                // Occasional comma
                if rng.next_usize() % 8 == 0 { out.push(','); }
                out.push(' ');
            }
        }
        // Sentence-ending punctuation
        let punct = match rng.next_usize() % 6 {
            0 => '?',
            1 => '!',
            _ => '.',
        };
        out.push(punct);
    }
    out
}

// ── Word list ─────────────────────────────────────────────────────────────────

static WORDS: &[&str] = &[
    "the","be","to","of","and","a","in","that","have","it","for","not","on","with",
    "he","as","you","do","at","this","but","his","by","from","they","we","say","her",
    "she","or","an","will","my","one","all","would","there","their","what","so","up",
    "out","if","about","who","get","which","go","me","when","make","can","like","time",
    "no","just","him","know","take","people","into","year","your","good","some","could",
    "them","see","other","than","then","now","look","only","come","its","over","think",
    "also","back","after","use","two","how","our","work","first","well","way","even",
    "new","want","because","any","these","give","day","most","us","great","between",
    "need","large","often","hand","high","place","hold","real","life","few","north",
    "open","seem","together","next","white","children","begin","got","walk","example",
    "ease","paper","group","always","music","those","both","mark","book","letter","until",
    "mile","river","car","feet","care","second","enough","plain","girl","usual","young",
    "ready","above","ever","red","list","thought","city","play","small","number","off",
    "always","move","try","kind","hand","picture","again","change","off","play","spell",
    "air","away","animal","house","point","page","letter","mother","answer","found",
    "still","learn","should","america","world","information","map","set","three","small",
    "mountain","cut","cold","cried","plan","notice","south","sing","war","ground","fall",
    "king","town","fire","upon","doing","horse","unit","figure","certain","field","travel",
    "wood","heart","order","pattern","slow","center","love","person","money","serve",
    "appear","road","map","rain","rule","govern","pull","cold","notice","voice","power",
    "town","fine","drive","led","cry","dark","machine","note","wait","plan","star","box",
    "noun","field","rest","correct","able","pound","done","beauty","drive","stood","contain",
    "front","teach","week","final","gave","green","quick","develop","ocean","warm","free",
    "minute","strong","special","behind","clear","tail","produce","fact","street","decide",
    "inch","lot","nothing","course","stay","wheel","full","force","blue","object","decide",
    "surface","deep","moon","island","foot","system","busy","test","record","boat","common",
    "gold","possible","plane","steady","dry","wonder","laugh","thousand","ago","ran","check",
    "game","shape","equate","miss","brought","heat","snow","tire","bring","yes","distant",
    "fill","east","paint","language","among","grand","ball","yet","wave","drop","heart",
    "present","heavy","dance","engine","alone","draw","east","early","myself","touch","low",
];

// ── LCG (Lehmer) RNG — no external dependency ─────────────────────────────────

struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self { Self(seed ^ 0x6c62272e07bb0142) }

    fn next_u64(&mut self) -> u64 {
        // Xorshift64
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn next_usize(&mut self) -> usize {
        self.next_u64() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_counts() {
        for mode in [GenerateMode::Noise, GenerateMode::Syllables, GenerateMode::Words, GenerateMode::Prose] {
            let lines = generate(mode, 20, 42);
            assert_eq!(lines.len(), 20);
            assert!(lines.iter().all(|l| !l.is_empty()));
        }
    }

    #[test]
    fn test_deterministic() {
        let a = generate(GenerateMode::Prose, 10, 1234);
        let b = generate(GenerateMode::Prose, 10, 1234);
        assert_eq!(a, b);
    }

    #[test]
    fn test_different_seeds() {
        let a = generate(GenerateMode::Prose, 5, 1);
        let b = generate(GenerateMode::Prose, 5, 2);
        assert_ne!(a, b);
    }

    #[test]
    fn test_mode_fromstr() {
        assert_eq!("prose".parse::<GenerateMode>().unwrap(), GenerateMode::Prose);
        assert_eq!("noise".parse::<GenerateMode>().unwrap(), GenerateMode::Noise);
        assert!("bad".parse::<GenerateMode>().is_err());
    }
}
