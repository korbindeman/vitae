use crate::check::Board;
use crate::moves::CastlingRights;
use crate::types::{Piece, PieceType, PlayerColor};

pub struct FenState {
    pub board: Board,
    pub turn: PlayerColor,
    pub castling: CastlingRights,
    pub en_passant_target: Option<usize>,
}

pub fn parse_fen(fen: &str) -> Result<FenState, String> {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    if parts.len() < 4 {
        return Err("FEN must have at least 4 parts".to_string());
    }

    let board = parse_board(parts[0])?;
    let turn = parse_turn(parts[1])?;
    let castling = parse_castling(parts[2]);
    let en_passant_target = parse_en_passant(parts[3]);

    Ok(FenState {
        board,
        turn,
        castling,
        en_passant_target,
    })
}

fn parse_board(placement: &str) -> Result<Board, String> {
    let mut board: Board = [[None; 8]; 8];
    let ranks: Vec<&str> = placement.split('/').collect();

    if ranks.len() != 8 {
        return Err("Board must have 8 ranks".to_string());
    }

    for (rank_idx, rank) in ranks.iter().enumerate() {
        let mut col = 0;
        for c in rank.chars() {
            if col >= 8 {
                return Err(format!("Too many squares in rank {}", 8 - rank_idx));
            }

            if let Some(digit) = c.to_digit(10) {
                col += digit as usize;
            } else {
                let piece = char_to_piece(c)?;
                board[rank_idx][col] = Some(piece);
                col += 1;
            }
        }

        if col != 8 {
            return Err(format!(
                "Rank {} has {} squares, expected 8",
                8 - rank_idx,
                col
            ));
        }
    }

    Ok(board)
}

fn char_to_piece(c: char) -> Result<Piece, String> {
    let color = if c.is_uppercase() {
        PlayerColor::White
    } else {
        PlayerColor::Black
    };

    let piece_type = match c.to_ascii_lowercase() {
        'k' => PieceType::King,
        'q' => PieceType::Queen,
        'r' => PieceType::Rook,
        'b' => PieceType::Bishop,
        'n' => PieceType::Knight,
        'p' => PieceType::Pawn,
        _ => return Err(format!("Unknown piece: {}", c)),
    };

    Ok(Piece { piece_type, color })
}

fn parse_turn(turn: &str) -> Result<PlayerColor, String> {
    match turn {
        "w" => Ok(PlayerColor::White),
        "b" => Ok(PlayerColor::Black),
        _ => Err(format!("Unknown turn: {}", turn)),
    }
}

fn parse_castling(castling: &str) -> CastlingRights {
    CastlingRights {
        white_king_moved: !castling.contains('K') && !castling.contains('Q'),
        black_king_moved: !castling.contains('k') && !castling.contains('q'),
        white_rook_h_moved: !castling.contains('K'),
        white_rook_a_moved: !castling.contains('Q'),
        black_rook_h_moved: !castling.contains('k'),
        black_rook_a_moved: !castling.contains('q'),
    }
}

fn parse_en_passant(ep: &str) -> Option<usize> {
    if ep == "-" {
        return None;
    }

    let chars: Vec<char> = ep.chars().collect();
    if chars.len() != 2 {
        return None;
    }

    let col = chars[0] as usize;
    if col >= 'a' as usize && col <= 'h' as usize {
        Some(col - 'a' as usize)
    } else {
        None
    }
}
