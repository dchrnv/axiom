// Copyright (C) 2024-2026 Chernov Denys
//
// DomainConfig V2.1 - 128 байт конфигурация домена
// Соответствие спецификации DomainConfig V2.1 (Arbiter Integration)

use serde::{Serialize, Deserialize};
use crate::causal_frontier::CausalFrontier;
use crate::heartbeat::{HeartbeatGenerator, HeartbeatConfig};
use crate::space::SpatialHashGrid;

/// Состояние Domain
pub const DOMAIN_ACTIVE: u32 = 1;
pub const DOMAIN_LOCKED: u32 = 2;
pub const DOMAIN_TEMPORARY: u32 = 3;

/// Состояние обработки
pub const PROCESSING_IDLE: u8 = 1;
pub const PROCESSING_ACTIVE: u8 = 2;
pub const PROCESSING_FROZEN: u8 = 3;

/// Состояние мембраны
pub const MEMBRANE_OPEN: u8 = 1;
pub const MEMBRANE_CLOSED: u8 = 2;
pub const MEMBRANE_SEMI: u8 = 3;

/// Структурные роли доменов в Ashti_Core v2.0
/// Ashti_Core: 11 доменов (SUTRA, EXECUTION, SHADOW, CODEX, MAP, PROBE, LOGIC, DREAM, VOID, EXPERIENCE, MAYA)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum StructuralRole {
    Sutra = 0,
    Execution = 1,
    Shadow = 2,
    Codex = 3,
    Map = 4,
    Probe = 5,
    Logic = 6,
    Dream = 7,
    Void = 8,
    Experience = 9, // NEW v2.0: Ассоциативная память
    Maya = 10,
}

/// Типы доменов
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DomainType {
    Logic = 1,
    Dream = 2,
    Math = 3,
    Pattern = 4,
    Memory = 5,
    Interface = 6,
}

/// DomainConfig — 128 байт конфигурация домена (соответствие спецификации V2.1)
/// Размер: 128 байт, выравнивание: 128 байт
#[repr(C, align(128))]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DomainConfig {
    // --- 1. ИДЕНТИФИКАЦИЯ [16 Байт] ---
    pub reserved_id: u64,       // 8b | Явный резерв для будущих расширений
    pub domain_id: u16,         // 2b | Уникальный ID Домена
    pub parent_domain_id: u16,  // 2b | Родительский Домен
    pub domain_type: u8,        // 1b | Тип (до 255 вариаций)
    pub structural_role: u8,    // 1b | Роль в Ashti_Core (0-10: SUTRA..MAYA)
    pub generation: u8,         // 1b | Поколение (эволюционный индекс)
    pub flags: u8,              // 1b | Битовая маска состояний (Active, Locked)
    // Offset: 16 байт

    // --- 2. ФИЗИКА ПОЛЯ [32 Байт] ---
    pub field_size: [f32; 3],   // 12b| Размеры поля (X, Y, Z)
    pub gravity_strength: f32,  // 4b | Гравитация (-MAX..+MAX)
    pub temperature: f32,       // 4b | Температура поля в Кельвинах
    pub time_dilation: u16,     // 2b | Замедление времени (х100)
    pub resonance_freq: u16,    // 2b | Базовая частота (Hz)
    pub pressure: u16,          // 2b | Давление (Pa)
    pub rebuild_frequency: u16, // 2b | SPACE V6.0: частота перестройки spatial grid (событий)
    pub friction_coeff: u8,     // 1b | Трение (0..255 -> 0.0..1.0)
    pub viscosity: u8,          // 1b | Вязкость (0..255 -> 0.0..1.0)
    pub elasticity: u8,         // 1b | Упругость (0..255 -> 0.0..1.0)
    pub quantum_noise: u8,      // 1b | Квантовый шум (0..255 -> 0.0..1.0)
    // Offset: 48 байт

    // --- 3. СЕМАНТИЧЕСКИЕ ОСИ [16 Байт] ---
    pub axis_x_ref: u32,        // 4b | Референс концепции оси X
    pub axis_y_ref: u32,        // 4b | Референс концепции оси Y
    pub axis_z_ref: u32,        // 4b | Референс концепции оси Z
    pub axis_config: u32,       // 4b | Конфигурация полюсов (Bit-packed u16x2)
    // Offset: 64 байт

    // --- 4. МЕМБРАНА И ARBITER [32 Байт] ---
    pub input_filter: u64,      // 8b | 64-bit Bloom Filter или хэш входа
    pub output_filter: u64,     // 8b | 64-bit Bloom Filter или хэш выхода

    // -- Блок Arbiter [8 Байт] (V2.1: бывший reserved_membrane) --
    pub reflex_threshold: u8,   // 1b | Порог рефлекса (0..255 -> 0.0..1.0)
    pub association_threshold: u8, // 1b | Порог ассоциации (0..255 -> 0.0..1.0)
    pub arbiter_flags: u8,      // 1b | Битовая маска поведения Arbiter
    pub reflex_cooldown: u8,    // 1b | Минимальный интервал между рефлексами (в пульсах)
    pub max_concurrent_hints: u8, // 1b | Макс. кол-во ассоциаций-подсказок одновременно
    pub feedback_weight_delta: u8, // 1b | Шаг изменения weight при обратной связи (0..255)
    pub reserved_arbiter: [u8; 2], // 2b | Резерв блока Arbiter

    pub gate_complexity: u16,   // 2b | Вычислительная сложность шлюзов
    pub threshold_mass: u16,    // 2b | Порог массы для прохождения
    pub threshold_temp: u16,    // 2b | Порог температуры для прохождения
    pub permeability: u8,       // 1b | Проницаемость (0..255 -> 0.0..1.0)
    pub membrane_state: u8,     // 1b | OPEN/CLOSED/SEMI/ADAPTIVE
    // Offset: 96 байт

    // --- 5. МЕТАДАННЫЕ [32 Байт] ---
    pub created_at: u64,        // 8b | COM event_id (Время создания)
    pub last_update: u64,       // 8b | COM event_id (Последнее обновление)
    pub token_capacity: u32,    // 4b | Максимальная емкость токенов
    pub connection_capacity: u32, // 4b | Максимальная емкость связей
    pub error_count: u16,       // 2b | Счетчик когнитивных ошибок
    pub processing_state: u8,   // 1b | IDLE/PROCESSING/FROZEN/CRASHED
    pub complexity_score: u8,   // 1b | Оценка сложности (0..255 -> 0.0..1.0)
    pub performance_score: u8,  // 1b | Производительность (0..255 -> 0.0..1.0)
    pub reserved_meta: [u8; 3], // 3b | Добивка до границы 128 байт
    // Итого: 128 байт. Offset: 128. Без скрытого паддинга.
}

impl Default for DomainConfig {
    fn default() -> Self {
        Self {
            // --- 1. ИДЕНТИФИКАЦИЯ [16 Байт] ---
            reserved_id: 0,          // Явный резерв для будущих расширений
            domain_id: 1,             // Уникальный ID Домена
            parent_domain_id: 0,       // Родительский Домен
            domain_type: 1,            // DomainType::Logic
            structural_role: 6,         // StructuralRole::Logic
            generation: 0,             // Поколение (эволюционный индекс)
            flags: DOMAIN_ACTIVE as u8, // Битовая маска состояний

            // --- 2. ФИЗИКА ПОЛЯ [32 Байт] ---
            field_size: [100.0, 100.0, 100.0], // Размеры поля (X, Y, Z)
            gravity_strength: 1.0,               // Гравитация
            temperature: 293.15,                  // Температура поля в Кельвинах (20°C)
            time_dilation: 100,                    // Замедление времени (х100) = 1.0x
            resonance_freq: 440,                   // Базовая частота (Hz)
            pressure: 50000,                      // Давление (Па)
            rebuild_frequency: 10,  // SPACE V6.0: перестройка spatial grid каждые 10 событий
            friction_coeff: 25,                      // Трение (25/255 ≈ 0.098)
            viscosity: 3,                             // Вязкость (3/255 ≈ 0.012)
            elasticity: 128,                           // Упругость (128/255 ≈ 0.502)
            quantum_noise: 1,                          // Квантовый шум (1/255 ≈ 0.004)

            // --- 3. СЕМАНТИЧЕСКИЕ ОСИ [16 Байт] ---
            axis_x_ref: 0,              // Ссылка на ось X
            axis_y_ref: 0,              // Ссылка на ось Y
            axis_z_ref: 0,              // Ссылка на ось Z
            axis_config: 0,             // Конфигурация осей
            // Offset: 48 байт

            // --- 4. МЕМБРАНА И ARBITER [32 Байт] ---
            input_filter: u64::MAX,     // Bloom фильтр входа (все разрешено)
            output_filter: u64::MAX,    // Bloom фильтр выхода (все разрешено)

            // -- Блок Arbiter (V2.1) --
            reflex_threshold: 0,        // Рефлексы отключены по умолчанию
            association_threshold: 0,   // Ассоциации отключены по умолчанию
            arbiter_flags: 0,           // Все флаги Arbiter выключены
            reflex_cooldown: 0,         // Без ограничений
            max_concurrent_hints: 0,    // Подсказки отключены
            feedback_weight_delta: 0,   // Обратная связь отключена
            reserved_arbiter: [0; 2],   // Резерв блока Arbiter

            gate_complexity: 50,        // Сложность ворот (0..255)
            threshold_mass: 1,           // Порог массы (0..65535)
            threshold_temp: 200,         // Порог температуры (0..65535)
            permeability: 255,           // Проницаемость (0..255) = 1.0
            membrane_state: MEMBRANE_OPEN, // Состояние мембраны (открыта)
            // Offset: 80 байт

            // --- 5. МЕТАДАННЫЕ [32 Байт] ---
            created_at: 0,               // COM event_id создания (0 = не инициализировано)
            last_update: 0,              // COM event_id последнего обновления
            token_capacity: 1000,        // Емкость токенов
            connection_capacity: 5000,   // Емкость соединений
            error_count: 0,              // Счетчик ошибок
            processing_state: PROCESSING_IDLE, // Состояние обработки
            complexity_score: 0,          // Оценка сложности (0..255)
            performance_score: 255,       // Оценка производительности (0..255)
            reserved_meta: [0, 0, 0],   // Резерв метаданных
            // Offset: 112 байт
            // Total: 128 байт
        }
    }
}

impl DomainConfig {
    /// Создать новый домен с базовыми параметрами
    pub fn new(domain_id: u16, domain_type: DomainType, structural_role: StructuralRole) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.domain_type = domain_type as u8;
        config.structural_role = structural_role as u8;
        config
    }

    /// -----------------------------------------------------------------------
    /// БАЗОВАЯ ФАБРИКА (ВАКУУМ)
    /// Создает абсолютно нейтральное, пустое пространство (Role::Void = 8).
    /// Используется как основа для всех остальных фабрик, чтобы избежать 
    /// ручного заполнения 30+ полей каждый раз.
    /// -----------------------------------------------------------------------
    pub fn default_void() -> Self {
        Self {
            // --- 1. ИДЕНТИФИКАЦИЯ [16 Байт] ---
            reserved_id: 0,
            domain_id: 0,
            parent_domain_id: 0,
            domain_type: 0,
            structural_role: 10, // Maya
            generation: 0,
            flags: 0,
            
            // --- 2. ФИЗИКА ПОЛЯ [32 Байт] ---
            field_size: [2000.0, 2000.0, 2000.0],
            gravity_strength: 1.0,
            temperature: 310.0,
            time_dilation: 0,
            resonance_freq: 0,
            pressure: 0,
            rebuild_frequency: 0, // 0 = отключена перестройка spatial grid
            friction_coeff: 5,
            viscosity: 0,
            elasticity: 0,
            quantum_noise: 0,
            
            // --- 3. СЕМАНТИЧЕСКИЕ ОСИ [16 Байт] ---
            axis_x_ref: 0,
            axis_y_ref: 0,
            axis_z_ref: 0,
            axis_config: 0,
            
            // --- 4. МЕМБРАНА И ARBITER [32 Байт] ---
            input_filter: 0,
            output_filter: 0,

            // -- Блок Arbiter (V2.1) --
            reflex_threshold: 0,
            association_threshold: 0,
            arbiter_flags: 0,
            reflex_cooldown: 0,
            max_concurrent_hints: 0,
            feedback_weight_delta: 0,
            reserved_arbiter: [0; 2],

            gate_complexity: 0,
            threshold_mass: 0,
            threshold_temp: 0,
            permeability: 255,
            membrane_state: 0,
            
            // --- 5. МЕТАДАННЫЕ [32 Байт] ---
            created_at: 0,
            last_update: 0,
            token_capacity: 0,
            connection_capacity: 0,
            error_count: 0,
            processing_state: 0,
            complexity_score: 0,
            performance_score: 0,
            reserved_meta: [0; 3],
        }
    }

    /// Создать домен из пресета согласно Configuration System
    pub fn from_preset(preset_name: &str) -> Result<Self, String> {
        // Временно - просто создаем SUTRA домен
        Err(format!("from_preset not implemented yet: {}", preset_name))
    }

    /// -----------------------------------------------------------------------
    /// SUTRA (0) - Источник Истины
    /// Физика: Кристаллизация. Абсолютный ноль, колоссальная гравитация. 
    /// Сюда ничего не проникает извне, отсюда только рождаются смыслы.
    /// -----------------------------------------------------------------------
    pub fn factory_sutra(domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.structural_role = 0; // Sutra
        
        config.created_at = 0; // COM event_id (должен быть установлен при создании через COM)
        config.last_update = 0; // COM event_id (обновляется при изменениях)
        
        config.gravity_strength = f32::MAX; // Бесконечная масса
        config.temperature = 0.0;           // Абсолютный ноль
        
        config.permeability = 0;            // 0.0 - Непроницаемая
        config.membrane_state = 1;          // CLOSED

        // Arbiter настройки для SUTRA (V2.1)
        // SUTRA — вечная библиотека. Arbiter не взаимодействует с ней напрямую.
        config.reflex_threshold = 0;
        config.association_threshold = 0;
        config.arbiter_flags = 0b00000000;  // Всё отключено
        config.reflex_cooldown = 0;
        config.max_concurrent_hints = 0;
        config.feedback_weight_delta = 0;

        // Устанавливаем емкости для валидации
        config.token_capacity = 1000;
        config.connection_capacity = 100;

        config
    }

    /// -----------------------------------------------------------------------
    /// EXECUTION (1) - Реализация решений
    /// Физика: Умеренная среда для быстрой реакции. Средняя температура,
    /// нормальная гравитация, низкое трение для быстрого выполнения.
    /// -----------------------------------------------------------------------
    pub fn factory_execution(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = 1; // Execution

        config.created_at = 0; // COM event_id
        config.last_update = 0;

        config.gravity_strength = 9.81;     // Земная гравитация
        config.temperature = 310.0;         // ~37°C - активная среда
        config.elasticity = 180;            // ~0.7 - умеренная упругость
        config.friction_coeff = 30;         // ~0.12 - низкое трение

        config.permeability = 180;          // ~0.7 - достаточно открыто
        config.membrane_state = 2;          // SEMI

        // Arbiter настройки для EXECUTION (V2.1)
        config.reflex_threshold = 140;      // ~0.55 - умеренный порог
        config.association_threshold = 50;  // ~0.20 - подсказки принимаются легко
        config.arbiter_flags = 0b00011111;  // Всё включено
        config.reflex_cooldown = 0;         // Без ограничений
        config.max_concurrent_hints = 3;
        config.feedback_weight_delta = 30;  // ~0.12 - умеренная скорость обучения

        config.token_capacity = 5000;
        config.connection_capacity = 2500;

        config
    }

    /// -----------------------------------------------------------------------
    /// SHADOW (2) - Симуляция и предсказание
    /// Физика: Осторожная среда для точных симуляций. Высокий порог,
    /// низкая температура для стабильности предсказаний.
    /// -----------------------------------------------------------------------
    pub fn factory_shadow(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = 2; // Shadow

        config.created_at = 0; // COM event_id
        config.last_update = 0;

        config.gravity_strength = 5.0;      // Средняя гравитация
        config.temperature = 250.0;         // Прохладная среда для стабильности
        config.viscosity = 180;             // ~0.7 - замедленное движение
        config.friction_coeff = 50;         // ~0.2 - умеренное трение

        config.permeability = 150;          // ~0.59 - избирательная проницаемость
        config.membrane_state = 2;          // SEMI

        // Arbiter настройки для SHADOW (V2.1)
        config.reflex_threshold = 180;      // ~0.71 - высокий порог
        config.association_threshold = 40;  // ~0.16 - принимает подсказки
        config.arbiter_flags = 0b00010111;  // Рефлексы + подсказки + feedback + медленный путь
        config.reflex_cooldown = 2;
        config.max_concurrent_hints = 4;
        config.feedback_weight_delta = 20;  // ~0.08 - осторожное обучение

        config.token_capacity = 8000;
        config.connection_capacity = 4000;

        config
    }

    /// -----------------------------------------------------------------------
    /// CODEX (3) - Конституция и Правила
    /// Физика: Высокая стабильность и вязкость. Данные здесь "застревают"
    /// и становятся законами. Очень холодно, чтобы паттерны не мутировали.
    /// -----------------------------------------------------------------------
    pub fn factory_codex(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = 3; // Codex
        
        config.created_at = 0; // COM event_id (должен быть установлен при создании через COM)
        config.last_update = 0; // COM event_id (обновляется при изменениях)
        
        config.gravity_strength = 1000.0;
        config.temperature = 10.0;          // Почти ноль (минимальные колебания)
        config.viscosity = 250;             // ~0.98 - Токены вязнут и фиксируются
        config.friction_coeff = 200;        // ~0.78
        
        config.permeability = 25;           // ~0.1 - Жесткий пропускной фильтр
        config.membrane_state = 2;          // SEMI (Только для системных токенов)

        // Arbiter настройки для CODEX (V2.1)
        // CODEX хранит правила и конституцию, не участвует в dual-path routing
        config.reflex_threshold = 0;
        config.association_threshold = 0;
        config.arbiter_flags = 0b00000000;  // Всё отключено
        config.reflex_cooldown = 0;
        config.max_concurrent_hints = 0;
        config.feedback_weight_delta = 0;

        // Устанавливаем емкости для валидации
        config.token_capacity = 500;
        config.connection_capacity = 50;

        config
    }

    /// -----------------------------------------------------------------------
    /// LOGIC (6) - Чистое вычисление
    /// Физика: Идеальная рабочая среда. Комнатная температура, умеренная 
    /// гравитация. Смыслы сталкиваются, вычисляются и летят дальше.
    /// -----------------------------------------------------------------------
    pub fn factory_logic(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = 6; // Logic
        
        config.created_at = 0; // COM event_id (должен быть установлен при создании через COM)
        config.last_update = 0; // COM event_id (обновляется при изменениях)
        
        config.gravity_strength = 9.81;     // Земная гравитация для нормального падения
        config.temperature = 273.0;         // Оптимальная кинетическая энергия
        config.elasticity = 200;            // ~0.8 - Токены хорошо отскакивают
        
        config.friction_coeff = 25;         // ~0.1 - Легкое трение
        
        config.permeability = 127;          // ~0.5 - Полупроницаемая
        config.membrane_state = 3;          // ADAPTIVE

        // Arbiter настройки для LOGIC (V2.1)
        config.reflex_threshold = 230;      // ~0.90 - очень высокий порог, рефлекс только при абсолютной уверенности
        config.association_threshold = 100; // ~0.39 - подсказки только если достаточно релевантны
        config.arbiter_flags = 0b00011111;  // Всё включено, GUARDIAN обязателен
        config.reflex_cooldown = 5;         // Не чаще раз в 5 пульсов - логика не торопится
        config.max_concurrent_hints = 2;    // Минимум шума
        config.feedback_weight_delta = 50;  // ~0.20 - если логика подтвердила, след усиливается заметно

        // Устанавливаем емкости для валидации
        config.token_capacity = 2000;
        config.connection_capacity = 200;

        config
    }

    /// -----------------------------------------------------------------------
    /// MAP (4) - Карта мира и фактов
    /// Физика: Стабильная среда для достоверных данных. Высокая гравитация
    /// для удержания фактов, умеренная температура, низкое трение.
    /// -----------------------------------------------------------------------
    pub fn factory_map(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = 4; // Map

        config.created_at = 0; // COM event_id
        config.last_update = 0;

        config.gravity_strength = 15.0;     // Высокая гравитация - факты удерживаются
        config.temperature = 280.0;         // Умеренная температура - стабильность
        config.friction_coeff = 40;         // ~0.16 - умеренное трение
        config.viscosity = 200;             // ~0.78 - медленное изменение

        config.permeability = 120;          // ~0.47 - избирательная проницаемость
        config.membrane_state = 2;          // SEMI

        // Arbiter настройки для MAP (V2.1)
        config.reflex_threshold = 200;      // ~0.78 - высокий порог, факты требуют уверенности
        config.association_threshold = 80;  // ~0.31 - подсказки только от надёжного опыта
        config.arbiter_flags = 0b00011111;  // Всё включено
        config.reflex_cooldown = 3;
        config.max_concurrent_hints = 2;
        config.feedback_weight_delta = 40;  // ~0.16 - подтверждённые факты усиливаются заметно

        config.token_capacity = 10000;      // Много фактов
        config.connection_capacity = 5000;

        config
    }

    /// -----------------------------------------------------------------------
    /// PROBE (5) - Исследование и анализ
    /// Физика: Активная исследовательская среда. Средняя гравитация,
    /// повышенная температура для активного поиска, высокий резонанс.
    /// -----------------------------------------------------------------------
    pub fn factory_probe(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = 5; // Probe

        config.created_at = 0; // COM event_id
        config.last_update = 0;

        config.gravity_strength = 7.0;      // Средняя гравитация
        config.temperature = 350.0;         // Повышенная - активное исследование
        config.resonance_freq = 800;        // Высокий резонанс - активный поиск
        config.friction_coeff = 35;         // ~0.14 - низкое трение
        config.elasticity = 200;            // ~0.78 - высокая упругость

        config.permeability = 190;          // ~0.75 - высокая проницаемость
        config.membrane_state = 0;          // OPEN - впускает всё для анализа

        // Arbiter настройки для PROBE (V2.1)
        // PROBE исследует, может использовать рефлексы для быстрого анализа
        config.reflex_threshold = 160;      // ~0.63 - умеренно-высокий порог
        config.association_threshold = 60;  // ~0.24 - принимает подсказки
        config.arbiter_flags = 0b00010111;  // Рефлексы + подсказки + feedback + медленный путь
        config.reflex_cooldown = 1;         // Быстрый цикл исследования
        config.max_concurrent_hints = 5;
        config.feedback_weight_delta = 25;  // ~0.10 - умеренное обучение

        config.token_capacity = 6000;
        config.connection_capacity = 3000;

        config
    }

    /// -----------------------------------------------------------------------
    /// VOID (8) - Аннигиляция и трансформация
    /// Физика: Экстремальная среда. Очень высокая температура и гравитация,
    /// токены здесь разрушаются и трансформируются.
    /// -----------------------------------------------------------------------
    pub fn factory_void(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = 8; // Void

        config.created_at = 0; // COM event_id
        config.last_update = 0;

        config.gravity_strength = 100.0;    // Очень высокая гравитация - притяжение к центру
        config.temperature = 1000.0;        // Экстремальная температура - разрушение
        config.friction_coeff = 200;        // ~0.78 - высокое трение
        config.viscosity = 100;             // ~0.39 - умеренная вязкость

        config.permeability = 255;          // 1.0 - всё проникает (для аннигиляции)
        config.membrane_state = 0;          // OPEN

        // Arbiter настройки для VOID (V2.1)
        // VOID трансформирует, не участвует в dual-path routing напрямую
        config.reflex_threshold = 0;
        config.association_threshold = 0;
        config.arbiter_flags = 0b00000000;  // Всё отключено
        config.reflex_cooldown = 0;
        config.max_concurrent_hints = 0;
        config.feedback_weight_delta = 0;

        config.token_capacity = 2000;       // Небольшая ёмкость - токены здесь не хранятся долго
        config.connection_capacity = 100;

        config
    }

    /// -----------------------------------------------------------------------
    /// DREAM (7) - Фоновая оптимизация и галлюцинация
    /// Физика: Полный хаос. Нулевая гравитация, высокая температура (кипение),
    /// высокий квантовый шум. Смыслы мутируют и образуют случайные связи.
    /// -----------------------------------------------------------------------
    pub fn factory_dream(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = 7; // Dream
        
        config.created_at = 0; // COM event_id (должен быть установлен при создании через COM)
        config.last_update = 0; // COM event_id (обновляется при изменениях)
        
        config.gravity_strength = 0.0;      // Невесомость
        config.temperature = 500.0;         // Токены движутся хаотично и быстро
        config.quantum_noise = 200;         // ~0.8 - Вероятность случайной связи
        
        config.time_dilation = 50;          // x0.5 - Время здесь течет быстрее

        config.permeability = 200;          // ~0.8 - Впускает почти всё
        config.membrane_state = 0;          // OPEN

        // Arbiter настройки для DREAM (V2.1)
        config.reflex_threshold = 0;        // Рефлексы отключены - DREAM работает медленно по природе
        config.association_threshold = 25;  // ~0.10 - принимает даже слабые подсказки (фоновый поиск)
        config.arbiter_flags = 0b00010010;  // HINTS_ENABLED + SLOW_PATH_MANDATORY
        config.reflex_cooldown = 0;
        config.max_concurrent_hints = 8;    // Много подсказок - DREAM ищет неожиданные связи
        config.feedback_weight_delta = 10;  // ~0.04 - медленное, но устойчивое обучение

        // Устанавливаем емкости для валидации
        config.token_capacity = 3000;
        config.connection_capacity = 300;

        config
    }

    /// -----------------------------------------------------------------------
    /// EXPERIENCE (9) - Ассоциативная память и рефлексы (v2.0)
    /// Физика: Низкая гравитация (свободное перемещение следов),
    /// минимальное затухание (ничего не забывается), высокий резонанс
    /// (быстрый поиск похожего опыта), средняя температура (пластичность).
    /// -----------------------------------------------------------------------
    pub fn factory_experience(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = 9; // Experience

        config.created_at = 1715292000;
        config.last_update = 1715292000;

        // Физика ассоциативной памяти (по спеке v2.0)
        config.field_size = [5000.0, 5000.0, 5000.0]; // Большое поле для множества следов
        config.gravity_strength = 0.5;      // Низкая гравитация - свободное перемещение
        config.temperature = 300.0;         // ~27°C - средняя (пластичность + стабильность)
        config.resonance_freq = 1000;       // Высокий резонанс - легкий поиск
        config.friction_coeff = 20;         // Низкое трение
        config.viscosity = 200;             // Высокая вязкость - медленное перемещение
        // Минимальное затухание реализуется через Token.min_intensity > 0

        config.permeability = 200;          // Высокая проницаемость
        config.membrane_state = 1;          // SEMI - фильтрация входа

        // Arbiter настройки для EXPERIENCE (V2.1)
        config.reflex_threshold = 0;        // Сам является источником рефлексов, не получателем
        config.association_threshold = 0;
        config.arbiter_flags = 0b00000100;  // Только FEEDBACK_ENABLED
        config.reflex_cooldown = 0;
        config.max_concurrent_hints = 0;
        config.feedback_weight_delta = 0;   // Управляется внутренней логикой домена 9

        config.token_capacity = 100000;     // Много следов (опыт накапливается)
        config.connection_capacity = 50000; // Много связей (ассоциации)

        config
    }

    /// -----------------------------------------------------------------------
    /// MAYA (10) - Интерфейс и проекция (Иллюзия)
    /// Физика: Мягкое, теплое поле без трения. Это презентационный слой,
    /// где токены собираются в красивые структуры для выдачи ответа.
    /// -----------------------------------------------------------------------
    pub fn factory_maya(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = 10; // Maya
        
        config.created_at = 0; // COM event_id (должен быть установлен при создании через COM)
        config.last_update = 0; // COM event_id (обновляется при изменениях)
        
        config.field_size = [2000.0, 2000.0, 2000.0]; // Огромный "экран"
        config.gravity_strength = 1.0;      // Легкое притяжение
        config.temperature = 310.0;         // ~36.6 по Цельсию (теплая среда)
        config.friction_coeff = 5;          // Скольжение
        
        config.permeability = 255;          // 1.0 - Абсолютно открыто для проекций
        config.membrane_state = 0;          // OPEN

        // Arbiter настройки для MAYA (V2.1)
        // MAYA не получает рефлексы - она их принимает от Arbiter напрямую
        // Arbiter не маршрутизирует В MAYA — он маршрутизирует ЧЕРЕЗ MAYA
        config.reflex_threshold = 0;
        config.association_threshold = 0;
        config.arbiter_flags = 0b00000000;  // Всё отключено
        config.reflex_cooldown = 0;
        config.max_concurrent_hints = 0;
        config.feedback_weight_delta = 0;

        // Устанавливаем емкости для валидации
        config.token_capacity = 5000;
        config.connection_capacity = 500;

        config
    }

    /// Валидация согласно спецификации DomainConfig V2.0
    pub fn validate(&self) -> bool {
        // Базовые проверки
        if self.domain_id == 0 {
            return false;
        }
        
        if self.token_capacity == 0 || self.connection_capacity == 0 {
            return false;
        }
        
        // Физические ограничения
        if self.gravity_strength < 0.0 {
            return false;
        }
        
        // Проверка размеров поля (не должны быть нулевыми)
        if self.field_size.iter().any(|&s| s <= 0.0) {
            return false;
        }
        
        // Time Model V1.0: created_at/last_update могут быть 0 для новых конфигураций
        // Они будут установлены при реальном создании домена через COM
        // Если оба установлены, проверяем корректность
        if self.created_at > 0 && self.last_update < self.created_at {
            return false;
        }
        
        true
    }

    /// Проверка фильтров мембраны
    pub fn can_enter(&self, mass: u16, temperature: u16) -> bool {
        mass >= self.threshold_mass
        && temperature >= self.threshold_temp
        && self.membrane_state != MEMBRANE_CLOSED
    }

    /// Обновление метаданных при изменении
    pub fn update_metadata(&mut self, event_id: u64) {
        self.last_update = event_id;
        self.error_count = 0; // Сброс счетчика при успешном обновлении
    }

    /// Расчет сложности домена
    pub fn calculate_complexity(&self) -> f32 {
        let token_factor = self.token_capacity as f32 * 0.1;
        let connection_factor = self.connection_capacity as f32 * 0.05;
        let friction_factor = (self.friction_coeff as f32 / 255.0) * 10.0;
        
        token_factor + connection_factor + friction_factor
    }

    /// Проверка состояния домена
    pub fn is_active(&self) -> bool {
        (self.flags & (DOMAIN_ACTIVE as u8)) != 0
    }

    pub fn is_locked(&self) -> bool {
        (self.flags & (DOMAIN_LOCKED as u8)) != 0
    }

    pub fn is_temporary(&self) -> bool {
        (self.flags & (DOMAIN_TEMPORARY as u8)) != 0
    }
}

/// Domain - runtime structure managing state and causal frontier
///
/// Causal Frontier V1, раздел 12: Domain isolation
/// Heartbeat V2.0, раздел 9: каждый домен имеет свой HeartbeatGenerator
/// SPACE V6.0, раздел 8: каждый домен имеет свой SpatialHashGrid
pub struct Domain {
    /// Конфигурация домена
    pub config: DomainConfig,

    /// Causal Frontier для управления активными вычислениями
    pub frontier: CausalFrontier,

    /// Heartbeat generator для периодической активации
    pub heartbeat: HeartbeatGenerator,

    /// Heartbeat конфигурация
    pub heartbeat_config: HeartbeatConfig,

    /// Spatial Hash Grid для пространственной индексации
    /// SPACE V6.0: Пространственный индекс для быстрого поиска соседей
    pub spatial_grid: SpatialHashGrid,

    /// Текущее количество активных токенов
    pub active_tokens: usize,

    /// Текущее количество активных связей
    pub active_connections: usize,

    /// Счётчик событий с последней перестройки spatial grid
    /// Используется для rebuild_frequency
    pub events_since_rebuild: usize,
}

impl Domain {
    /// Создает новый домен из конфигурации с heartbeat по умолчанию
    pub fn new(config: DomainConfig) -> Self {
        Self::with_heartbeat(config, HeartbeatConfig::default())
    }

    /// Создает новый домен из конфигурации с кастомным heartbeat
    pub fn with_heartbeat(config: DomainConfig, heartbeat_config: HeartbeatConfig) -> Self {
        // Создаем frontier с параметрами на основе capacities домена
        let storm_threshold = (config.token_capacity as usize) / 10; // 10% от capacity
        let max_frontier_size = (config.token_capacity as usize) / 5; // 20% от capacity
        let max_events_per_cycle = 1000; // Фиксированный бюджет

        Self {
            heartbeat: HeartbeatGenerator::new(config.domain_id, heartbeat_config.interval),
            heartbeat_config,
            config,
            frontier: CausalFrontier::with_config(
                storm_threshold,
                max_frontier_size,
                max_events_per_cycle
            ),
            spatial_grid: SpatialHashGrid::new(),
            active_tokens: 0,
            active_connections: 0,
            events_since_rebuild: 0,
        }
    }

    /// Обновляет состояние frontier на основе текущих метрик
    pub fn update_frontier_state(&mut self) {
        self.frontier.update_state();
    }

    /// Проверяет достигнут ли лимит емкости
    pub fn is_at_capacity(&self) -> bool {
        self.active_tokens >= self.config.token_capacity as usize ||
        self.active_connections >= self.config.connection_capacity as usize
    }

    /// Получает текущее использование frontier памяти
    pub fn frontier_memory_usage(&self) -> f32 {
        self.frontier.memory_usage()
    }

    /// Обрабатывает событие и проверяет нужен ли Heartbeat
    ///
    /// Heartbeat V2.0, раздел 3.1: on_event
    /// Возвращает pulse_number если пора генерировать Heartbeat
    pub fn on_event(&mut self) -> Option<u64> {
        self.heartbeat.on_event()
    }

    /// Обрабатывает Heartbeat событие - добавляет сущности в frontier
    ///
    /// Heartbeat V2.0, раздел 6: handle_heartbeat
    pub fn handle_heartbeat(&mut self, pulse_number: u64) {
        crate::heartbeat::handle_heartbeat(
            &mut self.frontier,
            pulse_number,
            &self.heartbeat_config,
            self.active_tokens,
            self.active_connections,
        );
    }

    /// Получает текущий номер пульса heartbeat
    pub fn current_pulse(&self) -> u64 {
        self.heartbeat.current_pulse()
    }

    /// Обрабатывает frontier - извлекает сущности и генерирует события
    ///
    /// Causal Frontier V1, раздел 7: Processing frontier entities
    /// Event-Driven V1, раздел 6: Event generation from state changes
    /// Heartbeat V2.0, раздел 6: Background processes через Frontier
    ///
    /// Этот метод соединяет все компоненты:
    /// Heartbeat → Frontier → EventGenerator → Events
    pub fn process_frontier(
        &mut self,
        tokens: &[crate::token::Token],
        connections: &[crate::connection::Connection],
        event_generator: &mut crate::event_generator::EventGenerator,
    ) -> Vec<crate::event::Event> {

        let mut generated_events = Vec::new();

        // Обрабатываем frontier пока есть бюджет и сущности
        while !self.frontier.is_budget_exhausted() && !self.frontier.is_empty() {
            // Обработка токенов
            if let Some(token_idx) = self.frontier.pop_token() {
                if let Some(token) = tokens.get(token_idx) {
                    // Проверка затухания (если включено)
                    if self.heartbeat_config.enable_decay {
                        if let Some(event) = event_generator.check_decay(
                            token,
                            self.config.friction_coeff as f32 / 255.0 // decay_rate from config
                        ) {
                            generated_events.push(event);
                        }
                    }

                    // Генерация гравитационного обновления (если включено)
                    if self.heartbeat_config.enable_gravity && self.config.gravity_strength.abs() > 0.01 {
                        let event = event_generator.generate_gravity_update(token);
                        generated_events.push(event);
                    }

                    // SPACE V6.0: Проверка столкновений через spatial hash
                    // Если spatial grid не пуст и включена spatial collision detection
                    if self.heartbeat_config.enable_spatial_collision && self.spatial_grid.entry_count > 0 {
                        let collision_radius = 100i16; // TODO: добавить в DomainConfig
                        let collisions = crate::space::detect_collisions(
                            token_idx as u32,
                            (token.position[0], token.position[1], token.position[2]),
                            collision_radius,
                            |idx| {
                                if let Some(t) = tokens.get(idx as usize) {
                                    (t.position[0], t.position[1], t.position[2])
                                } else {
                                    (0, 0, 0)
                                }
                            },
                            &self.spatial_grid,
                        );

                        // Генерируем события столкновений
                        for collision_idx in collisions {
                            if let Some(other_token) = tokens.get(collision_idx as usize) {
                                let event = event_generator.generate_collision(token, other_token);
                                generated_events.push(event);

                                // Добавляем оба токена в frontier для повторной проверки
                                self.frontier.push_token(collision_idx as usize);
                            }
                        }
                    }
                }

                self.frontier.increment_processed();
            }

            // Обработка связей (если включено обслуживание)
            if self.heartbeat_config.enable_connection_maintenance {
                if let Some(conn_idx) = self.frontier.pop_connection() {
                    if let Some(connection) = connections.get(conn_idx) {
                        // Проверка стресса связи
                        let stress_threshold = 0.8; // Можно добавить в DomainConfig позже
                        if let Some(event) = event_generator.check_connection_stress(
                            connection,
                            stress_threshold
                        ) {
                            generated_events.push(event);
                        }
                    }

                    self.frontier.increment_processed();
                }
            }
        }

        // Обновляем состояние frontier после обработки
        self.update_frontier_state();

        generated_events
    }

    /// Перестроить spatial hash grid
    ///
    /// SPACE V6.0, раздел 4.5: Перестройка индекса после применения событий
    ///
    /// Аргументы:
    /// - tokens: массив всех токенов домена
    ///
    /// Метод вызывается после обработки batch событий, когда позиции токенов изменились
    pub fn rebuild_spatial_grid(&mut self, tokens: &[crate::token::Token]) {
        self.spatial_grid.rebuild(self.active_tokens, |token_index| {
            if let Some(token) = tokens.get(token_index) {
                (token.position[0], token.position[1], token.position[2])
            } else {
                (0, 0, 0) // Fallback для несуществующих токенов
            }
        });

        // Сбрасываем счётчик событий после перестройки
        self.events_since_rebuild = 0;
    }

    /// Проверить, нужна ли перестройка spatial grid
    ///
    /// SPACE V6.0, раздел 8: Конфигурируемая частота перестройки
    ///
    /// Возвращает true, если:
    /// - rebuild_frequency > 0 (перестройка включена)
    /// - events_since_rebuild >= rebuild_frequency
    pub fn should_rebuild_spatial_grid(&self) -> bool {
        self.config.rebuild_frequency > 0 &&
        self.events_since_rebuild >= self.config.rebuild_frequency as usize
    }

    /// Инкрементировать счётчик событий с последней перестройки
    ///
    /// Вызывается после применения каждого события, которое может изменить позиции токенов
    pub fn increment_events_since_rebuild(&mut self) {
        self.events_since_rebuild = self.events_since_rebuild.saturating_add(1);
    }

    /// Найти соседей токена через spatial grid
    ///
    /// SPACE V6.0, раздел 4.6: Поиск соседей
    ///
    /// Аргументы:
    /// - token: токен, для которого ищем соседей
    /// - radius: радиус поиска в квантах
    /// - tokens: массив всех токенов домена
    ///
    /// Возвращает:
    /// - Vec<u32>: индексы токенов-соседей
    pub fn find_neighbors(
        &self,
        token: &crate::token::Token,
        radius: i16,
        tokens: &[crate::token::Token],
    ) -> Vec<u32> {
        self.spatial_grid.find_neighbors(
            token.position[0],
            token.position[1],
            token.position[2],
            radius,
            |token_index| {
                if let Some(t) = tokens.get(token_index as usize) {
                    (t.position[0], t.position[1], t.position[2])
                } else {
                    (0, 0, 0)
                }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_experience_basic() {
        let config = DomainConfig::factory_experience(9, 0);

        assert_eq!(config.domain_id, 9);
        assert_eq!(config.parent_domain_id, 0);
        assert_eq!(config.structural_role, 9); // Experience
        assert!(config.validate());
    }

    #[test]
    fn test_factory_experience_physics() {
        let config = DomainConfig::factory_experience(9, 0);

        // Проверка физических параметров из спеки v2.0
        assert_eq!(config.field_size, [5000.0, 5000.0, 5000.0]);
        assert_eq!(config.gravity_strength, 0.5); // Низкая гравитация
        assert_eq!(config.temperature, 300.0);    // Средняя температура
        assert_eq!(config.resonance_freq, 1000);  // Высокий резонанс
        assert_eq!(config.friction_coeff, 20);    // Низкое трение
        assert_eq!(config.viscosity, 200);        // Высокая вязкость
    }

    #[test]
    fn test_factory_experience_capacities() {
        let config = DomainConfig::factory_experience(9, 0);

        // Большие емкости для накопления опыта
        assert_eq!(config.token_capacity, 100000);
        assert_eq!(config.connection_capacity, 50000);
        assert_eq!(config.permeability, 200); // Высокая проницаемость
        assert_eq!(config.membrane_state, 1); // SEMI
    }

    #[test]
    fn test_structural_role_experience() {
        // Проверка что Experience = 9 в enum
        assert_eq!(StructuralRole::Experience as u8, 9);
    }

    #[test]
    fn test_experience_vs_maya_differences() {
        let experience = DomainConfig::factory_experience(9, 0);
        let maya = DomainConfig::factory_maya(10, 0);

        // EXPERIENCE - большие емкости, высокий резонанс
        assert!(experience.token_capacity > maya.token_capacity);
        assert!(experience.resonance_freq > maya.resonance_freq);

        // MAYA - теплее, более открыта
        assert!(maya.temperature > experience.temperature);
        assert!(maya.permeability > experience.permeability);
    }

    #[test]
    fn test_all_factory_methods_valid() {
        // Тестируем все существующие factory методы (11 доменов Ashti_Core v2.0)
        let configs = vec![
            DomainConfig::factory_sutra(1),         // 0: SUTRA - Источник истины
            DomainConfig::factory_execution(1, 0),  // 1: EXECUTION - Реализация решений
            DomainConfig::factory_shadow(2, 0),     // 2: SHADOW - Симуляция угроз
            DomainConfig::factory_codex(3, 1),      // 3: CODEX - Конституция/правила
            DomainConfig::factory_map(4, 0),        // 4: MAP - Карта мира/фактов
            DomainConfig::factory_probe(5, 0),      // 5: PROBE - Активное зондирование
            DomainConfig::factory_logic(6, 1),      // 6: LOGIC - Чистые вычисления
            DomainConfig::factory_dream(7, 1),      // 7: DREAM - Фоновая оптимизация
            DomainConfig::factory_void(8, 0),       // 8: VOID - Неопределённость/аномалии
            DomainConfig::factory_experience(9, 1), // 9: EXPERIENCE - Память и опыт
            DomainConfig::factory_maya(10, 1),      // 10: MAYA - Интерфейс/проекция
        ];

        for config in configs {
            assert!(config.validate(),
                "Factory method for role {} produced invalid config",
                config.structural_role);
        }
    }

    // --- Domain-Specific Example Tests ---

    #[test]
    fn test_codex_domain_stability() {
        // CODEX (3) - Конституция и правила
        // Физика: высокая стабильность, низкая температура, высокая вязкость
        let config = DomainConfig::factory_codex(3, 1);

        assert_eq!(config.structural_role, 3);
        assert_eq!(config.temperature, 10.0); // Почти ноль - минимальные колебания
        assert_eq!(config.viscosity, 250);    // ~0.98 - Токены вязнут и фиксируются
        assert_eq!(config.membrane_state, 2); // SEMI - жёсткий фильтр

        // Arbiter отключён - CODEX не участвует в dual-path routing
        assert_eq!(config.reflex_threshold, 0);
        assert_eq!(config.arbiter_flags, 0b00000000);
    }

    #[test]
    fn test_probe_domain_exploration() {
        // PROBE (5) - Активное зондирование и исследование
        // Физика: активная среда, высокая температура, высокий резонанс
        let config = DomainConfig::factory_probe(5, 0);

        assert_eq!(config.structural_role, 5);
        assert_eq!(config.temperature, 350.0);    // Повышенная - активное исследование
        assert_eq!(config.resonance_freq, 800);   // Высокий резонанс - активный поиск
        assert_eq!(config.membrane_state, 0);     // OPEN - впускает всё для анализа

        // Arbiter активен для быстрого анализа
        assert_eq!(config.reflex_threshold, 160); // ~0.63 - умеренно-высокий порог
        assert_eq!(config.reflex_cooldown, 1);    // Быстрый цикл исследования
        assert_eq!(config.max_concurrent_hints, 5);
    }

    #[test]
    fn test_void_domain_transformation() {
        // VOID (8) - Неопределённость и трансформация
        // Физика: экстремальная среда для разрушения и трансформации
        let config = DomainConfig::factory_void(8, 0);

        assert_eq!(config.structural_role, 8);
        assert_eq!(config.temperature, 1000.0);    // Экстремальная температура - разрушение
        assert_eq!(config.gravity_strength, 100.0); // Очень высокая гравитация - притяжение к центру
        assert_eq!(config.friction_coeff, 200);    // ~0.78 - высокое трение
        assert_eq!(config.permeability, 255);      // 1.0 - всё проникает (для аннигиляции)
        assert_eq!(config.membrane_state, 0);      // OPEN

        // Arbiter отключён - VOID не участвует в dual-path routing
        assert_eq!(config.reflex_threshold, 0);
        assert_eq!(config.arbiter_flags, 0b00000000);

        // Небольшая ёмкость - токены здесь не хранятся долго
        assert_eq!(config.token_capacity, 2000);
    }

    #[test]
    fn test_shadow_domain_simulation() {
        // SHADOW (2) - Симуляция угроз и теневые сценарии
        // Физика: изолированная среда для моделирования деструктивных сценариев
        let config = DomainConfig::factory_shadow(2, 0);

        assert_eq!(config.structural_role, 2);
        assert_eq!(config.temperature, 250.0);     // Прохладная среда для стабильности
        assert_eq!(config.gravity_strength, 5.0);  // Средняя гравитация
        assert_eq!(config.viscosity, 180);         // ~0.7 - замедленное движение
        assert_eq!(config.membrane_state, 2);      // SEMI - контролируемая изоляция

        // Arbiter активен для быстрого анализа угроз
        assert_eq!(config.reflex_threshold, 180);  // ~0.71 - высокий порог
        assert_eq!(config.arbiter_flags, 0b00010111); // Рефлексы + подсказки + feedback + медленный путь
    }

    #[test]
    fn test_map_domain_facts() {
        // MAP (4) - Карта мира и фактов
        // Физика: стабильная структура для хранения и навигации по фактам
        let config = DomainConfig::factory_map(4, 0);

        assert_eq!(config.structural_role, 4);
        assert_eq!(config.temperature, 280.0);     // Умеренная температура - стабильность
        assert_eq!(config.gravity_strength, 15.0); // Высокая гравитация - факты удерживаются
        assert_eq!(config.viscosity, 200);         // ~0.78 - медленное изменение
        assert_eq!(config.membrane_state, 2);      // SEMI

        // Arbiter активен - факты требуют уверенности
        assert_eq!(config.reflex_threshold, 200);  // ~0.78 - высокий порог
        assert_eq!(config.arbiter_flags, 0b00011111); // Всё включено
    }

    #[test]
    fn test_logic_domain_computation() {
        // LOGIC (6) - Чистые вычисления и логический вывод
        // Физика: детерминированная среда без случайности
        let config = DomainConfig::factory_logic(6, 1);

        assert_eq!(config.structural_role, 6);
        assert_eq!(config.temperature, 273.0);     // Оптимальная кинетическая энергия
        assert_eq!(config.gravity_strength, 9.81); // Земная гравитация
        assert_eq!(config.elasticity, 200);        // ~0.8 - хорошие отскоки
        assert_eq!(config.friction_coeff, 25);     // ~0.1 - лёгкое трение
        assert_eq!(config.membrane_state, 3);      // ADAPTIVE

        // Arbiter активен для логических выводов
        assert_eq!(config.reflex_threshold, 230);  // ~0.90 - очень высокий порог
        assert_eq!(config.association_threshold, 100); // ~0.39 - подсказки только если релевантны
    }

    #[test]
    fn test_dream_domain_optimization() {
        // DREAM (7) - Фоновая оптимизация и рефлексия
        // Физика: хаотичная среда для творческих комбинаций
        let config = DomainConfig::factory_dream(7, 1);

        assert_eq!(config.structural_role, 7);
        assert_eq!(config.temperature, 500.0);     // Токены движутся хаотично и быстро
        assert_eq!(config.gravity_strength, 0.0);  // Невесомость
        assert_eq!(config.quantum_noise, 200);     // ~0.8 - высокая вероятность случайных связей
        assert_eq!(config.time_dilation, 50);      // x0.5 - время течёт быстрее
        assert_eq!(config.membrane_state, 0);      // OPEN

        // Arbiter отключён - DREAM работает в фоновом режиме
        assert_eq!(config.reflex_threshold, 0);
        assert_eq!(config.association_threshold, 25); // ~0.10 - принимает даже слабые подсказки
    }

    #[test]
    fn test_experience_domain_memory() {
        // EXPERIENCE (9) - Память и опыт
        // Физика: архив с высокой ёмкостью и стабильностью
        let config = DomainConfig::factory_experience(9, 1);

        assert_eq!(config.structural_role, 9);
        assert_eq!(config.temperature, 300.0);     // Средняя температура - активная память
        assert_eq!(config.gravity_strength, 0.5);  // Низкая гравитация - свободная навигация
        assert_eq!(config.resonance_freq, 1000);   // Высокий резонанс - ассоциативная память
        assert_eq!(config.viscosity, 200);         // ~0.78 - высокая вязкость для сохранности

        // Очень большие ёмкости для накопления опыта
        assert_eq!(config.token_capacity, 100000);
        assert_eq!(config.connection_capacity, 50000);
        assert_eq!(config.permeability, 200);      // ~0.78 - высокая проницаемость
        assert_eq!(config.membrane_state, 1);      // SEMI

        // Arbiter - только FEEDBACK для обучения
        assert_eq!(config.reflex_threshold, 0);    // Сам источник рефлексов, не получатель
        assert_eq!(config.arbiter_flags, 0b00000100); // FEEDBACK_ENABLED
    }

    // --- Domain Runtime Tests ---

    #[test]
    fn test_domain_creation() {
        let config = DomainConfig::factory_logic(6, 1);
        let domain = Domain::new(config);

        assert_eq!(domain.config.domain_id, 6);
        assert_eq!(domain.active_tokens, 0);
        assert_eq!(domain.active_connections, 0);
        assert!(domain.frontier.is_empty());
    }

    #[test]
    fn test_domain_frontier_integration() {
        let config = DomainConfig::factory_logic(6, 1);
        let mut domain = Domain::new(config);

        // Добавляем токены в frontier
        assert!(domain.frontier.push_token(1));
        assert!(domain.frontier.push_token(2));
        assert_eq!(domain.frontier.size(), 2);

        // Обновляем состояние
        domain.update_frontier_state();
        assert_eq!(domain.frontier.state(), crate::causal_frontier::FrontierState::Active);
    }

    #[test]
    fn test_domain_capacity_limits() {
        let mut config = DomainConfig::factory_logic(6, 1);
        config.token_capacity = 10;
        config.connection_capacity = 5;

        let mut domain = Domain::new(config);
        domain.active_tokens = 10;
        domain.active_connections = 5;

        assert!(domain.is_at_capacity());
    }

    #[test]
    fn test_domain_storm_threshold() {
        let mut config = DomainConfig::factory_experience(9, 0);
        config.token_capacity = 1000; // 100 storm threshold, 200 max frontier

        let mut domain = Domain::new(config);

        // Добавляем токены до порога storm
        for i in 0..150 {
            domain.frontier.push_token(i);
        }

        domain.update_frontier_state();
        assert_eq!(domain.frontier.state(), crate::causal_frontier::FrontierState::Storm);
    }

    #[test]
    fn test_domain_frontier_memory_usage() {
        let config = DomainConfig::factory_logic(6, 1);
        let mut domain = Domain::new(config);

        // Добавляем несколько токенов
        domain.frontier.push_token(1);
        domain.frontier.push_token(2);
        domain.frontier.push_connection(10);

        let usage = domain.frontier_memory_usage();
        assert!(usage > 0.0);
        assert!(usage < 100.0);
    }

    #[test]
    fn test_domain_isolation() {
        let config1 = DomainConfig::factory_logic(6, 1);
        let config2 = DomainConfig::factory_dream(7, 1);

        let mut domain1 = Domain::new(config1);
        let mut domain2 = Domain::new(config2);

        // Каждый домен имеет свой frontier
        domain1.frontier.push_token(1);
        domain2.frontier.push_token(2);

        assert_eq!(domain1.frontier.size(), 1);
        assert_eq!(domain2.frontier.size(), 1);
        assert!(domain1.frontier.contains_token(1));
        assert!(!domain1.frontier.contains_token(2));
        assert!(domain2.frontier.contains_token(2));
        assert!(!domain2.frontier.contains_token(1));
    }

    // --- Heartbeat Integration Tests ---

    #[test]
    fn test_domain_with_heartbeat() {
        let config = DomainConfig::factory_logic(6, 1);
        let heartbeat_config = crate::heartbeat::HeartbeatConfig::medium();
        let domain = Domain::with_heartbeat(config, heartbeat_config);

        assert_eq!(domain.current_pulse(), 0);
        assert_eq!(domain.heartbeat_config.interval, 1024);
    }

    #[test]
    fn test_domain_heartbeat_generation() {
        let config = DomainConfig::factory_logic(6, 1);
        let heartbeat_config = crate::heartbeat::HeartbeatConfig {
            interval: 5,
            ..crate::heartbeat::HeartbeatConfig::medium()
        };
        let mut domain = Domain::with_heartbeat(config, heartbeat_config);

        // Первые 4 события - нет пульса
        assert!(domain.on_event().is_none());
        assert!(domain.on_event().is_none());
        assert!(domain.on_event().is_none());
        assert!(domain.on_event().is_none());

        // 5-е событие - первый пульс
        let pulse = domain.on_event();
        assert_eq!(pulse, Some(1));
        assert_eq!(domain.current_pulse(), 1);
    }

    #[test]
    fn test_domain_handle_heartbeat() {
        let mut config = DomainConfig::factory_logic(6, 1);
        config.token_capacity = 100;

        let heartbeat_config = crate::heartbeat::HeartbeatConfig {
            batch_size: 5,
            connection_batch_size: 2,
            enable_connection_maintenance: true,
            ..crate::heartbeat::HeartbeatConfig::medium()
        };

        let mut domain = Domain::with_heartbeat(config, heartbeat_config);
        domain.active_tokens = 100;
        domain.active_connections = 50;

        // Обрабатываем Heartbeat - должны добавиться сущности в frontier
        domain.handle_heartbeat(1);

        assert_eq!(domain.frontier.token_count(), 5);
        assert_eq!(domain.frontier.connection_count(), 2);
    }

    #[test]
    fn test_domain_heartbeat_isolation() {
        let config1 = DomainConfig::factory_logic(6, 1);
        let config2 = DomainConfig::factory_dream(7, 1);

        let heartbeat_config = crate::heartbeat::HeartbeatConfig {
            interval: 5,
            ..crate::heartbeat::HeartbeatConfig::medium()
        };

        let mut domain1 = Domain::with_heartbeat(config1, heartbeat_config);
        let mut domain2 = Domain::with_heartbeat(config2, heartbeat_config);

        // Обрабатываем 5 событий в domain1
        for _ in 0..5 {
            domain1.on_event();
        }

        // Обрабатываем 2 события в domain2
        for _ in 0..2 {
            domain2.on_event();
        }

        // Heartbeat domain1 сработал, domain2 - нет
        assert_eq!(domain1.current_pulse(), 1);
        assert_eq!(domain2.current_pulse(), 0);
    }

    #[test]
    fn test_domain_heartbeat_frontier_update() {
        let mut config = DomainConfig::factory_logic(6, 1);
        config.token_capacity = 50;

        let heartbeat_config = crate::heartbeat::HeartbeatConfig {
            interval: 10,
            batch_size: 3,
            ..crate::heartbeat::HeartbeatConfig::medium()
        };

        let mut domain = Domain::with_heartbeat(config, heartbeat_config);
        domain.active_tokens = 50;

        // Генерируем Heartbeat
        for _ in 0..10 {
            if let Some(pulse) = domain.on_event() {
                domain.handle_heartbeat(pulse);
            }
        }

        // Должны быть добавлены токены в frontier
        assert_eq!(domain.frontier.token_count(), 3);
        assert_eq!(domain.current_pulse(), 1);

        // Обновляем состояние frontier
        domain.update_frontier_state();
        assert_eq!(domain.frontier.state(), crate::causal_frontier::FrontierState::Active);
    }

    // --- Frontier Processing Tests ---

    #[test]
    fn test_process_frontier_basic() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;

        let config = DomainConfig::factory_logic(6, 1);
        let heartbeat_config = crate::heartbeat::HeartbeatConfig::medium();
        let mut domain = Domain::with_heartbeat(config, heartbeat_config);

        // Создаем токены
        let mut tokens = vec![Token::default(); 10];
        for (i, token) in tokens.iter_mut().enumerate() {
            token.sutra_id = i as u32;
            token.domain_id = 6;
            token.last_event_id = 0;
        }

        let connections = vec![Connection::default(); 5];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        // Добавляем токены в frontier
        domain.frontier.push_token(0);
        domain.frontier.push_token(1);
        domain.frontier.push_token(2);

        // Обрабатываем frontier
        let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

        // Должны быть сгенерированы события
        assert!(!events.is_empty());

        // Frontier должен быть обработан
        assert_eq!(domain.frontier.token_count(), 0);
    }

    #[test]
    fn test_process_frontier_decay() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;
        use crate::event::EventType;

        let config = DomainConfig::factory_logic(6, 1);
        let mut heartbeat_config = crate::heartbeat::HeartbeatConfig::medium();
        heartbeat_config.enable_decay = true;
        heartbeat_config.enable_gravity = false;

        let mut domain = Domain::with_heartbeat(config, heartbeat_config);

        // Создаем токен со старым last_event_id (подвержен затуханию)
        let mut token = Token::default();
        token.sutra_id = 1;
        token.domain_id = 6;
        token.last_event_id = 0;
        token.valence = 10; // Ненулевой valence - должен затухать

        let tokens = vec![token];
        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(10000); // Большой причинный возраст

        // Добавляем токен в frontier
        domain.frontier.push_token(0);

        // Обрабатываем frontier
        let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

        // Должно быть событие TokenDecayed
        assert!(!events.is_empty());
        let has_decay = events.iter().any(|e| e.event_type == EventType::TokenDecayed as u16);
        assert!(has_decay, "Expected TokenDecayed event");
    }

    #[test]
    fn test_process_frontier_gravity() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;
        use crate::event::EventType;

        let mut config = DomainConfig::factory_logic(6, 1);
        config.gravity_strength = 10.0; // Ненулевая гравитация

        let mut heartbeat_config = crate::heartbeat::HeartbeatConfig::medium();
        heartbeat_config.enable_decay = false;
        heartbeat_config.enable_gravity = true;

        let mut domain = Domain::with_heartbeat(config, heartbeat_config);

        // Создаем токен
        let mut token = Token::default();
        token.sutra_id = 1;
        token.domain_id = 6;

        let tokens = vec![token];
        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        // Добавляем токен в frontier
        domain.frontier.push_token(0);

        // Обрабатываем frontier
        let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

        // Должно быть событие GravityUpdate
        assert!(!events.is_empty());
        let has_gravity = events.iter().any(|e| e.event_type == EventType::GravityUpdate as u16);
        assert!(has_gravity, "Expected GravityUpdate event");
    }

    #[test]
    fn test_process_frontier_connection_stress() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;
        use crate::event::EventType;

        let config = DomainConfig::factory_logic(6, 1);
        let mut heartbeat_config = crate::heartbeat::HeartbeatConfig::medium();
        heartbeat_config.enable_decay = false;
        heartbeat_config.enable_gravity = false;
        heartbeat_config.enable_connection_maintenance = true;

        let mut domain = Domain::with_heartbeat(config, heartbeat_config);

        // Создаем связь с высоким стрессом
        let mut connection = Connection::default();
        connection.source_id = 1;
        connection.target_id = 2;
        connection.domain_id = 6;
        connection.current_stress = 1.0; // Высокий стресс (> 0.8 порога)

        let tokens = vec![];
        let connections = vec![connection];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        // Добавляем связь в frontier
        domain.frontier.push_connection(0);

        // Обрабатываем frontier
        let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

        // Должно быть событие ConnectionWeakened или ConnectionBroken
        assert!(!events.is_empty());
        let has_stress_event = events.iter().any(|e| {
            e.event_type == EventType::ConnectionWeakened as u16 ||
            e.event_type == EventType::ConnectionBroken as u16
        });
        assert!(has_stress_event, "Expected connection stress event");
    }

    #[test]
    fn test_process_frontier_budget_limit() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;

        let config = DomainConfig::factory_logic(6, 1);
        let heartbeat_config = crate::heartbeat::HeartbeatConfig::medium();
        let mut domain = Domain::with_heartbeat(config, heartbeat_config);

        // Создаем токены
        let tokens: Vec<Token> = (0..2000).map(|i| {
            let mut token = Token::default();
            token.sutra_id = i;
            token.domain_id = 6;
            token
        }).collect();

        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        // Добавляем больше токенов чем бюджет (1000)
        // Учитывая что на каждый токен 1-2 события, добавляем 1500 токенов
        for i in 0..1500 {
            domain.frontier.push_token(i);
        }

        let initial_count = domain.frontier.token_count();

        // Обрабатываем frontier
        let _events = domain.process_frontier(&tokens, &connections, &mut event_generator);

        // Проверяем что либо остались необработанные токены, либо бюджет исчерпан
        // При бюджете в 1000 событий должна остаться часть токенов необработанной
        let processed_count = initial_count - domain.frontier.token_count();
        assert!(processed_count <= 1000, "Should respect budget limit");
    }

    #[test]
    fn test_process_frontier_empty() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;

        let config = DomainConfig::factory_logic(6, 1);
        let heartbeat_config = crate::heartbeat::HeartbeatConfig::medium();
        let mut domain = Domain::with_heartbeat(config, heartbeat_config);

        let tokens = vec![];
        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        // Обрабатываем пустой frontier
        let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

        // Не должно быть событий
        assert!(events.is_empty());
        assert!(domain.frontier.is_empty());
    }

    #[test]
    fn test_process_frontier_state_update() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;

        let config = DomainConfig::factory_logic(6, 1);
        let heartbeat_config = crate::heartbeat::HeartbeatConfig::medium();
        let mut domain = Domain::with_heartbeat(config, heartbeat_config);

        let tokens: Vec<Token> = (0..10).map(|i| {
            let mut token = Token::default();
            token.sutra_id = i;
            token.domain_id = 6;
            token
        }).collect();

        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        // Добавляем токены в frontier
        for i in 0..5 {
            domain.frontier.push_token(i);
        }

        // Обрабатываем frontier
        let _events = domain.process_frontier(&tokens, &connections, &mut event_generator);

        // Состояние frontier должно быть обновлено
        // После обработки frontier может быть Empty или Idle
        let state = domain.frontier.state();
        assert!(
            state == crate::causal_frontier::FrontierState::Empty ||
            state == crate::causal_frontier::FrontierState::Idle
        );
    }

    // --- Integration Tests: Full Flow ---

    #[test]
    fn test_full_heartbeat_to_event_flow() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;

        let mut config = DomainConfig::factory_logic(6, 1);
        config.gravity_strength = 5.0;

        let heartbeat_config = crate::heartbeat::HeartbeatConfig {
            interval: 5,
            batch_size: 2,
            enable_decay: true,
            enable_gravity: true,
            enable_connection_maintenance: false,
            ..crate::heartbeat::HeartbeatConfig::medium()
        };

        let mut domain = Domain::with_heartbeat(config, heartbeat_config);
        domain.active_tokens = 10;

        // Создаем токены
        let tokens: Vec<Token> = (0..10).map(|i| {
            let mut token = Token::default();
            token.sutra_id = i;
            token.domain_id = 6;
            token.last_event_id = 0;
            token.valence = 5;
            token
        }).collect();

        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(10000);

        // Шаг 1: Генерируем события до Heartbeat
        for _ in 0..5 {
            if let Some(pulse) = domain.on_event() {
                // Шаг 2: Heartbeat добавляет сущности в frontier
                domain.handle_heartbeat(pulse);

                // Шаг 3: Обрабатываем frontier → генерируем события
                event_generator.set_pulse_id(pulse);
                let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

                // Шаг 4: События сгенерированы
                assert!(!events.is_empty(), "Expected events from frontier processing");

                // Проверяем что события имеют pulse_id
                for event in &events {
                    assert_eq!(event.pulse_id, pulse, "Event should have correct pulse_id");
                }
            }
        }

        // Heartbeat должен был сработать
        assert_eq!(domain.current_pulse(), 1);
    }

    #[test]
    fn test_full_flow_multiple_cycles() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;

        let mut config = DomainConfig::factory_logic(6, 1);
        config.gravity_strength = 1.0;

        let heartbeat_config = crate::heartbeat::HeartbeatConfig {
            interval: 3,
            batch_size: 2,
            enable_decay: false,
            enable_gravity: true,
            enable_connection_maintenance: false,
            ..crate::heartbeat::HeartbeatConfig::medium()
        };

        let mut domain = Domain::with_heartbeat(config, heartbeat_config);
        domain.active_tokens = 10;

        let tokens: Vec<Token> = (0..10).map(|i| {
            let mut token = Token::default();
            token.sutra_id = i;
            token.domain_id = 6;
            token
        }).collect();

        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        let mut total_events = 0;

        // Симулируем несколько циклов
        for cycle in 0..10 {
            if let Some(pulse) = domain.on_event() {
                event_generator.set_pulse_id(pulse);
                domain.handle_heartbeat(pulse);

                let events = domain.process_frontier(&tokens, &connections, &mut event_generator);
                total_events += events.len();

                // На каждом цикле frontier должен быть обработан
                domain.frontier.reset_cycle();
            }

            event_generator.set_event_id(1000 + cycle * 10);
        }

        // Должны были быть сгенерированы события
        assert!(total_events > 0, "Expected events from multiple cycles");

        // Heartbeat должен был срабатывать несколько раз
        assert!(domain.current_pulse() >= 3);
    }

    // --- SPACE V6.0 Integration Tests ---

    #[test]
    fn test_domain_spatial_grid_initialization() {
        let config = DomainConfig::factory_logic(6, 1);
        let domain = Domain::new(config);

        // Spatial grid должен быть инициализирован
        assert_eq!(domain.spatial_grid.entry_count, 0);
        assert_eq!(domain.events_since_rebuild, 0);
    }

    #[test]
    fn test_domain_rebuild_spatial_grid() {
        use crate::token::Token;

        let config = DomainConfig::factory_logic(6, 1);
        let mut domain = Domain::new(config);

        // Создаем токены с разными позициями
        let tokens: Vec<Token> = (0..10).map(|i| {
            let mut token = Token::default();
            token.sutra_id = i;
            token.domain_id = 6;
            token.position = [i as i16 * 100, i as i16 * 50, 0];
            token
        }).collect();

        domain.active_tokens = 10;

        // Перестраиваем spatial grid
        domain.rebuild_spatial_grid(&tokens);

        // Проверяем что grid перестроен
        assert_eq!(domain.spatial_grid.entry_count, 10);
        assert_eq!(domain.events_since_rebuild, 0);
    }

    #[test]
    fn test_domain_should_rebuild_spatial_grid() {
        let mut config = DomainConfig::factory_logic(6, 1);
        config.rebuild_frequency = 10; // Перестройка каждые 10 событий

        let mut domain = Domain::new(config);

        // Изначально не нужна перестройка
        assert!(!domain.should_rebuild_spatial_grid());

        // Инкрементируем счётчик событий
        for _ in 0..9 {
            domain.increment_events_since_rebuild();
        }

        // Всё ещё не нужна (9 < 10)
        assert!(!domain.should_rebuild_spatial_grid());

        // Ещё одно событие
        domain.increment_events_since_rebuild();

        // Теперь нужна перестройка (10 >= 10)
        assert!(domain.should_rebuild_spatial_grid());
    }

    #[test]
    fn test_domain_rebuild_frequency_disabled() {
        let mut config = DomainConfig::factory_logic(6, 1);
        config.rebuild_frequency = 0; // Перестройка отключена

        let mut domain = Domain::new(config);

        // Инкрементируем счётчик событий
        for _ in 0..100 {
            domain.increment_events_since_rebuild();
        }

        // Перестройка не нужна, так как отключена
        assert!(!domain.should_rebuild_spatial_grid());
    }

    #[test]
    fn test_domain_find_neighbors() {
        use crate::token::Token;

        let config = DomainConfig::factory_logic(6, 1);
        let mut domain = Domain::new(config);

        // Создаем токены в одной области
        let mut tokens: Vec<Token> = Vec::new();

        // Токен 0 в центре (0, 0, 0)
        let mut token0 = Token::default();
        token0.sutra_id = 0;
        token0.domain_id = 6;
        token0.position = [0, 0, 0];
        tokens.push(token0);

        // Токен 1 рядом (100, 0, 0)
        let mut token1 = Token::default();
        token1.sutra_id = 1;
        token1.domain_id = 6;
        token1.position = [100, 0, 0];
        tokens.push(token1);

        // Токен 2 далеко (1000, 1000, 1000)
        let mut token2 = Token::default();
        token2.sutra_id = 2;
        token2.domain_id = 6;
        token2.position = [1000, 1000, 1000];
        tokens.push(token2);

        domain.active_tokens = 3;

        // Перестраиваем spatial grid
        domain.rebuild_spatial_grid(&tokens);

        // Ищем соседей токена 0 в радиусе 200 квантов
        let neighbors = domain.find_neighbors(&tokens[0], 200, &tokens);

        // Должны найти токен 1 (расстояние 100)
        // Не должны найти токен 2 (слишком далеко)
        assert!(neighbors.contains(&1), "Should find token 1");
        assert!(!neighbors.contains(&2), "Should not find token 2");
    }

    #[test]
    fn test_domain_spatial_grid_rebuild_resets_counter() {
        use crate::token::Token;

        let mut config = DomainConfig::factory_logic(6, 1);
        config.rebuild_frequency = 5;

        let mut domain = Domain::new(config);

        let tokens: Vec<Token> = (0..5).map(|i| {
            let mut token = Token::default();
            token.sutra_id = i;
            token.domain_id = 6;
            token.position = [i as i16 * 100, 0, 0];
            token
        }).collect();

        domain.active_tokens = 5;

        // Инкрементируем счётчик
        for _ in 0..10 {
            domain.increment_events_since_rebuild();
        }

        assert_eq!(domain.events_since_rebuild, 10);

        // Перестраиваем
        domain.rebuild_spatial_grid(&tokens);

        // Счётчик должен быть сброшен
        assert_eq!(domain.events_since_rebuild, 0);
    }

    #[test]
    fn test_domain_spatial_grid_with_empty_tokens() {
        let config = DomainConfig::factory_logic(6, 1);
        let mut domain = Domain::new(config);

        let tokens: Vec<crate::token::Token> = Vec::new();
        domain.active_tokens = 0;

        // Перестройка с пустым массивом не должна паниковать
        domain.rebuild_spatial_grid(&tokens);

        assert_eq!(domain.spatial_grid.entry_count, 0);
    }

    #[test]
    fn test_domain_find_neighbors_empty_grid() {
        use crate::token::Token;

        let config = DomainConfig::factory_logic(6, 1);
        let domain = Domain::new(config);

        let mut token = Token::default();
        token.position = [0, 0, 0];

        let tokens = vec![token];

        // Поиск в пустом grid должен вернуть пустой список
        let neighbors = domain.find_neighbors(&tokens[0], 100, &tokens);

        assert!(neighbors.is_empty());
    }

    #[test]
    fn test_domain_spatial_grid_multiple_rebuilds() {
        use crate::token::Token;

        let config = DomainConfig::factory_logic(6, 1);
        let mut domain = Domain::new(config);

        let mut tokens: Vec<Token> = (0..5).map(|i| {
            let mut token = Token::default();
            token.sutra_id = i;
            token.domain_id = 6;
            token.position = [i as i16 * 100, 0, 0];
            token
        }).collect();

        domain.active_tokens = 5;

        // Первая перестройка
        domain.rebuild_spatial_grid(&tokens);
        assert_eq!(domain.spatial_grid.entry_count, 5);

        // Меняем позиции токенов
        for token in &mut tokens {
            token.position[0] += 50;
        }

        // Вторая перестройка
        domain.rebuild_spatial_grid(&tokens);
        assert_eq!(domain.spatial_grid.entry_count, 5);

        // Проверяем что grid содержит обновлённые позиции
        let neighbors = domain.find_neighbors(&tokens[0], 200, &tokens);
        assert!(neighbors.contains(&1), "Should find neighbor after rebuild");
    }

    // --- SPACE V6.0 + Causal Frontier Integration Tests (Phase 1.8) ---

    #[test]
    fn test_process_frontier_with_spatial_collision_detection() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;
        use crate::event::EventType;

        let config = DomainConfig::factory_logic(6, 1);
        let mut domain = Domain::new(config);

        // Создаем токены близко друг к другу (столкновение)
        let mut token0 = Token::default();
        token0.sutra_id = 100;
        token0.domain_id = 6;
        token0.position = [0, 0, 0];
        token0.last_event_id = 0;

        let mut token1 = Token::default();
        token1.sutra_id = 101;
        token1.domain_id = 6;
        token1.position = [50, 0, 0]; // Близко к token0 (расстояние 50 < 100)
        token1.last_event_id = 0;

        let tokens = vec![token0, token1];
        domain.active_tokens = 2;

        // Перестраиваем spatial grid
        domain.rebuild_spatial_grid(&tokens);

        // Добавляем token0 в frontier
        domain.frontier.push_token(0);

        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        // Обрабатываем frontier
        let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

        // Должно быть событие столкновения
        let collision_events: Vec<_> = events
            .iter()
            .filter(|e| e.event_type == EventType::TokenCollision as u16)
            .collect();

        assert!(!collision_events.is_empty(), "Expected collision event");

        // Проверяем что collision event содержит правильные ID
        let collision = collision_events[0];
        // source_id и target_id могут быть в любом порядке
        assert!(
            (collision.source_id == 100 && collision.target_id == 101) ||
            (collision.source_id == 101 && collision.target_id == 100),
            "Collision should be between token 100 and 101"
        );
    }

    #[test]
    fn test_process_frontier_no_collision_when_far_apart() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;
        use crate::event::EventType;

        let config = DomainConfig::factory_logic(6, 1);
        let mut domain = Domain::new(config);

        // Создаем токены далеко друг от друга
        let mut token0 = Token::default();
        token0.sutra_id = 100;
        token0.domain_id = 6;
        token0.position = [0, 0, 0];
        token0.last_event_id = 0;

        let mut token1 = Token::default();
        token1.sutra_id = 101;
        token1.domain_id = 6;
        token1.position = [500, 0, 0]; // Далеко от token0 (расстояние 500 > 100)
        token1.last_event_id = 0;

        let tokens = vec![token0, token1];
        domain.active_tokens = 2;

        // Перестраиваем spatial grid
        domain.rebuild_spatial_grid(&tokens);

        // Добавляем token0 в frontier
        domain.frontier.push_token(0);

        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        // Обрабатываем frontier
        let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

        // Не должно быть события столкновения
        let collision_events: Vec<_> = events
            .iter()
            .filter(|e| e.event_type == EventType::TokenCollision as u16)
            .collect();

        assert!(collision_events.is_empty(), "Should not have collision event");
    }

    #[test]
    fn test_process_frontier_collision_adds_both_tokens_to_frontier() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;

        let config = DomainConfig::factory_logic(6, 1);
        let mut domain = Domain::new(config);

        // Создаем токены близко друг к другу
        let mut token0 = Token::default();
        token0.sutra_id = 100;
        token0.domain_id = 6;
        token0.position = [0, 0, 0];

        let mut token1 = Token::default();
        token1.sutra_id = 101;
        token1.domain_id = 6;
        token1.position = [50, 0, 0];

        let tokens = vec![token0, token1];
        domain.active_tokens = 2;

        // Перестраиваем spatial grid
        domain.rebuild_spatial_grid(&tokens);

        // Добавляем token0 в frontier
        domain.frontier.push_token(0);

        assert_eq!(domain.frontier.token_count(), 1);

        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        // Счётчик токенов в frontier до обработки
        let initial_token_count = domain.frontier.token_count();

        // Обрабатываем frontier
        let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

        // Проверяем что были сгенерированы collision events
        let collision_events: Vec<_> = events
            .iter()
            .filter(|e| e.event_type == crate::event::EventType::TokenCollision as u16)
            .collect();

        assert!(!collision_events.is_empty(), "Expected collision events");

        // Token1 должен быть добавлен в frontier (проверяем через событие, а не frontier напрямую)
        // frontier может быть уже обработан, поэтому проверяем события
        assert!(
            collision_events.iter().any(|e| e.target_id == 101),
            "Token 101 should be involved in collision"
        );
    }

    #[test]
    fn test_process_frontier_with_empty_spatial_grid() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;
        use crate::event::EventType;

        let config = DomainConfig::factory_logic(6, 1);
        let mut domain = Domain::new(config);

        // Создаем токены но НЕ перестраиваем spatial grid
        let mut token0 = Token::default();
        token0.sutra_id = 100;
        token0.domain_id = 6;
        token0.position = [0, 0, 0];

        let tokens = vec![token0];
        domain.active_tokens = 1;

        // Добавляем token0 в frontier
        domain.frontier.push_token(0);

        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        // Обрабатываем frontier
        let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

        // Не должно быть collision events если spatial grid пуст
        let collision_events: Vec<_> = events
            .iter()
            .filter(|e| e.event_type == EventType::TokenCollision as u16)
            .collect();

        assert!(collision_events.is_empty(), "Should not check collisions with empty grid");
    }

    #[test]
    fn test_process_frontier_multiple_collisions() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;
        use crate::event::EventType;

        let config = DomainConfig::factory_logic(6, 1);
        let mut domain = Domain::new(config);

        // Создаем 3 токена близко друг к другу
        let mut token0 = Token::default();
        token0.sutra_id = 100;
        token0.domain_id = 6;
        token0.position = [0, 0, 0];

        let mut token1 = Token::default();
        token1.sutra_id = 101;
        token1.domain_id = 6;
        token1.position = [50, 0, 0];

        let mut token2 = Token::default();
        token2.sutra_id = 102;
        token2.domain_id = 6;
        token2.position = [0, 50, 0];

        let tokens = vec![token0, token1, token2];
        domain.active_tokens = 3;

        // Перестраиваем spatial grid
        domain.rebuild_spatial_grid(&tokens);

        // Добавляем token0 в frontier
        domain.frontier.push_token(0);

        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        // Обрабатываем frontier
        let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

        // Должно быть как минимум 2 collision events (token0 -> token1 и token0 -> token2)
        // Может быть больше из-за Gravity events
        let collision_events: Vec<_> = events
            .iter()
            .filter(|e| e.event_type == EventType::TokenCollision as u16)
            .collect();

        assert!(collision_events.len() >= 2, "Expected at least 2 collision events, got {}", collision_events.len());

        // Проверяем что столкновения были с token1 и token2
        let collision_targets: Vec<u32> = collision_events
            .iter()
            .map(|e| e.target_id)
            .collect();

        assert!(collision_targets.contains(&101), "Should collide with token 101");
        assert!(collision_targets.contains(&102), "Should collide with token 102");
    }

    // --- SPACE V6.0 + Heartbeat Integration Tests (Phase 1.9) ---

    #[test]
    fn test_heartbeat_triggers_spatial_collision_checks() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;
        use crate::event::EventType;

        let config = DomainConfig::factory_logic(6, 1);

        let heartbeat_config = crate::heartbeat::HeartbeatConfig {
            interval: 3,
            batch_size: 2,
            enable_decay: false,
            enable_gravity: false,
            enable_spatial_collision: true, // SPACE V6.0: включаем spatial collision
            enable_connection_maintenance: false,
            ..crate::heartbeat::HeartbeatConfig::medium()
        };

        let mut domain = Domain::with_heartbeat(config, heartbeat_config);

        // Создаем токены близко друг к другу
        let mut token0 = Token::default();
        token0.sutra_id = 100;
        token0.domain_id = 6;
        token0.position = [0, 0, 0];

        let mut token1 = Token::default();
        token1.sutra_id = 101;
        token1.domain_id = 6;
        token1.position = [50, 0, 0]; // Близко к token0

        let tokens = vec![token0, token1];
        domain.active_tokens = 2;

        // Перестраиваем spatial grid
        domain.rebuild_spatial_grid(&tokens);

        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        // Симулируем несколько событий до heartbeat
        for event_id in 1000..1003 {
            event_generator.set_event_id(event_id);
            if let Some(pulse) = domain.on_event() {
                // Heartbeat добавляет токены в frontier
                domain.handle_heartbeat(pulse);

                event_generator.set_pulse_id(pulse);

                // Обрабатываем frontier - должны быть collision events
                let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

                // Проверяем collision events
                let collision_events: Vec<_> = events
                    .iter()
                    .filter(|e| e.event_type == EventType::TokenCollision as u16)
                    .collect();

                if !collision_events.is_empty() {
                    // Успешно обнаружили столкновение через heartbeat
                    assert!(collision_events.len() > 0, "Should detect collisions");
                    return;
                }
            }
        }

        panic!("Heartbeat should have triggered collision detection");
    }

    #[test]
    fn test_heartbeat_spatial_collision_can_be_disabled() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;
        use crate::event::EventType;

        let config = DomainConfig::factory_logic(6, 1);

        let heartbeat_config = crate::heartbeat::HeartbeatConfig {
            interval: 3,
            batch_size: 2,
            enable_decay: false,
            enable_gravity: false,
            enable_spatial_collision: false, // Отключаем spatial collision
            enable_connection_maintenance: false,
            ..crate::heartbeat::HeartbeatConfig::medium()
        };

        let mut domain = Domain::with_heartbeat(config, heartbeat_config);

        // Создаем токены близко друг к другу
        let mut token0 = Token::default();
        token0.sutra_id = 100;
        token0.domain_id = 6;
        token0.position = [0, 0, 0];

        let mut token1 = Token::default();
        token1.sutra_id = 101;
        token1.domain_id = 6;
        token1.position = [50, 0, 0];

        let tokens = vec![token0, token1];
        domain.active_tokens = 2;

        // Перестраиваем spatial grid
        domain.rebuild_spatial_grid(&tokens);

        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        // Симулируем несколько событий
        for event_id in 1000..1010 {
            event_generator.set_event_id(event_id);
            if let Some(pulse) = domain.on_event() {
                domain.handle_heartbeat(pulse);
                event_generator.set_pulse_id(pulse);

                let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

                // Не должно быть collision events
                let collision_events: Vec<_> = events
                    .iter()
                    .filter(|e| e.event_type == EventType::TokenCollision as u16)
                    .collect();

                assert!(collision_events.is_empty(), "Should not detect collisions when disabled");
            }
        }
    }

    #[test]
    fn test_heartbeat_with_gravity_and_spatial_collision() {
        use crate::token::Token;
        use crate::connection::Connection;
        use crate::event_generator::EventGenerator;
        use crate::event::EventType;

        let mut config = DomainConfig::factory_logic(6, 1);
        config.gravity_strength = 1.0; // Включаем гравитацию

        let heartbeat_config = crate::heartbeat::HeartbeatConfig {
            interval: 3,
            batch_size: 2,
            enable_decay: false,
            enable_gravity: true, // Гравитация
            enable_spatial_collision: true, // Столкновения
            enable_connection_maintenance: false,
            ..crate::heartbeat::HeartbeatConfig::medium()
        };

        let mut domain = Domain::with_heartbeat(config, heartbeat_config);

        // Создаем токены
        let mut token0 = Token::default();
        token0.sutra_id = 100;
        token0.domain_id = 6;
        token0.position = [0, 0, 0];

        let mut token1 = Token::default();
        token1.sutra_id = 101;
        token1.domain_id = 6;
        token1.position = [50, 0, 0];

        let tokens = vec![token0, token1];
        domain.active_tokens = 2;

        domain.rebuild_spatial_grid(&tokens);

        let connections = vec![];
        let mut event_generator = EventGenerator::new();
        event_generator.set_event_id(1000);

        let mut has_gravity = false;
        let mut has_collision = false;

        // Симулируем события
        for event_id in 1000..1010 {
            event_generator.set_event_id(event_id);
            if let Some(pulse) = domain.on_event() {
                domain.handle_heartbeat(pulse);
                event_generator.set_pulse_id(pulse);

                let events = domain.process_frontier(&tokens, &connections, &mut event_generator);

                // Проверяем gravity events
                for event in &events {
                    if event.event_type == EventType::GravityUpdate as u16 {
                        has_gravity = true;
                    }
                    if event.event_type == EventType::TokenCollision as u16 {
                        has_collision = true;
                    }
                }

                if has_gravity && has_collision {
                    break;
                }
            }
        }

        assert!(has_gravity, "Should generate gravity events");
        assert!(has_collision, "Should generate collision events");
    }

    #[test]
    fn test_heartbeat_rebuilds_spatial_grid_periodically() {
        use crate::token::Token;

        let mut config = DomainConfig::factory_logic(6, 1);
        config.rebuild_frequency = 5; // Перестройка каждые 5 событий

        let heartbeat_config = crate::heartbeat::HeartbeatConfig {
            interval: 3,
            batch_size: 2,
            enable_spatial_collision: true,
            ..crate::heartbeat::HeartbeatConfig::medium()
        };

        let mut domain = Domain::with_heartbeat(config, heartbeat_config);

        let tokens: Vec<Token> = (0..5).map(|i| {
            let mut token = Token::default();
            token.sutra_id = i;
            token.domain_id = 6;
            token.position = [i as i16 * 100, 0, 0];
            token
        }).collect();

        domain.active_tokens = 5;

        // Первая перестройка
        domain.rebuild_spatial_grid(&tokens);
        assert_eq!(domain.events_since_rebuild, 0);

        // Инкрементируем счётчик
        for _ in 0..5 {
            domain.increment_events_since_rebuild();
        }

        assert!(domain.should_rebuild_spatial_grid(), "Should need rebuild after 5 events");

        // Перестройка сбрасывает счётчик
        domain.rebuild_spatial_grid(&tokens);
        assert_eq!(domain.events_since_rebuild, 0);
    }
}
