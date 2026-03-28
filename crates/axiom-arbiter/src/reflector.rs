// REFLECTOR — статистика рефлексов и профили доменов (Этап 4)
//
// ReflexStats: успех/провал на паттерн (pattern_hash → stats)
// DomainProfile: успех по доменам (role 1..8 → per-layer stats)

use std::collections::HashMap;

/// Статистика отдельного рефлекса
#[derive(Debug, Clone, Default)]
pub struct ReflexStats {
    /// Число успешных совпадений рефлекса с консолидированным результатом
    pub success_count: u32,
    /// Число провалов (рефлекс не совпал)
    pub fail_count: u32,
}

impl ReflexStats {
    /// Общее число попыток
    pub fn total(&self) -> u32 {
        self.success_count + self.fail_count
    }

    /// Доля успехов [0.0 .. 1.0]
    pub fn success_rate(&self) -> f32 {
        if self.total() == 0 {
            return 0.0;
        }
        self.success_count as f32 / self.total() as f32
    }

    /// Обновить счётчик
    pub fn record(&mut self, success: bool) {
        if success {
            self.success_count += 1;
        } else {
            self.fail_count += 1;
        }
    }
}

/// Профиль домена: успешность по семантическим слоям Shell (L1-L8)
///
/// При каждом рефлексе фиксируется, какие слои Shell были активны (> 0)
/// и был ли рефлекс успешным. Это даёт основу для адаптации порогов в Этапе 6.
#[derive(Debug, Clone)]
pub struct DomainProfile {
    /// Число успехов, когда слой i был активен
    pub layer_success: [u32; 8],
    /// Общее число попыток, когда слой i был активен
    pub layer_total: [u32; 8],
}

impl Default for DomainProfile {
    fn default() -> Self {
        Self {
            layer_success: [0; 8],
            layer_total: [0; 8],
        }
    }
}

impl DomainProfile {
    /// Обновить профиль по Shell-профилю токена и результату рефлекса
    pub fn record(&mut self, shell_profile: &[u8; 8], success: bool) {
        for i in 0..8 {
            if shell_profile[i] > 0 {
                self.layer_total[i] += 1;
                if success {
                    self.layer_success[i] += 1;
                }
            }
        }
    }

    /// Успешность для конкретного слоя [0.0 .. 1.0]
    pub fn layer_success_rate(&self, layer: usize) -> f32 {
        if self.layer_total[layer] == 0 {
            return 0.0;
        }
        self.layer_success[layer] as f32 / self.layer_total[layer] as f32
    }

    /// Суммарная активность (число непустых слоёв)
    pub fn active_layers(&self) -> usize {
        self.layer_total.iter().filter(|&&t| t > 0).count()
    }

    /// Общее число попыток по всем слоям
    pub fn total_calls(&self) -> u32 {
        self.layer_total.iter().sum()
    }

    /// Общая доля успехов по всем слоям [0.0 .. 1.0]
    pub fn overall_success_rate(&self) -> f32 {
        let total: u32 = self.layer_total.iter().sum();
        if total == 0 {
            return 0.0;
        }
        let success: u32 = self.layer_success.iter().sum();
        success as f32 / total as f32
    }
}

/// Reflector — накапливает статистику для адаптации системы
///
/// Хранит:
/// - Per-паттерн статистику рефлексов (pattern_hash → ReflexStats)
/// - Per-домен профиль (domain_role 1..8 → DomainProfile)
#[derive(Debug)]
pub struct Reflector {
    /// pattern_hash → статистика рефлекса
    reflex_stats: HashMap<u64, ReflexStats>,
    /// Профили доменов 1..8 (индекс 0 = role 1, индекс 7 = role 8)
    domain_profiles: [DomainProfile; 8],
}

impl Reflector {
    /// Создать новый Reflector
    pub fn new() -> Self {
        Self {
            reflex_stats: HashMap::new(),
            domain_profiles: Default::default(),
        }
    }

    /// Зафиксировать результат рефлекса для паттерна
    pub fn record_reflex(&mut self, pattern_hash: u64, success: bool) {
        self.reflex_stats
            .entry(pattern_hash)
            .or_default()
            .record(success);
    }

    /// Зафиксировать результат в профиль домена (role 1..8)
    ///
    /// `shell_profile`: Shell-профиль входного токена [L1..L8]
    pub fn record_domain(&mut self, role: u8, shell_profile: &[u8; 8], success: bool) {
        if role >= 1 && role <= 8 {
            self.domain_profiles[(role - 1) as usize].record(shell_profile, success);
        }
    }

    /// Получить статистику рефлекса по хэшу паттерна
    pub fn get_stats(&self, pattern_hash: u64) -> Option<&ReflexStats> {
        self.reflex_stats.get(&pattern_hash)
    }

    /// Профиль домена по его роли (1..8)
    pub fn domain_profile(&self, role: u8) -> Option<&DomainProfile> {
        if role >= 1 && role <= 8 {
            Some(&self.domain_profiles[(role - 1) as usize])
        } else {
            None
        }
    }

    /// Количество отслеживаемых паттернов
    pub fn tracked_patterns(&self) -> usize {
        self.reflex_stats.len()
    }

    /// Общая доля успехов по всем паттернам [0.0 .. 1.0]
    pub fn global_success_rate(&self) -> f32 {
        let total: u32 = self.reflex_stats.values().map(|s| s.total()).sum();
        if total == 0 {
            return 0.0;
        }
        let success: u32 = self.reflex_stats.values().map(|s| s.success_count).sum();
        success as f32 / total as f32
    }
}

impl Default for Reflector {
    fn default() -> Self {
        Self::new()
    }
}
