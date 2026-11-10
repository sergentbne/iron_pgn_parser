use crate::chess::structs::{ChessGame, ChessMove, ChessPiece};

trait ParsePgnMove {
    fn parse_san_move(&self, pgn_move: &str) -> ChessMove;
}

impl ParsePgnMove for ChessMove {
    fn parse_san_move(&self, pgn_move: &str) -> ChessMove {
        let piece_type = ChessPiece::from(pgn_move);
    }
}
