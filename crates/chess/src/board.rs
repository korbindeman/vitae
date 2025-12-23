use crate::types::{Piece, PieceType, PlayerColor};

pub fn setup_initial_board() -> [[Option<Piece>; 8]; 8] {
    let mut board = [[None; 8]; 8];

    // Set up black pieces (row 0 and 1)
    board[0] = [
        Some(Piece {
            piece_type: PieceType::Rook,
            color: PlayerColor::Black,
        }),
        Some(Piece {
            piece_type: PieceType::Knight,
            color: PlayerColor::Black,
        }),
        Some(Piece {
            piece_type: PieceType::Bishop,
            color: PlayerColor::Black,
        }),
        Some(Piece {
            piece_type: PieceType::Queen,
            color: PlayerColor::Black,
        }),
        Some(Piece {
            piece_type: PieceType::King,
            color: PlayerColor::Black,
        }),
        Some(Piece {
            piece_type: PieceType::Bishop,
            color: PlayerColor::Black,
        }),
        Some(Piece {
            piece_type: PieceType::Knight,
            color: PlayerColor::Black,
        }),
        Some(Piece {
            piece_type: PieceType::Rook,
            color: PlayerColor::Black,
        }),
    ];
    for col in 0..8 {
        board[1][col] = Some(Piece {
            piece_type: PieceType::Pawn,
            color: PlayerColor::Black,
        });
    }

    // Set up white pieces (row 6 and 7)
    for col in 0..8 {
        board[6][col] = Some(Piece {
            piece_type: PieceType::Pawn,
            color: PlayerColor::White,
        });
    }
    board[7] = [
        Some(Piece {
            piece_type: PieceType::Rook,
            color: PlayerColor::White,
        }),
        Some(Piece {
            piece_type: PieceType::Knight,
            color: PlayerColor::White,
        }),
        Some(Piece {
            piece_type: PieceType::Bishop,
            color: PlayerColor::White,
        }),
        Some(Piece {
            piece_type: PieceType::Queen,
            color: PlayerColor::White,
        }),
        Some(Piece {
            piece_type: PieceType::King,
            color: PlayerColor::White,
        }),
        Some(Piece {
            piece_type: PieceType::Bishop,
            color: PlayerColor::White,
        }),
        Some(Piece {
            piece_type: PieceType::Knight,
            color: PlayerColor::White,
        }),
        Some(Piece {
            piece_type: PieceType::Rook,
            color: PlayerColor::White,
        }),
    ];

    board
}
