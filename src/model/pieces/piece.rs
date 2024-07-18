use std::fmt::Display;

use crate::model::Board;

use super::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook};

pub trait Move {
    fn available_moves(board: &Board) -> Option<Vec<Position>>;
}

pub fn explore_moves(
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
            explore_moves(&new_position, legal_moves, board, color, dx, dy);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

impl PieceType {
    pub fn legal_moves(&self, board: &Board) -> Option<Vec<Position>> {
        match self {
            Self::King => King::available_moves(board),
            Self::Rook => Rook::available_moves(board),
            Self::Pawn => Pawn::available_moves(board),
            Self::Bishop => Bishop::available_moves(board),
            Self::Knight => Knight::available_moves(board),
            Self::Queen => Queen::available_moves(board),
        }
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceType::King => write!(f, "K"),
            PieceType::Queen => write!(f, "Q"),
            PieceType::Rook => write!(f, "R"),
            PieceType::Bishop => write!(f, "B"),
            PieceType::Knight => write!(f, "N"),
            PieceType::Pawn => write!(f, "p"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceColor {
    Black,
    White,
}

impl Display for PieceColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceColor::White => write!(f, "w"),
            PieceColor::Black => write!(f, "b"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub piece_color: PieceColor,
}

impl Piece {
    pub fn new(piece_type: PieceType, piece_color: PieceColor) -> Self {
        Self {
            piece_type,
            piece_color,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.piece_color, self.piece_type)
    }
}
