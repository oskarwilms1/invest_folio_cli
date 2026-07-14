use crate::app::App;

mod app;
mod portfolio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}
