// AXIOM MODULE: ASHTI PROCESSOR - Обработка токенов в ASHTI 1-8 доменах
//
// 8 интерпретирующих сред, каждая применяет уникальную физику
// для анализа входящего паттерна.
//
// Связанные спецификации:
// - docs/spec/Ashti_Core_v2_0.md (ASHTI 1-8)
// - docs/spec/Domain V2.0.md

use crate::token::Token;
use crate::domain::DomainConfig;
use crate::experience::ExperienceTrace;

/// ASHTI процессор - обработка токенов через 8 интерпретирующих доменов
pub struct AshtiProcessor;

impl AshtiProcessor {
    /// Главная функция обработки токена в ASHTI домене
    ///
    /// Применяет физику домена к токену на основе structural_role.
    /// Опционально использует hint из EXPERIENCE как контекст.
    pub fn process_token(
        token: &Token,
        domain: &DomainConfig,
        hint: Option<&ExperienceTrace>,
    ) -> Token {
        let mut result = token.clone();

        // Применяем hint если есть (ассоциация из EXPERIENCE)
        if let Some(trace) = hint {
            // Hint влияет на начальное состояние обработки
            Self::apply_hint(&mut result, trace);
        }

        // Применяем физику домена согласно structural_role
        match domain.structural_role {
            1 => Self::apply_execution_physics(&mut result, domain),
            2 => Self::apply_shadow_physics(&mut result, domain),
            3 => Self::apply_codex_physics(&mut result, domain),
            4 => Self::apply_map_physics(&mut result, domain),
            5 => Self::apply_probe_physics(&mut result, domain),
            6 => Self::apply_logic_physics(&mut result, domain),
            7 => Self::apply_dream_physics(&mut result, domain),
            8 => Self::apply_void_physics(&mut result, domain),
            _ => {
                // Неизвестная роль - без изменений
            }
        }

        result
    }

    /// Применить hint из EXPERIENCE (ассоциация)
    fn apply_hint(token: &mut Token, hint: &ExperienceTrace) {
        // Hint слегка сдвигает начальную температуру к ассоциации
        let hint_temp = hint.pattern.temperature as f32 * hint.weight;
        let current_temp = token.temperature as f32;
        token.temperature = ((current_temp * 0.8 + hint_temp * 0.2) as u16).min(255) as u8;
    }

    /// EXECUTION (1) - Проверка реальности, конкретизация
    ///
    /// Физика: Увеличивает массу (делает конкретным), снижает скорость (стабилизация),
    /// приводит температуру к среднему значению домена.
    fn apply_execution_physics(token: &mut Token, domain: &DomainConfig) {
        // Увеличиваем массу (конкретизация)
        token.mass = ((token.mass as u16 * 110) / 100).min(255) as u8;

        // Снижаем скорость (стабилизация)
        token.velocity = [
            token.velocity[0] / 2,
            token.velocity[1] / 2,
            token.velocity[2] / 2,
        ];

        // Приводим к температуре домена
        token.temperature = domain.temperature as u8;

        // Увеличиваем valence (позитивная конкретность)
        token.valence = (token.valence + 10).min(127);
    }

    /// SHADOW (2) - Симуляция угроз, негативные сценарии
    ///
    /// Физика: Добавляет негативный valence (маркер угрозы),
    /// увеличивает температуру (волатильность).
    fn apply_shadow_physics(token: &mut Token, domain: &DomainConfig) {
        // Добавляем негативный valence (угроза)
        token.valence = (token.valence - 20).max(-128);

        // Увеличиваем температуру (волатильность)
        token.temperature = ((token.temperature as u16 + 50).min(255)) as u8;

        // Увеличиваем скорость (динамика угрозы)
        token.velocity = [
            (token.velocity[0] as i32 * 3 / 2).clamp(-32768, 32767) as i16,
            (token.velocity[1] as i32 * 3 / 2).clamp(-32768, 32767) as i16,
            (token.velocity[2] as i32 * 3 / 2).clamp(-32768, 32767) as i16,
        ];

        // Немного увеличиваем массу (угроза материальна)
        token.mass = ((token.mass as u16 + 10).min(255)) as u8;
    }

    /// CODEX (3) - Проверка правил, бинарная фильтрация
    ///
    /// Физика: Если friction высокое - блокирует (LOCKED),
    /// если низкое - пропускает (ACTIVE).
    fn apply_codex_physics(token: &mut Token, domain: &DomainConfig) {
        use crate::token::{STATE_ACTIVE, STATE_LOCKED};

        // Бинарная фильтрация на основе friction
        if domain.friction_coeff > 128 {
            // Высокое трение - блокировка
            token.state = STATE_LOCKED;
            token.valence = (token.valence - 50).max(-128);
        } else {
            // Низкое трение - одобрение
            token.state = STATE_ACTIVE;
            token.valence = (token.valence + 30).min(127);
        }

        // Нормализация температуры (правила холодны)
        token.temperature = (domain.temperature as u8).min(100);
    }

    /// MAP (4) - Привязка к фактам, якорение
    ///
    /// Физика: Обнуляет скорость (фиксация), увеличивает массу (стабильность).
    fn apply_map_physics(token: &mut Token, domain: &DomainConfig) {
        // Обнуляем скорость (якорение к факту)
        token.velocity = [0, 0, 0];

        // Увеличиваем массу (тяжесть факта)
        token.mass = ((token.mass as u16 + 30).min(255)) as u8;

        // Снижаем температуру (факты холодны)
        token.temperature = (token.temperature / 2).max(50);

        // Позитивный valence (факт - это хорошо)
        token.valence = (token.valence + 15).min(127);
    }

    /// PROBE (5) - Исследование гипотез, добавление вариативности
    ///
    /// Физика: Добавляет случайность в velocity (исследование),
    /// слегка увеличивает температуру (активность).
    fn apply_probe_physics(token: &mut Token, domain: &DomainConfig) {
        // Добавляем вариативность в velocity (исследование)
        // Используем domain_id как seed для детерминированной "случайности"
        let seed = domain.domain_id as i32;
        token.velocity = [
            (token.velocity[0] as i32 + (seed % 100 - 50)).clamp(-32768, 32767) as i16,
            (token.velocity[1] as i32 + ((seed * 7) % 100 - 50)).clamp(-32768, 32767) as i16,
            (token.velocity[2] as i32 + ((seed * 13) % 100 - 50)).clamp(-32768, 32767) as i16,
        ];

        // Слегка увеличиваем температуру (активное исследование)
        token.temperature = ((token.temperature as u16 + 20).min(255)) as u8;

        // Нейтральный valence (гипотеза не хороша и не плоха)
        token.valence = token.valence / 2;
    }

    /// LOGIC (6) - Формальное рассуждение, нормализация
    ///
    /// Физика: Приводит значения к идеальным (нормализация),
    /// снижает шум, стабилизирует.
    fn apply_logic_physics(token: &mut Token, domain: &DomainConfig) {
        // Нормализация массы к среднему
        let target_mass = 128;
        token.mass = ((token.mass as i16 + target_mass) / 2).max(50).min(200) as u8;

        // Нормализация температуры
        let target_temp = domain.temperature as u16;
        token.temperature = ((token.temperature as u16 + target_temp) / 2).min(255) as u8;

        // Снижаем velocity (убираем шум)
        token.velocity = [
            token.velocity[0] / 4,
            token.velocity[1] / 4,
            token.velocity[2] / 4,
        ];

        // Нейтрализуем valence (логика беспристрастна)
        token.valence = token.valence / 4;
    }

    /// DREAM (7) - Мутации, оптимизация, творчество
    ///
    /// Физика: Высокая рандомизация (мутации для обучения),
    /// увеличенная температура (креативность).
    fn apply_dream_physics(token: &mut Token, domain: &DomainConfig) {
        // Высокая рандомизация (мутации)
        let seed = (token.sutra_id ^ domain.domain_id as u32) as i32;

        // Случайное изменение массы
        let mass_delta = (seed % 40) - 20;
        token.mass = ((token.mass as i32 + mass_delta).clamp(10, 255)) as u8;

        // Случайное изменение temperature
        let temp_delta = ((seed * 3) % 60) - 30;
        token.temperature = ((token.temperature as i32 + temp_delta).clamp(50, 255)) as u8;

        // Большие изменения velocity
        token.velocity = [
            ((seed % 200 - 100) as i16).saturating_add(token.velocity[0]),
            (((seed * 7) % 200 - 100) as i16).saturating_add(token.velocity[1]),
            (((seed * 13) % 200 - 100) as i16).saturating_add(token.velocity[2]),
        ];

        // Случайный valence
        token.valence = ((seed % 100 - 50) as i8);
    }

    /// VOID (8) - Детекция аномалий, маркировка странного
    ///
    /// Физика: Проверяет токен на аномальность, маркирует специальными флагами.
    fn apply_void_physics(token: &mut Token, domain: &DomainConfig) {
        // Проверяем на аномальность (экстремальные значения)
        let is_anomaly = token.mass < 20
            || token.mass > 240
            || token.temperature > 250
            || token.temperature < 10
            || token.valence.abs() > 100;

        if is_anomaly {
            // Маркируем аномалию через type_flags
            token.type_flags |= 0x8000; // Старший бит = аномалия

            // Снижаем температуру (охлаждаем аномалию)
            token.temperature = (token.temperature / 2).max(10);

            // Негативный valence (аномалия подозрительна)
            token.valence = (token.valence - 40).max(-128);
        } else {
            // Нормальный токен - легкая нормализация
            token.temperature = ((token.temperature as u16 + domain.temperature as u16) / 2)
                .min(255) as u8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::STATE_ACTIVE;

    fn create_test_token(id: u32) -> Token {
        let mut token = Token::default();
        token.sutra_id = id;
        token.temperature = 150;
        token.mass = 100;
        token.valence = 0;
        token.velocity = [10, 10, 10];
        token.state = STATE_ACTIVE;
        token
    }

    fn create_test_domain(role: u8) -> DomainConfig {
        let mut domain = DomainConfig::default();
        domain.domain_id = 1000 + role as u16;
        domain.structural_role = role;
        domain.temperature = 200.0;
        domain.friction_coeff = 64; // Medium friction
        domain
    }

    #[test]
    fn test_execution_physics() {
        let token = create_test_token(1);
        let domain = create_test_domain(1);

        let result = AshtiProcessor::process_token(&token, &domain, None);

        // EXECUTION увеличивает массу
        assert!(result.mass > token.mass);
        // Снижает velocity
        assert!(result.velocity[0] < token.velocity[0]);
        // Увеличивает valence
        assert!(result.valence > token.valence);
    }

    #[test]
    fn test_shadow_physics() {
        let token = create_test_token(2);
        let domain = create_test_domain(2);

        let result = AshtiProcessor::process_token(&token, &domain, None);

        // SHADOW добавляет негативный valence
        assert!(result.valence < token.valence);
        // Увеличивает температуру
        assert!(result.temperature > token.temperature);
        // Увеличивает velocity
        assert!(result.velocity[0].abs() > token.velocity[0].abs());
    }

    #[test]
    fn test_codex_physics_block() {
        let token = create_test_token(3);
        let mut domain = create_test_domain(3);
        domain.friction_coeff = 200; // High friction = block

        let result = AshtiProcessor::process_token(&token, &domain, None);

        // CODEX блокирует при высоком friction
        use crate::token::STATE_LOCKED;
        assert_eq!(result.state, STATE_LOCKED);
        assert!(result.valence < 0);
    }

    #[test]
    fn test_codex_physics_pass() {
        let token = create_test_token(3);
        let mut domain = create_test_domain(3);
        domain.friction_coeff = 50; // Low friction = pass

        let result = AshtiProcessor::process_token(&token, &domain, None);

        // CODEX пропускает при низком friction
        assert_eq!(result.state, STATE_ACTIVE);
        assert!(result.valence > 0);
    }

    #[test]
    fn test_map_physics() {
        let token = create_test_token(4);
        let domain = create_test_domain(4);

        let result = AshtiProcessor::process_token(&token, &domain, None);

        // MAP обнуляет velocity (якорение)
        assert_eq!(result.velocity, [0, 0, 0]);
        // Увеличивает массу
        assert!(result.mass > token.mass);
        // Снижает температуру
        assert!(result.temperature < token.temperature);
    }

    #[test]
    fn test_probe_physics() {
        let token = create_test_token(5);
        let domain = create_test_domain(5);

        let result = AshtiProcessor::process_token(&token, &domain, None);

        // PROBE добавляет вариативность в velocity
        assert_ne!(result.velocity, token.velocity);
        // Увеличивает температуру
        assert!(result.temperature > token.temperature);
    }

    #[test]
    fn test_logic_physics() {
        let mut token = create_test_token(6);
        token.mass = 50; // Низкая масса
        token.velocity = [100, 100, 100]; // Высокая скорость
        let domain = create_test_domain(6);

        let result = AshtiProcessor::process_token(&token, &domain, None);

        // LOGIC нормализует массу
        assert!(result.mass > token.mass);
        assert!(result.mass < 200);
        // Снижает velocity (убирает шум)
        assert!(result.velocity[0].abs() < token.velocity[0].abs());
        // Нейтрализует valence
        assert!(result.valence.abs() <= token.valence.abs());
    }

    #[test]
    fn test_dream_physics() {
        let token = create_test_token(7);
        let domain = create_test_domain(7);

        let result = AshtiProcessor::process_token(&token, &domain, None);

        // DREAM создаёт значительные изменения
        assert_ne!(result.mass, token.mass);
        assert_ne!(result.temperature, token.temperature);
        assert_ne!(result.velocity, token.velocity);
        // Valence может быть любым
    }

    #[test]
    fn test_void_physics_anomaly() {
        let mut token = create_test_token(8);
        token.mass = 5; // Аномально низкая масса
        let domain = create_test_domain(8);

        let result = AshtiProcessor::process_token(&token, &domain, None);

        // VOID маркирует аномалию
        assert!(result.type_flags & 0x8000 != 0);
        // Снижает температуру
        assert!(result.temperature < token.temperature);
        // Негативный valence
        assert!(result.valence < 0);
    }

    #[test]
    fn test_void_physics_normal() {
        let token = create_test_token(8);
        let domain = create_test_domain(8);

        let result = AshtiProcessor::process_token(&token, &domain, None);

        // VOID не маркирует нормальный токен
        assert!(result.type_flags & 0x8000 == 0);
    }

    #[test]
    fn test_hint_application() {
        let token = create_test_token(1);
        let domain = create_test_domain(1);

        let mut hint_pattern = Token::default();
        hint_pattern.temperature = 200;

        let hint = ExperienceTrace {
            pattern: hint_pattern,
            weight: 0.5,
            activation_count: 1,
            last_activation: 1,
        };

        let result = AshtiProcessor::process_token(&token, &domain, Some(&hint));

        // Hint должен слегка изменить начальную температуру
        // Проверяем что обработка произошла (результат не равен входу)
        assert_ne!(result.temperature, token.temperature);
    }

    #[test]
    fn test_all_domains_produce_different_results() {
        let token = create_test_token(100);

        let results: Vec<Token> = (1..=8)
            .map(|role| {
                let domain = create_test_domain(role);
                AshtiProcessor::process_token(&token, &domain, None)
            })
            .collect();

        // Проверяем что все 8 доменов дают разные результаты
        for i in 0..results.len() {
            for j in i + 1..results.len() {
                // Хотя бы одно свойство должно отличаться
                let different = results[i].mass != results[j].mass
                    || results[i].temperature != results[j].temperature
                    || results[i].valence != results[j].valence
                    || results[i].velocity != results[j].velocity;

                assert!(
                    different,
                    "Domain {} and {} produced identical results",
                    i + 1,
                    j + 1
                );
            }
        }
    }
}
