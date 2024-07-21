use chessterm::{
    game::{AppResult, Game},
    update::message::{Message, MessageHandler},
    view::tui::Tui,
};

use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> AppResult<()> {
    let mut game = Game::default();
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let message_handler = MessageHandler::new(250);
    let mut tui = Tui::new(terminal, message_handler);

    tui.init()?;

    while game.is_running {
        tui.draw(&mut game)?;

        match tui.message_handler.next()? {
            Message::Tick => game.tick(),
            Message::KeyPress(key_event) => tui
                .message_handler
                .handle_key_events(key_event, &mut game)?,
            _ => {}
        }
    }

    tui.exit()?;

    Ok(())
}
