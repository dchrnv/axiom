// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// NeuralAdvisorRegistry — хранит все пять слотов советников.
// Источник: docs/architecture/NeuralAdvisor_V1_0.md §5

use std::sync::Arc;

use crate::over_domain::neural_advisor::{
    implementations::{
        AnchorVotingAdvisor, DepthHistoryBiasAdvisor, DepthThresholdEmergentDetector,
        NullConflictResolver, NullDepthAdvisor, NullEmergentAdvisor, NullOctantAdvisor,
        NullSubsystemAdvisor, PatternLearningResolver, ReactivationDepthAdvisor,
        RuleBasedCorpusCallosumResolver,
    },
    traits::{
        CorpusCallosumResolver, DepthPredictionAdvisor, EmergentPatternAdvisor,
        OctantCorrectionAdvisor, SubsystemAttributionAdvisor,
    },
};

/// Реестр советников. Каждый слот — `Option<Arc<dyn Advisor>>`.
///
/// `None` — советник не сконфигурирован.
/// `Some(NullAdvisor)` — советник присутствует, но намеренно молчит.
pub struct NeuralAdvisorRegistry {
    pub depth: Option<Arc<dyn DepthPredictionAdvisor>>,
    pub octant: Option<Arc<dyn OctantCorrectionAdvisor>>,
    pub conflict: Option<Arc<dyn CorpusCallosumResolver>>,
    pub subsystem: Option<Arc<dyn SubsystemAttributionAdvisor>>,
    pub emergent: Option<Arc<dyn EmergentPatternAdvisor>>,
}

impl NeuralAdvisorRegistry {
    /// Конфигурация по умолчанию для V1.
    ///
    /// Активны: ReactivationDepthAdvisor + RuleBasedCorpusCallosumResolver + DepthThresholdEmergentDetector.
    /// Остальные слоты: None.
    pub fn default_v1() -> Self {
        Self {
            depth: Some(Arc::new(ReactivationDepthAdvisor)),
            octant: None,
            conflict: Some(Arc::new(RuleBasedCorpusCallosumResolver)),
            subsystem: None,
            emergent: Some(Arc::new(DepthThresholdEmergentDetector)),
        }
    }

    /// Конфигурация V2: все 5 слотов заполнены.
    ///
    /// depth: ReactivationDepthAdvisor
    /// octant: DepthHistoryBiasAdvisor
    /// conflict: RuleBasedCorpusCallosumResolver
    /// subsystem: AnchorVotingAdvisor
    /// emergent: DepthThresholdEmergentDetector
    pub fn default_v2() -> Self {
        Self {
            depth: Some(Arc::new(ReactivationDepthAdvisor)),
            octant: Some(Arc::new(DepthHistoryBiasAdvisor::default())),
            conflict: Some(Arc::new(RuleBasedCorpusCallosumResolver)),
            subsystem: Some(Arc::new(AnchorVotingAdvisor::default())),
            emergent: Some(Arc::new(DepthThresholdEmergentDetector)),
        }
    }

    /// Конфигурация V3 (G2): conflict slot → PatternLearningResolver.
    ///
    /// PatternLearningResolver учится на AdvisoryHistory per-Frame;
    /// fallback на RuleBasedCorpusCallosumResolver при недостатке данных.
    pub fn default_v3() -> Self {
        Self {
            depth: Some(Arc::new(ReactivationDepthAdvisor)),
            octant: Some(Arc::new(DepthHistoryBiasAdvisor::default())),
            conflict: Some(Arc::new(PatternLearningResolver::new())),
            subsystem: Some(Arc::new(AnchorVotingAdvisor::default())),
            emergent: Some(Arc::new(DepthThresholdEmergentDetector)),
        }
    }

    /// Пустой реестр — все слоты None. Для тестирования и быстрого отключения.
    pub fn empty() -> Self {
        Self {
            depth: None,
            octant: None,
            conflict: None,
            subsystem: None,
            emergent: None,
        }
    }

    /// Реестр с явными null-реализациями во всех слотах.
    ///
    /// Отличие от `empty()`: советники "присутствуют" (логируются, видны геному),
    /// но не дают рекомендаций.
    pub fn all_null() -> Self {
        Self {
            depth: Some(Arc::new(NullDepthAdvisor)),
            octant: Some(Arc::new(NullOctantAdvisor)),
            conflict: Some(Arc::new(NullConflictResolver)),
            subsystem: Some(Arc::new(NullSubsystemAdvisor)),
            emergent: Some(Arc::new(NullEmergentAdvisor)),
        }
    }
}
