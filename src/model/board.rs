use std::fmt::Display;

use super::{Direction, Piece, PieceColor, PieceType, Position};

#[derive(Debug, Clone)]
pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
    pub on_turn: PieceColor,
    pub cursor_position: Position,
    pub currently_legal_moves: Option<Vec<Position>>,
    pub is_check: bool,
    captured_pieces: Vec<Piece>,
    pub selected_position: Option<Position>,
    pub debugger: Vec<String>,
}

impl Board {
    pub fn is_cell_empty(&self, position: Position) -> bool {
        self.board[position.x][position.y].is_none()
    }

    pub(super) fn get_piece_color_in_position(&self, position: Position) -> Option<PieceColor> {
        self.board[position.x][position.y]
            .as_ref()
            .map(|piece| piece.piece_color)
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.cursor_position.x == 0 {
                    self.cursor_position.x = 7;
                } else {
                    self.cursor_position.x -= 1;
                }
            }
            Direction::Down => {
                if self.cursor_position.x == 7 {
                    self.cursor_position.x = 0;
                } else {
                    self.cursor_position.x += 1;
                }
            }
            Direction::Left => {
                if self.cursor_position.y == 0 {
                    self.cursor_position.y = 7;
                } else {
                    self.cursor_position.y -= 1;
                }
            }
            Direction::Right => {
                if self.cursor_position.y == 7 {
                    self.cursor_position.y = 0;
                } else {
                    self.cursor_position.y += 1;
                }
            }
        }
    }

    pub fn is_getting_checked(&mut self) -> bool {
        let current_player_color = self.on_turn;
        let mut opponent_king_position: Position = Position { x: 0, y: 0 };

        let mut is_check = false;

        // Locate opponent's king
        self.board.iter().enumerate().for_each(|(x, row)| {
            row.iter().enumerate().for_each(|(y, cell)| {
                if let Some(piece) = cell {
                    if piece.piece_type == PieceType::King
                        && piece.piece_color != current_player_color
                    {
                        opponent_king_position = Position { x, y };
                    }
                }
            })
        });

        let mut simulated_board = self.clone();

        // Determine whether king gets attacked
        simulated_board
            .board
            .into_iter()
            .enumerate()
            .for_each(|(i, row)| {
                row.iter().enumerate().for_each(|(j, cell)| {
                    if let Some(piece) = cell {
                        simulated_board.select_position(&Position { x: i, y: j });
                        if let Some(legal_moves) = piece.piece_type.legal_moves(&simulated_board) {
                            if legal_moves.contains(&opponent_king_position) {
                                is_check = true;
                            }
                        }
                    }
                })
            });
        is_check
    }

    pub fn select_position(&mut self, position: &Position) {
        match self.board[position.x][position.y] {
            Some(piece) => {
                if piece.piece_color == self.on_turn {
                    self.selected_position = Some(position.clone());
                    self.currently_legal_moves = piece.piece_type.legal_moves(self);
                } else {
                    self.currently_legal_moves = None;
                }
            }
            None => {
                self.selected_position = None;
                self.currently_legal_moves = None;
            }
        }
    }

    pub fn move_piece(&mut self, to: &Position) {
        if let Some(from) = &self.selected_position {
            if let Some(selected_piece) = self.board[from.x][from.y] {
                if let Some(legal_moves) = &self.currently_legal_moves {
                    if legal_moves.contains(to) {
                        // Set current position to empty
                        self.board[from.x][from.y] = None;

                        // If capture, add the captured piece to captured_pieces
                        if let Some(piece) = self.board[to.x][to.y] {
                            self.captured_pieces.push(piece);
                        }
                        // Place selected piece to new position
                        self.board[to.x][to.y] = Some(selected_piece);

                        self.selected_position = None;
                        self.currently_legal_moves = None;

                        if self.is_getting_checked() {
                            self.is_check = true;
                        }

                        self.switch_turn();
                    }
                }
            }
        }
    }

    fn switch_turn(&mut self) {
        match self.on_turn {
            PieceColor::White => self.on_turn = PieceColor::Black,
            PieceColor::Black => self.on_turn = PieceColor::White,
        }
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
            on_turn: PieceColor::White,
            cursor_position: Position { x: 7, y: 0 },
            selected_position: None,
            currently_legal_moves: None,
            is_check: false,
            captured_pieces: Vec::with_capacity(32),
            debugger: vec![],
        }
    }
}

impl Board {
    #[cfg(test)]
    pub fn init_empty() -> Self {
        Self {
            board: [[None; 8]; 8],
            on_turn: PieceColor::White,
            cursor_position: Position { x: 7, y: 0 },
            selected_position: None,
            currently_legal_moves: None,
            is_check: false,
            captured_pieces: Vec::with_capacity(32),
            debugger: vec![]
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
        writeln!(f, "On turn: {:?}", self.on_turn)?;
        writeln!(f, "Cursor position: {:?}", self.cursor_position)?;
        writeln!(f, "Selected position: {:?}", self.selected_position)?;
        writeln!(f, "Is check: {}", self.is_check)?; 
        writeln!(f, "Prints:")?;
        for line in &self.debugger {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}
