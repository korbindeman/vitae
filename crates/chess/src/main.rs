use vitae::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum PlayerColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Piece {
    piece_type: PieceType,
    color: PlayerColor,
}

impl Piece {
    fn unicode(&self) -> &'static str {
        match (self.color, self.piece_type) {
            (PlayerColor::White, PieceType::King) => "♔",
            (PlayerColor::White, PieceType::Queen) => "♕",
            (PlayerColor::White, PieceType::Rook) => "♖",
            (PlayerColor::White, PieceType::Bishop) => "♗",
            (PlayerColor::White, PieceType::Knight) => "♘",
            (PlayerColor::White, PieceType::Pawn) => "♙",
            (PlayerColor::Black, PieceType::King) => "♚",
            (PlayerColor::Black, PieceType::Queen) => "♛",
            (PlayerColor::Black, PieceType::Rook) => "♜",
            (PlayerColor::Black, PieceType::Bishop) => "♝",
            (PlayerColor::Black, PieceType::Knight) => "♞",
            (PlayerColor::Black, PieceType::Pawn) => "♟",
        }
    }
}

#[derive(Clone)]
struct ChessGame {
    board: [[Option<Piece>; 8]; 8],
    selected: Option<(usize, usize)>,
    last_move: Option<String>,
    turn: PlayerColor,
}

impl PlayerColor {
    fn opposite(self) -> Self {
        match self {
            PlayerColor::White => PlayerColor::Black,
            PlayerColor::Black => PlayerColor::White,
        }
    }
}

impl ChessGame {
    fn new() -> Self {
        let mut board = [[None; 8]; 8];

        // Set up black pieces (row 0 and 1)
        board[0] = [
            Some(Piece { piece_type: PieceType::Rook, color: PlayerColor::Black }),
            Some(Piece { piece_type: PieceType::Knight, color: PlayerColor::Black }),
            Some(Piece { piece_type: PieceType::Bishop, color: PlayerColor::Black }),
            Some(Piece { piece_type: PieceType::Queen, color: PlayerColor::Black }),
            Some(Piece { piece_type: PieceType::King, color: PlayerColor::Black }),
            Some(Piece { piece_type: PieceType::Bishop, color: PlayerColor::Black }),
            Some(Piece { piece_type: PieceType::Knight, color: PlayerColor::Black }),
            Some(Piece { piece_type: PieceType::Rook, color: PlayerColor::Black }),
        ];
        for col in 0..8 {
            board[1][col] = Some(Piece { piece_type: PieceType::Pawn, color: PlayerColor::Black });
        }

        // Set up white pieces (row 6 and 7)
        for col in 0..8 {
            board[6][col] = Some(Piece { piece_type: PieceType::Pawn, color: PlayerColor::White });
        }
        board[7] = [
            Some(Piece { piece_type: PieceType::Rook, color: PlayerColor::White }),
            Some(Piece { piece_type: PieceType::Knight, color: PlayerColor::White }),
            Some(Piece { piece_type: PieceType::Bishop, color: PlayerColor::White }),
            Some(Piece { piece_type: PieceType::Queen, color: PlayerColor::White }),
            Some(Piece { piece_type: PieceType::King, color: PlayerColor::White }),
            Some(Piece { piece_type: PieceType::Bishop, color: PlayerColor::White }),
            Some(Piece { piece_type: PieceType::Knight, color: PlayerColor::White }),
            Some(Piece { piece_type: PieceType::Rook, color: PlayerColor::White }),
        ];

        Self {
            board,
            selected: None,
            last_move: None,
            turn: PlayerColor::White,
        }
    }

    fn select_square(&mut self, row: usize, col: usize) {
        if let Some((selected_row, selected_col)) = self.selected {
            // Move piece (simplified - no validation)
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
                let mut square = div()
                    .bg(checkerboard(row, col))
                    .w(pc(100. / 8.))
                    .h(FULL);

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
                    square = square.child(
                        text(piece.unicode())
                            .font_size(48.0)
                    );
                }

                square.on_click(move |g: &mut ChessGame| g.select_square(row, col))
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
        .child(
            if let Some(ref last_move) = game.last_move {
                text(format!("Last move: {}", last_move))
            } else {
                text("No moves yet")
            }
        );

    div().size(FULL).row().child(chessboard).child(side_panel)
}

fn main() {
    let app = App::new(ChessGame::new(), view);
    app.run();
}
