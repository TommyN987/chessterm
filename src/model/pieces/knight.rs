use crate::model::{Board, Move, Position};

pub struct Knight;

impl Move for Knight {
    fn available_moves(board: &Board) -> Option<Vec<Position>> {
        board.selected_position.as_ref()?;

        let (x, y) = match &board.selected_position {
            Some(pos) => (pos.x, pos.y),
            None => return None,
        };

        let selected_piece_color = board.get_piece_color_in_position(Position { x, y })?;

        let mut legal_moves: Vec<Position> = Vec::with_capacity(8);

        let potential_moves: Vec<(isize, isize)> = vec![
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
        ];

        for (dx, dy) in potential_moves {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if (0..=7).contains(&new_x) && (0..=7).contains(&new_y) {
                let new_position = Position {
                    x: new_x as usize,
                    y: new_y as usize,
                };

                match board.board[new_x as usize][new_y as usize] {
                    Some(piece) => {
                        if piece.piece_color != selected_piece_color {
                            legal_moves.push(new_position);
                        }
                    }
                    None => legal_moves.push(new_position),
                }
            }
        }

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
    fn test_knight_initial_moves() {
        let mut board = Board::default();
        board.select_position(&Position { x: 7, y: 1 });

        let legal_moves = Knight::available_moves(&board).unwrap();

        let expected_moves = vec![Position { x: 5, y: 0 }, Position { x: 5, y: 2 }];

        assert_eq!(legal_moves.len(), expected_moves.len());

        for position in expected_moves.iter() {
            assert!(legal_moves.contains(position));
        }
    }

    #[test]
    fn test_knight_no_legal_moves() {
        let mut board = Board::default();
        board.select_position(&Position { x: 7, y: 1 });
        board.board[5][0] = Some(Piece::new(PieceType::Pawn, PieceColor::White));
        board.board[5][2] = Some(Piece::new(PieceType::Pawn, PieceColor::White));

        let legal_moves = Knight::available_moves(&board);

        assert!(legal_moves.is_none());
    }

    #[test]
    fn test_knight_free_board() {
        let mut board = Board::init_empty();
        board.board[4][4] = Some(Piece::new(PieceType::Knight, PieceColor::White));
        board.select_position(&Position { x: 4, y: 4 });

        let legal_moves = Knight::available_moves(&board).unwrap();

        let expected_moves = vec![
            Position { x: 2, y: 3 },
            Position { x: 2, y: 5 },
            Position { x: 3, y: 6 },
            Position { x: 3, y: 2 },
            Position { x: 5, y: 6 },
            Position { x: 5, y: 2 },
            Position { x: 6, y: 3 },
            Position { x: 6, y: 5 },
        ];

        assert_eq!(legal_moves.len(), expected_moves.len());

        for position in expected_moves.iter() {
            assert!(legal_moves.contains(position));
        }
    }

    #[test]
    fn test_knight_with_captures_and_blocking_pieces() {
        let mut board = Board::init_empty();
        board.board[4][4] = Some(Piece::new(PieceType::Knight, PieceColor::White));
        board.board[6][5] = Some(Piece::new(PieceType::Pawn, PieceColor::White));
        board.board[5][6] = Some(Piece::new(PieceType::Pawn, PieceColor::White));
        board.board[3][2] = Some(Piece::new(PieceType::Knight, PieceColor::Black));
        board.board[2][3] = Some(Piece::new(PieceType::Knight, PieceColor::Black));
        board.select_position(&Position { x: 4, y: 4 });

        let legal_moves = Knight::available_moves(&board).unwrap();

        let expected_moves = vec![
            Position { x: 2, y: 3 },
            Position { x: 2, y: 5 },
            Position { x: 3, y: 6 },
            Position { x: 3, y: 2 },
            Position { x: 5, y: 2 },
            Position { x: 6, y: 3 },
        ];

        assert_eq!(legal_moves.len(), expected_moves.len());

        for position in expected_moves.iter() {
            assert!(legal_moves.contains(position));
        }
    }
}
