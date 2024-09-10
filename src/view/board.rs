use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Paragraph, StatefulWidget, Widget, Wrap},
    Frame,
};

use crate::{game::Game, model::Board};

use super::constants::{BLACK, WHITE};

pub fn render_game(frame: &mut Frame, main_area: Rect, game: &mut Game) {
    let main_layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 18),
                Constraint::Ratio(16, 18),
                Constraint::Ratio(1, 18),
            ]
            .as_ref(),
        )
        .split(main_area);

    let main_layout_vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(2, 17),
                Constraint::Ratio(9, 17),
                Constraint::Ratio(1, 17),
                Constraint::Ratio(5, 17),
            ]
            .as_ref(),
        )
        .split(main_layout_horizontal[1]);

    frame.render_stateful_widget(game.board.clone(), main_layout_vertical[1], &mut game.board);
    frame.render_stateful_widget(Debugger, main_layout_vertical[3],  &mut game.board);
}

pub struct Debugger;

impl StatefulWidget for Debugger {
    type State = Board;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default().title("Debugger").bg(Color::Black).fg(Color::White);

        let paragraph = Paragraph::new(state.to_string())
            .block(block)
            .wrap( Wrap { trim: true})
            .style(Style::new().fg(Color::White).bg(Color::Black));

        Widget::render(paragraph, area, buf);
    }
}

impl StatefulWidget for Board {
    type State = Board;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let cell_side_length = area.width / 8;
        let border_length = area.width / 2 - (4 * cell_side_length);

        let columns = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(border_length),
                    Constraint::Length(cell_side_length),
                    Constraint::Length(cell_side_length),
                    Constraint::Length(cell_side_length),
                    Constraint::Length(cell_side_length),
                    Constraint::Length(cell_side_length),
                    Constraint::Length(cell_side_length),
                    Constraint::Length(cell_side_length),
                    Constraint::Length(cell_side_length),
                    Constraint::Length(border_length),
                ]
                .as_ref(),
            )
            .split(area);

        state.board.iter().enumerate().for_each(|(i, row)| {
            let lines = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(border_length),
                        Constraint::Length(cell_side_length),
                        Constraint::Length(cell_side_length),
                        Constraint::Length(cell_side_length),
                        Constraint::Length(cell_side_length),
                        Constraint::Length(cell_side_length),
                        Constraint::Length(cell_side_length),
                        Constraint::Length(cell_side_length),
                        Constraint::Length(cell_side_length),
                        Constraint::Length(border_length),
                    ]
                    .as_ref(),
                )
                .split(columns[i + 1]);
            row.iter().enumerate().for_each(|(j, c)| {
                let mut cell_color: Color = if (i + j) % 2 == 0 { WHITE } else { BLACK };

                if let Some(selected) = &state.selected_position {
                    if selected.x == i && selected.y == j {
                        cell_color = Color::Cyan;
                    }
                }

                if let Some(legal_moves) = &state.currently_legal_moves {
                    legal_moves.iter().for_each(|position| {
                        if position.x == i && position.y == j {
                            cell_color = Color::Magenta;
                        }
                    })
                }

                if state.cursor_position.x == i && state.cursor_position.y == j {
                    cell_color = Color::Blue;
                }

                let cell = Block::default().bg(cell_color);
                let square = lines[j + 1];
                Widget::render(cell, square, buf);
                if let Some(piece) = c {
                    piece.render(square, buf);
                }
            })
        })
    }
}
