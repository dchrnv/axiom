//! Domain Configuration
//!
//! DomainConfig — 128 байт конфигурация домена с 5 блоками:
//! 1. Идентификация [16 байт]
//! 2. Физика поля [32 байта]
//! 3. Семантические оси [16 байт]
//! 4. Мембрана и Arbiter [32 байта]
//! 5. Метаданные [32 байта]

use serde::{Deserialize, Serialize};

/// Домен активен и принимает токены
pub const DOMAIN_ACTIVE: u32 = 1;
/// Домен заблокирован — не принимает новые токены
pub const DOMAIN_LOCKED: u32 = 2;
/// Домен временный — будет уничтожен после завершения задачи
pub const DOMAIN_TEMPORARY: u32 = 3;

/// Домен простаивает
pub const PROCESSING_IDLE: u8 = 1;
/// Домен активно обрабатывает токены
pub const PROCESSING_ACTIVE: u8 = 2;
/// Домен заморожен
pub const PROCESSING_FROZEN: u8 = 3;

/// Мембрана полностью открыта — все токены проходят
pub const MEMBRANE_OPEN: u8 = 0;
/// Мембрана полупроницаема — фильтрация по порогу массы
pub const MEMBRANE_SEMI: u8 = 1;
/// Мембрана закрыта — только системные токены
pub const MEMBRANE_CLOSED: u8 = 2;
/// Мембрана адаптивная — меняется по контексту
pub const MEMBRANE_ADAPTIVE: u8 = 3;

/// Структурные роли доменов в системе Ashti_Core
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructuralRole {
    /// Исходный поток (роль 0)
    Sutra = 0,
    /// Исполнение (роль 1)
    Execution = 1,
    /// Тень / отражение (роль 2)
    Shadow = 2,
    /// Кодекс / правила (роль 3)
    Codex = 3,
    /// Карта / пространство (роль 4)
    Map = 4,
    /// Зонд / наблюдение (роль 5)
    Probe = 5,
    /// Логика (роль 6)
    Logic = 6,
    /// Сновидение / генерация (роль 7)
    Dream = 7,
    /// Вакуум / нейтральный (роль 8)
    Void = 8,
    /// Ассоциативная память (роль 9)
    Experience = 9,
    /// Консолидация результатов (роль 10)
    Maya = 10,
}

/// Типы доменов
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DomainType {
    /// Логический домен
    Logic = 1,
    /// Генеративный / сновидческий домен
    Dream = 2,
    /// Математический домен
    Math = 3,
    /// Домен паттернов
    Pattern = 4,
    /// Домен памяти
    Memory = 5,
    /// Интерфейсный домен
    Interface = 6,
}

/// DomainConfig — 128 байт конфигурация домена
///
/// Размер: 128 байт, выравнивание: 128 байт
/// Соответствует спецификации V2.1
#[repr(C, align(128))]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DomainConfig {
    // --- 1. ИДЕНТИФИКАЦИЯ [16 Байт] ---
    /// Явный резерв для будущих расширений
    pub reserved_id: u64,
    /// Уникальный ID Домена
    pub domain_id: u16,
    /// Родительский Домен
    pub parent_domain_id: u16,
    /// Тип домена (до 255 вариаций)
    pub domain_type: u8,
    /// Роль в Ashti_Core (0-10: SUTRA..MAYA)
    pub structural_role: u8,
    /// Поколение (эволюционный индекс)
    pub generation: u8,
    /// Битовая маска состояний (Active, Locked)
    pub flags: u8,

    // --- 2. ФИЗИКА ПОЛЯ [32 Байт] ---
    /// Размеры поля (X, Y, Z)
    pub field_size: [f32; 3],
    /// Гравитация (-MAX..+MAX)
    pub gravity_strength: f32,
    /// Температура поля в Кельвинах
    pub temperature: f32,
    /// Замедление времени (×100)
    pub time_dilation: u16,
    /// Базовая частота (Hz)
    pub resonance_freq: u16,
    /// Давление (Pa)
    pub pressure: u16,
    /// SPACE V6.0: частота перестройки spatial grid (в событиях)
    pub rebuild_frequency: u16,
    /// Трение (0..255 → 0.0..1.0)
    pub friction_coeff: u8,
    /// Вязкость (0..255 → 0.0..1.0)
    pub viscosity: u8,
    /// Упругость (0..255 → 0.0..1.0)
    pub elasticity: u8,
    /// Квантовый шум (0..255 → 0.0..1.0)
    pub quantum_noise: u8,

    // --- 3. СЕМАНТИЧЕСКИЕ ОСИ [16 Байт] ---
    /// Референс концепции оси X
    pub axis_x_ref: u32,
    /// Референс концепции оси Y
    pub axis_y_ref: u32,
    /// Референс концепции оси Z
    pub axis_z_ref: u32,
    /// Конфигурация полюсов (Bit-packed u16×2)
    pub axis_config: u32,

    // --- 4. МЕМБРАНА И ARBITER [32 Байт] ---
    /// 64-bit Bloom Filter или хэш входа
    pub input_filter: u64,
    /// 64-bit Bloom Filter или хэш выхода
    pub output_filter: u64,

    // -- Блок Arbiter [8 Байт] --
    /// Порог рефлекса (0..255 → 0.0..1.0)
    pub reflex_threshold: u8,
    /// Порог ассоциации (0..255 → 0.0..1.0)
    pub association_threshold: u8,
    /// Битовая маска поведения Arbiter
    pub arbiter_flags: u8,
    /// Минимальный интервал между рефлексами (в пульсах)
    pub reflex_cooldown: u8,
    /// Макс. количество одновременных ассоциаций-подсказок
    pub max_concurrent_hints: u8,
    /// Шаг изменения weight при обратной связи (0..255)
    pub feedback_weight_delta: u8,
    /// Резерв блока Arbiter
    pub reserved_arbiter: [u8; 2],

    /// Вычислительная сложность шлюзов
    pub gate_complexity: u16,
    /// Порог массы для прохождения мембраны
    pub threshold_mass: u16,
    /// Порог температуры для прохождения мембраны
    pub threshold_temp: u16,
    /// Проницаемость (0..255 → 0.0..1.0)
    pub permeability: u8,
    /// Состояние мембраны: OPEN/CLOSED/SEMI/ADAPTIVE
    pub membrane_state: u8,

    // --- 5. МЕТАДАННЫЕ [32 Байт] ---
    /// COM event_id момента создания
    pub created_at: u64,
    /// COM event_id последнего обновления
    pub last_update: u64,
    /// Максимальная ёмкость токенов
    pub token_capacity: u32,
    /// Максимальная ёмкость связей
    pub connection_capacity: u32,
    /// Счётчик когнитивных ошибок
    pub error_count: u16,
    /// Состояние обработки: IDLE/PROCESSING/FROZEN/CRASHED
    pub processing_state: u8,
    /// Оценка сложности (0..255 → 0.0..1.0)
    pub complexity_score: u8,
    /// Производительность (0..255 → 0.0..1.0)
    pub performance_score: u8,
    /// Добивка до границы 128 байт
    pub reserved_meta: [u8; 3],
}

// Compile-time size assertion
const _: () = assert!(std::mem::size_of::<DomainConfig>() == 128);

impl Default for DomainConfig {
    fn default() -> Self {
        Self {
            // --- 1. ИДЕНТИФИКАЦИЯ [16 Байт] ---
            reserved_id: 0,
            domain_id: 1,
            parent_domain_id: 0,
            domain_type: DomainType::Logic as u8,
            structural_role: StructuralRole::Logic as u8,
            generation: 0,
            flags: DOMAIN_ACTIVE as u8,

            // --- 2. ФИЗИКА ПОЛЯ [32 Байт] ---
            field_size: [100.0, 100.0, 100.0],
            gravity_strength: 1.0,
            temperature: 293.15, // 20°C
            time_dilation: 100,  // 1.0x
            resonance_freq: 440,
            pressure: 50000,
            rebuild_frequency: 10,
            friction_coeff: 25,
            viscosity: 3,
            elasticity: 128,
            quantum_noise: 0,

            // --- 3. СЕМАНТИЧЕСКИЕ ОСИ [16 Байт] ---
            axis_x_ref: 0,
            axis_y_ref: 0,
            axis_z_ref: 0,
            axis_config: 0,

            // --- 4. МЕМБРАНА И ARBITER [32 Байт] ---
            input_filter: 0,
            output_filter: 0,
            reflex_threshold: 128,
            association_threshold: 64,
            arbiter_flags: 0,
            reflex_cooldown: 5,
            max_concurrent_hints: 10,
            feedback_weight_delta: 10,
            reserved_arbiter: [0; 2],
            gate_complexity: 100,
            threshold_mass: 0,
            threshold_temp: 0,
            permeability: 128,
            membrane_state: 1, // OPEN

            // --- 5. МЕТАДАННЫЕ [32 Байт] ---
            created_at: 0,
            last_update: 0,
            token_capacity: 1000,
            connection_capacity: 5000,
            error_count: 0,
            processing_state: PROCESSING_IDLE,
            complexity_score: 0,
            performance_score: 255,
            reserved_meta: [0; 3],
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

    /// Базовая фабрика (ВАКУУМ)
    ///
    /// Создает абсолютно нейтральное, пустое пространство (Role::Void = 8).
    /// Используется как основа для всех остальных фабрик.
    pub fn default_void() -> Self {
        Self {
            // --- 1. ИДЕНТИФИКАЦИЯ [16 Байт] ---
            reserved_id: 0,
            domain_id: 0,
            parent_domain_id: 0,
            domain_type: 0,
            structural_role: StructuralRole::Void as u8,
            generation: 0,
            flags: 0,

            // --- 2. ФИЗИКА ПОЛЯ [32 Байт] ---
            field_size: [0.0, 0.0, 0.0],
            gravity_strength: 0.0,
            temperature: 0.0,
            time_dilation: 0,
            resonance_freq: 0,
            pressure: 0,
            rebuild_frequency: 0,
            friction_coeff: 0,
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
            permeability: 0,
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

    /// SUTRA (0) - Источник Истины
    ///
    /// Физика: Кристаллизация. Абсолютный ноль, колоссальная гравитация.
    /// Сюда ничего не проникает извне, отсюда только рождаются смыслы.
    pub fn factory_sutra(domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.structural_role = StructuralRole::Sutra as u8;

        config.gravity_strength = f32::MAX;
        config.temperature = 0.0;
        config.permeability = 0;
        config.membrane_state = 2; // CLOSED

        config.token_capacity = 100;
        config.connection_capacity = 1000;
        config
    }

    /// EXECUTION (1) — Реализация решений
    ///
    /// Физика: Умеренная среда для быстрой реакции.
    /// Средняя температура, нормальная гравитация, низкое трение.
    pub fn factory_execution(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = StructuralRole::Execution as u8;

        config.field_size = [2000.0, 2000.0, 2000.0];
        config.gravity_strength = 9.81;
        config.temperature = 310.0;         // ~37°C — активная среда
        config.elasticity = 180;            // ~0.7 — умеренная упругость
        config.friction_coeff = 30;         // ~0.12 — низкое трение

        config.permeability = 180;
        config.membrane_state = MEMBRANE_SEMI;

        config.reflex_threshold = 140;
        config.association_threshold = 50;
        config.arbiter_flags = 0b00011111;
        config.reflex_cooldown = 0;
        config.max_concurrent_hints = 3;
        config.feedback_weight_delta = 30;

        config.token_capacity = 5000;
        config.connection_capacity = 2500;
        config
    }

    /// SHADOW (2) — Симуляция и предсказание
    ///
    /// Физика: Осторожная среда для точных симуляций.
    /// Высокий порог, низкая температура для стабильности.
    pub fn factory_shadow(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = StructuralRole::Shadow as u8;

        config.field_size = [2000.0, 2000.0, 2000.0];
        config.gravity_strength = 5.0;
        config.temperature = 250.0;         // Прохладная среда для стабильности
        config.viscosity = 180;             // ~0.7 — замедленное движение
        config.friction_coeff = 50;

        config.permeability = 150;
        config.membrane_state = MEMBRANE_CLOSED;

        config.reflex_threshold = 180;
        config.association_threshold = 40;
        config.arbiter_flags = 0b00010111;
        config.reflex_cooldown = 2;
        config.max_concurrent_hints = 4;
        config.feedback_weight_delta = 20;

        config.token_capacity = 8000;
        config.connection_capacity = 4000;
        config
    }

    /// CODEX (3) — Конституция и правила
    ///
    /// Физика: Высокая стабильность и вязкость. Данные здесь «застревают»
    /// и становятся законами. Очень холодно — паттерны не мутируют.
    pub fn factory_codex(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = StructuralRole::Codex as u8;

        config.field_size = [2000.0, 2000.0, 2000.0];
        config.gravity_strength = 1000.0;
        config.temperature = 10.0;          // Почти ноль — минимальные колебания
        config.viscosity = 250;             // ~0.98 — токены вязнут и фиксируются
        config.friction_coeff = 200;

        config.permeability = 25;
        config.membrane_state = MEMBRANE_CLOSED;

        config.reflex_threshold = 0;
        config.association_threshold = 0;
        config.arbiter_flags = 0b00000000;
        config.reflex_cooldown = 0;
        config.max_concurrent_hints = 0;
        config.feedback_weight_delta = 0;

        config.token_capacity = 500;
        config.connection_capacity = 50;
        config
    }

    /// MAP (4) — Карта мира и фактов
    ///
    /// Физика: Стабильная среда для достоверных данных.
    /// Высокая гравитация для удержания фактов.
    pub fn factory_map(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = StructuralRole::Map as u8;

        config.field_size = [2000.0, 2000.0, 2000.0];
        config.gravity_strength = 15.0;
        config.temperature = 280.0;
        config.friction_coeff = 40;
        config.viscosity = 200;

        config.permeability = 120;
        config.membrane_state = MEMBRANE_CLOSED;

        config.reflex_threshold = 200;
        config.association_threshold = 80;
        config.arbiter_flags = 0b00011111;
        config.reflex_cooldown = 3;
        config.max_concurrent_hints = 2;
        config.feedback_weight_delta = 40;

        config.token_capacity = 10000;
        config.connection_capacity = 5000;
        config
    }

    /// PROBE (5) — Исследование и анализ
    ///
    /// Физика: Активная исследовательская среда.
    /// Повышенная температура, высокий резонанс для активного поиска.
    pub fn factory_probe(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = StructuralRole::Probe as u8;

        config.field_size = [2000.0, 2000.0, 2000.0];
        config.gravity_strength = 7.0;
        config.temperature = 350.0;
        config.resonance_freq = 800;
        config.friction_coeff = 35;
        config.elasticity = 200;

        config.permeability = 190;
        config.membrane_state = MEMBRANE_OPEN;

        config.reflex_threshold = 160;
        config.association_threshold = 60;
        config.arbiter_flags = 0b00010111;
        config.reflex_cooldown = 1;
        config.max_concurrent_hints = 5;
        config.feedback_weight_delta = 25;

        config.token_capacity = 6000;
        config.connection_capacity = 3000;
        config
    }

    /// LOGIC (6) — Чистое вычисление
    ///
    /// Физика: Идеальная рабочая среда. Комнатная температура,
    /// умеренная гравитация. Смыслы сталкиваются, вычисляются, летят дальше.
    pub fn factory_logic(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = StructuralRole::Logic as u8;

        config.field_size = [2000.0, 2000.0, 2000.0];
        config.gravity_strength = 9.81;
        config.temperature = 273.0;
        config.elasticity = 200;
        config.friction_coeff = 25;

        config.permeability = 127;
        config.membrane_state = MEMBRANE_ADAPTIVE;

        config.reflex_threshold = 230;
        config.association_threshold = 100;
        config.arbiter_flags = 0b00011111;
        config.reflex_cooldown = 5;
        config.max_concurrent_hints = 2;
        config.feedback_weight_delta = 50;

        config.token_capacity = 2000;
        config.connection_capacity = 200;
        config
    }

    /// DREAM (7) — Фоновая оптимизация и галлюцинация
    ///
    /// Физика: Полный хаос. Нулевая гравитация, высокая температура,
    /// высокий квантовый шум. Смыслы мутируют и образуют случайные связи.
    pub fn factory_dream(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = StructuralRole::Dream as u8;

        config.field_size = [2000.0, 2000.0, 2000.0];
        config.gravity_strength = 0.0;
        config.temperature = 500.0;
        config.quantum_noise = 200;
        config.time_dilation = 50;          // x0.5 — время течёт быстрее

        config.permeability = 200;
        config.membrane_state = MEMBRANE_OPEN;

        config.reflex_threshold = 0;
        config.association_threshold = 25;
        config.arbiter_flags = 0b00010010;
        config.reflex_cooldown = 0;
        config.max_concurrent_hints = 8;
        config.feedback_weight_delta = 10;

        config.token_capacity = 3000;
        config.connection_capacity = 300;
        config
    }

    /// VOID (8) — Аннигиляция и трансформация
    ///
    /// Физика: Экстремальная среда. Очень высокая температура и гравитация,
    /// токены здесь разрушаются и трансформируются.
    pub fn factory_void(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = StructuralRole::Void as u8;

        config.field_size = [2000.0, 2000.0, 2000.0];
        config.gravity_strength = 100.0;
        config.temperature = 1000.0;
        config.friction_coeff = 200;
        config.viscosity = 100;

        config.permeability = 255;
        config.membrane_state = MEMBRANE_OPEN;

        config.reflex_threshold = 0;
        config.association_threshold = 0;
        config.arbiter_flags = 0b00000000;
        config.reflex_cooldown = 0;
        config.max_concurrent_hints = 0;
        config.feedback_weight_delta = 0;

        config.token_capacity = 2000;
        config.connection_capacity = 100;
        config
    }

    /// EXPERIENCE (9) — Ассоциативная память и рефлексы (Ashti_Core V2.0)
    ///
    /// Физика: Низкая гравитация (свободное перемещение следов),
    /// минимальное затухание (ничего не забывается), высокий резонанс.
    pub fn factory_experience(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = StructuralRole::Experience as u8;

        config.field_size = [5000.0, 5000.0, 5000.0]; // Большое поле для множества следов
        config.gravity_strength = 0.5;
        config.temperature = 300.0;
        config.resonance_freq = 1000;       // Высокий резонанс — лёгкий поиск
        config.friction_coeff = 20;
        config.viscosity = 200;

        config.permeability = 200;
        config.membrane_state = MEMBRANE_SEMI;

        config.reflex_threshold = 0;        // Сам источник рефлексов, не получатель
        config.association_threshold = 0;
        config.arbiter_flags = 0b00000100;  // Только FEEDBACK_ENABLED
        config.reflex_cooldown = 0;
        config.max_concurrent_hints = 0;
        config.feedback_weight_delta = 0;   // Управляется внутренней логикой домена 9

        config.token_capacity = 100000;     // Много следов (опыт накапливается)
        config.connection_capacity = 50000;
        config
    }

    /// MAYA (10) — Интерфейс и проекция (Иллюзия)
    ///
    /// Физика: Мягкое, тёплое поле без трения. Презентационный слой,
    /// где токены собираются в структуры для выдачи ответа.
    pub fn factory_maya(domain_id: u16, parent_domain_id: u16) -> Self {
        let mut config = Self::default_void();
        config.domain_id = domain_id;
        config.parent_domain_id = parent_domain_id;
        config.structural_role = StructuralRole::Maya as u8;

        config.field_size = [2000.0, 2000.0, 2000.0];
        config.gravity_strength = 1.0;
        config.temperature = 310.0;
        config.friction_coeff = 5;

        config.permeability = 255;
        config.membrane_state = MEMBRANE_OPEN;

        config.reflex_threshold = 0;
        config.association_threshold = 0;
        config.arbiter_flags = 0b00000000;
        config.reflex_cooldown = 0;
        config.max_concurrent_hints = 0;
        config.feedback_weight_delta = 0;

        config.token_capacity = 5000;
        config.connection_capacity = 500;
        config
    }

    /// Проверить может ли токен войти в домен по параметрам мембраны
    pub fn can_enter(&self, mass: u16, _temperature: u16) -> bool {
        self.membrane_state != MEMBRANE_CLOSED && mass >= self.threshold_mass
    }

    /// Обновить метаданные при изменении (event_id из COM)
    pub fn update_metadata(&mut self, event_id: u64) {
        self.last_update = event_id;
        self.error_count = 0;
    }

    /// Домен активен
    pub fn is_active(&self) -> bool {
        (self.flags & (DOMAIN_ACTIVE as u8)) != 0
    }

    /// Домен заблокирован
    pub fn is_locked(&self) -> bool {
        (self.flags & (DOMAIN_LOCKED as u8)) != 0
    }

    /// Домен временный
    pub fn is_temporary(&self) -> bool {
        (self.flags & (DOMAIN_TEMPORARY as u8)) != 0
    }

    /// Расчёт сложности домена
    pub fn calculate_complexity(&self) -> f32 {
        let token_factor = self.token_capacity as f32 * 0.1;
        let connection_factor = self.connection_capacity as f32 * 0.05;
        let friction_factor = (self.friction_coeff as f32 / 255.0) * 10.0;
        token_factor + connection_factor + friction_factor
    }

    /// Валидация конфигурации
    pub fn validate(&self) -> Result<(), String> {
        // Проверка размера структуры
        if std::mem::size_of::<Self>() != 128 {
            return Err(format!(
                "DomainConfig size is {} bytes, expected 128",
                std::mem::size_of::<Self>()
            ));
        }

        // Проверка field_size
        if self.field_size[0] < 0.0
            || self.field_size[1] < 0.0
            || self.field_size[2] < 0.0
        {
            return Err("field_size components must be non-negative".to_string());
        }

        // Проверка temperature
        if self.temperature < 0.0 {
            return Err("temperature must be non-negative (Kelvin)".to_string());
        }

        // Проверка capacities
        if self.token_capacity == 0 {
            return Err("token_capacity must be > 0".to_string());
        }

        if self.connection_capacity == 0 {
            return Err("connection_capacity must be > 0".to_string());
        }

        Ok(())
    }
}
