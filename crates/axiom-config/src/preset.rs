// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Пресеты токенов и связей — загрузка готовых конфигураций из YAML

use serde::{Deserialize, Serialize};

/// Пресет токена — готовая конфигурация для инициализации Token.
///
/// Имя пресета берётся из имени YAML-файла (без расширения).
///
/// # YAML-формат
///
/// ```yaml
/// domain_id: 100
/// type_flags: 1
/// position: [0, 0, 0]
/// velocity: [0, 0, 0]
/// valence: 0
/// mass: 128
/// temperature: 64
/// state: 1
/// resonance: 440
/// description: "Базовый токен ввода"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPreset {
    /// Имя пресета (задаётся при загрузке из имени файла)
    #[serde(default)]
    pub name: String,
    /// Идентификатор домена
    pub domain_id: u16,
    /// Флаги типа токена
    #[serde(default)]
    pub type_flags: u16,
    /// Начальная позиция [x, y, z]
    #[serde(default)]
    pub position: [i16; 3],
    /// Начальная скорость [vx, vy, vz]
    #[serde(default)]
    pub velocity: [i16; 3],
    /// Валентность токена
    #[serde(default)]
    pub valence: i8,
    /// Масса токена
    #[serde(default = "default_mass")]
    pub mass: u8,
    /// Температура токена
    #[serde(default = "default_temperature")]
    pub temperature: u8,
    /// Состояние токена
    #[serde(default = "default_state")]
    pub state: u8,
    /// Резонанс токена
    #[serde(default = "default_resonance")]
    pub resonance: u32,
    /// Человекочитаемое описание пресета
    #[serde(default)]
    pub description: String,
}

fn default_mass() -> u8 {
    128
}
fn default_temperature() -> u8 {
    64
}
fn default_state() -> u8 {
    1
}
fn default_resonance() -> u32 {
    440
}

/// Пресет связи — готовая конфигурация для инициализации Connection.
///
/// Имя пресета берётся из имени YAML-файла (без расширения).
///
/// # YAML-формат
///
/// ```yaml
/// type: strong
/// strength: 1.0
/// decay_rate: 0.001
/// gate_complexity: 1
/// flags: 1
/// description: "Сильная стабильная связь"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPreset {
    /// Имя пресета (задаётся при загрузке из имени файла)
    #[serde(default)]
    pub name: String,
    /// Тип связи
    #[serde(rename = "type")]
    pub connection_type: String,
    /// Сила связи (0.0 – 10.0)
    pub strength: f32,
    /// Скорость затухания
    pub decay_rate: f32,
    /// Сложность гейта
    #[serde(default = "default_gate_complexity")]
    pub gate_complexity: u32,
    /// Флаги состояния
    #[serde(default = "default_flags")]
    pub flags: u16,
    /// Человекочитаемое описание пресета
    #[serde(default)]
    pub description: String,
}

fn default_gate_complexity() -> u32 {
    1
}
fn default_flags() -> u16 {
    1
}
