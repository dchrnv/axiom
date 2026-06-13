// layout/mod.rs — Геометрия кристалла якорей.
#![deny(unsafe_code)]

pub mod collision;
pub mod crystal;

use serde::{Deserialize, Serialize};

/// Конфигурация региона кристалла в глобальном пространстве AXIOM.
///
/// Кристалл живёт в ЛОКАЛЬНЫХ координатах (u, v, w) ∈ [0..size].
/// Глобальная позиция = origin + local(u, v, w).
/// origin + size[i] должно оставаться в пределах [0..32767].
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CrystalRegion {
    /// Глобальное начало региона [x, y, z].
    pub origin: [i16; 3],
    /// Размер региона [size_u, size_v, size_w].
    pub size: [u16; 3],
    /// Минимальная дистанция до существующих якорей (коллизия).
    #[serde(default = "default_min_dist")]
    pub min_collision_dist: f32,
}

fn default_min_dist() -> f32 {
    500.0
}

impl CrystalRegion {
    pub fn load(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let region: CrystalRegion = serde_yaml::from_str(&content)?;
        Ok(region)
    }

    /// Проверить что регион не выходит за i16::MAX (32767).
    pub fn validate(&self) -> Result<(), String> {
        for i in 0..3 {
            let max_coord = self.origin[i] as i32 + self.size[i] as i32;
            if max_coord > 32767 {
                return Err(format!(
                    "axis {}: origin[{}]={} + size[{}]={} = {} > 32767",
                    i, i, self.origin[i], i, self.size[i], max_coord
                ));
            }
            if self.origin[i] < 0 {
                return Err(format!("axis {}: origin[{}]={} < 0", i, i, self.origin[i]));
            }
        }
        Ok(())
    }
}

/// Движок раскладки: вычисляет позиции для набора графем в заданном регионе.
///
/// Детерминизм: одинаковый charset + region → одинаковые позиции всегда.
pub trait LayoutEngine {
    fn compute_positions(
        &self,
        graphemes: &[crate::charset::Grapheme],
        region: &CrystalRegion,
    ) -> Vec<[i16; 3]>;
}
