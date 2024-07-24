use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::model::{Piece, PieceColor, PieceType};

use super::constants::{BISHOP, KING, KNIGHT, PAWN, QUEEN, ROOK};

impl Widget for Piece {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let color: Color = match &self.piece_color {
            PieceColor::Black => Color::Black,
            PieceColor::White => Color::White,
        };

        let piece_str = match &self.piece_type {
            PieceType::Bishop => BISHOP,
            PieceType::King => KING,
            PieceType::Knight => KNIGHT,
            PieceType::Pawn => PAWN,
            PieceType::Queen => QUEEN,
            PieceType::Rook => ROOK,
        };

        let piece = Paragraph::new(piece_str)
            .style(Style::default())
            .fg(color)
            .alignment(Alignment::Center);

        Widget::render(piece, area, buf);
    }
}
