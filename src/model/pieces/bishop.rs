use crate::model::{explore_moves, Board, Move, Position};

pub struct Bishop;

impl Move for Bishop {
    fn available_moves(board: &Board) -> Option<Vec<Position>> {
        board.selected_position.as_ref()?;
        let selected_position = board.selected_position.clone().unwrap();
        let mut legal_moves: Vec<Position> = Vec::with_capacity(13);
        let color = board
            .get_piece_color_in_position(selected_position.clone())
            .unwrap();

        // Explore moves in all four directions using recursion
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
    fn test_bishop_initial_position() {
        let mut board = Board::default();
        board.selected_position = Some(Position { x: 7, y: 2 });

        let legal_moves = Bishop::available_moves(&board);

        assert!(legal_moves.is_none());
    }

    #[test]
    fn test_bishop_free_board() {
        let mut board = Board::init_empty();
        board.board[5][4] = Some(Piece::new(PieceType::Bishop, PieceColor::White));
        board.selected_position = Some(Position { x: 5, y: 4 });

        let legal_moves = Bishop::available_moves(&board).unwrap();

        let expected_moves = vec![
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

        assert_eq!(legal_moves.len(), 11);

        for position in expected_moves.iter() {
            assert!(legal_moves.contains(position));
        }
    }

    #[test]
    fn test_bishop_moves_with_capture() {
        let mut board = Board::init_empty();
        board.selected_position = Some(Position { x: 5, y: 4 });
        board.board[5][4] = Some(Piece::new(PieceType::Bishop, PieceColor::Black));
        board.board[3][2] = Some(Piece::new(PieceType::Pawn, PieceColor::White));

        let legal_moves = Bishop::available_moves(&board).unwrap();

        let expected_moves = vec![
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

        assert_eq!(legal_moves.len(), 9);

        for position in expected_moves.iter() {
            assert!(legal_moves.contains(position));
        }
    }

    #[test]
    fn test_bishop_with_captures_and_blocking_piece() {
        let mut board = Board::init_empty();
        board.selected_position = Some(Position { x: 5, y: 4 });
        board.board[5][4] = Some(Piece::new(PieceType::Bishop, PieceColor::Black));
        board.board[3][2] = Some(Piece::new(PieceType::Pawn, PieceColor::White));
        board.board[6][3] = Some(Piece::new(PieceType::Pawn, PieceColor::Black));
        board.board[2][7] = Some(Piece::new(PieceType::Knight, PieceColor::Black));

        let legal_moves = Bishop::available_moves(&board).unwrap();

        let expected_moves = vec![
            Position { x: 3, y: 2 },
            Position { x: 4, y: 3 },
            Position { x: 6, y: 5 },
            Position { x: 7, y: 6 },
            Position { x: 4, y: 5 },
            Position { x: 3, y: 6 },
        ];

        assert_eq!(legal_moves.len(), 6);

        for position in expected_moves.iter() {
            assert!(legal_moves.contains(position));
        }
    }
}
