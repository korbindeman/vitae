use crate::check::{is_path_clear, is_square_attacked, would_be_in_check, Board};
use crate::types::{Piece, PieceType, PlayerColor};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
}

impl Move {
    pub fn new(from_row: usize, from_col: usize, to_row: usize, to_col: usize) -> Self {
        Self {
            from: (from_row, from_col),
            to: (to_row, to_col),
        }
    }
}

#[derive(Clone, Copy)]
pub struct CastlingRights {
    pub white_king_moved: bool,
    pub black_king_moved: bool,
    pub white_rook_a_moved: bool,
    pub white_rook_h_moved: bool,
    pub black_rook_a_moved: bool,
    pub black_rook_h_moved: bool,
}

pub fn is_valid_move(
    board: &Board,
    from_row: usize,
    from_col: usize,
    to_row: usize,
    to_col: usize,
    en_passant_target: Option<usize>,
    castling: &CastlingRights,
) -> bool {
    if from_row == to_row && from_col == to_col {
        return false;
    }

    let piece = match board[from_row][from_col] {
        Some(p) => p,
        None => return false,
    };

    // Can't capture your own piece
    if let Some(target) = board[to_row][to_col] {
        if target.color == piece.color {
            return false;
        }
    }

    let row_diff = (to_row as isize - from_row as isize).abs();
    let col_diff = (to_col as isize - from_col as isize).abs();
    let is_capture = board[to_row][to_col].is_some();

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
                return !would_be_in_check(
                    board,
                    from_row,
                    from_col,
                    to_row,
                    to_col,
                    en_passant_target,
                );
            }

            // Two-square initial move
            if forward == 2 * direction
                && from_col == to_col
                && from_row == start_row
                && !is_capture
                && is_path_clear(board, from_row, from_col, to_row, to_col)
            {
                return !would_be_in_check(
                    board,
                    from_row,
                    from_col,
                    to_row,
                    to_col,
                    en_passant_target,
                );
            }

            // Diagonal capture
            if forward == direction && col_diff == 1 && is_capture {
                return !would_be_in_check(
                    board,
                    from_row,
                    from_col,
                    to_row,
                    to_col,
                    en_passant_target,
                );
            }

            // En passant capture
            if forward == direction
                && col_diff == 1
                && from_row == en_passant_row
                && en_passant_target == Some(to_col)
            {
                return !would_be_in_check(
                    board,
                    from_row,
                    from_col,
                    to_row,
                    to_col,
                    en_passant_target,
                );
            }

            false
        }
        PieceType::Rook => {
            if from_row != to_row && from_col != to_col {
                return false;
            }
            is_path_clear(board, from_row, from_col, to_row, to_col)
        }
        PieceType::Bishop => {
            if row_diff != col_diff {
                return false;
            }
            is_path_clear(board, from_row, from_col, to_row, to_col)
        }
        PieceType::Queen => {
            let is_straight = from_row == to_row || from_col == to_col;
            let is_diagonal = row_diff == col_diff;
            if !is_straight && !is_diagonal {
                return false;
            }
            is_path_clear(board, from_row, from_col, to_row, to_col)
        }
        PieceType::Knight => (row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2),
        PieceType::King => {
            // Normal king move
            if row_diff <= 1 && col_diff <= 1 {
                return !would_be_in_check(
                    board,
                    from_row,
                    from_col,
                    to_row,
                    to_col,
                    en_passant_target,
                );
            }

            // Castling
            if row_diff == 0 && col_diff == 2 {
                return is_valid_castling(board, from_row, from_col, to_col, piece.color, castling);
            }

            false
        }
    };

    if !is_valid_pattern {
        return false;
    }

    // Check that the move doesn't leave our king in check
    if piece.piece_type == PieceType::King && col_diff == 2 {
        true // Castling already validated
    } else {
        !would_be_in_check(board, from_row, from_col, to_row, to_col, en_passant_target)
    }
}

fn is_valid_castling(
    board: &Board,
    from_row: usize,
    from_col: usize,
    to_col: usize,
    color: PlayerColor,
    castling: &CastlingRights,
) -> bool {
    let king_row = match color {
        PlayerColor::White => 7,
        PlayerColor::Black => 0,
    };

    if from_row != king_row || from_col != 4 {
        return false;
    }

    let king_moved = match color {
        PlayerColor::White => castling.white_king_moved,
        PlayerColor::Black => castling.black_king_moved,
    };
    if king_moved {
        return false;
    }

    let enemy_color = color.opposite();
    if is_square_attacked(board, king_row, 4, enemy_color) {
        return false;
    }

    // Kingside castling
    if to_col == 6 {
        let rook_moved = match color {
            PlayerColor::White => castling.white_rook_h_moved,
            PlayerColor::Black => castling.black_rook_h_moved,
        };
        if rook_moved {
            return false;
        }
        if let Some(rook) = board[king_row][7] {
            if rook.piece_type != PieceType::Rook || rook.color != color {
                return false;
            }
        } else {
            return false;
        }
        if board[king_row][5].is_some() || board[king_row][6].is_some() {
            return false;
        }
        if is_square_attacked(board, king_row, 5, enemy_color)
            || is_square_attacked(board, king_row, 6, enemy_color)
        {
            return false;
        }
        return true;
    }

    // Queenside castling
    if to_col == 2 {
        let rook_moved = match color {
            PlayerColor::White => castling.white_rook_a_moved,
            PlayerColor::Black => castling.black_rook_a_moved,
        };
        if rook_moved {
            return false;
        }
        if let Some(rook) = board[king_row][0] {
            if rook.piece_type != PieceType::Rook || rook.color != color {
                return false;
            }
        } else {
            return false;
        }
        if board[king_row][1].is_some()
            || board[king_row][2].is_some()
            || board[king_row][3].is_some()
        {
            return false;
        }
        if is_square_attacked(board, king_row, 2, enemy_color)
            || is_square_attacked(board, king_row, 3, enemy_color)
        {
            return false;
        }
        return true;
    }

    false
}

pub fn generate_legal_moves(
    board: &Board,
    color: PlayerColor,
    en_passant_target: Option<usize>,
    castling: &CastlingRights,
) -> Vec<Move> {
    let mut moves = Vec::new();

    for from_row in 0..8 {
        for from_col in 0..8 {
            if let Some(piece) = board[from_row][from_col] {
                if piece.color != color {
                    continue;
                }
                generate_piece_moves(
                    board,
                    from_row,
                    from_col,
                    piece,
                    en_passant_target,
                    castling,
                    &mut moves,
                );
            }
        }
    }

    // Filter out moves that leave king in check
    moves.retain(|m| {
        !would_be_in_check(board, m.from.0, m.from.1, m.to.0, m.to.1, en_passant_target)
    });

    moves
}

fn generate_piece_moves(
    board: &Board,
    from_row: usize,
    from_col: usize,
    piece: Piece,
    en_passant_target: Option<usize>,
    castling: &CastlingRights,
    moves: &mut Vec<Move>,
) {
    match piece.piece_type {
        PieceType::Pawn => generate_pawn_moves(
            board,
            from_row,
            from_col,
            piece.color,
            en_passant_target,
            moves,
        ),
        PieceType::Knight => generate_knight_moves(board, from_row, from_col, piece.color, moves),
        PieceType::Bishop => {
            generate_sliding_moves(board, from_row, from_col, piece.color, &BISHOP_DIRS, moves)
        }
        PieceType::Rook => {
            generate_sliding_moves(board, from_row, from_col, piece.color, &ROOK_DIRS, moves)
        }
        PieceType::Queen => {
            generate_sliding_moves(board, from_row, from_col, piece.color, &QUEEN_DIRS, moves)
        }
        PieceType::King => {
            generate_king_moves(board, from_row, from_col, piece.color, castling, moves)
        }
    }
}

const ROOK_DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const BISHOP_DIRS: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const QUEEN_DIRS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
const KNIGHT_OFFSETS: [(isize, isize); 8] = [
    (2, 1),
    (2, -1),
    (-2, 1),
    (-2, -1),
    (1, 2),
    (1, -2),
    (-1, 2),
    (-1, -2),
];

fn generate_pawn_moves(
    board: &Board,
    from_row: usize,
    from_col: usize,
    color: PlayerColor,
    en_passant_target: Option<usize>,
    moves: &mut Vec<Move>,
) {
    let direction: isize = match color {
        PlayerColor::White => -1,
        PlayerColor::Black => 1,
    };
    let start_row = match color {
        PlayerColor::White => 6,
        PlayerColor::Black => 1,
    };
    let en_passant_row = match color {
        PlayerColor::White => 3,
        PlayerColor::Black => 4,
    };

    let to_row = (from_row as isize + direction) as usize;

    // One square forward
    if to_row < 8 && board[to_row][from_col].is_none() {
        moves.push(Move::new(from_row, from_col, to_row, from_col));

        // Two squares forward from start
        if from_row == start_row {
            let to_row2 = (from_row as isize + 2 * direction) as usize;
            if board[to_row2][from_col].is_none() {
                moves.push(Move::new(from_row, from_col, to_row2, from_col));
            }
        }
    }

    // Captures
    for col_offset in [-1isize, 1] {
        let to_col = from_col as isize + col_offset;
        if to_col < 0 || to_col >= 8 {
            continue;
        }
        let to_col = to_col as usize;

        // Normal capture
        if let Some(target) = board[to_row][to_col] {
            if target.color != color {
                moves.push(Move::new(from_row, from_col, to_row, to_col));
            }
        }

        // En passant
        if from_row == en_passant_row && en_passant_target == Some(to_col) {
            moves.push(Move::new(from_row, from_col, to_row, to_col));
        }
    }
}

fn generate_knight_moves(
    board: &Board,
    from_row: usize,
    from_col: usize,
    color: PlayerColor,
    moves: &mut Vec<Move>,
) {
    for (dr, dc) in KNIGHT_OFFSETS {
        let to_row = from_row as isize + dr;
        let to_col = from_col as isize + dc;

        if to_row < 0 || to_row >= 8 || to_col < 0 || to_col >= 8 {
            continue;
        }

        let to_row = to_row as usize;
        let to_col = to_col as usize;

        match board[to_row][to_col] {
            Some(target) if target.color == color => continue,
            _ => moves.push(Move::new(from_row, from_col, to_row, to_col)),
        }
    }
}

fn generate_sliding_moves(
    board: &Board,
    from_row: usize,
    from_col: usize,
    color: PlayerColor,
    directions: &[(isize, isize)],
    moves: &mut Vec<Move>,
) {
    for &(dr, dc) in directions {
        let mut to_row = from_row as isize + dr;
        let mut to_col = from_col as isize + dc;

        while to_row >= 0 && to_row < 8 && to_col >= 0 && to_col < 8 {
            let tr = to_row as usize;
            let tc = to_col as usize;

            match board[tr][tc] {
                Some(target) => {
                    if target.color != color {
                        moves.push(Move::new(from_row, from_col, tr, tc));
                    }
                    break;
                }
                None => {
                    moves.push(Move::new(from_row, from_col, tr, tc));
                }
            }

            to_row += dr;
            to_col += dc;
        }
    }
}

fn generate_king_moves(
    board: &Board,
    from_row: usize,
    from_col: usize,
    color: PlayerColor,
    castling: &CastlingRights,
    moves: &mut Vec<Move>,
) {
    // Normal moves
    for dr in -1isize..=1 {
        for dc in -1isize..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }

            let to_row = from_row as isize + dr;
            let to_col = from_col as isize + dc;

            if to_row < 0 || to_row >= 8 || to_col < 0 || to_col >= 8 {
                continue;
            }

            let to_row = to_row as usize;
            let to_col = to_col as usize;

            match board[to_row][to_col] {
                Some(target) if target.color == color => continue,
                _ => moves.push(Move::new(from_row, from_col, to_row, to_col)),
            }
        }
    }

    // Castling
    if is_valid_castling(board, from_row, from_col, 6, color, castling) {
        moves.push(Move::new(from_row, from_col, from_row, 6));
    }
    if is_valid_castling(board, from_row, from_col, 2, color, castling) {
        moves.push(Move::new(from_row, from_col, from_row, 2));
    }
}
