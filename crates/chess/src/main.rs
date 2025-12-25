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
    // En passant: column of pawn that just moved two squares (None if not applicable)
    en_passant_target: Option<usize>,
    // Castling rights
    white_king_moved: bool,
    black_king_moved: bool,
    white_rook_a_moved: bool,
    white_rook_h_moved: bool,
    black_rook_a_moved: bool,
    black_rook_h_moved: bool,
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
            en_passant_target: None,
            white_king_moved: false,
            black_king_moved: false,
            white_rook_a_moved: false,
            white_rook_h_moved: false,
            black_rook_a_moved: false,
            black_rook_h_moved: false,
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

    fn find_king(&self, color: PlayerColor) -> Option<(usize, usize)> {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.board[row][col] {
                    if piece.piece_type == PieceType::King && piece.color == color {
                        return Some((row, col));
                    }
                }
            }
        }
        None
    }

    /// Check if a square is attacked by any piece of the given color.
    /// This uses raw movement rules without recursion (no check validation).
    fn is_square_attacked(
        &self,
        target_row: usize,
        target_col: usize,
        by_color: PlayerColor,
    ) -> bool {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.board[row][col] {
                    if piece.color != by_color {
                        continue;
                    }
                    if self.can_piece_attack(row, col, target_row, target_col, piece) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Check if a piece can attack a target square (raw movement, no check validation).
    fn can_piece_attack(
        &self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
        piece: Piece,
    ) -> bool {
        if from_row == to_row && from_col == to_col {
            return false;
        }

        let row_diff = (to_row as isize - from_row as isize).abs();
        let col_diff = (to_col as isize - from_col as isize).abs();

        match piece.piece_type {
            PieceType::Pawn => {
                let direction: isize = match piece.color {
                    PlayerColor::White => -1,
                    PlayerColor::Black => 1,
                };
                let forward = to_row as isize - from_row as isize;
                // Pawns attack diagonally
                forward == direction && col_diff == 1
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

    fn is_in_check(&self, color: PlayerColor) -> bool {
        if let Some((king_row, king_col)) = self.find_king(color) {
            self.is_square_attacked(king_row, king_col, color.opposite())
        } else {
            false
        }
    }

    /// Check if making a move would leave the moving player's king in check.
    fn would_be_in_check(
        &self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> bool {
        let piece = match self.board[from_row][from_col] {
            Some(p) => p,
            None => return false,
        };

        // Create a temporary board state
        let mut temp_board = self.board;
        temp_board[to_row][to_col] = temp_board[from_row][from_col];
        temp_board[from_row][from_col] = None;

        // Handle en passant capture
        if piece.piece_type == PieceType::Pawn
            && from_col != to_col
            && self.board[to_row][to_col].is_none()
            && self.en_passant_target == Some(to_col)
        {
            temp_board[from_row][to_col] = None;
        }

        // Find king position (may have moved if we're moving the king)
        let king_pos = if piece.piece_type == PieceType::King {
            (to_row, to_col)
        } else {
            // Find king in temp board
            let mut pos = None;
            for row in 0..8 {
                for col in 0..8 {
                    if let Some(p) = temp_board[row][col] {
                        if p.piece_type == PieceType::King && p.color == piece.color {
                            pos = Some((row, col));
                            break;
                        }
                    }
                }
                if pos.is_some() {
                    break;
                }
            }
            match pos {
                Some(p) => p,
                None => return false,
            }
        };

        // Check if any enemy piece can attack the king
        let enemy_color = piece.color.opposite();
        for row in 0..8 {
            for col in 0..8 {
                if let Some(enemy) = temp_board[row][col] {
                    if enemy.color != enemy_color {
                        continue;
                    }
                    if self.can_piece_attack_on_board(
                        &temp_board,
                        row,
                        col,
                        king_pos.0,
                        king_pos.1,
                        enemy,
                    ) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Check if a piece can attack a target square on a given board state.
    fn can_piece_attack_on_board(
        &self,
        board: &[[Option<Piece>; 8]; 8],
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
        piece: Piece,
    ) -> bool {
        if from_row == to_row && from_col == to_col {
            return false;
        }

        let row_diff = (to_row as isize - from_row as isize).abs();
        let col_diff = (to_col as isize - from_col as isize).abs();

        match piece.piece_type {
            PieceType::Pawn => {
                let direction: isize = match piece.color {
                    PlayerColor::White => -1,
                    PlayerColor::Black => 1,
                };
                let forward = to_row as isize - from_row as isize;
                forward == direction && col_diff == 1
            }
            PieceType::Rook => {
                if from_row != to_row && from_col != to_col {
                    return false;
                }
                Self::is_path_clear_on_board(board, from_row, from_col, to_row, to_col)
            }
            PieceType::Bishop => {
                if row_diff != col_diff {
                    return false;
                }
                Self::is_path_clear_on_board(board, from_row, from_col, to_row, to_col)
            }
            PieceType::Queen => {
                let is_straight = from_row == to_row || from_col == to_col;
                let is_diagonal = row_diff == col_diff;
                if !is_straight && !is_diagonal {
                    return false;
                }
                Self::is_path_clear_on_board(board, from_row, from_col, to_row, to_col)
            }
            PieceType::Knight => {
                (row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2)
            }
            PieceType::King => row_diff <= 1 && col_diff <= 1,
        }
    }

    fn is_path_clear_on_board(
        board: &[[Option<Piece>; 8]; 8],
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
            if board[row as usize][col as usize].is_some() {
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

        let is_valid_pattern = match piece.piece_type {
            PieceType::Pawn => {
                let direction: isize = match piece.color {
                    PlayerColor::White => -1,
                    PlayerColor::Black => 1,
                };
                let start_row = match piece.color {
                    PlayerColor::White => 6,
                    PlayerColor::Black => 1,
                };
                let en_passant_row = match piece.color {
                    PlayerColor::White => 3,
                    PlayerColor::Black => 4,
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

                // En passant capture
                if forward == direction
                    && col_diff == 1
                    && from_row == en_passant_row
                    && self.en_passant_target == Some(to_col)
                {
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
            PieceType::King => {
                // Normal king move
                if row_diff <= 1 && col_diff <= 1 {
                    return true;
                }

                // Castling: king moves two squares horizontally
                if row_diff == 0 && col_diff == 2 {
                    let king_row = match piece.color {
                        PlayerColor::White => 7,
                        PlayerColor::Black => 0,
                    };

                    // Must be on starting row
                    if from_row != king_row || from_col != 4 {
                        return false;
                    }

                    let king_moved = match piece.color {
                        PlayerColor::White => self.white_king_moved,
                        PlayerColor::Black => self.black_king_moved,
                    };
                    if king_moved {
                        return false;
                    }

                    // Can't castle out of check
                    let enemy_color = piece.color.opposite();
                    if self.is_square_attacked(king_row, 4, enemy_color) {
                        return false;
                    }

                    // Kingside castling (to column 6)
                    if to_col == 6 {
                        let rook_moved = match piece.color {
                            PlayerColor::White => self.white_rook_h_moved,
                            PlayerColor::Black => self.black_rook_h_moved,
                        };
                        if rook_moved {
                            return false;
                        }
                        // Check rook is present
                        if let Some(rook) = self.board[king_row][7] {
                            if rook.piece_type != PieceType::Rook || rook.color != piece.color {
                                return false;
                            }
                        } else {
                            return false;
                        }
                        // Path must be clear (columns 5 and 6)
                        if self.board[king_row][5].is_some() || self.board[king_row][6].is_some() {
                            return false;
                        }
                        // Can't castle through or into check (columns 5 and 6)
                        if self.is_square_attacked(king_row, 5, enemy_color)
                            || self.is_square_attacked(king_row, 6, enemy_color)
                        {
                            return false;
                        }
                        return true;
                    }

                    // Queenside castling (to column 2)
                    if to_col == 2 {
                        let rook_moved = match piece.color {
                            PlayerColor::White => self.white_rook_a_moved,
                            PlayerColor::Black => self.black_rook_a_moved,
                        };
                        if rook_moved {
                            return false;
                        }
                        // Check rook is present
                        if let Some(rook) = self.board[king_row][0] {
                            if rook.piece_type != PieceType::Rook || rook.color != piece.color {
                                return false;
                            }
                        } else {
                            return false;
                        }
                        // Path must be clear (columns 1, 2, and 3)
                        if self.board[king_row][1].is_some()
                            || self.board[king_row][2].is_some()
                            || self.board[king_row][3].is_some()
                        {
                            return false;
                        }
                        // Can't castle through or into check (columns 2 and 3)
                        if self.is_square_attacked(king_row, 2, enemy_color)
                            || self.is_square_attacked(king_row, 3, enemy_color)
                        {
                            return false;
                        }
                        return true;
                    }
                }

                false
            }
        };

        // If the move pattern is invalid, reject it
        if !is_valid_pattern {
            return false;
        }

        // Check that the move doesn't leave our king in check
        // (Castling already handles its own check validation)
        if piece.piece_type == PieceType::King && col_diff == 2 {
            // Castling - already validated above
            true
        } else {
            !self.would_be_in_check(from_row, from_col, to_row, to_col)
        }
    }

    fn select_square(&mut self, row: usize, col: usize) {
        if let Some((selected_row, selected_col)) = self.selected {
            // Validate the move
            if !self.is_valid_move(selected_row, selected_col, row, col) {
                self.selected = None;
                return;
            }

            let piece = self.board[selected_row][selected_col].unwrap();

            // Track capture
            if let Some(captured) = self.board[row][col] {
                match self.turn {
                    PlayerColor::White => self.captured_by_white.push(captured),
                    PlayerColor::Black => self.captured_by_black.push(captured),
                }
            }

            // Handle en passant capture
            if piece.piece_type == PieceType::Pawn
                && selected_col != col
                && self.board[row][col].is_none()
            {
                // This is en passant - capture the pawn on the adjacent square
                let captured_pawn = self.board[selected_row][col].take().unwrap();
                match self.turn {
                    PlayerColor::White => self.captured_by_white.push(captured_pawn),
                    PlayerColor::Black => self.captured_by_black.push(captured_pawn),
                }
            }

            // Clear en passant target (will be set below if applicable)
            self.en_passant_target = None;

            // Set en passant target if pawn moved two squares
            if piece.piece_type == PieceType::Pawn {
                let row_diff = (row as isize - selected_row as isize).abs();
                if row_diff == 2 {
                    self.en_passant_target = Some(col);
                }
            }

            // Handle castling - move the rook
            if piece.piece_type == PieceType::King {
                let col_diff = col as isize - selected_col as isize;
                if col_diff == 2 {
                    // Kingside castling - move rook from h to f
                    self.board[row][5] = self.board[row][7].take();
                } else if col_diff == -2 {
                    // Queenside castling - move rook from a to d
                    self.board[row][3] = self.board[row][0].take();
                }
            }

            // Update castling flags
            match piece.piece_type {
                PieceType::King => match piece.color {
                    PlayerColor::White => self.white_king_moved = true,
                    PlayerColor::Black => self.black_king_moved = true,
                },
                PieceType::Rook => {
                    if selected_row == 7 && selected_col == 0 {
                        self.white_rook_a_moved = true;
                    } else if selected_row == 7 && selected_col == 7 {
                        self.white_rook_h_moved = true;
                    } else if selected_row == 0 && selected_col == 0 {
                        self.black_rook_a_moved = true;
                    } else if selected_row == 0 && selected_col == 7 {
                        self.black_rook_h_moved = true;
                    }
                }
                _ => {}
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

    fn reset(&mut self) {
        self.board = board::setup_initial_board();
        self.selected = None;
        self.last_move = None;
        self.turn = PlayerColor::White;
        self.captured_by_white.clear();
        self.captured_by_black.clear();
        self.en_passant_target = None;
        self.white_king_moved = false;
        self.black_king_moved = false;
        self.white_rook_a_moved = false;
        self.white_rook_h_moved = false;
        self.black_rook_a_moved = false;
        self.black_rook_h_moved = false;
    }

    fn toggle_auto_flip(&mut self) {
        self.flip_board = !self.flip_board;
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
                            .child(div().rounded().bg(Color::rgb(50, 50, 50)).size(pc(30.0)));
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

    let button = |label: &str| {
        div()
            .bg(Color::rgb(200, 200, 200))
            .p(px(8.0))
            .child(text(label))
    };

    let auto_flip_label = if game.flip_board {
        "Auto-Flip: On"
    } else {
        "Auto-Flip: Off"
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
        })
        .child(
            div()
                .row()
                .w(FULL)
                .child(button("Reset").on_left_click(|g: &mut ChessGame| g.reset()))
                .child(
                    button(auto_flip_label).on_left_click(|g: &mut ChessGame| g.toggle_auto_flip()),
                ),
        );

    div().size(FULL).row().child(chessboard).child(side_panel)
}

fn main() {
    let app = App::new(ChessGame::new(), view);
    app.run();
}
