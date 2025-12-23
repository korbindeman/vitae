#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerColor {
    White,
    Black,
}

impl PlayerColor {
    pub fn opposite(self) -> Self {
        match self {
            PlayerColor::White => PlayerColor::Black,
            PlayerColor::Black => PlayerColor::White,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PlayerColor,
}

impl Piece {
    pub fn unicode(&self) -> &'static str {
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
