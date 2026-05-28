// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// L0VisionPerceptor (V7-E2) — изображение → L0 visual примитивы → SUTRA.
//
// Минимальный pipeline: Sobel edge detection → stroke classification.
//
// Результат: один InjectToken в SUTRA (domain 100) для каждого L0 примитива,
// чья обнаруженная плотность превышает порог.

use axiom_config::Anchor;
use axiom_ucl::{OpCode, UclCommand};

/// SUTRA domain на уровне 1.
pub const L0_SUTRA_DOMAIN_ID: u16 = 100;

/// Минимальная доля edge-пикселей от общего числа пикселей, чтобы засчитать примитив.
pub const EDGE_DETECTION_THRESHOLD: f32 = 0.02;

/// Минимальная амплитуда Sobel-градиента для считывания пикселя как edge.
pub const SOBEL_MAG_THRESHOLD: f32 = 30.0;

/// Angle bins для классификации штрихов (от угла Sobel-градиента):
///   vertical edge:   |angle| < 22.5° или > 157.5°  →  visual_stroke_vertical
///   horizontal edge: 67.5° < |angle| < 112.5°       →  visual_stroke_horizontal
///   diagonal edge:   остальное                       →  visual_stroke_diagonal
const HORIZ_ANGLE_MIN: f32 = 67.5;
const HORIZ_ANGLE_MAX: f32 = 112.5;
const VERT_ANGLE_MAX: f32 = 22.5;

/// Категории L0-примитивов, детектируемые в V7-E2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum L0VisualPrimitive {
    /// Любой детектируемый край (суммарная edge-плотность).
    Edge,
    /// Горизонтальный штрих (края перпендикулярные горизонту).
    StrokeHorizontal,
    /// Вертикальный штрих (края перпендикулярные вертикали).
    StrokeVertical,
    /// Диагональный штрих (~45°).
    StrokeDiagonal,
}

impl L0VisualPrimitive {
    /// Слово-якорь, которое совпадает с `id` в visual_primitives.yaml.
    pub fn anchor_id(self) -> &'static str {
        match self {
            L0VisualPrimitive::Edge             => "visual_edge",
            L0VisualPrimitive::StrokeHorizontal => "visual_stroke_horizontal",
            L0VisualPrimitive::StrokeVertical   => "visual_stroke_vertical",
            L0VisualPrimitive::StrokeDiagonal   => "visual_stroke_diagonal",
        }
    }
}

/// Результат анализа изображения.
#[derive(Debug, Clone)]
pub struct EdgeAnalysis {
    /// Доля edge-пикселей от всех пикселей (0..1).
    pub edge_density: f32,
    /// Доля горизонтальных штрихов среди edge-пикселей.
    pub horizontal_fraction: f32,
    /// Доля вертикальных штрихов среди edge-пикселей.
    pub vertical_fraction: f32,
    /// Доля диагональных штрихов среди edge-пикселей.
    pub diagonal_fraction: f32,
}

/// Масса токена для L0 визуального примитива (фиксированная, из visual_primitives.yaml).
pub const L0_VISUAL_TOKEN_MASS: f32 = 130.0;
/// Температура токена для L0 визуального примитива (стабильный примитив).
pub const L0_VISUAL_TOKEN_TEMPERATURE: f32 = 4.0;

/// L0VisionPerceptor — детектирует L0 визуальные примитивы в изображении.
///
/// Принимает перцептивные якоря (`perceptual_anchors()` из AnchorSet).
/// Для тестов — вставьте предвычисленный `EdgeAnalysis` через `feed_analysis`.
pub struct L0VisionPerceptor {
    anchors: Vec<Anchor>,
    pending: std::collections::VecDeque<UclCommand>,
}

impl L0VisionPerceptor {
    /// Создать перцептор с L0 визуальными якорями.
    pub fn new(perceptual_anchors: &[Anchor]) -> Self {
        let visual: Vec<Anchor> = perceptual_anchors
            .iter()
            .filter(|a| {
                a.id.starts_with("visual_")
            })
            .cloned()
            .collect();
        Self {
            anchors: visual,
            pending: std::collections::VecDeque::new(),
        }
    }

    /// Обработать изображение: RGBA8 пиксели → edge detection → inject L0 токены.
    ///
    /// `width` × `height` × 4 = `pixels.len()`.
    pub fn process_image_rgba(&mut self, pixels: &[u8], width: u32, height: u32) {
        let gray = to_grayscale(pixels, width, height);
        let analysis = sobel_analysis(&gray, width, height);
        self.feed_analysis(&analysis);
    }

    /// Прямая инжекция предвычисленного анализа (для тестов).
    pub fn feed_analysis(&mut self, analysis: &EdgeAnalysis) {
        self.maybe_inject(L0VisualPrimitive::Edge, analysis.edge_density);
        self.maybe_inject(
            L0VisualPrimitive::StrokeHorizontal,
            analysis.horizontal_fraction * analysis.edge_density,
        );
        self.maybe_inject(
            L0VisualPrimitive::StrokeVertical,
            analysis.vertical_fraction * analysis.edge_density,
        );
        self.maybe_inject(
            L0VisualPrimitive::StrokeDiagonal,
            analysis.diagonal_fraction * analysis.edge_density,
        );
    }

    fn maybe_inject(&mut self, primitive: L0VisualPrimitive, strength: f32) {
        if strength < EDGE_DETECTION_THRESHOLD {
            return;
        }
        if let Some(anchor) = self.find_anchor(primitive.anchor_id()) {
            let cmd = anchor_to_ucl_command(anchor, strength.min(1.0));
            self.pending.push_back(cmd);
        }
    }

    fn find_anchor(&self, id: &str) -> Option<&Anchor> {
        self.anchors.iter().find(|a| a.id == id)
    }

    /// Число команд в очереди.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    /// Взять следующую команду из очереди.
    pub fn receive(&mut self) -> Option<UclCommand> {
        self.pending.pop_front()
    }
}

/// Преобразовать RGBA8 в одноканальный grayscale (luminance).
fn to_grayscale(rgba: &[u8], width: u32, height: u32) -> Vec<u8> {
    let n = (width * height) as usize;
    let mut gray = Vec::with_capacity(n);
    for i in 0..n {
        let base = i * 4;
        if base + 2 < rgba.len() {
            let r = rgba[base] as u32;
            let g = rgba[base + 1] as u32;
            let b = rgba[base + 2] as u32;
            // BT.601 luminance
            let lum = (r * 77 + g * 150 + b * 29) >> 8;
            gray.push(lum as u8);
        } else {
            gray.push(0);
        }
    }
    gray
}

/// Применить Sobel operator к grayscale-изображению.
/// Возвращает `EdgeAnalysis` с долями примитивов.
fn sobel_analysis(gray: &[u8], width: u32, height: u32) -> EdgeAnalysis {
    if width < 3 || height < 3 {
        return EdgeAnalysis {
            edge_density: 0.0,
            horizontal_fraction: 0.0,
            vertical_fraction: 0.0,
            diagonal_fraction: 0.0,
        };
    }
    let w = width as usize;
    let h = height as usize;

    let mut edge_count = 0u32;
    let mut horiz_count = 0u32;
    let mut vert_count = 0u32;
    let mut diag_count = 0u32;

    // Sobel kernels: iterate over interior pixels (skip 1-pixel border)
    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            let p = |dy: i32, dx: i32| -> i32 {
                let ny = (y as i32 + dy) as usize;
                let nx = (x as i32 + dx) as usize;
                gray[ny * w + nx] as i32
            };

            // Sobel Gx (horizontal gradient → detects vertical edges)
            let gx = -p(-1, -1) + p(-1, 1) - 2 * p(0, -1) + 2 * p(0, 1) - p(1, -1) + p(1, 1);
            // Sobel Gy (vertical gradient → detects horizontal edges)
            let gy = -p(-1, -1) - 2 * p(-1, 0) - p(-1, 1) + p(1, -1) + 2 * p(1, 0) + p(1, 1);

            let mag = ((gx * gx + gy * gy) as f32).sqrt();
            if mag < SOBEL_MAG_THRESHOLD {
                continue;
            }
            edge_count += 1;

            // Gradient angle determines edge orientation
            let angle_deg = (gy as f32).atan2(gx as f32).to_degrees().abs();

            if angle_deg < VERT_ANGLE_MAX || angle_deg > (180.0 - VERT_ANGLE_MAX) {
                vert_count += 1;
            } else if angle_deg >= HORIZ_ANGLE_MIN && angle_deg <= HORIZ_ANGLE_MAX {
                horiz_count += 1;
            } else {
                diag_count += 1;
            }
        }
    }

    let total = ((w - 2) * (h - 2)) as f32;
    let edge_density = edge_count as f32 / total;

    let (horiz_frac, vert_frac, diag_frac) = if edge_count > 0 {
        let ec = edge_count as f32;
        (horiz_count as f32 / ec, vert_count as f32 / ec, diag_count as f32 / ec)
    } else {
        (0.0, 0.0, 0.0)
    };

    EdgeAnalysis {
        edge_density,
        horizontal_fraction: horiz_frac,
        vertical_fraction: vert_frac,
        diagonal_fraction: diag_frac,
    }
}

/// Преобразовать L0 якорь в InjectToken команду для SUTRA.
/// `semantic_weight` = обнаруженная сила примитива (0..1).
fn anchor_to_ucl_command(anchor: &Anchor, semantic_weight: f32) -> UclCommand {
    let mut cmd = UclCommand::new(OpCode::InjectToken, L0_SUTRA_DOMAIN_ID as u32, 100, 0);
    let domain_id_bytes = L0_SUTRA_DOMAIN_ID.to_le_bytes();
    cmd.payload[0..2].copy_from_slice(&domain_id_bytes);
    cmd.payload[2] = 0; // token_type = generic
    // [3] padding
    cmd.payload[4..8].copy_from_slice(&L0_VISUAL_TOKEN_MASS.to_le_bytes());
    let px = anchor.position[0] as f32;
    let py = anchor.position[1] as f32;
    let pz = anchor.position[2] as f32;
    cmd.payload[8..12].copy_from_slice(&px.to_le_bytes());
    cmd.payload[12..16].copy_from_slice(&py.to_le_bytes());
    cmd.payload[16..20].copy_from_slice(&pz.to_le_bytes());
    // velocity = 0
    cmd.payload[20..32].fill(0);
    cmd.payload[32..36].copy_from_slice(&semantic_weight.to_le_bytes());
    cmd.payload[36..40].copy_from_slice(&L0_VISUAL_TOKEN_TEMPERATURE.to_le_bytes());
    cmd
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_config::{Anchor, AnchorLayer};

    fn make_visual_anchor(id: &str, pos: [i16; 3]) -> Anchor {
        Anchor {
            id: id.to_string(),
            word: id.to_string(),
            layer: AnchorLayer::L0,
            position: pos,
            shell: [200, 80, 0, 0, 0, 0, 0, 0],
            description: String::new(),
            tags: vec![],
            aliases: vec![],
        }
    }

    fn make_anchors() -> Vec<Anchor> {
        vec![
            make_visual_anchor("visual_edge",             [-9500, -4500, -800]),
            make_visual_anchor("visual_stroke_horizontal",[-9000, -4000, -500]),
            make_visual_anchor("visual_stroke_vertical",  [-8500, -4000,  500]),
            make_visual_anchor("visual_stroke_diagonal",  [-8000, -4000,   0]),
        ]
    }

    #[test]
    fn test_new_filters_non_visual_anchors() {
        let anchors = vec![
            make_visual_anchor("visual_edge", [-9500, -4500, -800]),
            make_visual_anchor("spatial_above", [0, 0, 0_i16]),
        ];
        let vp = L0VisionPerceptor::new(&anchors);
        // only visual_ prefixed anchors kept
        assert_eq!(vp.anchors.len(), 1);
        assert_eq!(vp.anchors[0].id, "visual_edge");
    }

    #[test]
    fn test_no_tokens_below_threshold() {
        let mut vp = L0VisionPerceptor::new(&make_anchors());
        vp.feed_analysis(&EdgeAnalysis {
            edge_density: 0.005, // below EDGE_DETECTION_THRESHOLD=0.02
            horizontal_fraction: 0.5,
            vertical_fraction: 0.3,
            diagonal_fraction: 0.2,
        });
        assert_eq!(vp.pending_count(), 0);
    }

    #[test]
    fn test_edge_token_injected_above_threshold() {
        let mut vp = L0VisionPerceptor::new(&make_anchors());
        vp.feed_analysis(&EdgeAnalysis {
            edge_density: 0.1,
            horizontal_fraction: 0.0,
            vertical_fraction: 0.0,
            diagonal_fraction: 0.0,
        });
        // Only visual_edge should fire (others: 0.0 * 0.1 = 0.0 < threshold)
        assert_eq!(vp.pending_count(), 1);
        let cmd = vp.receive().unwrap();
        assert_eq!(cmd.opcode, 2000); // InjectToken
    }

    #[test]
    fn test_all_stroke_tokens_injected() {
        let mut vp = L0VisionPerceptor::new(&make_anchors());
        vp.feed_analysis(&EdgeAnalysis {
            edge_density: 0.4,
            horizontal_fraction: 0.4,
            vertical_fraction: 0.3,
            diagonal_fraction: 0.3,
        });
        // visual_edge: 0.4 >= 0.02 ✓
        // horizontal: 0.4 * 0.4 = 0.16 >= 0.02 ✓
        // vertical:   0.4 * 0.3 = 0.12 >= 0.02 ✓
        // diagonal:   0.4 * 0.3 = 0.12 >= 0.02 ✓
        assert_eq!(vp.pending_count(), 4);
    }

    #[test]
    fn test_token_uses_anchor_position() {
        let anchors = vec![make_visual_anchor("visual_edge", [-9500, -4500, -800])];
        let mut vp = L0VisionPerceptor::new(&anchors);
        vp.feed_analysis(&EdgeAnalysis {
            edge_density: 0.1,
            horizontal_fraction: 0.0,
            vertical_fraction: 0.0,
            diagonal_fraction: 0.0,
        });
        let cmd = vp.receive().unwrap();
        let x = f32::from_le_bytes(cmd.payload[8..12].try_into().unwrap());
        let y = f32::from_le_bytes(cmd.payload[12..16].try_into().unwrap());
        let z = f32::from_le_bytes(cmd.payload[16..20].try_into().unwrap());
        assert!((x - (-9500.0_f32)).abs() < 1.0);
        assert!((y - (-4500.0_f32)).abs() < 1.0);
        assert!((z - (-800.0_f32)).abs() < 1.0);
    }

    #[test]
    fn test_sobel_flat_image_no_edges() {
        // 10×10 uniform gray
        let pixels: Vec<u8> = (0..10 * 10 * 4).map(|i| if i % 4 < 3 { 128 } else { 255 }).collect();
        let analysis = {
            let gray = to_grayscale(&pixels, 10, 10);
            sobel_analysis(&gray, 10, 10)
        };
        assert!(analysis.edge_density < EDGE_DETECTION_THRESHOLD,
            "flat image should have no edges: {}", analysis.edge_density);
    }

    #[test]
    fn test_sobel_vertical_edge_detected() {
        // 20×10: left half black, right half white → strong vertical edge in middle
        let mut pixels = vec![0u8; 20 * 10 * 4];
        for y in 0..10_usize {
            for x in 10..20_usize {
                let base = (y * 20 + x) * 4;
                pixels[base] = 255;
                pixels[base + 1] = 255;
                pixels[base + 2] = 255;
                pixels[base + 3] = 255;
            }
        }
        let gray = to_grayscale(&pixels, 20, 10);
        let analysis = sobel_analysis(&gray, 20, 10);
        assert!(analysis.edge_density >= EDGE_DETECTION_THRESHOLD,
            "vertical edge image should have edges: {}", analysis.edge_density);
        assert!(analysis.vertical_fraction > analysis.horizontal_fraction,
            "vertical fraction should dominate: vert={} horiz={}", analysis.vertical_fraction, analysis.horizontal_fraction);
    }

    #[test]
    fn test_sobel_horizontal_edge_detected() {
        // 10×20: top half black, bottom half white → horizontal edge
        let mut pixels = vec![0u8; 10 * 20 * 4];
        for y in 10..20_usize {
            for x in 0..10_usize {
                let base = (y * 10 + x) * 4;
                pixels[base] = 255;
                pixels[base + 1] = 255;
                pixels[base + 2] = 255;
                pixels[base + 3] = 255;
            }
        }
        let gray = to_grayscale(&pixels, 10, 20);
        let analysis = sobel_analysis(&gray, 10, 20);
        assert!(analysis.edge_density >= EDGE_DETECTION_THRESHOLD,
            "horizontal edge image should have edges: {}", analysis.edge_density);
        assert!(analysis.horizontal_fraction > analysis.vertical_fraction,
            "horizontal fraction should dominate: horiz={} vert={}", analysis.horizontal_fraction, analysis.vertical_fraction);
    }

    #[test]
    fn test_process_image_rgba_flat() {
        // Flat image → no tokens
        let pixels: Vec<u8> = vec![128u8; 16 * 16 * 4];
        let mut vp = L0VisionPerceptor::new(&make_anchors());
        vp.process_image_rgba(&pixels, 16, 16);
        assert_eq!(vp.pending_count(), 0, "flat image → no L0 tokens");
    }

    #[test]
    fn test_process_image_rgba_with_edges() {
        // Sharp horizontal edge image → tokens produced
        let mut pixels = vec![0u8; 32 * 32 * 4];
        for y in 16..32_usize {
            for x in 0..32_usize {
                let base = (y * 32 + x) * 4;
                pixels[base] = 255;
                pixels[base + 1] = 255;
                pixels[base + 2] = 255;
                pixels[base + 3] = 255;
            }
        }
        let mut vp = L0VisionPerceptor::new(&make_anchors());
        vp.process_image_rgba(&pixels, 32, 32);
        assert!(vp.pending_count() > 0, "edge image → at least one L0 token");
    }
}
