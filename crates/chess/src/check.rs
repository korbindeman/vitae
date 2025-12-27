use crate::moves::{generate_legal_moves, CastlingRights};
use crate::types::{Piece, PieceType, PlayerColor};

pub type Board = [[Option<Piece>; 8]; 8];

pub fn find_king(board: &Board, color: PlayerColor) -> Option<(usize, usize)> {
    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = board[row][col] {
                if piece.piece_type == PieceType::King && piece.color == color {
                    return Some((row, col));
                }
            }
        }
    }
    None
}

pub fn is_square_attacked(
    board: &Board,
    target_row: usize,
    target_col: usize,
    by_color: PlayerColor,
) -> bool {
    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = board[row][col] {
                if piece.color != by_color {
                    continue;
                }
                if can_piece_attack(board, row, col, target_row, target_col, piece) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn can_piece_attack(
    board: &Board,
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
        PieceType::King => row_diff <= 1 && col_diff <= 1,
    }
}

pub fn is_path_clear(
    board: &Board,
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

pub fn is_in_check(board: &Board, color: PlayerColor) -> bool {
    if let Some((king_row, king_col)) = find_king(board, color) {
        is_square_attacked(board, king_row, king_col, color.opposite())
    } else {
        false
    }
}

pub fn would_be_in_check(
    board: &Board,
    from_row: usize,
    from_col: usize,
    to_row: usize,
    to_col: usize,
    en_passant_target: Option<usize>,
) -> bool {
    let piece = match board[from_row][from_col] {
        Some(p) => p,
        None => return false,
    };

    let mut temp_board = *board;
    temp_board[to_row][to_col] = temp_board[from_row][from_col];
    temp_board[from_row][from_col] = None;

    // Handle en passant capture
    if piece.piece_type == PieceType::Pawn
        && from_col != to_col
        && board[to_row][to_col].is_none()
        && en_passant_target == Some(to_col)
    {
        temp_board[from_row][to_col] = None;
    }

    // Find king position (may have moved if we're moving the king)
    let king_pos = if piece.piece_type == PieceType::King {
        (to_row, to_col)
    } else {
        match find_king(&temp_board, piece.color) {
            Some(p) => p,
            None => return false,
        }
    };

    is_square_attacked(&temp_board, king_pos.0, king_pos.1, piece.color.opposite())
}

pub fn is_checkmate(
    board: &Board,
    color: PlayerColor,
    en_passant_target: Option<usize>,
    castling: &CastlingRights,
) -> bool {
    is_in_check(board, color)
        && generate_legal_moves(board, color, en_passant_target, castling).is_empty()
}

pub fn is_stalemate(
    board: &Board,
    color: PlayerColor,
    en_passant_target: Option<usize>,
    castling: &CastlingRights,
) -> bool {
    !is_in_check(board, color)
        && generate_legal_moves(board, color, en_passant_target, castling).is_empty()
}

pub fn is_insufficient_material(board: &Board) -> bool {
    let mut white_pieces: Vec<PieceType> = Vec::new();
    let mut black_pieces: Vec<PieceType> = Vec::new();
    let mut white_bishop_square_color: Option<bool> = None;
    let mut black_bishop_square_color: Option<bool> = None;

    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = board[row][col] {
                let is_light_square = (row + col) % 2 == 0;
                match piece.color {
                    PlayerColor::White => {
                        white_pieces.push(piece.piece_type);
                        if piece.piece_type == PieceType::Bishop {
                            white_bishop_square_color = Some(is_light_square);
                        }
                    }
                    PlayerColor::Black => {
                        black_pieces.push(piece.piece_type);
                        if piece.piece_type == PieceType::Bishop {
                            black_bishop_square_color = Some(is_light_square);
                        }
                    }
                }
            }
        }
    }

    // Remove kings from consideration
    white_pieces.retain(|&p| p != PieceType::King);
    black_pieces.retain(|&p| p != PieceType::King);

    // King vs King
    if white_pieces.is_empty() && black_pieces.is_empty() {
        return true;
    }

    // King + minor piece vs King
    if white_pieces.is_empty() && black_pieces.len() == 1 {
        let p = black_pieces[0];
        if p == PieceType::Bishop || p == PieceType::Knight {
            return true;
        }
    }
    if black_pieces.is_empty() && white_pieces.len() == 1 {
        let p = white_pieces[0];
        if p == PieceType::Bishop || p == PieceType::Knight {
            return true;
        }
    }

    // King + Bishop vs King + Bishop (same colored bishops)
    if white_pieces.len() == 1
        && black_pieces.len() == 1
        && white_pieces[0] == PieceType::Bishop
        && black_pieces[0] == PieceType::Bishop
    {
        if white_bishop_square_color == black_bishop_square_color {
            return true;
        }
    }

    // King + Knight vs King + Knight (cannot force checkmate)
    if white_pieces.len() == 1
        && black_pieces.len() == 1
        && white_pieces[0] == PieceType::Knight
        && black_pieces[0] == PieceType::Knight
    {
        return true;
    }

    false
}
