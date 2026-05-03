use iced::mouse;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::widget::{button, column, row, text};
use iced::{Color, Element, Length, Point, Rectangle, Size};

use axiom_protocol::snapshot::{DomainSnapshot, SystemSnapshot};

use crate::app::{DisplayOptions, LiveFieldOption, LiveFieldState, Message, OrbitCamera};

const MAX_POINTS: u32 = 300;

const LAYER_COLORS: [Color; 8] = [
    Color { r: 0.35, g: 0.60, b: 1.00, a: 0.85 },
    Color { r: 0.30, g: 0.85, b: 0.75, a: 0.85 },
    Color { r: 0.45, g: 0.90, b: 0.40, a: 0.85 },
    Color { r: 0.75, g: 0.90, b: 0.30, a: 0.85 },
    Color { r: 0.95, g: 0.80, b: 0.20, a: 0.85 },
    Color { r: 1.00, g: 0.58, b: 0.20, a: 0.85 },
    Color { r: 1.00, g: 0.35, b: 0.35, a: 0.85 },
    Color { r: 0.85, g: 0.30, b: 0.90, a: 0.85 },
];

// ── Public view ────────────────────────────────────────────────────────────

pub fn live_field_view<'a>(
    lf: &'a LiveFieldState,
    snapshot: &'a Option<SystemSnapshot>,
) -> Element<'a, Message> {
    let domains: &[DomainSnapshot] = snapshot
        .as_ref()
        .map(|s| s.domains.as_slice())
        .unwrap_or(&[]);

    row![
        side_panel(lf, domains),
        canvas_area(lf, snapshot),
    ]
    .height(Length::Fill)
    .into()
}

// ── Side panel ─────────────────────────────────────────────────────────────

fn side_panel<'a>(lf: &'a LiveFieldState, domains: &'a [DomainSnapshot]) -> Element<'a, Message> {
    let domain_btns: Vec<Element<Message>> = domains
        .iter()
        .map(|d| {
            let selected = lf.selected_domain == Some(d.id);
            button(text(d.name.as_str()).size(12))
                .on_press(Message::LiveFieldDomainSelected(d.id))
                .style(if selected { button::primary } else { button::secondary })
                .width(Length::Fill)
                .into()
        })
        .collect();

    let stats_domain = domains
        .iter()
        .find(|d| Some(d.id) == lf.selected_domain)
        .or_else(|| domains.first());

    let stats: Element<Message> = if let Some(d) = stats_domain {
        column![
            text("Stats").size(11).color(Color::from_rgb(0.5, 0.5, 0.5)),
            text(format!("Tokens:  {}", d.token_count)).size(12),
            text(format!("Conns:   {}", d.connection_count)).size(12),
            text(format!("Temp:    {}", d.temperature_avg)).size(12),
            text(format!("Active:  {}", d.recent_activity)).size(12),
        ]
        .spacing(3)
        .into()
    } else {
        text("No engine data")
            .size(12)
            .color(Color::from_rgb(0.45, 0.45, 0.45))
            .into()
    };

    let opt_btns: Vec<Element<Message>> = [
        (LiveFieldOption::ShowConnections, "Connections", lf.display.show_connections),
        (LiveFieldOption::ShowAnchors, "Anchors", lf.display.show_anchors),
        (LiveFieldOption::LayerColorCoding, "Layer colors", lf.display.layer_color_coding),
        (LiveFieldOption::HighlightRecent, "Highlight active", lf.display.highlight_recent),
    ]
    .iter()
    .map(|&(opt, label, active)| {
        button(text(label).size(11))
            .on_press(Message::LiveFieldToggleOption(opt))
            .style(if active { button::primary } else { button::secondary })
            .width(Length::Fill)
            .into()
    })
    .collect();

    column![
        text("Domains").size(11).color(Color::from_rgb(0.5, 0.5, 0.5)),
        column(domain_btns).spacing(3),
        stats,
        text("Display").size(11).color(Color::from_rgb(0.5, 0.5, 0.5)),
        column(opt_btns).spacing(3),
        button(text("Reset camera").size(11))
            .on_press(Message::LiveFieldCameraReset)
            .style(button::secondary)
            .width(Length::Fill),
    ]
    .spacing(8)
    .padding(12)
    .width(180)
    .height(Length::Fill)
    .into()
}

// ── Canvas widget ──────────────────────────────────────────────────────────

fn canvas_area<'a>(
    lf: &'a LiveFieldState,
    snapshot: &'a Option<SystemSnapshot>,
) -> Element<'a, Message> {
    Canvas::new(LiveFieldCanvas { lf, snapshot })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

struct LiveFieldCanvas<'a> {
    lf: &'a LiveFieldState,
    snapshot: &'a Option<SystemSnapshot>,
}

#[derive(Default)]
struct DragState {
    active: bool,
    last: Option<Point>,
}

impl<'a> canvas::Program<Message> for LiveFieldCanvas<'a> {
    type State = DragState;

    fn draw(
        &self,
        _state: &DragState,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let domains: &[DomainSnapshot] = self
            .snapshot
            .as_ref()
            .map(|s| s.domains.as_slice())
            .unwrap_or(&[]);
        draw_scene(&mut frame, &self.lf.camera, &self.lf.display, self.lf.selected_domain, domains);
        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut DragState,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        match event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let mouse::Cursor::Available(pos) = cursor {
                    if bounds.contains(pos) {
                        state.active = true;
                        state.last = Some(pos);
                        return (canvas::event::Status::Captured, None);
                    }
                }
            }
            canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.active = false;
                state.last = None;
            }
            canvas::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if state.active {
                    if let Some(last) = state.last {
                        let dx = (position.x - last.x) / bounds.width;
                        let dy = (position.y - last.y) / bounds.height;
                        state.last = Some(position);
                        return (
                            canvas::event::Status::Captured,
                            Some(Message::LiveFieldCameraRotate { dx, dy }),
                        );
                    }
                }
            }
            canvas::Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if let mouse::Cursor::Available(pos) = cursor {
                    if bounds.contains(pos) {
                        let amount = match delta {
                            mouse::ScrollDelta::Lines { y, .. } => -y * 0.3,
                            mouse::ScrollDelta::Pixels { y, .. } => -y * 0.01,
                        };
                        return (
                            canvas::event::Status::Captured,
                            Some(Message::LiveFieldCameraZoom(amount)),
                        );
                    }
                }
            }
            _ => {}
        }
        (canvas::event::Status::Ignored, None)
    }

    fn mouse_interaction(
        &self,
        state: &DragState,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if state.active {
            mouse::Interaction::Grabbing
        } else {
            mouse::Interaction::Grab
        }
    }
}

// ── Scene ──────────────────────────────────────────────────────────────────

fn draw_scene(
    frame: &mut Frame,
    camera: &OrbitCamera,
    display: &DisplayOptions,
    selected: Option<u16>,
    domains: &[DomainSnapshot],
) {
    let size = frame.size();

    frame.fill_rectangle(Point::ORIGIN, size, Color { r: 0.04, g: 0.04, b: 0.07, a: 1.0 });

    draw_axes(frame, camera, size);

    if display.show_anchors {
        draw_anchor_marker(frame, camera, size);
    }

    for domain in domains {
        let active = selected.map(|id| id == domain.id).unwrap_or(true);
        draw_domain_points(frame, camera, display, domain, active, size);
    }

    if domains.is_empty() {
        frame.fill_text(canvas::Text {
            content: "Waiting for Engine data…".to_string(),
            position: Point::new(size.width / 2.0, size.height / 2.0),
            color: Color::from_rgba(0.5, 0.5, 0.6, 0.6),
            size: iced::Pixels(14.0),
            horizontal_alignment: iced::alignment::Horizontal::Center,
            vertical_alignment: iced::alignment::Vertical::Center,
            ..canvas::Text::default()
        });
    }
}

fn draw_axes(frame: &mut Frame, camera: &OrbitCamera, size: Size) {
    let axis_color = Color::from_rgba(0.22, 0.22, 0.32, 0.45);
    let stroke = Stroke::default().with_color(axis_color).with_width(0.6);

    let endpoints: [((f32, f32, f32), (f32, f32, f32)); 3] = [
        ((-1.2, 0.0, 0.0), (1.2, 0.0, 0.0)),
        ((0.0, -1.2, 0.0), (0.0, 1.2, 0.0)),
        ((0.0, 0.0, -1.2), (0.0, 0.0, 1.2)),
    ];
    for (p0, p1) in &endpoints {
        let Some(s0) = project(*p0, camera, size) else { continue };
        let Some(s1) = project(*p1, camera, size) else { continue };
        frame.stroke(&Path::line(s0, s1), stroke.clone());
    }
}

fn draw_anchor_marker(frame: &mut Frame, camera: &OrbitCamera, size: Size) {
    let r = 0.06;
    let verts: [(f32, f32, f32); 6] = [
        (r, 0.0, 0.0), (-r, 0.0, 0.0),
        (0.0, r, 0.0), (0.0, -r, 0.0),
        (0.0, 0.0, r), (0.0, 0.0, -r),
    ];
    let edges = [(0, 2), (0, 3), (0, 4), (0, 5), (1, 2), (1, 3), (1, 4), (1, 5), (2, 4), (2, 5), (3, 4), (3, 5)];
    let stroke = Stroke::default()
        .with_color(Color::from_rgba(0.8, 0.8, 0.9, 0.6))
        .with_width(0.8);

    for (a, b) in &edges {
        let Some(sa) = project(verts[*a], camera, size) else { continue };
        let Some(sb) = project(verts[*b], camera, size) else { continue };
        frame.stroke(&Path::line(sa, sb), stroke.clone());
    }
}

fn draw_domain_points(
    frame: &mut Frame,
    camera: &OrbitCamera,
    display: &DisplayOptions,
    domain: &DomainSnapshot,
    active: bool,
    size: Size,
) {
    let n = domain.token_count.min(MAX_POINTS);
    if n == 0 {
        return;
    }
    let base_alpha: f32 = if active { 0.85 } else { 0.18 };
    let dot_size_boost =
        display.highlight_recent && domain.recent_activity > 50 && active;

    let conn_stroke = Stroke::default()
        .with_color(Color::from_rgba(0.4, 0.5, 0.8, base_alpha * 0.18))
        .with_width(0.5);

    // Compute anchor (first point projected) for connection lines
    let anchor_screen = if display.show_connections {
        let s0 = hash_seed(domain.id as u64, 0);
        project(pseudo_pos(s0), camera, size)
    } else {
        None
    };

    for i in 0..n {
        let seed = hash_seed(domain.id as u64, i as u64);
        let pos = pseudo_pos(seed);
        let Some(screen) = project(pos, camera, size) else { continue };

        // Connection line to anchor point (skip self)
        if display.show_connections && i > 0 {
            if let Some(anchor) = anchor_screen {
                frame.stroke(&Path::line(screen, anchor), conn_stroke.clone());
            }
        }

        let color = if display.layer_color_coding {
            let layer = pick_layer(&domain.layer_activations, lcg_next(seed));
            Color { a: base_alpha, ..LAYER_COLORS[layer] }
        } else {
            Color::from_rgba(0.6, 0.72, 1.0, base_alpha)
        };

        let radius = if dot_size_boost { 3.0 } else { 2.0 };
        frame.fill(&Path::circle(screen, radius), color);
    }
}

// ── Math ───────────────────────────────────────────────────────────────────

fn project((px, py, pz): (f32, f32, f32), cam: &OrbitCamera, size: Size) -> Option<Point> {
    // Rotate around Y by -azimuth
    let (sin_az, cos_az) = (-cam.azimuth).sin_cos();
    let rx = px * cos_az + pz * sin_az;
    let ry = py;
    let rz = -px * sin_az + pz * cos_az;

    // Rotate around X by -elevation
    let (sin_el, cos_el) = (-cam.elevation).sin_cos();
    let fx = rx;
    let fy = ry * cos_el - rz * sin_el;
    let fz = (ry * sin_el + rz * cos_el) - cam.distance;

    if fz >= -0.05 {
        return None;
    }

    let fov = 1.5;
    let sx = fx / (-fz) * fov;
    let sy = fy / (-fz) * fov;

    let min_dim = size.width.min(size.height);
    let cx = size.width / 2.0 + sx * min_dim / 2.0;
    let cy = size.height / 2.0 - sy * min_dim / 2.0;

    if cx >= -10.0 && cx <= size.width + 10.0 && cy >= -10.0 && cy <= size.height + 10.0 {
        Some(Point::new(cx, cy))
    } else {
        None
    }
}

fn hash_seed(domain_id: u64, index: u64) -> u64 {
    domain_id
        .wrapping_mul(1_000_003)
        .wrapping_add(index.wrapping_mul(2_654_435_761))
}

fn lcg_next(seed: u64) -> u64 {
    seed.wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407)
}

fn lcg_f32(seed: u64) -> f32 {
    (seed >> 33) as f32 / 0x7FFF_FFFFu32 as f32
}

fn pseudo_pos(seed: u64) -> (f32, f32, f32) {
    let s1 = lcg_next(seed);
    let s2 = lcg_next(s1);
    let s3 = lcg_next(s2);
    let s4 = lcg_next(s3);

    let x = lcg_f32(s1) * 2.0 - 1.0;
    let y = lcg_f32(s2) * 2.0 - 1.0;
    let z = lcg_f32(s3) * 2.0 - 1.0;
    let r = 0.3 + lcg_f32(s4) * 0.7;

    let len = (x * x + y * y + z * z).sqrt().max(1e-6);
    (x / len * r, y / len * r, z / len * r)
}

fn pick_layer(layer_activations: &[u8; 8], seed: u64) -> usize {
    let total: u32 = layer_activations.iter().map(|&v| v as u32).sum();
    if total == 0 {
        return (seed % 8) as usize;
    }
    let threshold = (lcg_f32(seed) * total as f32) as u32;
    let mut cumulative = 0u32;
    for (i, &v) in layer_activations.iter().enumerate() {
        cumulative += v as u32;
        if threshold < cumulative {
            return i;
        }
    }
    7
}
