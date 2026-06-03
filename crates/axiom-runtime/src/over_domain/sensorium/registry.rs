// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

use crate::over_domain::sensorium::levels::CollectionLevel;

/// Запись о потребителе среза.
#[derive(Debug, Clone)]
pub struct ConsumerEntry {
    /// Имя потребителя (для диагностики).
    pub name: &'static str,
    /// Максимальный уровень который разрешён данному потребителю (CODEX-права V1: захардкожено).
    pub max_level: CollectionLevel,
}

/// Реестр потребителей Sensorium.
///
/// Sensorium знает всех потребителей заранее и строит оптимальное расписание:
/// не собирает то что на этом тике никому не нужно,
/// не собирает дважды если двоим нужно одно.
///
/// V1.0: потребители захардкожены. V2.0: регистрация динамическая (через CODEX).
#[derive(Debug, Clone)]
pub struct ConsumerRegistry {
    consumers: Vec<ConsumerEntry>,
}

impl Default for ConsumerRegistry {
    fn default() -> Self {
        Self::v1_defaults()
    }
}

impl ConsumerRegistry {
    /// Дефолтный реестр V1.0: Workstation (State), NeuralAdvisor (Full), Debug (Full).
    pub fn v1_defaults() -> Self {
        Self {
            consumers: vec![
                ConsumerEntry {
                    name: "Workstation",
                    max_level: CollectionLevel::State,
                },
                ConsumerEntry {
                    name: "NeuralAdvisor",
                    max_level: CollectionLevel::Full,
                },
                ConsumerEntry {
                    name: "Debug",
                    max_level: CollectionLevel::Full,
                },
            ],
        }
    }

    /// Максимальный уровень нужный кому-либо из потребителей прямо сейчас.
    /// Sensorium собирает не более этого уровня — нет избыточной работы.
    pub fn max_required_level(&self) -> CollectionLevel {
        self.consumers
            .iter()
            .map(|c| c.max_level)
            .max()
            .unwrap_or(CollectionLevel::Pulse)
    }

    pub fn consumer_count(&self) -> usize {
        self.consumers.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ConsumerEntry> {
        self.consumers.iter()
    }
}
