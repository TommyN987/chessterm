use std::fmt::Display;

use super::{Piece, PieceColor, PieceType};

#[derive(Debug)]
pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
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
            write!(f, "\n")?;
        }
        Ok(())
    }
}
