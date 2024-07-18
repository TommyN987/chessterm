use crate::model::{explore_moves, Board, Move, Position};

pub struct Queen;

impl Move for Queen {
    fn available_moves(board: &Board) -> Option<Vec<Position>> {
        let selected_position = board.selected_position.clone()?;
        let mut legal_moves: Vec<Position> = Vec::with_capacity(27);
        let color = board
            .get_piece_color_in_position(selected_position.clone())
            .unwrap();

        // Explore moves in all four directions using recursion
        explore_moves(&selected_position, &mut legal_moves, board, color, 1, 0);
        explore_moves(&selected_position, &mut legal_moves, board, color, -1, 0);
        explore_moves(&selected_position, &mut legal_moves, board, color, 0, 1);
        explore_moves(&selected_position, &mut legal_moves, board, color, 0, -1);
        explore_moves(&selected_position, &mut legal_moves, board, color, 1, 1);
        explore_moves(&selected_position, &mut legal_moves, board, color, -1, 1);
        explore_moves(&selected_position, &mut legal_moves, board, color, -1, -1);
        explore_moves(&selected_position, &mut legal_moves, board, color, 1, -1);

        if legal_moves.is_empty() {
            None
        } else {
            Some(legal_moves)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{Piece, PieceColor, PieceType};

    use super::*;

    #[test]
    fn test_queen_initial_position() {
        let mut board = Board::default();
        board.select_position(&Position { x: 0, y: 0 });

        let legal_moves = Queen::available_moves(&board);

        assert!(legal_moves.is_none());
    }

    #[test]
    fn test_queen_free_board() {
        let mut board = Board::init_empty();
        board.on_turn = PieceColor::Black;
        board.board[5][4] = Some(Piece::new(PieceType::Queen, PieceColor::Black));
        board.select_position(&Position { x: 5, y: 4 });

        let legal_moves = Queen::available_moves(&board).unwrap();

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
            Position { x: 1, y: 0 },
            Position { x: 2, y: 1 },
            Position { x: 3, y: 2 },
            Position { x: 4, y: 3 },
            Position { x: 6, y: 5 },
            Position { x: 7, y: 6 },
            Position { x: 7, y: 2 },
            Position { x: 6, y: 3 },
            Position { x: 4, y: 5 },
            Position { x: 3, y: 6 },
            Position { x: 2, y: 7 },
        ];

        assert_eq!(legal_moves.len(), expected_moves.len());

        for position in expected_moves.iter() {
            assert!(legal_moves.contains(position));
        }
    }

    #[test]
    fn test_queen_moves_with_capture() {
        let mut board = Board::init_empty();
        board.on_turn = PieceColor::Black;
        board.board[5][4] = Some(Piece::new(PieceType::Queen, PieceColor::Black));
        board.board[3][4] = Some(Piece::new(PieceType::Pawn, PieceColor::White));
        board.select_position(&Position { x: 5, y: 4 });

        let legal_moves = Queen::available_moves(&board).unwrap();

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
            Position { x: 1, y: 0 },
            Position { x: 2, y: 1 },
            Position { x: 3, y: 2 },
            Position { x: 4, y: 3 },
            Position { x: 6, y: 5 },
            Position { x: 7, y: 6 },
            Position { x: 7, y: 2 },
            Position { x: 6, y: 3 },
            Position { x: 4, y: 5 },
            Position { x: 3, y: 6 },
            Position { x: 2, y: 7 },
        ];

        assert_eq!(legal_moves.len(), expected_moves.len());

        for position in expected_moves.iter() {
            assert!(legal_moves.contains(position));
        }
    }

    #[test]
    fn test_queen_moves_with_captures_and_blocking_piece() {
        let mut board = Board::init_empty();
        board.on_turn = PieceColor::Black;
        board.board[5][4] = Some(Piece::new(PieceType::Queen, PieceColor::Black));
        board.board[6][4] = Some(Piece::new(PieceType::Pawn, PieceColor::Black));
        board.board[3][4] = Some(Piece::new(PieceType::Pawn, PieceColor::White));
        board.board[5][1] = Some(Piece::new(PieceType::Knight, PieceColor::White));
        board.board[5][5] = Some(Piece::new(PieceType::Knight, PieceColor::White));
        board.select_position(&Position { x: 5, y: 4 });

        let legal_moves = Queen::available_moves(&board).unwrap();

        let expected_moves = vec![
            Position { x: 5, y: 1 },
            Position { x: 5, y: 2 },
            Position { x: 5, y: 3 },
            Position { x: 5, y: 5 },
            Position { x: 3, y: 4 },
            Position { x: 4, y: 4 },
            Position { x: 3, y: 2 },
            Position { x: 4, y: 3 },
            Position { x: 6, y: 5 },
            Position { x: 7, y: 6 },
            Position { x: 6, y: 3 },
            Position { x: 4, y: 5 },
            Position { x: 3, y: 6 },
        ];

        for position in expected_moves.iter() {
            assert!(legal_moves.contains(position));
        }
    }
}
