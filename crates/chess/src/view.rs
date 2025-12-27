use crate::game::{ChessGame, DrawReason, GameResult};
use crate::types::{PieceType, PlayerColor};
use vitae::prelude::*;

fn move_list(game: &ChessGame) -> ElementBuilder {
    let moves: Vec<String> = game
        .history
        .chunks(2)
        .enumerate()
        .map(|(i, pair)| {
            let move_num = i + 1;
            let white_move = &pair[0].notation;
            let black_move = pair.get(1).map(|m| m.notation.as_str()).unwrap_or("");
            format!("{}. {} {}", move_num, white_move, black_move)
        })
        .collect();

    div().col().w(FULL).children(
        moves
            .into_iter()
            .map(|line| text(line).color(Color::from_hex("#b0b0b0"))),
    )
}

fn promotion_ui() -> ElementBuilder {
    let promotion_button = |label: &str, piece_type: PieceType| {
        div()
            .bg(Color::from_hex("#5a5a5a"))
            .p(px(8.0))
            .child(text(label).color(Color::from_hex("#e0e0e0")))
            .on_left_click(move |g: &mut ChessGame| g.promote_to(piece_type))
    };

    div()
        .col()
        .w(FULL)
        .bg(Color::from_hex("#4a4a4a"))
        .p(px(8.0))
        .child(text("Promote pawn to:").color(Color::from_hex("#ffcc00")))
        .child(
            div()
                .row()
                .w(FULL)
                .child(promotion_button("Queen", PieceType::Queen))
                .child(promotion_button("Rook", PieceType::Rook))
                .child(promotion_button("Bishop", PieceType::Bishop))
                .child(promotion_button("Knight", PieceType::Knight)),
        )
}

fn debug_menu() -> ElementBuilder {
    let fen_button = |label: &str, fen: &'static str| {
        div()
            .bg(Color::from_hex("#3a3a5a"))
            .p(px(4.0))
            .child(text(label).color(Color::from_hex("#c0c0e0")))
            .on_left_click(move |g: &mut ChessGame| g.load_fen(fen))
    };

    div()
        .col()
        .w(FULL)
        .bg(Color::from_hex("#2a2a3a"))
        .p(px(8.0))
        .gap(px(4.0))
        .child(text("Debug Positions").color(Color::from_hex("#8080a0")))
        .child(fen_button(
            "White promotes",
            "8/4P3/8/8/8/8/8/4K2k w - - 0 1",
        ))
        .child(fen_button(
            "Black promotes",
            "4k2K/8/8/8/8/8/4p3/8 b - - 0 1",
        ))
        .child(fen_button(
            "Castling available",
            "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1",
        ))
        .child(fen_button(
            "En passant",
            "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1",
        ))
        .child(fen_button(
            "Checkmate in 1",
            "6k1/5ppp/8/8/8/8/8/R3K3 w Q - 0 1",
        ))
        .child(fen_button("Stalemate", "k7/8/1K6/8/8/8/8/8 b - - 0 1"))
        .child(fen_button("King vs King", "4k3/8/8/8/8/8/8/4K3 w - - 0 1"))
}

fn checkerboard_colors(x: usize, y: usize) -> (Color, Color) {
    let light_square = Color::rgb(242, 229, 229);
    let dark_square = Color::rgb(163, 82, 76);

    if ((x + y) & 1) == 0 {
        (light_square, dark_square) // (bg, text)
    } else {
        (dark_square, light_square) // (bg, text)
    }
}

pub fn view(game: &ChessGame) -> ElementBuilder {
    let hover = use_signal(|| None::<(usize, usize)>);

    let flipped = game.flip_board && game.turn == PlayerColor::Black;
    let king_in_check = game.king_in_check();

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
                    let row = if flipped { 7 - view_row } else { view_row };
                    let col = if flipped { 7 - view_col } else { view_col };

                    let (bg_color, label_color) = checkerboard_colors(row, col);
                    let mut square = div().bg(bg_color).w(pc(100. / 8.)).h(FULL);

                    // Highlight king in check
                    if king_in_check == Some((row, col)) {
                        square = square.bg(Color::rgb(220, 60, 60));
                    }

                    if game.selected == Some((row, col)) {
                        square = square.bg(Color::rgb(100, 200, 100));
                    }

                    if hover.get() == Some((row, col)) {
                        square = square.bg(Color::rgb(200, 200, 100));
                    }

                    let is_valid_target = if let Some((sel_row, sel_col)) = game.selected {
                        game.is_valid_move(sel_row, sel_col, row, col)
                    } else {
                        false
                    };

                    if let Some(piece) = game.board[row][col] {
                        if is_valid_target {
                            square = square.bg(Color::rgb(200, 80, 80));
                        }
                        if let Some(piece_svg) = game.pieces.get(&piece) {
                            square = square.center().child(svg(piece_svg).size(pc(80.0)));
                        }
                    } else if is_valid_target {
                        square = square.center().child(
                            div()
                                .rounded()
                                .opacity(0.5)
                                .bg(Color::rgb(50, 50, 50))
                                .size(pc(30.0)),
                        );
                    }

                    // Rank number (1-8) in top-left of left edge squares
                    if view_col == 0 {
                        let rank = 8 - row;
                        square = square.child(
                            text(rank.to_string())
                                .color(label_color)
                                .absolute()
                                .top(px(2.0))
                                .left(px(2.0)),
                        );
                    }

                    // File letter (a-h) in bottom-right of bottom edge squares
                    if view_row == 7 {
                        let file = (b'a' + col as u8) as char;
                        square = square.child(
                            text(file.to_string())
                                .color(label_color)
                                .absolute()
                                .bottom(px(2.0))
                                .right(px(2.0)),
                        );
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
                Color::from_hex("#4a6a4a")
            } else {
                Color::from_hex("#444444")
            })
            .child(text(format!("{}{}", name, score_str)).color(Color::from_hex("#e0e0e0")))
            .child(
                text(if captured_str.is_empty() {
                    "-".to_string()
                } else {
                    captured_str
                })
                .color(Color::from_hex("#b0b0b0")),
            )
    };

    let button = |label: &str| {
        div()
            .bg(Color::from_hex("#444444"))
            .p(px(8.0))
            .child(text(label).color(Color::from_hex("#e0e0e0")))
    };

    let auto_flip_label = if game.flip_board {
        "Auto-Flip: On"
    } else {
        "Auto-Flip: Off"
    };

    let side_panel = div()
        .size(FULL)
        .border_l(2.0, Color::from_hex("#4a4a4a"))
        .bg(Color::from_hex("#333333"))
        .p(px(12.0))
        .col()
        .child(text("Chess").color(Color::from_hex("#e0e0e0")))
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
            text(format!("Last: {}", last_move)).color(Color::from_hex("#b0b0b0"))
        } else {
            text("No moves yet").color(Color::from_hex("#b0b0b0"))
        })
        .child(match game.result {
            GameResult::Checkmate(winner) => {
                let winner_name = match winner {
                    PlayerColor::White => "White",
                    PlayerColor::Black => "Black",
                };
                text(format!("Checkmate! {} wins", winner_name)).color(Color::from_hex("#ffcc00"))
            }
            GameResult::Draw(reason) => {
                let reason_str = match reason {
                    DrawReason::Stalemate => "Stalemate",
                    DrawReason::InsufficientMaterial => "Insufficient material",
                    DrawReason::FiftyMoveRule => "Fifty-move rule",
                    DrawReason::ThreefoldRepetition => "Threefold repetition",
                };
                text(format!("Draw - {}", reason_str)).color(Color::from_hex("#ffcc00"))
            }
            GameResult::Ongoing => text(""),
        })
        .child(if game.is_awaiting_promotion() {
            promotion_ui()
        } else {
            div()
        })
        .child(
            div()
                .row()
                .w(FULL)
                .child(button("Undo").on_left_click(|g: &mut ChessGame| g.undo()))
                .child(button("Redo").on_left_click(|g: &mut ChessGame| g.redo())),
        )
        .child(
            div()
                .row()
                .w(FULL)
                .child(button("Reset").on_left_click(|g: &mut ChessGame| g.reset()))
                .child(
                    button(auto_flip_label).on_left_click(|g: &mut ChessGame| g.toggle_auto_flip()),
                ),
        )
        .child(move_list(game))
        .child({
            let show_debug = use_signal(|| false);
            let toggle_label = if show_debug.get() {
                "Hide Debug"
            } else {
                "Show Debug"
            };
            div()
                .col()
                .w(FULL)
                .child(
                    div()
                        .bg(Color::from_hex("#3a3a3a"))
                        .p(px(8.0))
                        .child(text(toggle_label).color(Color::from_hex("#a0a0a0")))
                        .on_left_click(move |_: &mut ChessGame| show_debug.set(!show_debug.get())),
                )
                .child(if show_debug.get() {
                    debug_menu()
                } else {
                    div()
                })
        });

    div().size(FULL).row().child(chessboard).child(side_panel)
}
