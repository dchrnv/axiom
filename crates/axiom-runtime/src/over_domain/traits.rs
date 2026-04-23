// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Over-Domain Layer — базовые trait'ы
// Спецификация: docs/spec/Weaver/Over_Domain_Layer_V1_1.md

use std::sync::Arc;
use axiom_core::Token;
use axiom_domain::{AshtiCore, DomainState};
use axiom_genome::{Genome, ModuleId};
use axiom_ucl::UclCommand;

/// Идентификатор Weaver-компонента в TickSchedule.
pub type WeaverId = u16;

/// Ошибка жизненного цикла Over-Domain компонента.
#[derive(Debug)]
pub enum OverDomainError {
    /// Ошибка при инициализации компонента
    BootFailed(String),
    /// Ошибка при обработке тика
    TickFailed(String),
    /// GENOME не выдал доступ к запрошенному ресурсу
    GenomeDenied,
}

impl std::fmt::Display for OverDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverDomainError::BootFailed(s) => write!(f, "boot failed: {s}"),
            OverDomainError::TickFailed(s) => write!(f, "tick failed: {s}"),
            OverDomainError::GenomeDenied   => write!(f, "genome denied access"),
        }
    }
}

impl std::error::Error for OverDomainError {}

/// Предложение на кристаллизацию узора в EXPERIENCE (или SUTRA через CODEX).
///
/// Генерируется Weaver::propose_to_dream, обрабатывается DREAM-фазой.
pub struct CrystallizationProposal {
    /// Источник предложения
    pub weaver_id: WeaverId,
    /// Целевой домен (109 = EXPERIENCE по умолчанию, 100 = SUTRA через промоцию)
    pub target_domain: u16,
    /// UCL-команды для выполнения при принятии предложения
    pub commands: Vec<UclCommand>,
    /// Приоритет предложения (0 = низкий, 255 = критический)
    pub priority: u8,
}

/// Предложение на промоцию Frame из EXPERIENCE в SUTRA.
///
/// Генерируется Weaver::check_promotion. Требует санкции CODEX.
pub struct PromotionProposal {
    /// Источник предложения
    pub weaver_id: WeaverId,
    /// sutra_id Frame-анкера в EXPERIENCE, подлежащего промоции
    pub anchor_id: u32,
    /// UCL-команды для создания промотированного анкера в SUTRA
    pub commands: Vec<UclCommand>,
}

// ============================================================================
// OverDomainComponent — базовый trait (object-safe)
// ============================================================================

/// Базовый trait для всех Over-Domain компонентов.
///
/// Object-safe: компоненты хранятся как `Box<dyn OverDomainComponent>` в AxiomEngine.
///
/// Инварианты (Over_Domain_Layer_V1_1.md раздел 4):
/// - Нет собственного хранилища доменных данных
/// - Чтение только через peek_state (передаётся в on_tick через &AshtiCore)
/// - Запись только через UCL (возврат из on_shutdown или отдельных методов)
/// - Подчиняются GUARDIAN
pub trait OverDomainComponent: Send {
    /// Уникальное имя компонента (для логирования и диагностики)
    fn name(&self) -> &'static str;

    /// ModuleId компонента в GENOME (для проверки прав доступа)
    fn module_id(&self) -> ModuleId;

    /// Вызывается один раз при boot AxiomEngine, после загрузки Genome.
    ///
    /// Используется для регистрации в GENOME (проверка, что ModuleId существует),
    /// инициализации внутренних структур на основе Genome-конфигурации.
    fn on_boot(&mut self, genome: &Arc<Genome>) -> Result<(), OverDomainError>;

    /// Интервал вызова on_tick в тиках (default: 1 = каждый тик).
    ///
    /// AxiomEngine вызывает on_tick только когда `tick_count % on_tick_interval() == 0`.
    /// Weavers переопределяют для настройки частоты сканирования.
    fn on_tick_interval(&self) -> u32 {
        1
    }

    /// Вызывается каждые `on_tick_interval()` тиков.
    ///
    /// - `tick` — текущий tick_count AxiomEngine
    /// - `ashti` — read-only доступ к состоянию всех доменов
    ///
    /// Компонент обновляет внутреннее состояние (кандидаты, статистику).
    /// Для генерации UCL-команд используется on_shutdown или отдельные методы.
    fn on_tick(&mut self, tick: u64, ashti: &AshtiCore) -> Result<(), OverDomainError>;

    /// Вызывается при shutdown AxiomEngine.
    ///
    /// Возвращает финальные UCL-команды (сброс незакристаллизованных кандидатов,
    /// финальная запись статистики в CODEX).
    fn on_shutdown(&mut self) -> Vec<UclCommand>;
}

// ============================================================================
// Weaver — trait для Weavers (не object-safe из-за type Pattern)
// ============================================================================

/// Trait для компонентов типа Weaver.
///
/// Weavers сканируют MAYA, распознают паттерны и предлагают кристаллизацию в EXPERIENCE.
/// Особо устойчивые Frame могут быть промоутированы в SUTRA через CODEX.
///
/// Не object-safe из-за `type Pattern` — конкретные Weavers хранятся по значению,
/// не через `Box<dyn Weaver>`.
pub trait Weaver: OverDomainComponent {
    /// Тип распознаваемого паттерна (зависит от категории Weaver)
    type Pattern: Send;

    /// Сканировать MAYA и вернуть список распознанных паттернов-кандидатов.
    ///
    /// Вызывается из on_tick по расписанию weaver_scan_intervals.
    fn scan(&mut self, maya_state: &DomainState) -> Vec<Self::Pattern>;

    /// Преобразовать стабильных кандидатов в предложения для DREAM-фазы.
    ///
    /// DREAM решает: кристаллизовать / отклонить / отложить.
    fn propose_to_dream(&self, patterns: &[Self::Pattern]) -> Vec<CrystallizationProposal>;

    /// Проверить Frame в EXPERIENCE на соответствие правилам промоции.
    ///
    /// Вызывается по расписанию weaver_promotion_intervals (значительно реже scan).
    /// Возвращает пустой Vec если нет кандидатов на промоцию.
    fn check_promotion(&self, experience_state: &DomainState, anchors: &[&Token]) -> Vec<PromotionProposal>;

    /// Numeric ID компонента для TickSchedule
    fn weaver_id(&self) -> WeaverId;

    /// Целевой домен для кристаллизации (по умолчанию EXPERIENCE = 109).
    ///
    /// Промоция в SUTRA (100) — отдельный путь через check_promotion + CODEX.
    fn target_domain(&self) -> u16 {
        109
    }
}
