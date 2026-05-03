mod app;
mod connection;
mod settings;
mod ui;

use app::{Message, WorkstationApp};

fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "axiom_workstation=info".to_string()),
        )
        .init();

    iced::daemon("AXIOM Workstation", WorkstationApp::update, WorkstationApp::view)
        .subscription(WorkstationApp::subscription)
        .run_with(|| {
            let mut app = WorkstationApp::new();
            let (id, open_task) = iced::window::open(iced::window::Settings {
                size: iced::Size::new(1200.0, 800.0),
                position: iced::window::Position::Centered,
                min_size: Some(iced::Size::new(800.0, 600.0)),
                ..Default::default()
            });
            app.main_window = Some(id);
            (app, open_task.map(|_| Message::AnimationTick))
        })
}
