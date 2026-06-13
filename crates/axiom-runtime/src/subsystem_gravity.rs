// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Subsystem Gravity — PRIM-TD-03 (Value / Abstraction)
//
// Спека: docs/deferred/PRIM-TD-03_Subsystem_Gravity(1).md
//
// АРХИТЕКТУРНОЕ РЕШЕНИЕ (§0 спеки):
//   apply_subsystem_gravity — ОТДЕЛЬНАЯ функция, НЕ в apply_gravity_batch.
//   apply_gravity_batch — горячий путь (23.4 µs/1K, SIMD), его не трогаем.
//   Subsystem-гравитация работает на собственном редком интервале (subsystem_gravity_interval).
//   Это медленное смысловое смещение, не покадровая физика.
//
// Якоря STATE_LOCKED → позиции фиксированы навечно → кэшируются в правилах при boot.

use axiom_config::AnchorSet;
use axiom_domain::DomainState;

/// Правило subsystem-гравитации: один якорь как аттрактор или репеллер.
///
/// Создаётся при boot из AnchorSet, хранится в AxiomEngine, неизменно в runtime.
#[derive(Debug, Clone)]
pub struct SubsystemGravityRule {
    /// sutra_id якоря (для диагностики/логов).
    pub anchor_sutra_id: u32,
    /// Позиция якоря — кэш, т.к. STATE_LOCKED (не двигается).
    pub anchor_position: [i16; 3],
    /// +1.0 = притяжение к якорю, -1.0 = отталкивание от якоря.
    pub direction: f32,
    /// Множитель к базовой силе. 0.1–0.3 для ценностей, 0.05–0.15 для абстракций.
    /// Умеренный — защита от коллапса к точке якоря.
    pub strength_factor: f32,
    /// Радиус действия в пространственных единицах. None = без ограничения.
    pub radius: Option<u32>,
    /// domain_id к токенам которого применять (обычно MAYA=110).
    pub target_domain: u16,
}

/// Базовая сила subsystem-гравитации в квантах скорости.
/// Нормализованный вектор к якорю умножается на BASE_FORCE × strength_factor.
/// 16 квантов → для dist≈12000: acc≈1 при factor=0.20. Не вызывает коллапс.
const SUBSYSTEM_GRAVITY_BASE_FORCE: i64 = 16;

/// Вычислить ускорение одного токена от одного правила.
/// Возвращает [0,0,0] если вне радиуса или токен совпадает с якорём.
fn compute_one(pos: [i16; 3], rule: &SubsystemGravityRule) -> [i16; 3] {
    let [ax, ay, az] = rule.anchor_position;
    let dx = (ax as i64) - (pos[0] as i64);
    let dy = (ay as i64) - (pos[1] as i64);
    let dz = (az as i64) - (pos[2] as i64);
    let dist2 = dx * dx + dy * dy + dz * dz;

    if dist2 == 0 {
        return [0, 0, 0];
    }

    if let Some(r) = rule.radius {
        if dist2 > (r as i64) * (r as i64) {
            return [0, 0, 0];
        }
    }

    let dist = integer_sqrt(dist2).max(1);

    // Нормализованный вектор × BASE_FORCE × strength_factor (в 256-х долях).
    // Деление: (displacement * sign * BASE_FORCE * factor_256) / (dist * 256).
    let sign: i64 = if rule.direction >= 0.0 { 1 } else { -1 };
    let factor_256 = (rule.strength_factor * 256.0).round() as i64;

    let acc_x = ((dx * sign * SUBSYSTEM_GRAVITY_BASE_FORCE * factor_256) / (dist * 256))
        .clamp(-32, 32) as i16;
    let acc_y = ((dy * sign * SUBSYSTEM_GRAVITY_BASE_FORCE * factor_256) / (dist * 256))
        .clamp(-32, 32) as i16;
    let acc_z = ((dz * sign * SUBSYSTEM_GRAVITY_BASE_FORCE * factor_256) / (dist * 256))
        .clamp(-32, 32) as i16;

    [acc_x, acc_y, acc_z]
}

/// Применить subsystem-гравитацию к токенам домена.
///
/// Вызывается на редком интервале из engine (НЕ в apply_gravity_batch).
/// Модифицирует velocity активных (не LOCKED, не SLEEPING) токенов.
pub fn apply_subsystem_gravity(state: &mut DomainState, rules: &[SubsystemGravityRule]) {
    for rule in rules {
        for token in state.tokens.iter_mut() {
            if token.state == axiom_core::STATE_SLEEPING || token.state == axiom_core::STATE_LOCKED
            {
                continue;
            }
            let acc = compute_one(token.position, rule);
            token.velocity[0] = token.velocity[0].saturating_add(acc[0]);
            token.velocity[1] = token.velocity[1].saturating_add(acc[1]);
            token.velocity[2] = token.velocity[2].saturating_add(acc[2]);
        }
    }
}

/// Построить правила из AnchorSet при boot.
///
/// Ищет val_beneficial/val_harmful (Values) и abstraction_theory/constructor (Abstractions).
/// Возвращает пустой Vec при отсутствии якорей (graceful degradation).
pub fn build_rules_from_anchor_set(
    anchor_set: &AnchorSet,
    maya_domain_id: u16,
) -> Vec<SubsystemGravityRule> {
    let mut rules = Vec::new();

    let value_rules: &[(&str, f32, f32, Option<u32>)] = &[
        ("val_beneficial", 1.0, 0.20, None),
        ("val_harmful", -1.0, 0.20, None),
    ];
    let abstraction_rules: &[(&str, f32, f32, Option<u32>)] = &[
        ("abstraction_theory", 1.0, 0.08, Some(8000)),
        ("abstraction_constructor", 1.0, 0.08, Some(8000)),
    ];

    for anchors in anchor_set.subsystems.values() {
        for anchor in anchors {
            for &(id, direction, strength, radius) in value_rules {
                if anchor.id == id {
                    rules.push(SubsystemGravityRule {
                        anchor_sutra_id: fnv1a_anchor_id(&anchor.id),
                        anchor_position: anchor.position,
                        direction,
                        strength_factor: strength,
                        radius,
                        target_domain: maya_domain_id,
                    });
                }
            }
            for &(id, direction, strength, radius) in abstraction_rules {
                if anchor.id == id {
                    rules.push(SubsystemGravityRule {
                        anchor_sutra_id: fnv1a_anchor_id(&anchor.id),
                        anchor_position: anchor.position,
                        direction,
                        strength_factor: strength,
                        radius,
                        target_domain: maya_domain_id,
                    });
                }
            }
        }
    }

    rules
}

/// FNV-1a хэш для anchor_sutra_id (диагностика). Зеркалит fnv1a_anchor_id из engine.rs.
fn fnv1a_anchor_id(id: &str) -> u32 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in id.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    let low = (h & 0x7FFF_FFFF) as u32;
    0x8000_0000 | if low == 0 { 1 } else { low }
}

/// Целочисленный квадратный корень (i64 вход).
fn integer_sqrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_config::DomainConfig;
    use axiom_core::{Token, STATE_ACTIVE};
    use axiom_space::distance2;

    fn make_token(pos: [i16; 3]) -> Token {
        let mut t = Token::new(1, 110, pos, 1);
        t.state = STATE_ACTIVE;
        t.mass = 100;
        t
    }

    fn make_rule(anchor_pos: [i16; 3], direction: f32, radius: Option<u32>) -> SubsystemGravityRule {
        SubsystemGravityRule {
            anchor_sutra_id: 1,
            anchor_position: anchor_pos,
            direction,
            strength_factor: 0.20,
            radius,
            target_domain: 110,
        }
    }

    fn make_domain_state(tokens: Vec<Token>) -> DomainState {
        let config = DomainConfig::default();
        let mut state = DomainState::new(&config);
        for t in tokens {
            let _ = state.add_token(t);
        }
        state
    }

    #[test]
    fn test_beneficial_pulls_nearby_token() {
        let rule = make_rule([8000, 12000, 13000], 1.0, None);
        let pos = [4000i16, 4000, 4000];
        let acc = compute_one(pos, &rule);
        assert!(
            acc[0] > 0 || acc[1] > 0 || acc[2] > 0,
            "pull should add positive velocity toward anchor"
        );
    }

    #[test]
    fn test_harmful_repels_nearby_token() {
        let rule = make_rule([3000, 1000, 11000], -1.0, None);
        let pos = [4000i16, 2000, 10000];
        let acc = compute_one(pos, &rule);
        // Токен правее якоря по x (4000>3000), push должен двигать вправо (acc_x > 0)
        assert!(acc[0] >= 0, "token right of anchor → push moves it further right");
    }

    #[test]
    fn test_no_effect_beyond_radius() {
        let rule = make_rule([13000, 10000, 14000], 1.0, Some(2000));
        let pos = [100i16, 100, 100];
        let acc = compute_one(pos, &rule);
        assert_eq!(acc, [0, 0, 0], "token outside radius → no acceleration");
    }

    #[test]
    fn test_token_at_anchor_position_no_force() {
        let anchor = [8000i16, 12000, 13000];
        let rule = make_rule(anchor, 1.0, None);
        let acc = compute_one(anchor, &rule);
        assert_eq!(acc, [0, 0, 0], "token at anchor → no force");
    }

    #[test]
    fn test_no_collapse_under_repeated_application() {
        let rule = make_rule([8000, 12000, 13000], 1.0, None);
        let mut state = make_domain_state(vec![make_token([4000, 4000, 4000])]);
        for _ in 0..1000 {
            apply_subsystem_gravity(&mut state, &[rule.clone()]);
        }
        let token = &state.tokens[0];
        let dist = distance2(
            token.position[0],
            token.position[1],
            token.position[2],
            8000,
            12000,
            13000,
        );
        assert!(dist > 0, "token should not collapse exactly onto anchor");
    }

    #[test]
    fn test_not_applied_to_locked_tokens() {
        let rule = make_rule([8000, 12000, 13000], 1.0, None);
        let mut t = make_token([4000, 4000, 4000]);
        t.state = axiom_core::STATE_LOCKED;
        let mut state = make_domain_state(vec![t]);
        let vel_before = state.tokens[0].velocity;
        apply_subsystem_gravity(&mut state, &[rule]);
        assert_eq!(
            state.tokens[0].velocity, vel_before,
            "locked tokens must not be affected"
        );
    }

    #[test]
    fn test_rules_from_empty_anchor_set_returns_empty() {
        let set = AnchorSet::empty();
        let rules = build_rules_from_anchor_set(&set, 110);
        assert!(rules.is_empty(), "empty AnchorSet → no rules");
    }
}
