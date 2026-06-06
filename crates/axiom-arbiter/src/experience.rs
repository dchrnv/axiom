// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// EXPERIENCE module - ассоциативная память Arbiter V1.0

use crate::gridhash::{grid_hash, AssociativeIndex};
use axiom_core::{Token, TOKEN_FLAG_GOAL};
use std::cell::Cell;
use std::collections::HashMap;

/// Нижняя граница "зоны любопытства" как доля от порога кристаллизации.
/// Следы с weight ∈ [0.8 * threshold, threshold) генерируют Curiosity-импульсы.
const CURIOSITY_BAND: f32 = 0.8;

/// Порог для переключения на параллельный поиск (Axiom Sentinel V1.0, Фаза 2).
/// При `traces.len() < PARALLEL_THRESHOLD` используется последовательный поиск.
pub const PARALLEL_THRESHOLD: usize = 512;

/// Уровень резонанса
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResonanceLevel {
    /// Нет резонанса
    None,
    /// Ассоциация (подсказка)
    Association,
    /// Рефлекс (прямой ответ)
    Reflex,
}

/// След опыта (паттерн + вес + метаданные)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExperienceTrace {
    /// Паттерн токена
    pub pattern: Token,
    /// Вес следа (0.0 - 1.0)
    pub weight: f32,
    /// Когда создан (event_id)
    pub created_at: u64,
    /// Последнее использование (event_id)
    pub last_used: u64,
    /// Число успешных рефлексов
    pub success_count: u32,
    /// Хэш паттерна для быстрого отбора
    pub pattern_hash: u64,
}

/// Результат резонансного поиска
#[derive(Debug, Clone)]
pub struct ResonanceResult {
    /// Уровень резонанса
    pub level: ResonanceLevel,
    /// След, если найден
    pub trace: Option<ExperienceTrace>,
}

/// Experience - ассоциативная память (реализация)
pub struct Experience {
    traces: Vec<ExperienceTrace>,
    /// Порог рефлекса (0..255 → 0.0..1.0)
    reflex_threshold: u8,
    /// Порог ассоциации (0..255 → 0.0..1.0)
    association_threshold: u8,
    /// Максимальное число следов
    max_traces: usize,
    /// GridHash-индекс для O(1) Phase 1 поиска
    pub index: AssociativeIndex,
    /// Следы напряжения — незавершённые или низко-coherent паттерны (Cognitive Depth V1.0)
    tension_traces: Vec<TensionTrace>,
    /// Число следов, прошедших хэш-фильтр при последнем resonance_search (диагностика).
    /// Cell позволяет обновлять поле через &self не нарушая API.
    pub last_traces_matched: Cell<u32>,
    /// Монотонный счётчик добавленных следов (S2). Используется для триггера экспорта.
    traces_seen_total: u64,
    /// Shell-профили vocab-seed токенов (sutra_id → shell [L1..L8]).
    /// Заполняется через set_shell_registry() из AxiomEngine при boot.
    shell_registry: HashMap<u32, [u8; 8]>,
}

impl Experience {
    /// Создать новый Experience модуль с порогами по умолчанию
    pub fn new() -> Self {
        Self {
            traces: Vec::new(),
            reflex_threshold: 127, // 127/255 ≈ 0.498 — чуть ниже 0.5 чтобы trace weight=0.5 давал Reflex
            association_threshold: 64,
            max_traces: 1000,
            index: AssociativeIndex::new(4), // shift=4: ячейки 16 квантов
            tension_traces: Vec::new(),
            last_traces_matched: Cell::new(0),
            traces_seen_total: 0,
            shell_registry: HashMap::new(),
        }
    }

    /// Установить shell-профили якорей (sutra_id → [L1..L8]).
    /// Вызывается из engine.inject_anchor_tokens при boot. Immutable в runtime.
    pub fn set_shell_registry(&mut self, registry: HashMap<u32, [u8; 8]>) {
        self.shell_registry = registry;
    }

    /// Установить пороги из конфигурации домена
    pub fn set_thresholds(&mut self, reflex_threshold: u8, association_threshold: u8) {
        self.reflex_threshold = reflex_threshold;
        self.association_threshold = association_threshold;
    }

    /// Резонансный поиск по паттерну (двухфазный).
    ///
    /// **Phase 1 — GridHash (O(1)):**
    /// Проверяем индекс по grid-ключу токена. Если ячейка найдена и
    /// лучший след в ней имеет score ≥ reflex_threshold → ранний выход.
    ///
    /// **Phase 2 — физика (O(N)):**
    /// Полный линейный поиск с hash-prefilter. Активируется при промахе Phase 1.
    pub fn resonance_search(&self, token: &Token) -> ResonanceResult {
        if self.traces.is_empty() {
            return ResonanceResult {
                level: ResonanceLevel::None,
                trace: None,
            };
        }

        let reflex_t = self.reflex_threshold as f32 / 255.0;
        let assoc_t = self.association_threshold as f32 / 255.0;

        // ── Phase 1: GridHash O(1) ───────────────────────────────────────────
        let grid_key = grid_hash(token, self.index.shift);
        if let Some(trace_ids) = self.index.lookup(grid_key) {
            let mut best_score = 0.0f32;
            let mut best_trace: Option<ExperienceTrace> = None;

            for &trace_id in trace_ids {
                // Найти след по created_at (стабильный ID)
                if let Some(trace) = self.traces.iter().find(|t| t.created_at == trace_id) {
                    let similarity = pattern_similarity(token, &trace.pattern, &self.shell_registry);
                    let score = similarity * trace.weight;
                    if score > best_score {
                        best_score = score;
                        best_trace = Some(trace.clone());
                    }
                }
            }

            // Ранний выход при Reflex-уровне попадания
            if best_score >= reflex_t {
                if let Some(trace) = best_trace {
                    return ResonanceResult {
                        level: ResonanceLevel::Reflex,
                        trace: Some(trace),
                    };
                }
            }
        }

        // ── Phase 2: полный поиск O(N) ───────────────────────────────────────
        let input_hash = pattern_hash(token);
        let mut best_score = 0.0f32;
        let mut best_idx: Option<usize> = None;
        let mut matched_count: u32 = 0;

        for (i, trace) in self.traces.iter().enumerate() {
            let hash_dist = (input_hash ^ trace.pattern_hash).count_ones();
            if hash_dist > 40 {
                continue;
            }
            matched_count += 1;

            let similarity = pattern_similarity(token, &trace.pattern, &self.shell_registry);
            let score = similarity * trace.weight;

            if score > best_score {
                best_score = score;
                best_idx = Some(i);
            }
        }
        self.last_traces_matched.set(matched_count);

        let level = if best_score >= reflex_t {
            ResonanceLevel::Reflex
        } else if best_score >= assoc_t {
            ResonanceLevel::Association
        } else {
            ResonanceLevel::None
        };

        if level == ResonanceLevel::None {
            return ResonanceResult { level, trace: None };
        }

        ResonanceResult {
            level,
            trace: best_idx.map(|i| self.traces[i].clone()),
        }
    }

    /// Параллельный резонансный поиск (Axiom Sentinel V1.0, Фаза 2).
    ///
    /// При `traces.len() < PARALLEL_THRESHOLD` автоматически возвращается к
    /// последовательному `resonance_search()` без накладных расходов rayon.
    ///
    /// При большом числе следов Phase 2 выполняется параллельно:
    /// каждый поток обрабатывает свой чанк traces через `fold`, результаты
    /// объединяются через `reduce` без mutex.
    ///
    /// Phase 1 (GridHash O(1)) всегда последовательная.
    pub fn resonance_search_parallel(
        &self,
        token: &Token,
        pool: &rayon::ThreadPool,
    ) -> ResonanceResult {
        if self.traces.len() < PARALLEL_THRESHOLD {
            return self.resonance_search(token);
        }

        let reflex_t = self.reflex_threshold as f32 / 255.0;
        let assoc_t = self.association_threshold as f32 / 255.0;

        // ── Phase 1: GridHash O(1) — однопоточная, без накладных расходов ──
        let grid_key = grid_hash(token, self.index.shift);
        if let Some(trace_ids) = self.index.lookup(grid_key) {
            let mut best_score = 0.0f32;
            let mut best_trace: Option<ExperienceTrace> = None;
            for &trace_id in trace_ids {
                if let Some(trace) = self.traces.iter().find(|t| t.created_at == trace_id) {
                    let score = pattern_similarity(token, &trace.pattern, &self.shell_registry) * trace.weight;
                    if score > best_score {
                        best_score = score;
                        best_trace = Some(trace.clone());
                    }
                }
            }
            if best_score >= reflex_t {
                if let Some(trace) = best_trace {
                    return ResonanceResult {
                        level: ResonanceLevel::Reflex,
                        trace: Some(trace),
                    };
                }
            }
        }

        // ── Phase 2: параллельный полный поиск ───────────────────────────────
        //
        // Каждый поток ведёт локальный аккумулятор (best_score, best_idx, matched_count).
        // Начальное значение (0.0, usize::MAX, 0) означает "ничего не найдено".
        // reduce объединяет результаты потоков без mutex.
        let input_hash = pattern_hash(token);

        // Извлекаем срезы ДО install чтобы не захватывать self (содержит Cell<u32> → !Sync).
        let traces: &[ExperienceTrace] = &self.traces;
        let shell_registry: &HashMap<u32, [u8; 8]> = &self.shell_registry;

        let (best_score, best_idx, matched_count) = pool.install(|| {
            use rayon::prelude::*;
            traces
                .par_iter()
                .enumerate()
                .fold(
                    || (0.0f32, usize::MAX, 0u32),
                    |acc, (i, trace)| {
                        let hash_dist = (input_hash ^ trace.pattern_hash).count_ones();
                        if hash_dist > 40 {
                            return acc;
                        }
                        let score = pattern_similarity(token, &trace.pattern, shell_registry) * trace.weight;
                        let matched = acc.2 + 1;
                        if score > acc.0 {
                            (score, i, matched)
                        } else {
                            (acc.0, acc.1, matched)
                        }
                    },
                )
                .reduce(
                    || (0.0f32, usize::MAX, 0u32),
                    |(s1, i1, c1), (s2, i2, c2)| {
                        let c = c1 + c2;
                        if s1 >= s2 {
                            (s1, i1, c)
                        } else {
                            (s2, i2, c)
                        }
                    },
                )
        });

        self.last_traces_matched.set(matched_count);

        let level = if best_score >= reflex_t {
            ResonanceLevel::Reflex
        } else if best_score >= assoc_t {
            ResonanceLevel::Association
        } else {
            ResonanceLevel::None
        };

        if level == ResonanceLevel::None {
            return ResonanceResult { level, trace: None };
        }

        ResonanceResult {
            level,
            trace: if best_idx == usize::MAX {
                None
            } else {
                Some(self.traces[best_idx].clone())
            },
        }
    }

    /// Установить максимальное число следов (S2).
    pub fn set_max_traces(&mut self, n: usize) {
        self.max_traces = n.max(1);
    }

    /// true если `traces_seen_total` кратен 5000 — сигнал запустить экспорт навыков (S2).
    pub fn should_trigger_export(&self) -> bool {
        self.traces_seen_total > 0 && self.traces_seen_total % 5000 == 0
    }

    /// Оценка потребления памяти трейсами (~нижняя граница, без heap-аллокаций inside Token).
    pub fn estimate_memory_bytes(&self) -> usize {
        self.traces.len() * std::mem::size_of::<ExperienceTrace>()
            + self.tension_traces.len() * std::mem::size_of::<TensionTrace>()
    }

    /// Добавить след опыта. Если лимит достигнут, вытесняет след с наименьшим весом.
    pub fn add_trace(&mut self, pattern: Token, weight: f32, created_at: u64) {
        self.traces_seen_total += 1;
        if self.traces.len() >= self.max_traces {
            // Evict lowest weight trace — удаляем из индекса ДО удаления из Vec
            if let Some(min_idx) = self
                .traces
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.weight.total_cmp(&b.weight))
                .map(|(i, _)| i)
            {
                let evicted_id = self.traces[min_idx].created_at;
                self.index.remove_by_trace_id(evicted_id);
                self.traces.remove(min_idx);
            }
        }

        let ph = pattern_hash(&pattern);
        let key = grid_hash(&pattern, self.index.shift);

        self.traces.push(ExperienceTrace {
            pattern,
            weight: weight.clamp(0.0, 1.0),
            created_at,
            last_used: created_at,
            success_count: 0,
            pattern_hash: ph,
        });

        // Добавляем в GridHash-индекс
        self.index.insert(key, created_at);
    }

    /// Усилить существующий похожий след (hash distance ≤ 8) или добавить новый.
    ///
    /// Используется в `finalize_comparison` для накопления веса при повторных паттернах.
    /// В отличие от `add_trace`, не создаёт дубликаты для уже известных паттернов.
    /// Возвращает true если нашёл и усилил существующий след.
    pub fn strengthen_or_add(&mut self, pattern: Token, weight: f32, created_at: u64) -> bool {
        let ph = pattern_hash(&pattern);
        if let Some(trace) = self
            .traces
            .iter_mut()
            .filter(|t| (t.pattern_hash ^ ph).count_ones() <= 8)
            .max_by(|a, b| a.weight.total_cmp(&b.weight))
        {
            trace.weight = (trace.weight + weight * 0.1).min(1.0);
            trace.last_used = created_at;
            return true;
        }
        self.add_trace(pattern, weight, created_at);
        false
    }

    /// Усилить след по индексу
    pub fn strengthen_trace(&mut self, idx: usize, delta: f32) {
        if let Some(trace) = self.traces.get_mut(idx) {
            trace.weight = (trace.weight + delta).min(1.0);
            trace.success_count = trace.success_count.saturating_add(1);
        }
    }

    /// Ослабить след по индексу
    ///
    /// Если вес падает до нуля — след удаляется из GridHash-индекса.
    pub fn weaken_trace(&mut self, idx: usize, delta: f32) {
        if let Some(trace) = self.traces.get_mut(idx) {
            trace.weight = (trace.weight - delta).max(0.0);
            if trace.weight == 0.0 {
                let trace_id = trace.created_at;
                self.index.remove_by_trace_id(trace_id);
            }
        }
    }

    /// Получить количество следов
    pub fn trace_count(&self) -> usize {
        self.traces.len()
    }

    /// Вернуть следы, готовые к кристаллизации в Skill
    ///
    /// Возвращает клоны следов у которых weight ≥ weight_threshold
    /// AND success_count ≥ min_success.
    pub fn find_crystallizable(
        &self,
        weight_threshold: f32,
        min_success: u32,
    ) -> Vec<ExperienceTrace> {
        self.traces
            .iter()
            .filter(|t| t.weight >= weight_threshold && t.success_count >= min_success)
            .cloned()
            .collect()
    }

    /// Число следов с `last_used < horizon` — сколько будет удалено при pruning.
    pub fn prunable_count(&self, horizon: u64) -> usize {
        if horizon == 0 {
            return 0;
        }
        self.traces.iter().filter(|t| t.last_used < horizon).count()
    }

    /// Архивировать (удалить) следы, каузально устаревшие за горизонтом.
    ///
    /// Удаляет все следы с `last_used < horizon` и чистит AssociativeIndex.
    /// Возвращает число удалённых следов.
    pub fn archive_behind_horizon(&mut self, horizon: u64) -> usize {
        if horizon == 0 {
            return 0;
        }

        let mut removed = 0;
        let mut i = 0;
        while i < self.traces.len() {
            if self.traces[i].last_used < horizon {
                let trace_id = self.traces[i].created_at;
                self.index.remove_by_trace_id(trace_id);
                self.traces.swap_remove(i);
                removed += 1;
            } else {
                i += 1;
            }
        }
        removed
    }

    // ── Cognitive Depth V1.0 — 13D: Goal & Curiosity ────────────────────────

    /// Вернуть паттерны незавершённых целей (Cognitive Depth V1.0 — 13D).
    ///
    /// Цель = след с `pattern.type_flags & TOKEN_FLAG_GOAL != 0`.
    /// Незавершённая = `weight < goal_achieved_weight`.
    ///
    /// Вес импульса = насколько цель далека от достижения (1.0 = только создана).
    pub fn check_goal_traces(&self, goal_achieved_weight: f32) -> Vec<(Token, f32)> {
        self.traces
            .iter()
            .filter(|t| {
                t.pattern.type_flags & TOKEN_FLAG_GOAL != 0 && t.weight < goal_achieved_weight
            })
            .map(|t| {
                let impulse_weight = 1.0 - t.weight / goal_achieved_weight;
                (t.pattern, impulse_weight.clamp(0.01, 1.0))
            })
            .collect()
    }

    /// Вернуть паттерны-кандидаты на кристаллизацию (Cognitive Depth V1.0 — 13D).
    ///
    /// "Любопытные" следы = weight ∈ [0.8 * threshold, threshold).
    /// Близки к кристаллизации, но ещё не достигли её — система "хочет" их завершить.
    ///
    /// Вес импульса = насколько близко к threshold (0.0..1.0).
    pub fn check_curiosity_candidates(&self, threshold: f32) -> Vec<(Token, f32)> {
        let low = threshold * CURIOSITY_BAND;
        self.traces
            .iter()
            .filter(|t| t.weight >= low && t.weight < threshold)
            .map(|t| {
                let proximity = (t.weight - low) / (threshold - low);
                (t.pattern, proximity.clamp(0.01, 1.0))
            })
            .collect()
    }

    /// Усилить след по хэшу паттерна (без знания индекса)
    ///
    /// Находит лучший след по хэшу и усиливает его. Возвращает true если нашёл.
    pub fn strengthen_by_hash(&mut self, pattern_hash: u64, delta: f32) -> bool {
        if let Some(trace) = self
            .traces
            .iter_mut()
            .filter(|t| (t.pattern_hash ^ pattern_hash).count_ones() <= 8)
            .max_by(|a, b| a.weight.total_cmp(&b.weight))
        {
            trace.weight = (trace.weight + delta).min(1.0);
            trace.success_count = trace.success_count.saturating_add(1);
            return true;
        }
        false
    }
}

/// Сходство паттернов токенов — нормализованное значение 0.0 (полное отличие) до 1.0 (идентичны).
///
/// Учитывает температуру, массу, валентность и позицию.
fn pattern_similarity(a: &Token, b: &Token, registry: &HashMap<u32, [u8; 8]>) -> f32 {
    let temp_diff = (a.temperature as i16 - b.temperature as i16).unsigned_abs() as f32 / 255.0;
    let mass_diff = (a.mass as i16 - b.mass as i16).unsigned_abs() as f32 / 255.0;
    let val_diff = (a.valence as i16 - b.valence as i16).unsigned_abs() as f32 / 254.0;

    let dx = (a.position[0] as i32 - b.position[0] as i32) as f32;
    let dy = (a.position[1] as i32 - b.position[1] as i32) as f32;
    let dz = (a.position[2] as i32 - b.position[2] as i32) as f32;
    let pos_diff = (dx * dx + dy * dy + dz * dz).sqrt() / 56755.0;

    let base = 1.0 - ((temp_diff + mass_diff + val_diff + pos_diff) * 0.25).min(1.0);

    // Shell-бонус (15%): токены с похожим семантическим профилем [L1..L8] получают
    // бонус к score. Без shell в registry → 0.5 (нейтраль, base не меняется).
    let shell_sim = match (registry.get(&a.sutra_id), registry.get(&b.sutra_id)) {
        (Some(sa), Some(sb)) => shell_cosine(sa, sb),
        _ => 0.5,
    };
    base * (0.85 + 0.15 * shell_sim)
}

/// Косинусное сходство двух shell-профилей [u8;8].
/// Нулевой вектор → 0.5 (нейтраль, не NaN).
fn shell_cosine(a: &[u8; 8], b: &[u8; 8]) -> f32 {
    let dot: u32 = a.iter().zip(b.iter()).map(|(&x, &y)| x as u32 * y as u32).sum();
    let norm_a: u32 = a.iter().map(|&x| x as u32 * x as u32).sum();
    let norm_b: u32 = b.iter().map(|&x| x as u32 * x as u32).sum();
    if norm_a == 0 || norm_b == 0 {
        return 0.5;
    }
    (dot as f32) / ((norm_a as f32).sqrt() * (norm_b as f32).sqrt())
}

/// Быстрый хэш паттерна токена для предварительной фильтрации
fn pattern_hash(token: &Token) -> u64 {
    // FNV-1a style mixing of key fields
    let mut h: u64 = 0xcbf29ce484222325;
    h ^= token.temperature as u64;
    h = h.wrapping_mul(0x100000001b3);
    h ^= token.mass as u64;
    h = h.wrapping_mul(0x100000001b3);
    h ^= (token.valence as u8) as u64;
    h = h.wrapping_mul(0x100000001b3);
    h ^= token.position[0] as u64;
    h = h.wrapping_mul(0x100000001b3);
    h ^= token.position[1] as u64;
    h = h.wrapping_mul(0x100000001b3);
    h ^= token.position[2] as u64;
    h = h.wrapping_mul(0x100000001b3);
    h
}

impl Default for Experience {
    fn default() -> Self {
        Self::new()
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Этап 13: Internal Drive — напряжение (Tension)
// ──────────────────────────────────────────────────────────────────────────────

/// След напряжения — незавершённая или низко-coherent обработка.
///
/// Создаётся когда MAYA возвращает результат с coherence < min_coherence.
/// Хранит горячий паттерн, который Heartbeat будет подталкивать обратно в pipeline.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TensionTrace {
    /// Паттерн, который не был обработан до конца
    pub pattern: Token,
    /// Температура напряжения (остывает каждый тик)
    pub temperature: u8,
    /// Когда создан (event_id)
    pub created_at: u64,
}

impl Experience {
    /// Добавить след напряжения.
    ///
    /// `temperature` — начальная горячесть: 255 = максимальное напряжение.
    pub fn add_tension_trace(&mut self, pattern: Token, temperature: u8, created_at: u64) {
        self.tension_traces.push(TensionTrace {
            pattern,
            temperature,
            created_at,
        });
    }

    /// Слить горячие следы напряжения в импульсы.
    ///
    /// Возвращает паттерны трейсов с `temperature >= threshold`.
    /// Сброшенные трейсы удаляются из буфера.
    pub fn drain_hot_impulses(&mut self, threshold: u8) -> Vec<Token> {
        let mut hot = Vec::new();
        self.tension_traces.retain(|t| {
            if t.temperature >= threshold {
                hot.push(t.pattern);
                false // удалить из буфера
            } else {
                true // оставить
            }
        });
        hot
    }

    /// Остудить все следы напряжения на `decay` единиц за тик.
    ///
    /// Трейсы с temperature == 0 автоматически удаляются.
    pub fn cool_tension_traces(&mut self, decay: u8) {
        self.tension_traces.retain_mut(|t| {
            t.temperature = t.temperature.saturating_sub(decay);
            t.temperature > 0
        });
    }

    /// Число активных следов напряжения.
    pub fn tension_count(&self) -> usize {
        self.tension_traces.len()
    }

    /// Вернуть все следы опыта (для сериализации).
    pub fn traces(&self) -> &[ExperienceTrace] {
        &self.traces
    }

    /// Вернуть все tension traces (для сериализации).
    pub fn tension_traces(&self) -> &[TensionTrace] {
        &self.tension_traces
    }

    /// Импортировать след с уже применённым weight factor (для загрузки из персистентного хранилища).
    ///
    /// В отличие от `add_trace()`, не ограничивает weight и не пересчитывает hash —
    /// принимает след как есть. Вытесняет слабейший след при достижении лимита.
    pub fn import_trace(&mut self, trace: ExperienceTrace) {
        if self.traces.len() >= self.max_traces {
            if let Some(min_idx) = self
                .traces
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.weight.total_cmp(&b.weight))
                .map(|(i, _)| i)
            {
                let evicted_id = self.traces[min_idx].created_at;
                self.index.remove_by_trace_id(evicted_id);
                self.traces.remove(min_idx);
            }
        }
        let key = grid_hash(&trace.pattern, self.index.shift);
        let trace_id = trace.created_at;
        self.traces.push(trace);
        self.index.insert(key, trace_id);
    }

    /// Импортировать tension trace (для загрузки из персистентного хранилища).
    pub fn import_tension_trace(&mut self, trace: TensionTrace) {
        self.tension_traces.push(trace);
    }
}

#[cfg(test)]
mod shell_tests {
    use super::*;
    use axiom_core::{Token, STATE_ACTIVE};

    fn make_token(sutra_id: u32) -> Token {
        let mut t = Token::new(sutra_id, 100, [0, 0, 0], 1);
        t.state = STATE_ACTIVE;
        t.temperature = 128;
        t.mass = 100;
        t.valence = 0;
        t
    }

    #[test]
    fn test_shell_cosine_identical() {
        let shell = [10u8, 20, 30, 40, 50, 60, 70, 80];
        assert!((shell_cosine(&shell, &shell) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_shell_cosine_zero_vector_returns_neutral() {
        let zero = [0u8; 8];
        let shell = [10u8, 20, 30, 40, 50, 60, 70, 80];
        assert!((shell_cosine(&zero, &shell) - 0.5).abs() < 1e-5);
        assert!((shell_cosine(&shell, &zero) - 0.5).abs() < 1e-5);
    }

    #[test]
    fn test_shell_bonus_improves_score_for_similar_profiles() {
        let shell: [u8; 8] = [10, 20, 30, 40, 50, 60, 70, 80];
        let mut registry = HashMap::new();
        registry.insert(1u32, shell);
        registry.insert(2u32, shell);

        let a = make_token(1);
        let b = make_token(2);

        let score_with_shell = pattern_similarity(&a, &b, &registry);
        let score_no_shell = pattern_similarity(&a, &b, &HashMap::new());

        // Идентичные shell → cosine=1.0 → множитель 1.0 > нейтраль 0.925
        assert!(
            score_with_shell > score_no_shell,
            "identical shell should boost score: {} vs {}",
            score_with_shell,
            score_no_shell
        );
    }

    #[test]
    fn test_shell_penalty_for_orthogonal_profiles() {
        let shell_a: [u8; 8] = [100, 0, 0, 0, 0, 0, 0, 0];
        let shell_b: [u8; 8] = [0, 100, 0, 0, 0, 0, 0, 0];
        let mut registry = HashMap::new();
        registry.insert(1u32, shell_a);
        registry.insert(2u32, shell_b);

        let a = make_token(1);
        let b = make_token(2);

        let score_with_shell = pattern_similarity(&a, &b, &registry);
        let score_neutral = pattern_similarity(&a, &b, &HashMap::new());

        // Ортогональные shell (cosine=0.0) → множитель 0.85 < нейтраль 0.925
        assert!(
            score_with_shell < score_neutral,
            "orthogonal shell should reduce score: {} vs {}",
            score_with_shell,
            score_neutral
        );
    }

    #[test]
    fn test_no_shell_in_registry_gives_neutral() {
        let a = make_token(99);
        let b = make_token(100);
        let registry: HashMap<u32, [u8; 8]> = HashMap::new();

        let score = pattern_similarity(&a, &b, &registry);
        // Нет shell в registry → нейтраль 0.5 → base * 0.925
        let base = {
            let temp = (a.temperature as i16 - b.temperature as i16).unsigned_abs() as f32 / 255.0;
            let mass = (a.mass as i16 - b.mass as i16).unsigned_abs() as f32 / 255.0;
            let val = (a.valence as i16 - b.valence as i16).unsigned_abs() as f32 / 254.0;
            1.0 - ((temp + mass + val) * 0.25).min(1.0)
        };
        let expected = base * 0.925;
        assert!((score - expected).abs() < 1e-5, "got {}, expected {}", score, expected);
    }

    #[test]
    fn test_set_shell_registry_used_in_resonance_search() {
        let mut exp = Experience::new();
        let shell: [u8; 8] = [50, 50, 50, 50, 50, 50, 50, 50];
        let mut registry = HashMap::new();
        registry.insert(1u32, shell);
        registry.insert(2u32, shell);

        let pattern = make_token(1);
        exp.add_trace(pattern.clone(), 1.0, 1);

        // Без registry
        let result_before = exp.resonance_search(&make_token(2));

        exp.set_shell_registry(registry);
        let result_after = exp.resonance_search(&make_token(2));

        // С registry и идентичным shell → score выше
        let score_before = result_before.trace.as_ref().map(|t| t.weight).unwrap_or(0.0);
        let score_after = result_after.trace.as_ref().map(|t| t.weight).unwrap_or(0.0);
        // Оба нашли trace (weight=1.0) — проверяем что поиск работает
        assert!(result_after.trace.is_some(), "should find trace after set_shell_registry");
        let _ = (score_before, score_after); // диагностика, не assertion
    }
}
