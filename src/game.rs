use std::error;

use crate::model::{menu::MenuState, Board, Direction};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub enum CurrentScreen {
    Menu,
    Game,
    Exit,
}

pub struct Game {
    pub is_running: bool,
    pub board: Board,
    pub current_screen: CurrentScreen,
    pub menu_state: MenuState,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            is_running: true,
            board: Board::default(),
            current_screen: CurrentScreen::Menu,
            menu_state: MenuState::default(),
        }
    }
}

impl Game {
    pub fn run(&mut self) {
        self.current_screen = CurrentScreen::Game;
    }

    pub fn move_menu_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.menu_state.previous(),
            Direction::Down => self.menu_state.next(),
            _ => {}
        }
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }

    pub fn tick(&self) {}
}
