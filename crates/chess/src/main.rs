mod board;
mod types;

use types::{Piece, PlayerColor};
use vitae::prelude::*;

#[derive(Clone)]
struct ChessGame {
    board: [[Option<Piece>; 8]; 8],
    selected: Option<(usize, usize)>,
    last_move: Option<String>,
    turn: PlayerColor,
}

impl ChessGame {
    fn new() -> Self {
        Self {
            board: board::setup_initial_board(),
            selected: None,
            last_move: None,
            turn: PlayerColor::White,
        }
    }

    fn is_valid_move(
        &self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> bool {
        // Disallow move to the same square
        if from_row == to_row && from_col == to_col {
            return false;
        }

        true
    }

    fn select_square(&mut self, row: usize, col: usize) {
        if let Some((selected_row, selected_col)) = self.selected {
            // Validate the move
            if !self.is_valid_move(selected_row, selected_col, row, col) {
                self.selected = None;
                return;
            }

            // Move piece
            self.board[row][col] = self.board[selected_row][selected_col].take();
            self.last_move = Some(format!(
                "{}{}-{}{}",
                (b'a' + selected_col as u8) as char,
                8 - selected_row,
                (b'a' + col as u8) as char,
                8 - row
            ));
            self.selected = None;
            self.turn = self.turn.opposite();
        } else {
            // Select piece (only if there's a piece and it's your turn)
            if let Some(piece) = self.board[row][col] {
                if piece.color == self.turn {
                    self.selected = Some((row, col));
                }
            }
        }
    }
}

fn checkerboard(x: usize, y: usize) -> Color {
    let light_square = Color::rgb(242, 229, 229);
    let dark_square = Color::rgb(163, 82, 76);

    if ((x + y) & 1) == 0 {
        light_square
    } else {
        dark_square
    }
}

fn view(game: &ChessGame) -> ElementBuilder {
    // Use signal for hover state
    let hover = use_signal(|| None::<(usize, usize)>);

    let chessboard = div().h(FULL).square().col().children((0..8).map(|row| {
        div()
            .row()
            .h(pc(100. / 8.))
            .w(FULL)
            .children((0..8).map(move |col| {
                let mut square = div().bg(checkerboard(row, col)).w(pc(100. / 8.)).h(FULL);

                // Highlight selected square
                if game.selected == Some((row, col)) {
                    square = square.bg(Color::rgb(100, 200, 100));
                }

                // Highlight hovered square (using signal!)
                if hover.get() == Some((row, col)) {
                    square = square.bg(Color::rgb(200, 200, 100));
                }

                // Add piece if present
                if let Some(piece) = game.board[row][col] {
                    square = square.child(text(piece.unicode()).font_size(64.0));
                }

                square.on_left_click(move |g: &mut ChessGame| g.select_square(row, col))
            }))
    }));

    let side_panel = div()
        .size(FULL)
        .bg(WHITE)
        .p(px(12.0))
        .col()
        .child(text("Chess"))
        .child(text(match game.turn {
            PlayerColor::White => "White to move",
            PlayerColor::Black => "Black to move",
        }))
        .child(if let Some(ref last_move) = game.last_move {
            text(format!("Last move: {}", last_move))
        } else {
            text("No moves yet")
        });

    div().size(FULL).row().child(chessboard).child(side_panel)
}

fn main() {
    let app = App::new(ChessGame::new(), view);
    app.run();
}
