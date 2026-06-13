// layout/crystal.rs — Crystal Layout V1.1.
//
// Геометрия кристалла якорей:
//   d-ось (w): слои абстракции. d=0 = поверхность C0 (графемы). 8 слоёв жёстко.
//   Плоскость слоя: полярный веер.
//     θ = природа знака (секторы с зазорами защищают от хаоса).
//     r = частота: частое в ядре (r≈0), редкое на ободе (r≈r_max).
//
// Детерминизм: одинаковый charset + region → одинаковые позиции всегда.
#![deny(unsafe_code)]

use std::collections::HashMap;
use std::f32::consts::PI;

use crate::charset::{Grapheme, GraphemeClass};
use super::{CrystalRegion, LayoutEngine};

pub struct CrystalLayout;

/// Сектор класса в полярном веере: [start_deg, end_deg) в градусах [0..360).
///
/// Зазоры между секторами: цифры и буквы разведены геометрически.
fn sector_range(class: &GraphemeClass) -> (f32, f32) {
    use GraphemeClass::*;
    match class {
        // Крыло гласных (0°–120°)
        VowelCyr            => (  0.0,  55.0),
        VowelLat            => ( 65.0, 120.0),  // зазор 55°–65°

        // Крыло согласных (130°–250°)
        ConsonantCyr        => (130.0, 185.0),  // зазор 120°–130°
        ConsonantLat        => (195.0, 250.0),  // зазор 185°–195°

        // Цифры (265°–295°)
        Digit               => (265.0, 295.0),  // зазор 250°–265°

        // Пунктуация письма (305°–344°)
        Period | Comma | Semicolon | Colon
                            => (305.0, 320.0),  // зазор 295°–305°
        Exclaim | Question  => (320.0, 330.0),
        Dash                => (330.0, 337.0),
        Quote               => (337.0, 341.0),
        Ellipsis            => (341.0, 344.0),
        BracketOpen | BracketClose
                            => (344.0, 349.0),

        // Математические операторы (349°–358°)
        OpArith             => (349.0, 353.0),  // зазор 344°–349° — уже есть
        OpCompare           => (353.0, 356.0),
        OpMisc | Amp        => (356.0, 359.0),

        // Прочие (358°–360°)
        Space | Underscore | At | Hash | Dollar | SlashBack
                            => (359.0, 360.0),
    }
}

impl LayoutEngine for CrystalLayout {
    /// Вычислить позиции графем в кристалле.
    ///
    /// Слой C0 (layer_index=0) размещается на поверхности (d≈0).
    /// В плоскости слоя: θ из класса + порядкового номера внутри класса,
    /// r из глобального ранга частоты.
    fn compute_positions(
        &self,
        graphemes: &[Grapheme],
        region: &CrystalRegion,
    ) -> Vec<[i16; 3]> {
        let size_u = region.size[0] as f32;
        let size_v = region.size[1] as f32;
        let size_w = region.size[2] as f32;

        // Полярный центр в плоскости u-v
        let center_u = size_u / 2.0;
        let center_v = size_v / 2.0;
        // Максимальный радиус с отступом от края
        let r_max = center_u.min(center_v) * 0.85;

        // C0 — поверхность: середина первого слоя (d=0..size_w/8)
        let layer_thickness = size_w / 8.0;
        let w_c0 = layer_thickness / 2.0;

        // Нормализуем r по максимальному рангу в датасете, а не по размеру массива.
        // Это корректно: rank — глобальный (0=частейшее), max_rank — наредчайшее.
        let max_rank = graphemes.iter().map(|g| g.rank).max().unwrap_or(1) as f32;

        // Подсчёт числа графем в каждом секторе для равномерного распределения θ
        let mut class_count: HashMap<&GraphemeClass, usize> = HashMap::new();
        for g in graphemes {
            *class_count.entry(&g.class).or_insert(0) += 1;
        }

        // Индексы внутри класса (для равномерного θ в секторе)
        let mut class_idx: HashMap<String, usize> = HashMap::new();

        graphemes
            .iter()
            .map(|g| {
                let key = format!("{:?}", g.class);
                let idx = *class_idx.get(&key).unwrap_or(&0);
                *class_idx.entry(key.clone()).or_insert(0) = idx + 1;

                let count = *class_count.get(&g.class).unwrap_or(&1);
                let (deg_start, deg_end) = sector_range(&g.class);

                // θ: равномерно в пределах сектора по индексу внутри класса
                // count=1 → θ = середина сектора
                let frac = if count > 1 {
                    idx as f32 / (count - 1) as f32
                } else {
                    0.5
                };
                let theta_deg = deg_start + frac * (deg_end - deg_start);
                let theta = theta_deg * PI / 180.0;

                // r: глобальный ранг → расстояние от центра (частое = ближе к нулю)
                let r = r_max * (g.rank as f32 / max_rank.max(1.0));

                // Локальные координаты в плоскости
                let u_local = center_u + r * theta.cos();
                let v_local = center_v + r * theta.sin();

                // Глобальные координаты
                let x = (region.origin[0] as f32 + u_local).round() as i16;
                let y = (region.origin[1] as f32 + v_local).round() as i16;
                let z = (region.origin[2] as f32 + w_c0).round() as i16;

                [x, y, z]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::charset::GraphemeClass;
    use crate::layout::{CrystalRegion, LayoutEngine};

    fn make_grapheme(ch: &str, rank: u32, class: GraphemeClass) -> Grapheme {
        Grapheme { ch: ch.to_string(), rank, class, subsystem: "writing".into() }
    }

    fn region() -> CrystalRegion {
        CrystalRegion {
            origin: [26500, 26500, 26500],
            size: [4000, 4000, 1600],
            min_collision_dist: 500.0,
        }
    }

    #[test]
    fn test_determinism() {
        let graphemes = vec![
            make_grapheme("о", 0, GraphemeClass::VowelCyr),
            make_grapheme("e", 1, GraphemeClass::VowelLat),
            make_grapheme("1", 2, GraphemeClass::Digit),
        ];
        let layout = CrystalLayout;
        let p1 = layout.compute_positions(&graphemes, &region());
        let p2 = layout.compute_positions(&graphemes, &region());
        assert_eq!(p1, p2, "layout должен быть детерминированным");
    }

    #[test]
    fn test_positions_in_region() {
        let r = region();
        let graphemes = vec![
            make_grapheme("о", 0, GraphemeClass::VowelCyr),
            make_grapheme("1", 59, GraphemeClass::Digit),
        ];
        let layout = CrystalLayout;
        let positions = layout.compute_positions(&graphemes, &r);
        for pos in &positions {
            for (i, &coord) in pos.iter().enumerate() {
                assert!(
                    coord >= r.origin[i] && coord <= r.origin[i] + r.size[i] as i16,
                    "coord[{}]={} выходит за регион [{}, {}]",
                    i, coord, r.origin[i], r.origin[i] + r.size[i] as i16
                );
            }
        }
    }

    #[test]
    fn test_c0_surface_layer() {
        let r = region();
        let graphemes = vec![make_grapheme("о", 0, GraphemeClass::VowelCyr)];
        let layout = CrystalLayout;
        let positions = layout.compute_positions(&graphemes, &r);
        let layer_thickness = r.size[2] as f32 / 8.0;
        let z_local = positions[0][2] - r.origin[2];
        // C0 в первом слое (0..layer_thickness)
        assert!(
            z_local >= 0 && (z_local as f32) < layer_thickness,
            "C0 должен быть на поверхности (z_local={} < layer_thickness={})",
            z_local, layer_thickness
        );
    }

    #[test]
    fn test_vowels_consonants_different_sectors() {
        // Используем rank=4 ("а") и rank=8 ("н") — оба ненулевые, чтобы r > 0.
        // rank=0 кладёт графему в центр (r=0), у центра нет сектора.
        let graphemes = vec![
            make_grapheme("а", 4, GraphemeClass::VowelCyr),
            make_grapheme("н", 8, GraphemeClass::ConsonantCyr),
        ];
        let layout = CrystalLayout;
        let positions = layout.compute_positions(&graphemes, &region());
        let r = region();
        let cx = r.origin[0] as f32 + r.size[0] as f32 / 2.0;
        let cy = r.origin[1] as f32 + r.size[1] as f32 / 2.0;
        // VowelCyr в [0°,55°] → cos > 0 → u > 0
        // ConsonantCyr в [130°,185°] → cos < 0 → u < 0
        let vow_x_local = positions[0][0] as f32 - cx;
        let cons_x_local = positions[1][0] as f32 - cx;
        assert!(vow_x_local > 0.0, "vowel_cyr должна быть справа от центра (u>0), получено {vow_x_local}");
        assert!(cons_x_local < 0.0, "consonant_cyr должна быть слева от центра (u<0), получено {cons_x_local}");
    }

    #[test]
    fn test_frequent_closer_to_center() {
        let r = region();
        let graphemes = vec![
            make_grapheme("о", 0, GraphemeClass::VowelCyr),   // rank=0 → в ядре
            make_grapheme("е", 2, GraphemeClass::VowelCyr),   // rank=2 → дальше
        ];
        let layout = CrystalLayout;
        let positions = layout.compute_positions(&graphemes, &r);
        let cx = r.origin[0] as f32 + r.size[0] as f32 / 2.0;
        let cy = r.origin[1] as f32 + r.size[1] as f32 / 2.0;
        let dist = |p: [i16; 3]| {
            let dx = p[0] as f32 - cx;
            let dy = p[1] as f32 - cy;
            (dx * dx + dy * dy).sqrt()
        };
        assert!(
            dist(positions[0]) < dist(positions[1]),
            "rank=0 должен быть ближе к центру чем rank=2"
        );
    }

    #[test]
    fn test_digits_not_in_vowel_sector() {
        let r = region();
        let graphemes = vec![
            make_grapheme("о", 0, GraphemeClass::VowelCyr),
            make_grapheme("1", 59, GraphemeClass::Digit),
        ];
        let layout = CrystalLayout;
        let positions = layout.compute_positions(&graphemes, &r);
        // Гласные в [0°,55°], цифры в [265°,295°] — должны быть в разных секторах
        let cx = r.origin[0] as f32 + r.size[0] as f32 / 2.0;
        let cy = r.origin[1] as f32 + r.size[1] as f32 / 2.0;
        let angle = |p: [i16; 3]| -> f32 {
            let dx = p[0] as f32 - cx;
            let dy = p[1] as f32 - cy;
            let a = dy.atan2(dx).to_degrees();
            if a < 0.0 { a + 360.0 } else { a }
        };
        let vowel_angle = angle(positions[0]);
        let digit_angle = angle(positions[1]);
        // Зазор 250°–265° — должны быть по разные стороны
        assert!(
            (vowel_angle - digit_angle).abs() > 50.0,
            "гласные (θ={:.0}°) и цифры (θ={:.0}°) должны быть в разных секторах",
            vowel_angle, digit_angle
        );
    }
}
