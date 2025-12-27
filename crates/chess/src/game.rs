use crate::assets::PieceSvgs;
use crate::board;
use crate::check::{
    find_king, is_checkmate, is_in_check, is_insufficient_material, is_stalemate, Board,
};
use crate::fen::parse_fen;
use crate::moves::{is_valid_move, CastlingRights, Move};
use crate::types::{Piece, PieceType, PlayerColor};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DrawReason {
    Stalemate,
    InsufficientMaterial,
    FiftyMoveRule,
    ThreefoldRepetition,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Ongoing,
    Checkmate(PlayerColor), // The color that won
    Draw(DrawReason),
}

#[derive(Clone)]
pub struct MoveRecord {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub piece: Piece,
    pub captured: Option<Piece>,
    pub was_en_passant: bool,
    pub was_castling: Option<CastlingSide>,
    pub promotion: Option<PieceType>,
    // State before the move (for undo)
    pub prev_en_passant_target: Option<usize>,
    pub prev_castling_rights: CastlingRights,
    pub prev_halfmove_clock: u32,
    pub notation: String,
}

#[derive(Clone, Copy)]
pub struct PendingPromotion {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub captured: Option<Piece>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CastlingSide {
    Kingside,
    Queenside,
}

#[derive(Clone)]
pub struct ChessGame {
    pub board: Board,
    pub selected: Option<(usize, usize)>,
    pub last_move: Option<String>,
    pub turn: PlayerColor,
    pub pieces: PieceSvgs,
    pub flip_board: bool,
    pub captured_by_white: Vec<Piece>,
    pub captured_by_black: Vec<Piece>,
    pub en_passant_target: Option<usize>,
    pub white_king_moved: bool,
    pub black_king_moved: bool,
    pub white_rook_a_moved: bool,
    pub white_rook_h_moved: bool,
    pub black_rook_a_moved: bool,
    pub black_rook_h_moved: bool,
    pub result: GameResult,
    pub history: Vec<MoveRecord>,
    pub redo_stack: Vec<MoveRecord>,
    pub pending_promotion: Option<PendingPromotion>,
    pub halfmove_clock: u32,
    pub position_history: Vec<u64>,
}

impl ChessGame {
    pub fn new() -> Self {
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
            result: GameResult::Ongoing,
            history: Vec::new(),
            redo_stack: Vec::new(),
            pending_promotion: None,
            halfmove_clock: 0,
            position_history: vec![Self::hash_position(
                &board::setup_initial_board(),
                PlayerColor::White,
                None,
                &CastlingRights {
                    white_king_moved: false,
                    black_king_moved: false,
                    white_rook_a_moved: false,
                    white_rook_h_moved: false,
                    black_rook_a_moved: false,
                    black_rook_h_moved: false,
                },
            )],
        }
    }

    fn hash_position(
        board: &Board,
        turn: PlayerColor,
        en_passant: Option<usize>,
        castling: &CastlingRights,
    ) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        board.hash(&mut hasher);
        turn.hash(&mut hasher);
        en_passant.hash(&mut hasher);
        castling.white_king_moved.hash(&mut hasher);
        castling.black_king_moved.hash(&mut hasher);
        castling.white_rook_a_moved.hash(&mut hasher);
        castling.white_rook_h_moved.hash(&mut hasher);
        castling.black_rook_a_moved.hash(&mut hasher);
        castling.black_rook_h_moved.hash(&mut hasher);
        hasher.finish()
    }

    pub fn is_game_over(&self) -> bool {
        self.result != GameResult::Ongoing
    }

    pub fn is_awaiting_promotion(&self) -> bool {
        self.pending_promotion.is_some()
    }

    pub fn king_in_check(&self) -> Option<(usize, usize)> {
        if is_in_check(&self.board, self.turn) {
            find_king(&self.board, self.turn)
        } else {
            None
        }
    }

    pub fn castling_rights(&self) -> CastlingRights {
        CastlingRights {
            white_king_moved: self.white_king_moved,
            black_king_moved: self.black_king_moved,
            white_rook_a_moved: self.white_rook_a_moved,
            white_rook_h_moved: self.white_rook_h_moved,
            black_rook_a_moved: self.black_rook_a_moved,
            black_rook_h_moved: self.black_rook_h_moved,
        }
    }

    pub fn is_valid_move(
        &self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> bool {
        is_valid_move(
            &self.board,
            from_row,
            from_col,
            to_row,
            to_col,
            self.en_passant_target,
            &self.castling_rights(),
        )
    }

    pub fn select_square(&mut self, row: usize, col: usize) {
        if self.is_game_over() || self.is_awaiting_promotion() {
            return;
        }

        if let Some((selected_row, selected_col)) = self.selected {
            if !self.is_valid_move(selected_row, selected_col, row, col) {
                self.selected = None;
                return;
            }

            // Check if this is a pawn promotion
            if let Some(piece) = self.board[selected_row][selected_col] {
                if piece.piece_type == PieceType::Pawn {
                    let promotion_rank = match piece.color {
                        PlayerColor::White => 0,
                        PlayerColor::Black => 7,
                    };
                    if row == promotion_rank {
                        // Store pending promotion and wait for user choice
                        self.pending_promotion = Some(PendingPromotion {
                            from: (selected_row, selected_col),
                            to: (row, col),
                            captured: self.board[row][col],
                        });
                        self.selected = None;
                        return;
                    }
                }
            }

            self.make_move(Move::new(selected_row, selected_col, row, col), None);
            self.selected = None;
        } else {
            // Select piece (only if there's a piece and it's your turn)
            if let Some(piece) = self.board[row][col] {
                if piece.color == self.turn {
                    self.selected = Some((row, col));
                }
            }
        }
    }

    pub fn promote_to(&mut self, piece_type: PieceType) {
        let pending = match self.pending_promotion.take() {
            Some(p) => p,
            None => return,
        };

        self.make_move(
            Move::new(pending.from.0, pending.from.1, pending.to.0, pending.to.1),
            Some(piece_type),
        );
    }

    fn make_move(&mut self, mv: Move, promotion: Option<PieceType>) {
        let (from_row, from_col) = mv.from;
        let (to_row, to_col) = mv.to;
        let piece = self.board[from_row][from_col].unwrap();

        // Save state for undo
        let prev_en_passant_target = self.en_passant_target;
        let prev_castling_rights = self.castling_rights();

        // Detect special moves
        let is_en_passant = piece.piece_type == PieceType::Pawn
            && from_col != to_col
            && self.board[to_row][to_col].is_none();

        let castling_side = if piece.piece_type == PieceType::King {
            let col_diff = to_col as isize - from_col as isize;
            if col_diff == 2 {
                Some(CastlingSide::Kingside)
            } else if col_diff == -2 {
                Some(CastlingSide::Queenside)
            } else {
                None
            }
        } else {
            None
        };

        // Determine captured piece
        let captured = if is_en_passant {
            self.board[from_row][to_col]
        } else {
            self.board[to_row][to_col]
        };

        // Build notation
        let promotion_char = promotion
            .map(|p| match p {
                PieceType::Queen => "=Q",
                PieceType::Rook => "=R",
                PieceType::Bishop => "=B",
                PieceType::Knight => "=N",
                _ => "",
            })
            .unwrap_or("");
        let notation = format!(
            "{}{}-{}{}{}",
            (b'a' + from_col as u8) as char,
            8 - from_row,
            (b'a' + to_col as u8) as char,
            8 - to_row,
            promotion_char
        );

        // Save halfmove clock for undo
        let prev_halfmove_clock = self.halfmove_clock;

        // Record the move
        let record = MoveRecord {
            from: mv.from,
            to: mv.to,
            piece,
            captured,
            was_en_passant: is_en_passant,
            was_castling: castling_side,
            promotion,
            prev_en_passant_target,
            prev_castling_rights,
            prev_halfmove_clock,
            notation: notation.clone(),
        };
        self.history.push(record);
        self.redo_stack.clear();

        // Track capture
        if let Some(captured_piece) = captured {
            match self.turn {
                PlayerColor::White => self.captured_by_white.push(captured_piece),
                PlayerColor::Black => self.captured_by_black.push(captured_piece),
            }
        }

        // Handle en passant capture
        if is_en_passant {
            self.board[from_row][to_col] = None;
        }

        // Clear en passant target
        self.en_passant_target = None;

        // Set en passant target if pawn moved two squares
        if piece.piece_type == PieceType::Pawn {
            let row_diff = (to_row as isize - from_row as isize).abs();
            if row_diff == 2 {
                self.en_passant_target = Some(to_col);
            }
        }

        // Handle castling - move the rook
        if let Some(side) = castling_side {
            match side {
                CastlingSide::Kingside => {
                    self.board[to_row][5] = self.board[to_row][7].take();
                }
                CastlingSide::Queenside => {
                    self.board[to_row][3] = self.board[to_row][0].take();
                }
            }
        }

        // Update castling flags
        match piece.piece_type {
            PieceType::King => match piece.color {
                PlayerColor::White => self.white_king_moved = true,
                PlayerColor::Black => self.black_king_moved = true,
            },
            PieceType::Rook => {
                if from_row == 7 && from_col == 0 {
                    self.white_rook_a_moved = true;
                } else if from_row == 7 && from_col == 7 {
                    self.white_rook_h_moved = true;
                } else if from_row == 0 && from_col == 0 {
                    self.black_rook_a_moved = true;
                } else if from_row == 0 && from_col == 7 {
                    self.black_rook_h_moved = true;
                }
            }
            _ => {}
        }

        // Move piece
        self.board[to_row][to_col] = self.board[from_row][from_col].take();

        // Handle promotion
        if let Some(promote_to) = promotion {
            self.board[to_row][to_col] = Some(Piece {
                piece_type: promote_to,
                color: piece.color,
            });
        }

        // Update halfmove clock (reset on pawn move or capture, otherwise increment)
        if piece.piece_type == PieceType::Pawn || captured.is_some() {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        self.last_move = Some(notation);
        self.turn = self.turn.opposite();

        // Add current position to history for threefold repetition
        let position_hash = Self::hash_position(
            &self.board,
            self.turn,
            self.en_passant_target,
            &self.castling_rights(),
        );
        self.position_history.push(position_hash);

        // Check for checkmate or draw
        self.update_game_result();
    }

    fn update_game_result(&mut self) {
        let castling = self.castling_rights();

        // Check for checkmate first
        if is_checkmate(&self.board, self.turn, self.en_passant_target, &castling) {
            self.result = GameResult::Checkmate(self.turn.opposite());
            return;
        }

        // Check for stalemate
        if is_stalemate(&self.board, self.turn, self.en_passant_target, &castling) {
            self.result = GameResult::Draw(DrawReason::Stalemate);
            return;
        }

        // Check for insufficient material
        if is_insufficient_material(&self.board) {
            self.result = GameResult::Draw(DrawReason::InsufficientMaterial);
            return;
        }

        // Check for fifty-move rule (100 half-moves = 50 full moves)
        if self.halfmove_clock >= 100 {
            self.result = GameResult::Draw(DrawReason::FiftyMoveRule);
            return;
        }

        // Check for threefold repetition
        if let Some(&current_hash) = self.position_history.last() {
            let count = self
                .position_history
                .iter()
                .filter(|&&h| h == current_hash)
                .count();
            if count >= 3 {
                self.result = GameResult::Draw(DrawReason::ThreefoldRepetition);
                return;
            }
        }

        self.result = GameResult::Ongoing;
    }

    pub fn undo(&mut self) {
        let record = match self.history.pop() {
            Some(r) => r,
            None => return,
        };

        let (from_row, from_col) = record.from;
        let (to_row, to_col) = record.to;

        // Move piece back
        self.board[from_row][from_col] = Some(record.piece);
        self.board[to_row][to_col] = None;

        // Restore captured piece
        if let Some(captured) = record.captured {
            if record.was_en_passant {
                // En passant: captured pawn was on the same row as the moving pawn
                self.board[from_row][to_col] = Some(captured);
            } else {
                self.board[to_row][to_col] = Some(captured);
            }

            // Remove from captured list
            match record.piece.color {
                PlayerColor::White => self.captured_by_white.pop(),
                PlayerColor::Black => self.captured_by_black.pop(),
            };
        }

        // Undo castling - move rook back
        if let Some(side) = record.was_castling {
            match side {
                CastlingSide::Kingside => {
                    self.board[to_row][7] = self.board[to_row][5].take();
                }
                CastlingSide::Queenside => {
                    self.board[to_row][0] = self.board[to_row][3].take();
                }
            }
        }

        // Restore previous state
        self.en_passant_target = record.prev_en_passant_target;
        self.white_king_moved = record.prev_castling_rights.white_king_moved;
        self.black_king_moved = record.prev_castling_rights.black_king_moved;
        self.white_rook_a_moved = record.prev_castling_rights.white_rook_a_moved;
        self.white_rook_h_moved = record.prev_castling_rights.white_rook_h_moved;
        self.black_rook_a_moved = record.prev_castling_rights.black_rook_a_moved;
        self.black_rook_h_moved = record.prev_castling_rights.black_rook_h_moved;
        self.halfmove_clock = record.prev_halfmove_clock;

        // Remove the position from history
        self.position_history.pop();

        // Switch turn back
        self.turn = self.turn.opposite();

        // Update last_move to previous move
        self.last_move = self.history.last().map(|r| r.notation.clone());

        // Push to redo stack
        self.redo_stack.push(record);

        // Clear selection and update result
        self.selected = None;
        self.update_game_result();
    }

    pub fn redo(&mut self) {
        let record = match self.redo_stack.pop() {
            Some(r) => r,
            None => return,
        };

        // Save remaining redo stack (make_move will clear it)
        let remaining_redo = std::mem::take(&mut self.redo_stack);

        // Re-apply the move
        self.make_move(
            Move::new(record.from.0, record.from.1, record.to.0, record.to.1),
            record.promotion,
        );

        // Restore remaining redo stack
        self.redo_stack = remaining_redo;
    }

    pub fn can_undo(&self) -> bool {
        !self.history.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn points_for(&self, color: PlayerColor) -> i32 {
        let captured = match color {
            PlayerColor::White => &self.captured_by_white,
            PlayerColor::Black => &self.captured_by_black,
        };
        captured.iter().map(|p| p.piece_type.points()).sum()
    }

    pub fn reset(&mut self) {
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
        self.result = GameResult::Ongoing;
        self.history.clear();
        self.redo_stack.clear();
        self.pending_promotion = None;
        self.halfmove_clock = 0;
        self.position_history = vec![Self::hash_position(
            &self.board,
            self.turn,
            None,
            &self.castling_rights(),
        )];
    }

    pub fn toggle_auto_flip(&mut self) {
        self.flip_board = !self.flip_board;
    }

    pub fn load_fen(&mut self, fen: &str) {
        let state = match parse_fen(fen) {
            Ok(s) => s,
            Err(_) => return,
        };

        self.board = state.board;
        self.turn = state.turn;
        self.en_passant_target = state.en_passant_target;
        self.white_king_moved = state.castling.white_king_moved;
        self.black_king_moved = state.castling.black_king_moved;
        self.white_rook_a_moved = state.castling.white_rook_a_moved;
        self.white_rook_h_moved = state.castling.white_rook_h_moved;
        self.black_rook_a_moved = state.castling.black_rook_a_moved;
        self.black_rook_h_moved = state.castling.black_rook_h_moved;

        // Clear game state
        self.selected = None;
        self.last_move = None;
        self.captured_by_white.clear();
        self.captured_by_black.clear();
        self.history.clear();
        self.redo_stack.clear();
        self.pending_promotion = None;
        self.halfmove_clock = 0;
        self.position_history = vec![Self::hash_position(
            &self.board,
            self.turn,
            self.en_passant_target,
            &self.castling_rights(),
        )];

        self.update_game_result();
    }
}
