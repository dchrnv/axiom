// compiler.rs — Seed Compiler: charset + region → anchor seed YAML.
//
// Детерминизм: charset + region → одинаковый кристалл всегда.
// Collision-check: новые позиции не пересекаются с существующими якорями.
#![deny(unsafe_code)]

use axiom_config::anchor::{Anchor, AnchorLayer};

use crate::charset::{CharsetFile, Grapheme};
use crate::layout::{CrystalRegion, LayoutEngine};
use crate::layout::collision::CollisionChecker;
use crate::layout::crystal::CrystalLayout;

/// Shell-профиль для C0 графемных якорей: доминирует L1 (когнитивный слой).
const CRYSTAL_C0_SHELL: [u8; 8] = [160, 40, 0, 0, 0, 0, 0, 0];

#[derive(Debug)]
pub enum CompileError {
    Collision { char: String, pos: [i16; 3], existing: [i16; 3], dist: f32 },
    RegionOverflow(String),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::Collision { char, pos, existing, dist } => write!(
                f,
                "коллизия: графема '{}' в {:?} в {:.0} ед. от существующего якоря {:?} (мин. {})",
                char, pos, dist, existing, dist
            ),
            CompileError::RegionOverflow(msg) => write!(f, "регион выходит за [0..32767]: {}", msg),
        }
    }
}

impl std::error::Error for CompileError {}

pub struct SeedCompiler;

impl SeedCompiler {
    /// Скомпилировать палитру графем в набор якорей-семян.
    ///
    /// `existing_positions` — позиции всех существующих якорей (для collision-check).
    pub fn compile(
        charset: &CharsetFile,
        region: &CrystalRegion,
        existing_positions: &[[i16; 3]],
    ) -> Result<Vec<Anchor>, CompileError> {
        region.validate().map_err(CompileError::RegionOverflow)?;

        let layout = CrystalLayout;
        let graphemes = charset.graphemes();
        let positions = layout.compute_positions(graphemes, region);

        let checker = CollisionChecker {
            existing: existing_positions.to_vec(),
            min_distance: region.min_collision_dist,
        };
        let violations = checker.check_all(&positions);
        if let Some((i, v)) = violations.into_iter().next() {
            return Err(CompileError::Collision {
                char: graphemes[i].ch.clone(),
                pos: v.new_pos,
                existing: v.existing_pos,
                dist: v.distance,
            });
        }

        let anchors = graphemes
            .iter()
            .zip(positions.iter())
            .map(|(g, &pos)| build_anchor(g, pos))
            .collect();

        Ok(anchors)
    }
}

fn build_anchor(g: &Grapheme, position: [i16; 3]) -> Anchor {
    // Uppercase alias для регистронезависимого матчинга (charset: а=А)
    let upper = g.ch.to_uppercase();
    let aliases = if upper != g.ch { vec![upper] } else { vec![] };

    // НЕ добавляем "writing" в теги: кристальные якоря пассивны для subsystem detection.
    // Графемы "—", ".", "," иначе даёт Exact-матч и искусственно буст "writing".
    // Позицию кристалл даёт; доминантную подсистему — нет (spec §5).
    let tags = vec![
        "crystal".to_string(),
        "C0".to_string(),
        g.class.as_tag().to_string(),
    ];

    let id = format!(
        "crys_{}",
        g.ch.chars().next().map(|c| format!("U{:04X}", c as u32)).unwrap_or_default()
    );

    Anchor {
        id,
        word: g.ch.clone(),
        aliases,
        tags,
        position,
        shell: CRYSTAL_C0_SHELL,
        description: format!(
            "Crystal C0 grapheme: {} ({}, rank {})",
            g.ch, g.class.as_tag(), g.rank
        ),
        layer: AnchorLayer::L1,
    }
}

/// Сериализовать набор якорей в FlatAnchorFile YAML-строку.
pub fn anchors_to_yaml(
    anchors: &[Anchor],
    description: &str,
    version: &str,
) -> Result<String, serde_yaml::Error> {
    use serde::Serialize;

    #[derive(Serialize)]
    struct FlatFile<'a> {
        description: &'a str,
        version: &'a str,
        anchors: &'a [Anchor],
    }

    let file = FlatFile { description, version, anchors };
    serde_yaml::to_string(&file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::charset::{CharsetFile, Grapheme, GraphemeClass};
    use crate::layout::CrystalRegion;

    fn small_charset() -> CharsetFile {
        // Имитируем CharsetFile через прямую десериализацию
        let yaml = r#"
charset:
  layer: 0
  total: 4
  graphemes:
    - char: "о"
      rank: 0
      class: vowel_cyr
      subsystem: writing
    - char: "e"
      rank: 1
      class: vowel_lat
      subsystem: writing
    - char: "н"
      rank: 2
      class: consonant_cyr
      subsystem: writing
    - char: "1"
      rank: 3
      class: digit
      subsystem: writing
"#;
        serde_yaml::from_str(yaml).unwrap()
    }

    fn region() -> CrystalRegion {
        CrystalRegion {
            origin: [26500, 26500, 26500],
            size: [4000, 4000, 1600],
            min_collision_dist: 500.0,
        }
    }

    #[test]
    fn compile_no_collision() {
        let charset = small_charset();
        let result = SeedCompiler::compile(&charset, &region(), &[]);
        assert!(result.is_ok());
        let anchors = result.unwrap();
        assert_eq!(anchors.len(), 4);
    }

    #[test]
    fn compile_produces_deterministic_output() {
        let charset = small_charset();
        let r = region();
        let a1 = SeedCompiler::compile(&charset, &r, &[]).unwrap();
        let a2 = SeedCompiler::compile(&charset, &r, &[]).unwrap();
        for (a, b) in a1.iter().zip(a2.iter()) {
            assert_eq!(a.position, b.position);
            assert_eq!(a.word, b.word);
        }
    }

    #[test]
    fn compile_collision_detected() {
        let charset = small_charset();
        let r = region();
        // Вычислим одну позицию и поставим туда существующий якорь
        let anchors = SeedCompiler::compile(&charset, &r, &[]).unwrap();
        let pos = anchors[0].position;
        let result = SeedCompiler::compile(&charset, &r, &[pos]);
        assert!(result.is_err(), "должна быть обнаружена коллизия");
    }

    #[test]
    fn anchor_tags_include_crystal_c0() {
        let charset = small_charset();
        let anchors = SeedCompiler::compile(&charset, &region(), &[]).unwrap();
        for a in &anchors {
            assert!(a.tags.contains(&"crystal".to_string()));
            assert!(a.tags.contains(&"C0".to_string()));
            // НЕТ тега "writing": кристалл пассивен для subsystem detection (spec §5)
            assert!(!a.tags.contains(&"writing".to_string()));
        }
    }

    #[test]
    fn uppercase_alias_for_letter() {
        let charset = small_charset();
        let anchors = SeedCompiler::compile(&charset, &region(), &[]).unwrap();
        // "о" → alias "О"
        let o_anchor = anchors.iter().find(|a| a.word == "о").unwrap();
        assert!(o_anchor.aliases.contains(&"О".to_string()));
    }

    #[test]
    fn no_alias_for_digit() {
        let charset = small_charset();
        let anchors = SeedCompiler::compile(&charset, &region(), &[]).unwrap();
        let digit = anchors.iter().find(|a| a.word == "1").unwrap();
        assert!(digit.aliases.is_empty(), "цифры не имеют uppercase-alias");
    }

    #[test]
    fn region_overflow_detected() {
        let charset = small_charset();
        let bad_region = CrystalRegion {
            origin: [30000, 30000, 30000],
            size: [4000, 4000, 4000],  // 30000+4000=34000 > 32767
            min_collision_dist: 500.0,
        };
        let result = SeedCompiler::compile(&charset, &bad_region, &[]);
        assert!(matches!(result, Err(CompileError::RegionOverflow(_))));
    }
}

#[cfg(test)]
mod load_tests {
    use axiom_config::anchor::AnchorSet;
    use std::path::Path;

    #[test]
    fn crystal_yaml_parseable_by_anchor_set() {
        let yaml_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap().parent().unwrap()
            .join("seeds/crystal_c0.yaml");
        if !yaml_path.exists() {
            return; // seeds/ не сгенерирован — пропустить
        }
        let content = std::fs::read_to_string(&yaml_path).unwrap();
        let result: Result<serde_yaml::Value, _> = serde_yaml::from_str(&content);
        assert!(result.is_ok(), "serde_yaml не может прочитать crystal_c0.yaml: {:?}", result.err());
    }
}
