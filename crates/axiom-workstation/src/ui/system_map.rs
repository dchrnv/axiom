use std::cell::Cell;
use std::f32::consts::PI;
use std::time::{Duration, Instant};

use iced::alignment;
use iced::mouse;
use iced::widget::canvas::path::Arc;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{Color, Element, Length, Point, Radians, Rectangle, Size};

use axiom_protocol::events::EngineState;
use axiom_protocol::snapshot::SystemSnapshot;

use crate::app::Message;

const BG: Color = Color {
    r: 0.05,
    g: 0.05,
    b: 0.08,
    a: 1.0,
};
const CORE_COLOR: Color = Color {
    r: 0.12,
    g: 0.12,
    b: 0.16,
    a: 1.0,
};
const DOMAIN_IDLE: Color = Color {
    r: 0.3,
    g: 0.3,
    b: 0.38,
    a: 1.0,
};
const LABEL_COLOR: Color = Color {
    r: 0.75,
    g: 0.75,
    b: 0.8,
    a: 1.0,
};
const ALERT_COLOR: Color = Color {
    r: 0.9,
    g: 0.25,
    b: 0.25,
    a: 0.7,
};

/// ASHTI domain considered active if it has at least this many tokens.
const SECTOR_TOKEN_THRESHOLD: u32 = 3;
/// Flow line stays bright for this duration after last DomainActivity event.
const FLOW_ACTIVE_DURATION: Duration = Duration::from_millis(500);

pub fn system_map_view<'a>(
    snapshot: &'a Option<SystemSnapshot>,
    phase: f32,
    last_domain_active: &'a [Option<Instant>; 8],
) -> Element<'a, Message> {
    Canvas::new(SystemMapCanvas {
        snapshot,
        phase,
        last_domain_active,
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

struct SystemMapCanvas<'a> {
    snapshot: &'a Option<SystemSnapshot>,
    phase: f32,
    last_domain_active: &'a [Option<Instant>; 8],
}

pub struct SystemMapState {
    static_cache: canvas::Cache,
    last_tick: Cell<u64>,
}

impl Default for SystemMapState {
    fn default() -> Self {
        Self {
            static_cache: canvas::Cache::default(),
            last_tick: Cell::new(0),
        }
    }
}

impl<'a> canvas::Program<Message> for SystemMapCanvas<'a> {
    type State = SystemMapState;

    fn draw(
        &self,
        state: &SystemMapState,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let current_tick = self.snapshot.as_ref().map(|s| s.current_tick).unwrap_or(0);
        if current_tick != state.last_tick.get() {
            state.static_cache.clear();
            state.last_tick.set(current_tick);
        }

        let min_dim = bounds.width.min(bounds.height);
        let mandala_r = min_dim * 0.18;
        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);

        // ── Static layer: background + domain labels + bottom labels ──────────
        let static_geo = state.static_cache.draw(renderer, bounds.size(), |frame| {
            frame.fill_rectangle(Point::ORIGIN, Size::new(bounds.width, bounds.height), BG);
            if let Some(snap) = self.snapshot {
                draw_domain_labels(frame, center, mandala_r * 2.8, snap);
                draw_bottom_labels(frame, snap, bounds);
            }
        });

        // ── Dynamic layer: mandala pulse + sector fills + dots + flow lines ───
        let mut dyn_frame = Frame::new(renderer, bounds.size());

        let now = Instant::now();
        let flow_active: [bool; 8] = std::array::from_fn(|i| {
            self.last_domain_active[i]
                .map_or(false, |t| now.duration_since(t) < FLOW_ACTIVE_DURATION)
        });

        match self.snapshot {
            Some(snap) => {
                let state_color = engine_state_color(snap.engine_state);
                let has_vetoes = snap.guardian_stats.total_vetoes > 0;

                draw_sector_fills(&mut dyn_frame, center, mandala_r, snap, state_color, self.phase);
                draw_mandala(&mut dyn_frame, center, mandala_r, state_color, self.phase, has_vetoes);
                draw_domain_nodes(&mut dyn_frame, center, mandala_r * 2.8, snap, state_color, &flow_active);
            }
            None => {
                draw_loading(&mut dyn_frame, center, mandala_r, self.phase);
            }
        }

        vec![static_geo, dyn_frame.into_geometry()]
    }
}

fn engine_state_color(state: EngineState) -> Color {
    match state {
        EngineState::Wake => Color::from_rgb(0.4, 0.65, 0.9),
        EngineState::FallingAsleep => Color::from_rgb(0.5, 0.35, 0.75),
        EngineState::Dreaming => Color::from_rgb(0.45, 0.25, 0.7),
        EngineState::Waking => Color::from_rgb(0.45, 0.55, 0.85),
    }
}

/// Draw filled pie slices for ASHTI domains (101–108) with tokens above threshold.
/// Slices go from center to outer mandala ring; the core circle is drawn on top.
fn draw_sector_fills(
    frame: &mut Frame,
    center: Point,
    r: f32,
    snap: &SystemSnapshot,
    state_color: Color,
    phase: f32,
) {
    let pulse = 1.0 + 0.04 * (phase * 2.0 * PI).sin();
    let outer_r = r * 0.68 * pulse;
    let sector_span = 2.0 * PI / 8.0;

    for i in 0usize..8 {
        let domain_id = 101 + i as u16;
        let token_count = snap
            .domains
            .iter()
            .find(|d| d.id == domain_id)
            .map_or(0, |d| d.token_count);

        if token_count < SECTOR_TOKEN_THRESHOLD {
            continue;
        }

        let angle_start = (i as f32 / 8.0) * 2.0 * PI - PI / 2.0;
        let angle_end = angle_start + sector_span;

        let slice = Path::new(|b| {
            b.move_to(center);
            b.arc(Arc {
                center,
                radius: outer_r,
                start_angle: Radians(angle_start),
                end_angle: Radians(angle_end),
            });
            b.close();
        });

        // Intensity scales with token count (capped)
        let intensity = (token_count as f32 / 30.0).min(1.0);
        frame.fill(
            &slice,
            Color {
                a: 0.06 + 0.1 * intensity,
                ..state_color
            },
        );
    }
}

fn draw_mandala(
    frame: &mut Frame,
    center: Point,
    r: f32,
    color: Color,
    phase: f32,
    has_vetoes: bool,
) {
    let pulse = 1.0 + 0.04 * (phase * 2.0 * PI).sin();

    // Alert ring — drawn outside the outer mandala ring when Guardian vetoes exist
    if has_vetoes {
        let alert_ring = Path::circle(center, r * pulse + 5.0);
        frame.stroke(
            &alert_ring,
            Stroke::default()
                .with_color(ALERT_COLOR)
                .with_width(1.5),
        );
    }

    // Outer ring
    let outer = Path::circle(center, r * pulse);
    frame.stroke(
        &outer,
        Stroke::default()
            .with_color(Color { a: 0.35, ..color })
            .with_width(2.0),
    );

    // Middle ring
    let mid = Path::circle(center, r * 0.68 * pulse);
    frame.stroke(
        &mid,
        Stroke::default()
            .with_color(Color { a: 0.2, ..color })
            .with_width(1.5),
    );

    // Sector lines (8 ASHTI boundaries)
    for i in 0..8 {
        let angle = (i as f32 / 8.0) * 2.0 * PI - PI / 2.0;
        let inner_pt = Point::new(
            center.x + r * 0.34 * pulse * angle.cos(),
            center.y + r * 0.34 * pulse * angle.sin(),
        );
        let outer_pt = Point::new(
            center.x + r * 0.67 * pulse * angle.cos(),
            center.y + r * 0.67 * pulse * angle.sin(),
        );
        let line = Path::line(inner_pt, outer_pt);
        frame.stroke(
            &line,
            Stroke::default()
                .with_color(Color { a: 0.15, ..color })
                .with_width(1.0),
        );
    }

    // Inner core (SUTRA) — drawn last to cover sector fill centers
    let core = Path::circle(center, r * 0.32 * pulse);
    frame.fill(&core, CORE_COLOR);
    frame.stroke(
        &core,
        Stroke::default()
            .with_color(Color { a: 0.6, ..color })
            .with_width(1.0),
    );
}

/// Static layer: domain name labels only (no animation dependency).
fn draw_domain_labels(frame: &mut Frame, center: Point, ring_radius: f32, snap: &SystemSnapshot) {
    let n = snap.domains.len();
    if n == 0 {
        return;
    }
    for (i, domain) in snap.domains.iter().enumerate() {
        let angle = (i as f32 / n as f32) * 2.0 * PI - PI / 2.0;
        let x = center.x + ring_radius * angle.cos();
        let y = center.y + ring_radius * angle.sin();
        frame.fill_text(canvas::Text {
            content: domain.name.clone(),
            position: Point::new(x, y + 16.0),
            color: LABEL_COLOR,
            size: iced::Pixels(11.0),
            horizontal_alignment: alignment::Horizontal::Center,
            vertical_alignment: alignment::Vertical::Top,
            ..canvas::Text::default()
        });
    }
}

/// Dynamic layer: domain dots and flow lines (time + activity dependent).
fn draw_domain_nodes(
    frame: &mut Frame,
    center: Point,
    ring_radius: f32,
    snap: &SystemSnapshot,
    state_color: Color,
    flow_active: &[bool; 8],
) {
    let n = snap.domains.len();
    if n == 0 {
        return;
    }

    for (i, domain) in snap.domains.iter().enumerate() {
        let angle = (i as f32 / n as f32) * 2.0 * PI - PI / 2.0;
        let x = center.x + ring_radius * angle.cos();
        let y = center.y + ring_radius * angle.sin();
        let pos = Point::new(x, y);

        let is_active = domain.recent_activity > 0;
        let flow_lit = domain.id >= 101
            && domain.id <= 108
            && flow_active[(domain.id - 101) as usize];

        let line_color = if flow_lit {
            Color { a: 0.55, ..state_color }
        } else if is_active {
            Color { a: 0.25, ..state_color }
        } else {
            Color { a: 0.1, ..DOMAIN_IDLE }
        };
        let line = Path::line(center, pos);
        frame.stroke(
            &line,
            Stroke::default()
                .with_color(line_color)
                .with_width(if flow_lit { 1.8 } else { 1.0 }),
        );

        let dot_color = if is_active || flow_lit { state_color } else { DOMAIN_IDLE };
        let dot_r = if is_active || flow_lit { 9.0 } else { 6.0 };
        frame.fill(&Path::circle(pos, dot_r), dot_color);
    }
}

fn fmt_ns(ns: u64) -> String {
    if ns < 1_000 {
        format!("{ns}ns")
    } else if ns < 1_000_000 {
        format!("{:.1}µs", ns as f64 / 1_000.0)
    } else {
        format!("{:.2}ms", ns as f64 / 1_000_000.0)
    }
}

fn draw_bottom_labels(frame: &mut Frame, snap: &SystemSnapshot, bounds: Rectangle) {
    let state_str = match snap.engine_state {
        EngineState::Wake => "WAKE",
        EngineState::FallingAsleep => "FALLING ASLEEP",
        EngineState::Dreaming => "DREAMING",
        EngineState::Waking => "WAKING",
    };

    let fatigue_pct = (snap.fatigue.current * 100.0) as u32;
    let (frames, promotions) = snap
        .frame_weaver_stats
        .as_ref()
        .map(|fw| (fw.total_frames, fw.promotions_since_wake))
        .unwrap_or((0, 0));
    let vetoes = snap.guardian_stats.total_vetoes;

    let hot = if snap.hot_path_ns > 0 {
        format!("   hot: {}", fmt_ns(snap.hot_path_ns))
    } else {
        String::new()
    };

    let left = format!(
        "State: {}   Fatigue: {}%   tick: {}{}",
        state_str, fatigue_pct, snap.current_tick, hot
    );

    let dream_info = if snap.dream_phase_stats.last_transition_tick > 0 {
        format!("   dream@{}", snap.dream_phase_stats.last_transition_tick)
    } else {
        String::new()
    };

    let right = if vetoes > 0 {
        format!(
            "Frames: {}   promo: {}   events: {}   vetoes: {}{}",
            frames, promotions, snap.current_event, vetoes, dream_info
        )
    } else {
        format!(
            "Frames: {}   promo: {}   events: {}{}",
            frames, promotions, snap.current_event, dream_info
        )
    };

    frame.fill_text(canvas::Text {
        content: left,
        position: Point::new(12.0, bounds.height - 24.0),
        color: LABEL_COLOR,
        size: iced::Pixels(12.0),
        ..canvas::Text::default()
    });

    frame.fill_text(canvas::Text {
        content: right,
        position: Point::new(bounds.width - 12.0, bounds.height - 24.0),
        color: LABEL_COLOR,
        size: iced::Pixels(12.0),
        horizontal_alignment: alignment::Horizontal::Right,
        ..canvas::Text::default()
    });
}

fn draw_loading(frame: &mut Frame, center: Point, r: f32, phase: f32) {
    let pulse = 1.0 + 0.06 * (phase * 2.0 * PI).sin();
    let color = Color::from_rgba(0.4, 0.5, 0.7, 0.4);

    for i in 0..3 {
        let ring_r = r * (0.35 + 0.33 * i as f32) * pulse;
        let ring = Path::circle(center, ring_r);
        frame.stroke(&ring, Stroke::default().with_color(color).with_width(1.5));
    }

    let arc_start = phase * 2.0 * PI;
    let arc_path = Path::new(|b| {
        b.arc(Arc {
            center,
            radius: r * 0.68 * pulse,
            start_angle: Radians(arc_start),
            end_angle: Radians(arc_start + PI * 0.8),
        });
    });
    frame.stroke(
        &arc_path,
        Stroke::default()
            .with_color(Color::from_rgba(0.5, 0.65, 0.9, 0.7))
            .with_width(2.0),
    );

    frame.fill_text(canvas::Text {
        content: "Waiting for Engine...".to_string(),
        position: Point::new(center.x, center.y + r * 1.4),
        color: LABEL_COLOR,
        size: iced::Pixels(13.0),
        horizontal_alignment: alignment::Horizontal::Center,
        vertical_alignment: alignment::Vertical::Top,
        ..canvas::Text::default()
    });
}
