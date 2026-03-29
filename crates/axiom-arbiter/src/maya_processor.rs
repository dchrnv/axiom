// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// MAYA Processor - консолидация результатов ASHTI

use axiom_config::DomainConfig;
use axiom_core::Token;

/// MAYA Processor - консолидация результатов ASHTI через взвешенное усреднение
pub struct MayaProcessor;

impl MayaProcessor {
    /// Консолидировать результаты и вернуть оценку согласованности (coherence).
    ///
    /// Возвращает `(token, confidence)` где confidence ∈ 0.0..=1.0.
    /// При confidence < порога (min_coherence из DomainConfig) Arbiter может
    /// инициировать повторный проход (multi-pass, Cognitive Depth V1.0).
    pub fn consolidate_with_confidence(
        ashti_results: Vec<Token>,
        maya_domain: &DomainConfig,
    ) -> (Token, f32) {
        let confidence = if ashti_results.len() >= 2 {
            compute_confidence(&ashti_results)
        } else {
            1.0 // один или ноль результатов — нет конфликта
        };
        let token = Self::consolidate_results(ashti_results, maya_domain);
        (token, confidence)
    }

    /// Консолидировать результаты от ASHTI доменов.
    ///
    /// - Если результатов нет → нулевой токен
    /// - Если один результат → возвращается без изменений
    /// - Если несколько → взвешенное среднее числовых полей
    ///
    /// Confidence score (0.0–1.0) вычисляется как доля полей, согласующихся
    /// у большинства результатов. При низком confidence возвращается медианный
    /// результат вместо среднего.
    pub fn consolidate_results(
        ashti_results: Vec<Token>,
        maya_domain: &DomainConfig,
    ) -> Token {
        match ashti_results.len() {
            0 => zero_token(),
            1 => ashti_results.into_iter().next().unwrap(),
            _ => {
                let confidence = compute_confidence(&ashti_results);

                // arbiter_flags bit 0: force median consolidation
                let force_median = (maya_domain.arbiter_flags & 0x01) != 0;

                if force_median || confidence < 0.5 {
                    median_token(&ashti_results)
                } else {
                    average_token(&ashti_results)
                }
            }
        }
    }
}

/// Нулевой токен как sentinel для пустого результата
fn zero_token() -> Token {
    Token {
        sutra_id: 0,
        domain_id: 0,
        type_flags: 0,
        position: [0, 0, 0],
        velocity: [0, 0, 0],
        target: [0, 0, 0],
        reserved_phys: 0,
        valence: 0,
        mass: 0,
        temperature: 0,
        state: 0,
        lineage_hash: 0,
        momentum: [0, 0, 0],
        resonance: 0,
        last_event_id: 0,
    }
}

/// Взвешенное (равновесное) среднее числовых полей токена.
/// Идентификационные поля берутся от первого токена.
fn average_token(tokens: &[Token]) -> Token {
    let n = tokens.len() as f32;
    let first = &tokens[0];

    let temp = tokens.iter().map(|t| t.temperature as f32).sum::<f32>() / n;
    let mass = tokens.iter().map(|t| t.mass as f32).sum::<f32>() / n;
    let valence = tokens.iter().map(|t| t.valence as f32).sum::<f32>() / n;
    let resonance = tokens.iter().map(|t| t.resonance as f64).sum::<f64>() / n as f64;

    let px = tokens.iter().map(|t| t.position[0] as f32).sum::<f32>() / n;
    let py = tokens.iter().map(|t| t.position[1] as f32).sum::<f32>() / n;
    let pz = tokens.iter().map(|t| t.position[2] as f32).sum::<f32>() / n;

    let vx = tokens.iter().map(|t| t.velocity[0] as f32).sum::<f32>() / n;
    let vy = tokens.iter().map(|t| t.velocity[1] as f32).sum::<f32>() / n;
    let vz = tokens.iter().map(|t| t.velocity[2] as f32).sum::<f32>() / n;

    // Lineage hash: XOR-fold all hashes for provenance
    let lineage = tokens.iter().fold(0u64, |acc, t| acc ^ t.lineage_hash);

    Token {
        sutra_id: first.sutra_id,
        domain_id: first.domain_id,
        type_flags: first.type_flags,
        position: [px.round() as i16, py.round() as i16, pz.round() as i16],
        velocity: [vx.round() as i16, vy.round() as i16, vz.round() as i16],
        target: first.target,
        reserved_phys: 0,
        valence: valence.round() as i8,
        mass: mass.round() as u8,
        temperature: temp.round() as u8,
        state: first.state,
        lineage_hash: lineage,
        momentum: first.momentum,
        resonance: resonance.round() as u32,
        last_event_id: first.last_event_id,
    }
}

/// Медианный токен: выбирает токен с наименьшим суммарным отклонением от остальных.
fn median_token(tokens: &[Token]) -> Token {
    let mut best_idx = 0;
    let mut best_score = f32::MAX;

    for (i, a) in tokens.iter().enumerate() {
        let score: f32 = tokens.iter()
            .map(|b| token_distance(a, b))
            .sum();
        if score < best_score {
            best_score = score;
            best_idx = i;
        }
    }

    tokens[best_idx]
}

/// Оценка согласованности: доля числовых полей, где все токены отличаются
/// не более чем на tolerance от среднего.
fn compute_confidence(tokens: &[Token]) -> f32 {
    let n = tokens.len() as f32;
    let mut agreed = 0u32;
    let mut total = 0u32;

    // Temperature
    let avg_t = tokens.iter().map(|t| t.temperature as f32).sum::<f32>() / n;
    if tokens.iter().all(|t| (t.temperature as f32 - avg_t).abs() < 20.0) { agreed += 1; }
    total += 1;

    // Mass
    let avg_m = tokens.iter().map(|t| t.mass as f32).sum::<f32>() / n;
    if tokens.iter().all(|t| (t.mass as f32 - avg_m).abs() < 15.0) { agreed += 1; }
    total += 1;

    // Valence
    let avg_v = tokens.iter().map(|t| t.valence as f32).sum::<f32>() / n;
    if tokens.iter().all(|t| (t.valence as f32 - avg_v).abs() < 10.0) { agreed += 1; }
    total += 1;

    // Position X
    let avg_px = tokens.iter().map(|t| t.position[0] as f32).sum::<f32>() / n;
    if tokens.iter().all(|t| (t.position[0] as f32 - avg_px).abs() < 50.0) { agreed += 1; }
    total += 1;

    agreed as f32 / total as f32
}

/// Евклидово расстояние по ключевым числовым полям
fn token_distance(a: &Token, b: &Token) -> f32 {
    let dt = (a.temperature as f32 - b.temperature as f32).powi(2);
    let dm = (a.mass as f32 - b.mass as f32).powi(2);
    let dv = (a.valence as f32 - b.valence as f32).powi(2);
    let dx = (a.position[0] as f32 - b.position[0] as f32).powi(2);
    let dy = (a.position[1] as f32 - b.position[1] as f32).powi(2);
    let dz = (a.position[2] as f32 - b.position[2] as f32).powi(2);
    (dt + dm + dv + dx + dy + dz).sqrt()
}
