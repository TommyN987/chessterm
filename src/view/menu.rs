use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, StatefulWidget, Widget},
    Frame,
};

use crate::{game::Game, model::menu::MenuState};

use super::constants::TITLE;

pub fn render_menu(frame: &mut Frame, main_area: Rect, game: &mut Game) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(main_area);

    frame.render_widget(AppTitle, layout[0]);

    frame.render_stateful_widget(game.menu_state.clone(), layout[1], &mut game.menu_state);
}

struct AppTitle;

impl Widget for AppTitle {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let lines: Vec<&str> = TITLE.trim().lines().collect();
        let text_height = lines.len() as u16;
        let vertical_offset = area.y + (area.height.saturating_sub(text_height)) / 2;

        // Create a new Rect with the vertical offset
        let centered_area = Rect {
            x: area.x,
            y: vertical_offset - 1,
            width: area.width,
            height: text_height + 1,
        };

        Paragraph::new(TITLE)
            .alignment(Alignment::Center)
            .block(Block::default())
            .render(centered_area, buf);
    }
}

impl StatefulWidget for MenuState {
    type State = Self;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let list_items: Vec<ListItem> = state
            .items
            .iter()
            .enumerate()
            .map(|(i, &item)| {
                if state.selected == i {
                    ListItem::new(format!("> {}", item)).style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    ListItem::from(item).style(Style::default())
                }
            })
            .collect();
        let list = List::new(list_items)
            .block(Block::default().borders(Borders::ALL).title("Menu"))
            .style(Style::default().fg(Color::White));

        Widget::render(list, area, buf);
    }
}
