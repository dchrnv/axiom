// layout/collision.rs — Проверка коллизий кристалла с существующими якорями.
#![deny(unsafe_code)]

/// Проверяет что новые позиции кристалла не пересекаются с существующими якорями.
pub struct CollisionChecker {
    /// Позиции существующих якорей.
    pub existing: Vec<[i16; 3]>,
    /// Минимально допустимая дистанция (евклидова).
    pub min_distance: f32,
}

/// Результат коллизии: позиция нового якоря и нарушение.
#[derive(Debug)]
pub struct CollisionViolation {
    pub new_pos: [i16; 3],
    pub existing_pos: [i16; 3],
    pub distance: f32,
}

impl CollisionChecker {
    /// Проверить одну позицию. Возвращает ближайший нарушитель если есть.
    pub fn check_one(&self, pos: [i16; 3]) -> Option<CollisionViolation> {
        let mut min_dist = f32::MAX;
        let mut closest = [0i16; 3];
        for &ep in &self.existing {
            let d = dist3(pos, ep);
            if d < min_dist {
                min_dist = d;
                closest = ep;
            }
        }
        if min_dist < self.min_distance {
            Some(CollisionViolation {
                new_pos: pos,
                existing_pos: closest,
                distance: min_dist,
            })
        } else {
            None
        }
    }

    /// Проверить все позиции. Возвращает вектор нарушений (пустой = всё чисто).
    pub fn check_all(&self, positions: &[[i16; 3]]) -> Vec<(usize, CollisionViolation)> {
        positions
            .iter()
            .enumerate()
            .filter_map(|(i, &pos)| self.check_one(pos).map(|v| (i, v)))
            .collect()
    }
}

fn dist3(a: [i16; 3], b: [i16; 3]) -> f32 {
    let dx = (a[0] - b[0]) as f32;
    let dy = (a[1] - b[1]) as f32;
    let dz = (a[2] - b[2]) as f32;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_collision_empty_existing() {
        let c = CollisionChecker { existing: vec![], min_distance: 500.0 };
        assert!(c.check_one([100, 200, 300]).is_none());
    }

    #[test]
    fn collision_detected() {
        let c = CollisionChecker {
            existing: vec![[100, 200, 300]],
            min_distance: 500.0,
        };
        // Та же позиция → distance=0 < 500
        assert!(c.check_one([100, 200, 300]).is_some());
    }

    #[test]
    fn no_collision_far() {
        let c = CollisionChecker {
            existing: vec![[0, 0, 0]],
            min_distance: 500.0,
        };
        // Дистанция = sqrt(600²+600²+600²) ≈ 1039 > 500
        assert!(c.check_one([600, 600, 600]).is_none());
    }

    #[test]
    fn check_all_returns_violations() {
        let c = CollisionChecker {
            existing: vec![[1000, 1000, 1000]],
            min_distance: 500.0,
        };
        let positions = vec![[1000, 1000, 1000], [5000, 5000, 5000]];
        let violations = c.check_all(&positions);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].0, 0);
    }
}
