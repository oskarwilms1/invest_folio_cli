use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
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
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let app_title = Line::from("Invest-folio".bold());

        let portfolio_title = Line::from("List of Portfolios".bold());
        let portfolio_list_block = Block::bordered()
            .title(portfolio_title.centered())
            .border_set(border::THICK);

        let assets_title = Line::from("Assets".bold());
        let assets_list_block = Block::bordered().title(assets_title.centered());

        let dashboard_title = Line::from("Dashboard".bold());
        let dashboard_block = Block::bordered().title(dashboard_title.centered());

        let [top, bottom] =
            Layout::vertical([Constraint::Min(0), Constraint::Percentage(50)]).areas(area);
        let [top_left, top_right] =
            Layout::horizontal([Constraint::Percentage(20), Constraint::Min(0)]).areas(top);

        let [bottom_left, bottom_right] =
            Layout::horizontal([Constraint::Percentage(20), Constraint::Min(0)]).areas(bottom);

        app_title.render(area, buf);
        portfolio_list_block.render(top_left, buf);
        assets_list_block.render(bottom_left, buf);
        dashboard_block.render(top_right.union(bottom_right), buf);
    }
}
