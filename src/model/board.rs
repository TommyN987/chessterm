use std::fmt::Display;

use super::{Piece, PieceColor, PieceType, Position};

#[derive(Debug)]
pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
    pub selected_position: Option<Position>,
    pub currently_legal_moves: Option<Vec<Position>>,
}

impl Board {
    pub fn is_cell_empty(&self, position: Position) -> bool {
        self.board[position.x][position.y].is_none()
    }

    pub fn get_piece_color_in_position(&self, position: Position) -> Option<PieceColor> {
        self.board[position.x][position.y]
            .as_ref()
            .map(|piece| piece.piece_color)
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [
                [
                    Some(Piece::new(PieceType::Rook, PieceColor::Black)),
                    Some(Piece::new(PieceType::Knight, PieceColor::Black)),
                    Some(Piece::new(PieceType::Bishop, PieceColor::Black)),
                    Some(Piece::new(PieceType::Queen, PieceColor::Black)),
                    Some(Piece::new(PieceType::King, PieceColor::Black)),
                    Some(Piece::new(PieceType::Bishop, PieceColor::Black)),
                    Some(Piece::new(PieceType::Knight, PieceColor::Black)),
                    Some(Piece::new(PieceType::Rook, PieceColor::Black)),
                ],
                [
                    Some(Piece::new(PieceType::Pawn, PieceColor::Black)),
                    Some(Piece::new(PieceType::Pawn, PieceColor::Black)),
                    Some(Piece::new(PieceType::Pawn, PieceColor::Black)),
                    Some(Piece::new(PieceType::Pawn, PieceColor::Black)),
                    Some(Piece::new(PieceType::Pawn, PieceColor::Black)),
                    Some(Piece::new(PieceType::Pawn, PieceColor::Black)),
                    Some(Piece::new(PieceType::Pawn, PieceColor::Black)),
                    Some(Piece::new(PieceType::Pawn, PieceColor::Black)),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(Piece::new(PieceType::Pawn, PieceColor::White)),
                    Some(Piece::new(PieceType::Pawn, PieceColor::White)),
                    Some(Piece::new(PieceType::Pawn, PieceColor::White)),
                    Some(Piece::new(PieceType::Pawn, PieceColor::White)),
                    Some(Piece::new(PieceType::Pawn, PieceColor::White)),
                    Some(Piece::new(PieceType::Pawn, PieceColor::White)),
                    Some(Piece::new(PieceType::Pawn, PieceColor::White)),
                    Some(Piece::new(PieceType::Pawn, PieceColor::White)),
                ],
                [
                    Some(Piece::new(PieceType::Rook, PieceColor::White)),
                    Some(Piece::new(PieceType::Knight, PieceColor::White)),
                    Some(Piece::new(PieceType::Bishop, PieceColor::White)),
                    Some(Piece::new(PieceType::Queen, PieceColor::White)),
                    Some(Piece::new(PieceType::King, PieceColor::White)),
                    Some(Piece::new(PieceType::Bishop, PieceColor::White)),
                    Some(Piece::new(PieceType::Knight, PieceColor::White)),
                    Some(Piece::new(PieceType::Rook, PieceColor::White)),
                ],
            ],
            selected_position: None,
            currently_legal_moves: None,
        }
    }
}

impl Board {
    #[cfg(test)]
    pub fn init_empty() -> Self {
        Self {
            board: [[None; 8]; 8],
            selected_position: None,
            currently_legal_moves: None,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.board {
            for cell in row {
                match cell {
                    Some(piece) => write!(f, "[{}]", piece),
                    None => write!(f, "[ ]"),
                }?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
