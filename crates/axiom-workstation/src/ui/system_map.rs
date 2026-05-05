use std::f32::consts::PI;

use iced::alignment;
use iced::mouse;
use iced::widget::canvas::path::Arc;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{Color, Element, Length, Point, Radians, Rectangle};

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

pub fn system_map_view(snapshot: &Option<SystemSnapshot>, phase: f32) -> Element<'_, Message> {
    Canvas::new(SystemMapCanvas { snapshot, phase })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

struct SystemMapCanvas<'a> {
    snapshot: &'a Option<SystemSnapshot>,
    phase: f32,
}

impl<'a> canvas::Program<Message> for SystemMapCanvas<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let center = frame.center();
        let min_dim = bounds.width.min(bounds.height);
        let mandala_r = min_dim * 0.18;

        frame.fill_rectangle(Point::ORIGIN, bounds.size(), BG);

        match self.snapshot {
            Some(snap) => {
                let state_color = engine_state_color(snap.engine_state);
                draw_mandala(&mut frame, center, mandala_r, state_color, self.phase);
                draw_domains(&mut frame, center, mandala_r * 2.8, snap, state_color);
                draw_bottom_labels(&mut frame, snap, bounds);
            }
            None => {
                draw_loading(&mut frame, center, mandala_r, self.phase);
            }
        }

        vec![frame.into_geometry()]
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

fn draw_mandala(frame: &mut Frame, center: Point, r: f32, color: Color, phase: f32) {
    let pulse = 1.0 + 0.04 * (phase * 2.0 * PI).sin();

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

    // Inner core (SUTRA)
    let core = Path::circle(center, r * 0.32 * pulse);
    frame.fill(&core, CORE_COLOR);
    frame.stroke(
        &core,
        Stroke::default()
            .with_color(Color { a: 0.6, ..color })
            .with_width(1.0),
    );
}

fn draw_domains(
    frame: &mut Frame,
    center: Point,
    ring_radius: f32,
    snap: &SystemSnapshot,
    state_color: Color,
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
        let dot_color = if is_active { state_color } else { DOMAIN_IDLE };
        let dot_r = if is_active { 9.0 } else { 6.0 };

        // Connection line to center
        let line = Path::line(center, pos);
        frame.stroke(
            &line,
            Stroke::default()
                .with_color(if is_active {
                    Color {
                        a: 0.25,
                        ..state_color
                    }
                } else {
                    Color {
                        a: 0.1,
                        ..DOMAIN_IDLE
                    }
                })
                .with_width(1.0),
        );

        // Domain dot
        let dot = Path::circle(pos, dot_r);
        frame.fill(&dot, dot_color);

        // Domain name label
        frame.fill_text(canvas::Text {
            content: domain.name.clone(),
            position: Point::new(x, y + dot_r + 10.0),
            color: LABEL_COLOR,
            size: iced::Pixels(11.0),
            horizontal_alignment: alignment::Horizontal::Center,
            vertical_alignment: alignment::Vertical::Top,
            ..canvas::Text::default()
        });
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
    let frames = snap
        .frame_weaver_stats
        .as_ref()
        .map(|fw| fw.total_frames)
        .unwrap_or(0);

    let left = format!(
        "State: {}   Fatigue: {}%   tick: {}",
        state_str, fatigue_pct, snap.current_tick
    );
    let right = format!("Frames: {}   events: {}", frames, snap.current_event);

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

    // Rotating arc
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
