mod board;
mod types;

use std::collections::HashMap;
use std::rc::Rc;

use types::{Piece, PieceType, PlayerColor};
use vitae::prelude::*;

#[derive(Clone)]
struct PieceSvgs {
    svgs: Rc<HashMap<(PieceType, PlayerColor), Svg>>,
}

impl PieceSvgs {
    fn load() -> Self {
        let pieces = [
            (PieceType::King, PlayerColor::White),
            (PieceType::Queen, PlayerColor::White),
            (PieceType::Rook, PlayerColor::White),
            (PieceType::Bishop, PlayerColor::White),
            (PieceType::Knight, PlayerColor::White),
            (PieceType::Pawn, PlayerColor::White),
            (PieceType::King, PlayerColor::Black),
            (PieceType::Queen, PlayerColor::Black),
            (PieceType::Rook, PlayerColor::Black),
            (PieceType::Bishop, PlayerColor::Black),
            (PieceType::Knight, PlayerColor::Black),
            (PieceType::Pawn, PlayerColor::Black),
        ];

        let mut svgs = HashMap::new();
        for (piece_type, color) in pieces {
            let piece = Piece { piece_type, color };
            let path = format!("crates/chess/assets/pieces/{}", piece.svg_filename());
            if let Ok(svg) = load_svg(&path) {
                svgs.insert((piece_type, color), svg);
            }
        }

        Self {
            svgs: Rc::new(svgs),
        }
    }

    fn get(&self, piece: &Piece) -> Option<&Svg> {
        self.svgs.get(&(piece.piece_type, piece.color))
    }
}

#[derive(Clone)]
struct ChessGame {
    board: [[Option<Piece>; 8]; 8],
    selected: Option<(usize, usize)>,
    last_move: Option<String>,
    turn: PlayerColor,
    pieces: PieceSvgs,
    flip_board: bool,
    captured_by_white: Vec<Piece>,
    captured_by_black: Vec<Piece>,
}

impl ChessGame {
    fn new() -> Self {
        Self {
            board: board::setup_initial_board(),
            selected: None,
            last_move: None,
            turn: PlayerColor::White,
            pieces: PieceSvgs::load(),
            flip_board: true,
            captured_by_white: Vec::new(),
            captured_by_black: Vec::new(),
        }
    }

    fn is_path_clear(
        &self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> bool {
        let row_step = (to_row as isize - from_row as isize).signum();
        let col_step = (to_col as isize - from_col as isize).signum();

        let mut row = from_row as isize + row_step;
        let mut col = from_col as isize + col_step;

        while (row, col) != (to_row as isize, to_col as isize) {
            if self.board[row as usize][col as usize].is_some() {
                return false;
            }
            row += row_step;
            col += col_step;
        }
        true
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

        let piece = match self.board[from_row][from_col] {
            Some(p) => p,
            None => return false,
        };

        // Can't capture your own piece
        if let Some(target) = self.board[to_row][to_col] {
            if target.color == piece.color {
                return false;
            }
        }

        let row_diff = (to_row as isize - from_row as isize).abs();
        let col_diff = (to_col as isize - from_col as isize).abs();
        let is_capture = self.board[to_row][to_col].is_some();

        match piece.piece_type {
            PieceType::Pawn => {
                let direction: isize = match piece.color {
                    PlayerColor::White => -1,
                    PlayerColor::Black => 1,
                };
                let start_row = match piece.color {
                    PlayerColor::White => 6,
                    PlayerColor::Black => 1,
                };

                let forward = to_row as isize - from_row as isize;

                // Standard one-square move
                if forward == direction && from_col == to_col && !is_capture {
                    return true;
                }

                // Two-square initial move
                if forward == 2 * direction
                    && from_col == to_col
                    && from_row == start_row
                    && !is_capture
                    && self.is_path_clear(from_row, from_col, to_row, to_col)
                {
                    return true;
                }

                // Diagonal capture
                if forward == direction && col_diff == 1 && is_capture {
                    return true;
                }

                false
            }
            PieceType::Rook => {
                if from_row != to_row && from_col != to_col {
                    return false;
                }
                self.is_path_clear(from_row, from_col, to_row, to_col)
            }
            PieceType::Bishop => {
                if row_diff != col_diff {
                    return false;
                }
                self.is_path_clear(from_row, from_col, to_row, to_col)
            }
            PieceType::Queen => {
                let is_straight = from_row == to_row || from_col == to_col;
                let is_diagonal = row_diff == col_diff;
                if !is_straight && !is_diagonal {
                    return false;
                }
                self.is_path_clear(from_row, from_col, to_row, to_col)
            }
            PieceType::Knight => {
                (row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2)
            }
            PieceType::King => row_diff <= 1 && col_diff <= 1,
        }
    }

    fn select_square(&mut self, row: usize, col: usize) {
        if let Some((selected_row, selected_col)) = self.selected {
            // Validate the move
            if !self.is_valid_move(selected_row, selected_col, row, col) {
                self.selected = None;
                return;
            }

            // Track capture
            if let Some(captured) = self.board[row][col] {
                match self.turn {
                    PlayerColor::White => self.captured_by_white.push(captured),
                    PlayerColor::Black => self.captured_by_black.push(captured),
                }
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

impl ChessGame {
    fn points_for(&self, color: PlayerColor) -> i32 {
        let captured = match color {
            PlayerColor::White => &self.captured_by_white,
            PlayerColor::Black => &self.captured_by_black,
        };
        captured.iter().map(|p| p.piece_type.points()).sum()
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

    // Flip board so current player is at the bottom
    let flipped = game.flip_board && game.turn == PlayerColor::Black;

    let chessboard = div()
        .h(FULL)
        .square()
        .col()
        .children((0..8).map(move |view_row| {
            div()
                .row()
                .h(pc(100. / 8.))
                .w(FULL)
                .children((0..8).map(move |view_col| {
                    // Map view coordinates to board coordinates
                    let row = if flipped { 7 - view_row } else { view_row };
                    let col = if flipped { 7 - view_col } else { view_col };

                    let mut square = div().bg(checkerboard(row, col)).w(pc(100. / 8.)).h(FULL);

                    // Highlight selected square
                    if game.selected == Some((row, col)) {
                        square = square.bg(Color::rgb(100, 200, 100));
                    }

                    // Highlight hovered square (using signal!)
                    if hover.get() == Some((row, col)) {
                        square = square.bg(Color::rgb(200, 200, 100));
                    }

                    // Show valid move indicator
                    let is_valid_target = if let Some((sel_row, sel_col)) = game.selected {
                        game.is_valid_move(sel_row, sel_col, row, col)
                    } else {
                        false
                    };

                    // Add piece if present
                    if let Some(piece) = game.board[row][col] {
                        // Highlight capturable pieces with red
                        if is_valid_target {
                            square = square.bg(Color::rgb(200, 80, 80));
                        }
                        if let Some(piece_svg) = game.pieces.get(&piece) {
                            square = square.center().child(svg(piece_svg).size(pc(80.0)));
                        }
                    } else if is_valid_target {
                        // Empty square - show dot
                        square = square
                            .center()
                            .child(div().bg(Color::rgb(50, 50, 50)).size(pc(30.0)));
                    }

                    square.on_left_click(move |g: &mut ChessGame| g.select_square(row, col))
                }))
        }));

    let player_panel = |name: &str, color: PlayerColor, is_turn: bool| {
        let my_points = game.points_for(color);
        let their_points = game.points_for(color.opposite());
        let diff = my_points - their_points;
        let captured = match color {
            PlayerColor::White => &game.captured_by_white,
            PlayerColor::Black => &game.captured_by_black,
        };
        let captured_str: String = captured.iter().map(|p| p.unicode()).collect();

        let score_str = if diff > 0 {
            format!(" (+{})", diff)
        } else {
            String::new()
        };

        div()
            .w(FULL)
            .col()
            .p(px(8.0))
            .bg(if is_turn {
                Color::rgb(220, 240, 220)
            } else {
                Color::rgb(240, 240, 240)
            })
            .child(text(format!("{}{}", name, score_str)))
            .child(text(if captured_str.is_empty() {
                "-".to_string()
            } else {
                captured_str
            }))
    };

    let side_panel = div()
        .size(FULL)
        .bg(Color::rgb(240, 240, 240))
        .p(px(12.0))
        .col()
        .child(text("Chess"))
        .child(player_panel(
            "Player 2 (Black)",
            PlayerColor::Black,
            game.turn == PlayerColor::Black,
        ))
        .child(player_panel(
            "Player 1 (White)",
            PlayerColor::White,
            game.turn == PlayerColor::White,
        ))
        .child(if let Some(ref last_move) = game.last_move {
            text(format!("Last: {}", last_move))
        } else {
            text("No moves yet")
        });

    div().size(FULL).row().child(chessboard).child(side_panel)
}

fn main() {
    let app = App::new(ChessGame::new(), view);
    app.run();
}
