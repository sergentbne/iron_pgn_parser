use crate::chess::structs::{ChessGame, ChessMove, ChessPiece};

fn value_for(k: char) -> Option<u8> {
    // fast binary arithmetic for the sake of "speed"
    match k {
        'a'..='h' => Some((k as u8) - b'a' + 1),
        _ => None,
    }
}
trait ParsePgnMove {
    fn parse_san_move(&self, pgn_move: &str) -> ChessMove;
}

impl ParsePgnMove for ChessMove {
    fn parse_san_move(&self, pgn_move: &str) -> ChessMove {
        let piece_type = ChessPiece::from(pgn_move);
    }
}
