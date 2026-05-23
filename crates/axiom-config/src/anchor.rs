// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// anchor.rs — Якорные токены V1.0.
//
// Якоря — фиксированные точки в семантическом пространстве, которые задают
// смысл координат X/Y/Z и Shell-профилей. TextPerceptor позиционирует новые
// токены относительно якорей.
//
// Пять уровней якорей:
//   axes             — 6 полюсов (X+/X-/Y+/Y-/Z+/Z-)
//   layers           — по набору на каждый Shell-слой L1..L8
//   domains          — по набору на каждый ASHTI-домен D1..D8
//   octants          — 8 архетипов октантов (octants.yaml)
//   semantic_centers — универсальные центры смысла (semantic_centers.yaml)
//   subsystems       — подсистемы знания: writing/, mathematics/, ... (subdirs)

use crate::loader::ConfigError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

// ─── Основные структуры ──────────────────────────────────────────────────────

/// Один якорный токен — постоянный ориентир в семантическом пространстве.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Anchor {
    /// Уникальный идентификатор (опционально, для ссылок)
    #[serde(default)]
    pub id: String,
    /// Основное слово-якорь
    pub word: String,
    /// Синонимы и однокоренные слова
    #[serde(default)]
    pub aliases: Vec<String>,
    /// Категории для группировки и tag-совпадений
    #[serde(default)]
    pub tags: Vec<String>,
    /// Позиция в семантическом пространстве [x, y, z], диапазон i16
    pub position: [i16; 3],
    /// Shell-профиль [L1..L8], 0=не затронут, 255=максимум
    pub shell: [u8; 8],
    /// Описание (для документации и `:anchor` команды)
    #[serde(default)]
    pub description: String,
}

/// Тип совпадения входного текста с якорем.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchType {
    /// Точное совпадение слова
    Exact,
    /// Совпадение с одним из aliases
    Alias,
    /// Одно содержит другое (не короче 4 символов)
    Substring,
}

impl MatchType {
    /// Returns the match type as a static string slice.
    pub fn as_str(self) -> &'static str {
        match self {
            MatchType::Exact => "exact",
            MatchType::Alias => "alias",
            MatchType::Substring => "substring",
        }
    }
}

/// Результат сопоставления одного слова из текста с якорем.
#[derive(Debug)]
pub struct AnchorMatch<'a> {
    /// The anchor that matched.
    pub anchor: &'a Anchor,
    /// Вес совпадения: Exact=1.0, Alias=0.9, Substring=0.5
    pub score: f32,
    /// The type of match that was found.
    pub match_type: MatchType,
    /// Слово из ввода, которое совпало
    pub matched_word: String,
}

// ─── YAML-обёртки для парсинга файлов ────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct AxesFile {
    axes: Vec<Anchor>,
}

#[derive(Debug, Deserialize)]
struct LayerFile {
    #[allow(dead_code)]
    layer: String,
    #[serde(default)]
    #[allow(dead_code)]
    description: String,
    anchors: Vec<Anchor>,
}

#[derive(Debug, Deserialize)]
struct DomainFile {
    #[allow(dead_code)]
    domain: String,
    #[serde(default)]
    #[allow(dead_code)]
    domain_id: u16,
    #[serde(default)]
    #[allow(dead_code)]
    description: String,
    anchors: Vec<Anchor>,
}

/// Универсальный файл с плоским списком якорей (octants.yaml, semantic_centers.yaml,
/// и все файлы внутри subsystem-директорий writing/, mathematics/, ...).
#[derive(Debug, Deserialize)]
struct FlatAnchorFile {
    #[serde(default)]
    #[allow(dead_code)]
    description: String,
    anchors: Vec<Anchor>,
}

// ─── AnchorSet ────────────────────────────────────────────────────────────────

/// Полный набор якорей: осевые + слоевые + доменные + октанты + семцентры + подсистемы.
///
/// Загружается из `config/anchors/`:
///   axes.yaml                — 6 осевых якорей (±30000, исключение из правила +only)
///   octants.yaml             — 8 архетипов октантов
///   semantic_centers.yaml    — универсальные центры: истина/ложь/жизнь/смерть и др.
///   layers/L{n}_*.yaml       — якоря Shell-слоёв L1..L8
///   domains/D{n}_*.yaml      — якоря ASHTI-доменов D1..D8
///   {name}/*.yaml            — подсистемы знания (writing, mathematics, ...)
pub struct AnchorSet {
    /// 6 осевых якорей (X+/X-/Y+/Y-/Z+/Z-)
    pub axes: Vec<Anchor>,
    /// Слоевые якоря [0..7] = L1..L8
    pub layers: Vec<Vec<Anchor>>,
    /// Доменные якоря [0..7] = D1..D8 (EXECUTION..D8)
    pub domains: Vec<Vec<Anchor>>,
    /// 8 архетипов октантов (из octants.yaml)
    pub octants: Vec<Anchor>,
    /// Универсальные семантические центры (из semantic_centers.yaml)
    pub semantic_centers: Vec<Anchor>,
    /// Подсистемы знания: "writing" → примитивы письма, "mathematics" → мат. примитивы, ...
    pub subsystems: HashMap<String, Vec<Anchor>>,
}

impl AnchorSet {
    /// Создать пустой набор (fallback если файлы не найдены).
    pub fn empty() -> Self {
        Self {
            axes: Vec::new(),
            layers: vec![Vec::new(); 8],
            domains: vec![Vec::new(); 8],
            octants: Vec::new(),
            semantic_centers: Vec::new(),
            subsystems: HashMap::new(),
        }
    }

    /// Якоря конкретной подсистемы (например, "writing" или "mathematics").
    /// Возвращает пустой срез если подсистема не загружена.
    pub fn get_subsystem(&self, name: &str) -> &[Anchor] {
        self.subsystems.get(name).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Найти якорь по его id-строке (первое совпадение).
    /// Используется AnchorMatchTable для lookup позиций по id примитива.
    pub fn get_by_id(&self, id: &str) -> Option<&Anchor> {
        self.all_anchors().find(|a| a.id == id)
    }

    /// Загрузить из `config_dir/anchors/`. Возвращает empty если директории нет.
    /// Логирует ошибки в stderr — не паникует.
    pub fn load_or_empty(config_dir: &Path) -> Self {
        match Self::load(config_dir) {
            Ok(set) => set,
            Err(e) => {
                eprintln!("[anchors] load failed: {e}, using empty anchor set");
                Self::empty()
            }
        }
    }

    /// Загрузить напрямую из директории якорей (уже содержит axes.yaml, writing/, ...).
    /// Используется когда путь к директории уже указывает на anchors/, а не на config/.
    pub fn load_dir(anchors_dir: &Path) -> Result<Self, ConfigError> {
        if !anchors_dir.exists() {
            return Ok(Self::empty());
        }
        let axes = Self::load_axes(anchors_dir)?;
        let mut layers = vec![Vec::new(); 8];
        let layers_dir = anchors_dir.join("layers");
        if layers_dir.exists() {
            for (i, layer) in layers.iter_mut().enumerate() {
                let prefix = format!("L{}_", i + 1);
                if let Some(path) = Self::find_yaml_prefix(&layers_dir, &prefix) {
                    *layer = Self::parse_layer(&path)?;
                }
            }
        }
        let mut domains = vec![Vec::new(); 8];
        let domains_dir = anchors_dir.join("domains");
        if domains_dir.exists() {
            for (i, domain) in domains.iter_mut().enumerate() {
                let prefix = format!("D{}_", i + 1);
                if let Some(path) = Self::find_yaml_prefix(&domains_dir, &prefix) {
                    *domain = Self::parse_domain(&path)?;
                }
            }
        }
        let octants = Self::load_flat(&anchors_dir.join("octants.yaml"))?;
        let semantic_centers = Self::load_flat(&anchors_dir.join("semantic_centers.yaml"))?;
        let subsystems = Self::load_subsystems(anchors_dir)?;
        Ok(Self { axes, layers, domains, octants, semantic_centers, subsystems })
    }

    /// Загрузить из `config_dir/anchors/`. Возвращает ошибку при YAML-синтаксических проблемах.
    pub fn load(config_dir: &Path) -> Result<Self, ConfigError> {
        let anchors_dir = config_dir.join("anchors");
        if !anchors_dir.exists() {
            return Ok(Self::empty());
        }

        let axes = Self::load_axes(&anchors_dir)?;

        let mut layers = vec![Vec::new(); 8];
        let layers_dir = anchors_dir.join("layers");
        if layers_dir.exists() {
            for (i, layer) in layers.iter_mut().enumerate() {
                let prefix = format!("L{}_", i + 1);
                if let Some(path) = Self::find_yaml_prefix(&layers_dir, &prefix) {
                    *layer = Self::parse_layer(&path)?;
                }
            }
        }

        let mut domains = vec![Vec::new(); 8];
        let domains_dir = anchors_dir.join("domains");
        if domains_dir.exists() {
            for (i, domain) in domains.iter_mut().enumerate() {
                let prefix = format!("D{}_", i + 1);
                if let Some(path) = Self::find_yaml_prefix(&domains_dir, &prefix) {
                    *domain = Self::parse_domain(&path)?;
                }
            }
        }

        let octants = Self::load_flat(&anchors_dir.join("octants.yaml"))?;
        let semantic_centers = Self::load_flat(&anchors_dir.join("semantic_centers.yaml"))?;
        let subsystems = Self::load_subsystems(&anchors_dir)?;

        Ok(Self {
            axes,
            layers,
            domains,
            octants,
            semantic_centers,
            subsystems,
        })
    }

    // ─── Private loaders ─────────────────────────────────────────────────────

    /// Загрузить плоский файл с якорями (octants.yaml, semantic_centers.yaml,
    /// или любой файл в subsystem-директории). Возвращает пустой Vec если файл не существует.
    fn load_flat(path: &Path) -> Result<Vec<Anchor>, ConfigError> {
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = std::fs::read_to_string(path).map_err(ConfigError::IoError)?;
        let file: FlatAnchorFile =
            serde_yaml::from_str(&content).map_err(ConfigError::ParseError)?;
        Ok(file.anchors)
    }

    /// Сканировать поддиректории `anchors_dir` как подсистемы знания.
    /// Пропускает `layers/` и `domains/` — они обрабатываются отдельно.
    fn load_subsystems(anchors_dir: &Path) -> Result<HashMap<String, Vec<Anchor>>, ConfigError> {
        let mut result: HashMap<String, Vec<Anchor>> = HashMap::new();
        let skip = ["layers", "domains"];

        let entries = match std::fs::read_dir(anchors_dir) {
            Ok(e) => e,
            Err(_) => return Ok(result),
        };

        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let name = match path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };
            if skip.contains(&name.as_str()) {
                continue;
            }
            // Загрузить все *.yaml из этой поддиректории
            let mut anchors = Vec::new();
            let yamls = match std::fs::read_dir(&path) {
                Ok(e) => e,
                Err(_) => continue,
            };
            let mut yaml_paths: Vec<_> = yamls
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("yaml"))
                .collect();
            yaml_paths.sort(); // детерминированный порядок
            for yaml_path in yaml_paths {
                let loaded = Self::load_flat(&yaml_path)?;
                anchors.extend(loaded);
            }
            if !anchors.is_empty() {
                result.insert(name, anchors);
            }
        }

        Ok(result)
    }

    fn load_axes(anchors_dir: &Path) -> Result<Vec<Anchor>, ConfigError> {
        let path = anchors_dir.join("axes.yaml");
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = std::fs::read_to_string(&path).map_err(ConfigError::IoError)?;
        let file: AxesFile = serde_yaml::from_str(&content).map_err(ConfigError::ParseError)?;
        Ok(file.axes)
    }

    fn parse_layer(path: &Path) -> Result<Vec<Anchor>, ConfigError> {
        let content = std::fs::read_to_string(path).map_err(ConfigError::IoError)?;
        let file: LayerFile = serde_yaml::from_str(&content).map_err(ConfigError::ParseError)?;
        Ok(file.anchors)
    }

    fn parse_domain(path: &Path) -> Result<Vec<Anchor>, ConfigError> {
        let content = std::fs::read_to_string(path).map_err(ConfigError::IoError)?;
        let file: DomainFile = serde_yaml::from_str(&content).map_err(ConfigError::ParseError)?;
        Ok(file.anchors)
    }

    fn find_yaml_prefix(dir: &Path, prefix: &str) -> Option<std::path::PathBuf> {
        std::fs::read_dir(dir)
            .ok()?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .find(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with(prefix) && n.ends_with(".yaml"))
                    .unwrap_or(false)
            })
    }

    // ─── Statistics ───────────────────────────────────────────────────────────

    /// Returns the total number of anchors across all categories.
    pub fn total_count(&self) -> usize {
        self.axes.len()
            + self.layers.iter().map(|l| l.len()).sum::<usize>()
            + self.domains.iter().map(|d| d.len()).sum::<usize>()
            + self.octants.len()
            + self.semantic_centers.len()
            + self.subsystems.values().map(|v| v.len()).sum::<usize>()
    }

    /// Returns `true` if the registry contains no anchors.
    pub fn is_empty(&self) -> bool {
        self.total_count() == 0
    }

    // ─── Text matching ────────────────────────────────────────────────────────

    /// Найти якоря, совпадающие с текстом.
    ///
    /// Порядок проверки: Exact → Alias → Substring (≥4 символов).
    /// Возвращает до 5 лучших совпадений, отсортированных по score.
    pub fn match_text<'a>(&'a self, text: &str) -> Vec<AnchorMatch<'a>> {
        let text_lower = text.to_lowercase();
        let words: Vec<&str> = text_lower.split_whitespace().collect();
        if words.is_empty() {
            return Vec::new();
        }

        let mut matches = Vec::new();

        for anchor in self.all_anchors() {
            let word_lower = anchor.word.to_lowercase();

            // 1. Точное совпадение
            if let Some(w) = words.iter().find(|&&w| w == word_lower.as_str()) {
                matches.push(AnchorMatch {
                    anchor,
                    score: 1.0,
                    match_type: MatchType::Exact,
                    matched_word: w.to_string(),
                });
                continue;
            }

            // 2. Alias совпадение
            let alias_hit = anchor.aliases.iter().find_map(|alias| {
                let a = alias.to_lowercase();
                words
                    .iter()
                    .find(|&&w| w == a.as_str())
                    .map(|w| w.to_string())
            });
            if let Some(w) = alias_hit {
                matches.push(AnchorMatch {
                    anchor,
                    score: 0.9,
                    match_type: MatchType::Alias,
                    matched_word: w,
                });
                continue;
            }

            // 3. Substring (только значимые — 4+ символов с обеих сторон)
            let substr_hit = words.iter().find_map(|&w| {
                if w.len() < 4 || word_lower.len() < 4 {
                    return None;
                }
                if w.contains(word_lower.as_str()) || word_lower.contains(w) {
                    Some(w.to_string())
                } else {
                    None
                }
            });
            if let Some(w) = substr_hit {
                matches.push(AnchorMatch {
                    anchor,
                    score: 0.5,
                    match_type: MatchType::Substring,
                    matched_word: w,
                });
            }
        }

        matches.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        matches.truncate(5);
        matches
    }

    fn all_anchors(&self) -> impl Iterator<Item = &Anchor> {
        self.axes
            .iter()
            .chain(self.layers.iter().flatten())
            .chain(self.domains.iter().flatten())
            .chain(self.octants.iter())
            .chain(self.semantic_centers.iter())
            .chain(self.subsystems.values().flatten())
    }

    // ─── Position / weight computation ───────────────────────────────────────

    /// Вычислить взвешенную позицию на основе совпадений.
    /// Если совпадений нет — вернуть [0.0; 3] (caller использует FNV-1a fallback).
    pub fn compute_position(&self, matches: &[AnchorMatch<'_>]) -> [f32; 3] {
        if matches.is_empty() {
            return [0.0; 3];
        }
        let total: f32 = matches.iter().map(|m| m.score).sum();
        let mut pos = [0.0f32; 3];
        for m in matches {
            let w = m.score / total;
            pos[0] += m.anchor.position[0] as f32 * w;
            pos[1] += m.anchor.position[1] as f32 * w;
            pos[2] += m.anchor.position[2] as f32 * w;
        }
        pos
    }

    /// Вычислить semantic_weight (0.0..1.0) на основе лучшего совпадения.
    ///
    /// Exact=0.95, Alias=0.90, Substring=0.75, no match=0.80 (текущий дефолт).
    pub fn compute_semantic_weight(&self, matches: &[AnchorMatch<'_>]) -> f32 {
        match matches.first() {
            None => 0.80,
            Some(m) => 0.70 + m.score * 0.25,
        }
    }

    /// Вычислить взвешенный Shell-профиль [L1..L8] на основе совпадений.
    /// Если совпадений нет — вернуть [0; 8].
    pub fn compute_shell(&self, matches: &[AnchorMatch<'_>]) -> [u8; 8] {
        if matches.is_empty() {
            return [0u8; 8];
        }
        let total: f32 = matches.iter().map(|m| m.score).sum();
        let mut shell = [0f32; 8];
        for m in matches {
            let w = m.score / total;
            for (sh, &av) in shell.iter_mut().zip(m.anchor.shell.iter()) {
                *sh += av as f32 * w;
            }
        }
        shell.map(|v| v.round() as u8)
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_anchor(word: &str, aliases: &[&str], pos: [i16; 3]) -> Anchor {
        Anchor {
            id: word.to_string(),
            word: word.to_string(),
            aliases: aliases.iter().map(|s| s.to_string()).collect(),
            tags: Vec::new(),
            position: pos,
            shell: [0; 8],
            description: String::new(),
        }
    }

    fn set_with_axes() -> AnchorSet {
        let mut s = AnchorSet::empty();
        s.axes.push(make_anchor(
            "порядок",
            &["структура", "закон"],
            [30000, 0, 0],
        ));
        s.axes.push(make_anchor(
            "хаос",
            &["творчество", "поток"],
            [-30000, 0, 0],
        ));
        s.axes
            .push(make_anchor("жизнь", &["рост", "связь"], [0, 30000, 0]));
        s
    }

    #[test]
    fn test_empty_set_no_matches() {
        let s = AnchorSet::empty();
        assert!(s.match_text("тест").is_empty());
    }

    #[test]
    fn test_exact_match() {
        let s = set_with_axes();
        let m = s.match_text("порядок");
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].match_type, MatchType::Exact);
        assert!((m[0].score - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_alias_match() {
        let s = set_with_axes();
        let m = s.match_text("структура");
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].match_type, MatchType::Alias);
        assert!((m[0].score - 0.9).abs() < 1e-6);
    }

    #[test]
    fn test_multiple_words_blended_position() {
        let s = set_with_axes();
        // "порядок" → [30000, 0, 0], "жизнь" → [0, 30000, 0]
        // Blended: score одинаковый → avg = [15000, 15000, 0]
        let m = s.match_text("порядок жизнь");
        assert_eq!(m.len(), 2);
        let pos = s.compute_position(&m);
        assert!((pos[0] - 15000.0).abs() < 1.0);
        assert!((pos[1] - 15000.0).abs() < 1.0);
    }

    #[test]
    fn test_empty_input_no_matches() {
        let s = set_with_axes();
        assert!(s.match_text("").is_empty());
    }

    #[test]
    fn test_no_match_default_weight() {
        let s = set_with_axes();
        let m = s.match_text("нетакогослова");
        assert!((s.compute_semantic_weight(&m) - 0.80).abs() < 1e-6);
    }

    #[test]
    fn test_total_count() {
        let s = set_with_axes();
        assert_eq!(s.total_count(), 3);
    }

    #[test]
    fn test_load_nonexistent_dir() {
        let s = AnchorSet::load_or_empty(Path::new("/nonexistent/dir/that/does/not/exist"));
        assert!(s.is_empty());
    }

    #[test]
    fn test_get_subsystem_empty() {
        let s = AnchorSet::empty();
        assert!(s.get_subsystem("writing").is_empty());
        assert!(s.get_subsystem("mathematics").is_empty());
    }

    #[test]
    fn test_get_subsystem_insert_get() {
        let mut s = AnchorSet::empty();
        let a = make_anchor("точка", &["dot"], [0, 0, 6000]);
        s.subsystems.insert("writing".to_string(), vec![a]);
        let ws = s.get_subsystem("writing");
        assert_eq!(ws.len(), 1);
        assert_eq!(ws[0].word, "точка");
        assert!(s.get_subsystem("mathematics").is_empty());
    }

    #[test]
    fn test_total_count_includes_octants_and_subsystems() {
        let mut s = AnchorSet::empty();
        s.octants.push(make_anchor("октант1", &[], [20000, 18000, 22000]));
        s.semantic_centers.push(make_anchor("истина", &[], [22000, 15000, 18000]));
        s.subsystems.insert(
            "writing".to_string(),
            vec![
                make_anchor("точка", &[], [0, 0, 6000]),
                make_anchor("вертикаль", &[], [10000, 3000, 12000]),
            ],
        );
        // 1 octant + 1 semantic_center + 2 subsystem = 4
        assert_eq!(s.total_count(), 4);
    }

    #[test]
    fn test_octants_included_in_match() {
        let mut s = AnchorSet::empty();
        s.octants.push(make_anchor("утверждение", &["affirmation"], [20000, 18000, 22000]));
        let m = s.match_text("утверждение");
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].match_type, MatchType::Exact);
    }

    #[test]
    fn test_semantic_centers_included_in_match() {
        let mut s = AnchorSet::empty();
        s.semantic_centers.push(make_anchor("истина", &["правда"], [22000, 15000, 18000]));
        let m = s.match_text("правда");
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].match_type, MatchType::Alias);
    }

    #[test]
    fn test_subsystem_anchors_included_in_match() {
        let mut s = AnchorSet::empty();
        s.subsystems.insert(
            "mathematics".to_string(),
            vec![make_anchor("функция", &["отображение"], [12000, 10000, 9000])],
        );
        let m = s.match_text("функция");
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].match_type, MatchType::Exact);
    }

    #[test]
    fn test_load_flat_missing_file_returns_empty() {
        let result = AnchorSet::load_flat(Path::new("/nonexistent/octants.yaml"));
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_load_subsystems_missing_dir_returns_empty() {
        let result = AnchorSet::load_subsystems(Path::new("/nonexistent/anchors/"));
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_integration_load_config_dir() {
        // Integration test: load from actual config directory if present.
        let config_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.join("config"));
        let Some(config_dir) = config_dir else { return };
        if !config_dir.exists() { return }

        let s = AnchorSet::load_or_empty(&config_dir);
        // writing/primitives.yaml → 7 anchors
        assert_eq!(s.get_subsystem("writing").len(), 7, "writing primitives");
        // mathematics/primitives.yaml → 7 anchors
        assert_eq!(s.get_subsystem("mathematics").len(), 7, "mathematics primitives");
        // octants.yaml → 8 anchors
        assert_eq!(s.octants.len(), 8, "octants");
        // semantic_centers.yaml → 10 anchors
        assert_eq!(s.semantic_centers.len(), 10, "semantic centers");
        // all axes loaded
        assert_eq!(s.axes.len(), 6, "axes");
    }
}
