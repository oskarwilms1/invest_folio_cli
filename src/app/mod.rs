use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

mod ui;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    state: AppState,
}
#[derive(Debug, Default)]
enum AppState {
    #[default]
    Portfolio,
    Assets,
    Dashboard,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if let KeyCode::Char('q') = key_event.code {
            self.exit();
        }
        self.handle_navigation_event(key_event);
    }
    fn handle_navigation_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('p') => self.state = AppState::Portfolio,
            KeyCode::Char('a') => self.state = AppState::Assets,
            KeyCode::Char('d') => self.state = AppState::Dashboard,
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }
}
