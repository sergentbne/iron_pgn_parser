#[derive(Debug, Clone)]
struct ParseMoveError<'a> {
    invalid_move: &'a str,
}

impl std::fmt::Display for ParseMoveError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Move has Errored during parse. ")
    }
}

#[derive(Debug, Clone)]
struct PieceNotFoundError {
    invalid_piece: char,
}

impl std::fmt::Display for PieceNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Passed move's piece is invalid, cannot parse.")
    }
}
