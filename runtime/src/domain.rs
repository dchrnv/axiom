// Copyright (C) 2024-2026 Chernov Denys
//
// DomainConfig V2.1 - 128 байт конфигурация домена
// Соответствие спецификации DomainConfig V2.1 (Arbiter Integration)

use serde::{Serialize, Deserialize};
use crate::causal_frontier::CausalFrontier;
use crate::heartbeat::{HeartbeatGenerator, HeartbeatConfig};

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

/// Структурные роли в Ashti_Core v2.0
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
    pub structural_role: u8,    // 1b | Роль в Ashti_Core (Sutra, Logic, Dream)
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
    pub reserved_physics: u16,  // 2b | Резерв блока физики
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
            reserved_physics: 0,  // Резерв блока физики
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
            created_at: 1715292000,      // Время создания (Unix timestamp)
            last_update: 1715292000,     // Последнее обновление
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
            reserved_physics: 0,
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
        
        config.created_at = 1715292000; // Время создания
        config.last_update = 1715292000; // Последнее обновление
        
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
    /// CODEX (3) - Конституция и Правила
    /// Физика: Высокая стабильность и вязкость. Данные здесь "застревают"
    /// и становятся законами. Очень холодно, чтобы паттерны не мутировали.
    /// -----------------------------------------------------------------------
    pub fn factory_codex(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = 3; // Codex
        
        config.created_at = 1715292000; // Время создания
        config.last_update = 1715292000; // Последнее обновление
        
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
        
        config.created_at = 1715292000; // Время создания
        config.last_update = 1715292000; // Последнее обновление
        
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
    /// DREAM (7) - Фоновая оптимизация и галлюцинация
    /// Физика: Полный хаос. Нулевая гравитация, высокая температура (кипение),
    /// высокий квантовый шум. Смыслы мутируют и образуют случайные связи.
    /// -----------------------------------------------------------------------
    pub fn factory_dream(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = 7; // Dream
        
        config.created_at = 1715292000; // Время создания
        config.last_update = 1715292000; // Последнее обновление
        
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
        
        config.created_at = 1715292000; // Время создания
        config.last_update = 1715292000; // Последнее обновление
        
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
        
        // COM синхронизация
        if self.created_at == 0 || self.last_update < self.created_at {
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
pub struct Domain {
    /// Конфигурация домена
    pub config: DomainConfig,

    /// Causal Frontier для управления активными вычислениями
    pub frontier: CausalFrontier,

    /// Heartbeat generator для периодической активации
    pub heartbeat: HeartbeatGenerator,

    /// Heartbeat конфигурация
    pub heartbeat_config: HeartbeatConfig,

    /// Текущее количество активных токенов
    pub active_tokens: usize,

    /// Текущее количество активных связей
    pub active_connections: usize,
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
            active_tokens: 0,
            active_connections: 0,
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
        // Тестируем все существующие factory методы
        let configs = vec![
            DomainConfig::factory_sutra(1),     // SUTRA domain_id не может быть 0
            DomainConfig::factory_codex(3, 1),
            DomainConfig::factory_logic(6, 1),
            DomainConfig::factory_dream(7, 1),
            DomainConfig::factory_experience(9, 1),
            DomainConfig::factory_maya(10, 1),
        ];

        for config in configs {
            assert!(config.validate(),
                "Factory method for role {} produced invalid config",
                config.structural_role);
        }
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
}
