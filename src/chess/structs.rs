pub enum ChessPositionRow {
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
    H = 8,
}

pub enum ChessPositionCollumn {
    ONE = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
}

pub enum ChessPiece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

impl<T: Into<&str>> From<T> for ChessPiece {
    fn from(object: T) -> Self {
        fn get_specific_piece(first_char: &char) -> ChessPiece {
            // I am aware this is not the best design decision, but shit it works
            // Why didn't I inline it? good question!
            match first_char {
                'R' => ChessPiece::Rook,
                'N' => ChessPiece::Knight,
                'Q' => ChessPiece::Queen,
                'B' => ChessPiece::Bishop,
                'K' => ChessPiece::King,
                _ => panic!("AAAAAAAAAAAH"), // TODO: finish the actual error logic for this branch
            }
        }

        let first_char = object.into().chars().next().expect("should not be empty");
        if first_char.is_uppercase() {
            return get_specific_piece(&first_char);
        } else {
            return ChessPiece::Pawn;
        }
    }
}

pub struct ChessGame {
    moves: Vec<ChessMove>,
}

pub struct ChessMove {
    piece: ChessPiece,
    begin: ChessPosition,
    end: ChessPosition,
}

struct ChessPosition {
    row: ChessPositionRow,
    collumn: ChessPositionCollumn,
}

impl ChessGame {
    fn new() -> Self {
        ChessGame { moves: Vec::new() }
    }
}
