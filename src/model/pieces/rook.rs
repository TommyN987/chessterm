use crate::model::{Board, Move, PieceColor, Position};

pub struct Rook;

impl Move for Rook {
    fn available_moves(board: &Board) -> Option<Vec<Position>> {
        board.selected_position.as_ref()?;
        let selected_position = board.selected_position.clone().unwrap();
        let mut legal_moves: Vec<Position> = Vec::with_capacity(14);
        let color = board
            .get_piece_color_in_position(selected_position.clone())
            .unwrap();

        // Explore moves in all four directions using recursion
        Self::explore_moves(&selected_position, &mut legal_moves, board, color, 1, 0);
        Self::explore_moves(&selected_position, &mut legal_moves, board, color, -1, 0);
        Self::explore_moves(&selected_position, &mut legal_moves, board, color, 0, 1);
        Self::explore_moves(&selected_position, &mut legal_moves, board, color, 0, -1);

        if legal_moves.is_empty() {
            None
        } else {
            Some(legal_moves)
        }
    }
}

impl Rook {
    fn explore_moves(
        position: &Position,
        legal_moves: &mut Vec<Position>,
        board: &Board,
        color: PieceColor,
        dx: isize,
        dy: isize,
    ) {
        let new_x = position.x as isize + dx;
        let new_y = position.y as isize + dy;

        // Base case: check bounds
        if !(0..=7).contains(&new_x) || !(0..=7).contains(&new_y) {
            return;
        }

        let new_position = Position {
            x: new_x as usize,
            y: new_y as usize,
        };

        match board.board[new_x as usize][new_y as usize] {
            Some(piece) => {
                // If it's the opponent's piece, allow capture and stop recursion
                if piece.piece_color != color {
                    legal_moves.push(new_position);
                }
            }
            None => {
                legal_moves.push(new_position.clone());
                Self::explore_moves(&new_position, legal_moves, board, color, dx, dy);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{Piece, PieceColor, PieceType};

    use super::*;

    #[test]
    fn test_rook_initial_position() {
        let mut board = Board::default();
        board.selected_position = Some(Position { x: 0, y: 0 });

        let legal_moves = Rook::available_moves(&board);

        assert!(legal_moves.is_none());
    }

    #[test]
    fn test_rook_free_board() {
        let mut board = Board::init_empty();
        board.board[5][4] = Some(Piece::new(PieceType::Rook, PieceColor::Black));
        board.selected_position = Some(Position { x: 5, y: 4 });

        let legal_moves = Rook::available_moves(&board).unwrap();

        let expected_moves = vec![
            Position { x: 4, y: 4 },
            Position { x: 3, y: 4 },
            Position { x: 2, y: 4 },
            Position { x: 1, y: 4 },
            Position { x: 0, y: 4 },
            Position { x: 6, y: 4 },
            Position { x: 7, y: 4 },
            Position { x: 5, y: 3 },
            Position { x: 5, y: 2 },
            Position { x: 5, y: 1 },
            Position { x: 5, y: 0 },
            Position { x: 5, y: 5 },
            Position { x: 5, y: 6 },
            Position { x: 5, y: 7 },
        ];

        assert_eq!(legal_moves.len(), expected_moves.len());

        for position in expected_moves.iter() {
            assert!(legal_moves.contains(position));
        }
    }

    #[test]
    fn test_rook_moves_with_capture() {
        let mut board = Board::init_empty();
        board.selected_position = Some(Position { x: 5, y: 4 });
        board.board[5][4] = Some(Piece::new(PieceType::Rook, PieceColor::Black));
        board.board[3][4] = Some(Piece::new(PieceType::Pawn, PieceColor::White));

        let legal_moves = Rook::available_moves(&board).unwrap();

        let expected_moves = vec![
            Position { x: 5, y: 0 },
            Position { x: 5, y: 1 },
            Position { x: 5, y: 2 },
            Position { x: 5, y: 3 },
            Position { x: 5, y: 5 },
            Position { x: 5, y: 6 },
            Position { x: 5, y: 7 },
            Position { x: 3, y: 4 },
            Position { x: 4, y: 4 },
            Position { x: 6, y: 4 },
            Position { x: 7, y: 4 },
        ];

        for position in expected_moves.iter() {
            assert!(legal_moves.contains(position));
        }
    }

    #[test]
    fn test_rook_moves_with_captures_and_blocking_piece() {
        let mut board = Board::init_empty();
        board.selected_position = Some(Position { x: 5, y: 4 });
        board.board[5][4] = Some(Piece::new(PieceType::Rook, PieceColor::Black));
        board.board[6][4] = Some(Piece::new(PieceType::Pawn, PieceColor::Black));
        board.board[3][4] = Some(Piece::new(PieceType::Pawn, PieceColor::White));
        board.board[5][1] = Some(Piece::new(PieceType::Knight, PieceColor::White));
        board.board[5][5] = Some(Piece::new(PieceType::Knight, PieceColor::White));

        let legal_moves = Rook::available_moves(&board).unwrap();

        let expected_moves = vec![
            Position { x: 5, y: 1 },
            Position { x: 5, y: 2 },
            Position { x: 5, y: 3 },
            Position { x: 5, y: 5 },
            Position { x: 3, y: 4 },
            Position { x: 4, y: 4 },
        ];

        for position in expected_moves.iter() {
            assert!(legal_moves.contains(position));
        }
    }
}
