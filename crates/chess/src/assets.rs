use std::collections::HashMap;
use std::rc::Rc;

use crate::types::{Piece, PieceType, PlayerColor};
use vitae::prelude::*;

#[derive(Clone)]
pub struct PieceSvgs {
    svgs: Rc<HashMap<(PieceType, PlayerColor), Svg>>,
}

impl PieceSvgs {
    pub fn load() -> Self {
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

    pub fn get(&self, piece: &Piece) -> Option<&Svg> {
        self.svgs.get(&(piece.piece_type, piece.color))
    }
}
