use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Widget},
};

use crate::app::{App, AppState};

const TITLE: &str = " Investui ";
const PORTFOLIO_LIST_TITLE: &str = " Portfolio List ";
const ASSET_LIST_TITLE: &str = " Assets ";
const DASHBOARD_TITLE: &str = " Dashboard ";
const FOCUS_COLOR: Color = Color::Green;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = AppLayout::new(area);

        OuterBorder.render(layout.outer, buf);

        PortfolioList {
            focused: matches!(self.state, AppState::Portfolio),
        }
        .render(layout.top_left, buf);
        AssetList {
            focused: matches!(self.state, AppState::Assets),
        }
        .render(layout.bottom_left, buf);
        Dashboard {
            focused: matches!(self.state, AppState::Dashboard),
        }
        .render(layout.top_right.union(layout.bottom_right), buf);
    }
}
struct AppLayout {
    outer: Rect,
    top_left: Rect,
    top_right: Rect,
    bottom_left: Rect,
    bottom_right: Rect,
}

struct OuterBorder;

struct PortfolioList {
    focused: bool,
}

struct AssetList {
    focused: bool,
}

struct Dashboard {
    focused: bool,
}

struct Tooltip;

impl AppLayout {
    fn new(area: Rect) -> Self {
        let inner = Block::bordered().inner(area);

        let [top, bottom] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(inner);

        let [top_left, top_right] =
            Layout::horizontal([Constraint::Percentage(20), Constraint::Min(0)]).areas(top);

        let [bottom_left, bottom_right] =
            Layout::horizontal([Constraint::Percentage(20), Constraint::Min(0)]).areas(bottom);

        Self {
            outer: area,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }
}

impl Widget for &OuterBorder {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let tooltip = Line::from(vec![
            " Portfolio: ".into(),
            " <P> ".blue().bold(),
            " Assets: ".into(),
            " <A> ".blue().bold(),
            " Dashboard: ".into(),
            " <D> ".blue().bold(),
            " Quit: ".into(),
            " <Q> ".blue().bold(),
        ]);
        Block::bordered()
            .gray()
            .title(Line::from(TITLE).centered())
            .title_bottom(tooltip)
            .render(area, buf);
    }
}

impl Widget for &PortfolioList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let color: Color = if self.focused {
            FOCUS_COLOR
        } else {
            Color::Reset
        };

        Block::bordered()
            .border_style(color)
            .title(Line::from(PORTFOLIO_LIST_TITLE).bold().centered())
            .render(area, buf);
    }
}

impl Widget for &AssetList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let color: Color = if self.focused {
            FOCUS_COLOR
        } else {
            Color::Reset
        };
        Block::bordered()
            .border_style(color)
            .title(Line::from(ASSET_LIST_TITLE).bold().centered())
            .render(area, buf);
    }
}

impl Widget for &Dashboard {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let color: Color = if self.focused {
            FOCUS_COLOR
        } else {
            Color::Reset
        };
        Block::bordered()
            .border_style(color)
            .title(Line::from(DASHBOARD_TITLE).bold().centered())
            .render(area, buf);
    }
}
