// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// ModalityStore — хранилище модальностей Frame-анкеров.
//
// Источник: Cross_Modal_Binding_V1_0.md §2
//
// Инвариант: modality НЕ хранится в Token (64 байта HARD) — только здесь.
// Ключ: sutra_id Frame-анкера в EXPERIENCE.
// Дефолт для неизвестных: Modality::Text (все существующие Frame до V1.0).

use std::collections::HashMap;

/// Модальность — источник перцептивного входа для Frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modality {
    /// TextPerceptor — символьный вход (работает).
    Text,
    /// L0VisionPerceptor — визуальный вход (частично, V7-E2).
    Vision,
    /// EXPERIENCE без внешнего входа (воспоминание, DREAM-отчёт).
    Internal,
}

impl Modality {
    pub fn name(self) -> &'static str {
        match self {
            Modality::Text => "text",
            Modality::Vision => "vision",
            Modality::Internal => "internal",
        }
    }
}

/// Хранилище модальностей Frame-анкеров.
///
/// Поле ContextRecognizer (аналогично DilemmaStore).
/// Заполняется engine при обработке InjectToken + InjectFrameAnchor.
#[derive(Debug, Clone, Default)]
pub struct ModalityStore {
    map: HashMap<u32, Modality>,
}

impl ModalityStore {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    /// Зарегистрировать/обновить модальность Frame.
    pub fn insert(&mut self, sutra_id: u32, modality: Modality) {
        self.map.insert(sutra_id, modality);
    }

    /// Модальность Frame (дефолт Text если неизвестна).
    pub fn get(&self, sutra_id: u32) -> Modality {
        self.map.get(&sutra_id).copied().unwrap_or(Modality::Text)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Удалить устаревшие записи (Frame больше не в EXPERIENCE).
    pub fn retain_known(&mut self, known_ids: &[u32]) {
        let set: std::collections::HashSet<u32> = known_ids.iter().copied().collect();
        self.map.retain(|id, _| set.contains(id));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_text() {
        let store = ModalityStore::new();
        assert_eq!(store.get(42), Modality::Text);
    }

    #[test]
    fn test_insert_and_get() {
        let mut store = ModalityStore::new();
        store.insert(1, Modality::Vision);
        assert_eq!(store.get(1), Modality::Vision);
        store.insert(2, Modality::Internal);
        assert_eq!(store.get(2), Modality::Internal);
        assert_eq!(store.get(3), Modality::Text); // unknown → Text
    }

    #[test]
    fn test_retain_known() {
        let mut store = ModalityStore::new();
        store.insert(1, Modality::Vision);
        store.insert(2, Modality::Vision);
        store.insert(3, Modality::Text);
        store.retain_known(&[1, 3]);
        assert_eq!(store.get(1), Modality::Vision);
        assert_eq!(store.get(3), Modality::Text);
        assert_eq!(store.get(2), Modality::Text); // evicted → default Text
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn test_modality_names() {
        assert_eq!(Modality::Text.name(), "text");
        assert_eq!(Modality::Vision.name(), "vision");
        assert_eq!(Modality::Internal.name(), "internal");
    }
}
