mod assets;
mod board;
mod check;
mod fen;
mod game;
mod moves;
mod types;
mod view;

use game::ChessGame;
use vitae::prelude::*;

fn main() {
    let app = App::new(ChessGame::new(), view::view);
    app.run();
}
