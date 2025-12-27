#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PlayerColor,
}

impl PieceType {
    pub fn points(self) -> i32 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 0,
        }
    }
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

    pub fn svg_filename(&self) -> &'static str {
        match (self.piece_type, self.color) {
            (PieceType::King, PlayerColor::White) => "king-w.svg",
            (PieceType::Queen, PlayerColor::White) => "queen-w.svg",
            (PieceType::Rook, PlayerColor::White) => "rook-w.svg",
            (PieceType::Bishop, PlayerColor::White) => "bishop-w.svg",
            (PieceType::Knight, PlayerColor::White) => "knight-w.svg",
            (PieceType::Pawn, PlayerColor::White) => "pawn-w.svg",
            (PieceType::King, PlayerColor::Black) => "king-b.svg",
            (PieceType::Queen, PlayerColor::Black) => "queen-b.svg",
            (PieceType::Rook, PlayerColor::Black) => "rook-b.svg",
            (PieceType::Bishop, PlayerColor::Black) => "bishop-b.svg",
            (PieceType::Knight, PlayerColor::Black) => "knight-b.svg",
            (PieceType::Pawn, PlayerColor::Black) => "pawn-b.svg",
        }
    }
}
