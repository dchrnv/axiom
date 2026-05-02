mod app;
mod connection;
mod settings;

use app::WorkstationApp;

fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "axiom_workstation=info".to_string()),
        )
        .init();

    iced::application("AXIOM Workstation", WorkstationApp::update, WorkstationApp::view)
        .subscription(WorkstationApp::subscription)
        .run_with(|| (WorkstationApp::new(), iced::Task::none()))
}
