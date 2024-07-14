use crate::model::{Board, Move, PieceColor, Position};

pub struct Pawn;

impl Move for Pawn {
    fn available_moves(position: Position, board: &Board) -> Option<Vec<Position>> {
        let (x, y) = (position.x as usize, position.y as usize);

        let legal_moves = match &board.board[x][y] {
            Some(piece) => match piece.piece_color {
                PieceColor::White => Self::get_legal_moves_for_white(&position, board),
                PieceColor::Black => Self::get_legal_moves_for_black(&position, board),
            },
            None => vec![],
        };

        if !legal_moves.is_empty() {
            None
        } else {
            Some(legal_moves)
        }
    }
}

impl Pawn {
    fn get_legal_moves_for_white(position: &Position, board: &Board) -> Vec<Position> {
        let mut legal_moves = Vec::with_capacity(28);
        let (x, y) = (position.x, position.y);

        match board.board[x - 1][y] {
            Some(_) => {}
            None => {
                legal_moves.push(Position { x: x - 1, y });

                // If pawn is on starting position, check for double move
                if x == 6 {
                    match board.board[x - 2][y] {
                        Some(_) => {}
                        None => legal_moves.push(Position { x: x - 2, y }),
                    };
                }
            }
        };

        // Check for possible captures
        match board.get_piece_color_in_position(Position { x: x - 1, y: y - 1 }) {
            Some(color) => match color {
                PieceColor::White => {}
                PieceColor::Black => legal_moves.push(Position { x: x - 1, y: y - 1 }),
            },
            None => {}
        };

        match board.get_piece_color_in_position(Position { x: x - 1, y: y + 1 }) {
            Some(color) => match color {
                PieceColor::White => {}
                PieceColor::Black => legal_moves.push(Position { x: x - 1, y: y + 1 }),
            },
            None => {}
        };

        legal_moves
    }

    fn get_legal_moves_for_black(position: &Position, board: &Board) -> Vec<Position> {
        let mut legal_moves = Vec::with_capacity(28);
        let (x, y) = (position.x, position.y);

        match board.board[x + 1][y] {
            Some(_) => {}
            None => {
                legal_moves.push(Position { x: x + 1, y });

                // If pawn is on starting position, check for double move
                if x == 1 {
                    match board.board[x + 2][y] {
                        Some(_) => {}
                        None => legal_moves.push(Position { x: x + 2, y }),
                    };
                }
            }
        };

        // Check for possible captures
        match board.get_piece_color_in_position(Position { x: x + 1, y: y - 1 }) {
            Some(color) => match color {
                PieceColor::White => legal_moves.push(Position { x: x + 1, y: y - 1 }),
                PieceColor::Black => {}
            },
            None => {}
        };

        match board.get_piece_color_in_position(Position { x: x + 1, y: y + 1 }) {
            Some(color) => match color {
                PieceColor::White => legal_moves.push(Position { x: x + 1, y: y + 1 }),
                PieceColor::Black => {}
            },
            None => {}
        };

        legal_moves
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{Piece, PieceType};

    use super::*;

    #[test]
    fn test_white_pawn_initial_moves() {
        let board = Board::default();
        let position = Position { x: 6, y: 3 };

        let legal_moves = Pawn::get_legal_moves_for_white(&position, &board);

        let expected_moves = vec![Position { x: 5, y: 3 }, Position { x: 4, y: 3 }];

        assert_eq!(legal_moves, expected_moves);
    }

    #[test]
    fn test_white_pawn_blocked_moves() {
        let mut board = Board::default();
        let position = Position { x: 6, y: 3 };
        board.board[5][3] = Some(Piece::new(PieceType::Pawn, PieceColor::White));

        let legal_moves = Pawn::get_legal_moves_for_white(&position, &board);

        let expected_moves: Vec<Position> = vec![];

        assert_eq!(legal_moves, expected_moves);
    }

    #[test]
    fn test_black_pawn_initial_moves() {
        let board = Board::default();
        let position = Position { x: 1, y: 3 };

        let legal_moves = Pawn::get_legal_moves_for_black(&position, &board);

        let expected_moves = vec![Position { x: 2, y: 3 }, Position { x: 3, y: 3 }];

        assert_eq!(legal_moves, expected_moves);
    }

    #[test]
    fn test_black_pawn_blocked_moves() {
        let mut board = Board::default();
        let position = Position { x: 1, y: 3 };
        board.board[2][3] = Some(Piece::new(PieceType::Pawn, PieceColor::Black));

        let legal_moves = Pawn::get_legal_moves_for_black(&position, &board);

        let expected_moves: Vec<Position> = vec![];

        assert_eq!(legal_moves, expected_moves);
    }

    #[test]
    fn test_white_pawn_capture() {
        let mut board = Board::default();
        let position = Position { x: 4, y: 3 };
        board.board[3][2] = Some(Piece::new(PieceType::Pawn, PieceColor::Black));
        board.board[3][4] = Some(Piece::new(PieceType::Pawn, PieceColor::Black));

        let legal_moves = Pawn::get_legal_moves_for_white(&position, &board);

        let expected_moves = vec![
            Position { x: 3, y: 3 },
            Position { x: 3, y: 2 },
            Position { x: 3, y: 4 },
        ];

        assert_eq!(legal_moves, expected_moves);
    }

    #[test]
    fn test_black_pawn_capture() {
        let mut board = Board::default();
        let position = Position { x: 3, y: 3 };
        board.board[4][2] = Some(Piece::new(PieceType::Pawn, PieceColor::White));
        board.board[4][4] = Some(Piece::new(PieceType::Pawn, PieceColor::White));

        let legal_moves = Pawn::get_legal_moves_for_black(&position, &board);

        let expected_moves = vec![
            Position { x: 4, y: 3 },
            Position { x: 4, y: 2 },
            Position { x: 4, y: 4 },
        ];

        assert_eq!(legal_moves, expected_moves);
    }
}
