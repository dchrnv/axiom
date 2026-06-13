// dataset.rs — AxiomDataset формат (.axiom.yaml) + Chunk структуры.
//
// Дефолт: inject_mode = grow (книга через MAYA).
// anchor = только явно, для фундаментных наборов (boot/DREAMING, инвариант 11).
// subsystem_hint = верификационный, не директивный (кроме anchor-режима).

use serde::Deserialize;

/// Режим инъекции чанка.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InjectMode {
    /// Живой режим: через MAYA, кристаллизуется, может забыться. Дефолт.
    #[default]
    Grow,
    /// Якорный режим: STATE_LOCKED в SUTRA, boot/DREAMING, инвариант 11.
    Anchor,
}

/// Метаданные датасета.
#[derive(Debug, Clone, Deserialize)]
pub struct DatasetMetadata {
    pub name: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub inject_mode: InjectMode,
    /// Пауза между инъекциями чанков (в тиках). 0 = без паузы.
    #[serde(default)]
    pub ticks_between: u32,
    /// Верификационная подсказка подсистемы. Не навязывает в grow-режиме.
    #[serde(default)]
    pub subsystem_hint: Option<String>,
}

fn default_version() -> String {
    "1.0".to_string()
}

/// Одна запись датасета.
#[derive(Debug, Clone, Deserialize)]
pub struct DatasetEntry {
    #[serde(default)]
    pub id: String,
    pub content: String,
    #[serde(default)]
    pub tags: Vec<String>,
    /// Верификационная подсказка подсистемы для этой записи.
    #[serde(default)]
    pub subsystem: Option<String>,
    /// Переопределить inject_mode из metadata.
    #[serde(default)]
    pub inject_mode: Option<InjectMode>,
}

/// Загруженный AxiomDataset (.axiom.yaml).
#[derive(Debug, Clone, Deserialize)]
pub struct AxiomDataset {
    pub metadata: DatasetMetadata,
    pub entries: Vec<DatasetEntry>,
}

impl AxiomDataset {
    /// Загрузить датасет из .axiom.yaml файла.
    pub fn from_yaml(path: &std::path::Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        serde_yaml::from_str(&content)
            .map_err(|e| format!("parse error in {}: {e}", path.display()))
    }

    /// Конвертировать записи в Chunk-и для инъекции.
    pub fn to_chunks(&self) -> Vec<Chunk> {
        self.entries
            .iter()
            .map(|e| Chunk {
                content: e.content.clone(),
                kind: ChunkKind::Paragraph,
                section_header: None,
                tags: e.tags.clone(),
                subsystem_hint: e
                    .subsystem
                    .clone()
                    .or_else(|| self.metadata.subsystem_hint.clone()),
                inject_mode: e.inject_mode.unwrap_or(self.metadata.inject_mode),
            })
            .collect()
    }
}

/// Вид чанка.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkKind {
    /// Заголовок секции (# / ## / ###)
    Header,
    /// Обычный абзац
    Paragraph,
}

/// Один семантический чанк для инъекции.
#[derive(Debug, Clone)]
pub struct Chunk {
    pub content: String,
    pub kind: ChunkKind,
    /// Контент заголовка секции, которой принадлежит этот абзац (для COMPOSITION bonds).
    pub section_header: Option<String>,
    pub tags: Vec<String>,
    /// Верификационная подсказка подсистемы (не директива в grow-режиме).
    pub subsystem_hint: Option<String>,
    pub inject_mode: InjectMode,
}

impl Chunk {
    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inject_mode_default_is_grow() {
        let yaml = r#"
metadata:
  name: "test"
entries:
  - id: "e1"
    content: "Тестовый текст"
"#;
        let ds: AxiomDataset = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(ds.metadata.inject_mode, InjectMode::Grow);
    }

    #[test]
    fn test_dataset_to_chunks_subsystem_hint() {
        let yaml = r#"
metadata:
  name: "test"
  subsystem_hint: "logic"
entries:
  - id: "e1"
    content: "Если A то B"
  - id: "e2"
    content: "Причина и следствие"
    subsystem: "abstractions"
"#;
        let ds: AxiomDataset = serde_yaml::from_str(yaml).unwrap();
        let chunks = ds.to_chunks();
        assert_eq!(chunks.len(), 2);
        // e1 наследует hint из metadata
        assert_eq!(chunks[0].subsystem_hint.as_deref(), Some("logic"));
        // e2 переопределяет своим subsystem
        assert_eq!(chunks[1].subsystem_hint.as_deref(), Some("abstractions"));
    }

    #[test]
    fn test_anchor_mode_explicit() {
        let yaml = r#"
metadata:
  name: "foundation"
  inject_mode: "anchor"
entries:
  - id: "f1"
    content: "Фундаментальное знание"
"#;
        let ds: AxiomDataset = serde_yaml::from_str(yaml).unwrap();
        let chunks = ds.to_chunks();
        assert_eq!(chunks[0].inject_mode, InjectMode::Anchor);
    }
}
