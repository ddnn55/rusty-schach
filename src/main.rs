enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(PartialEq, Eq)]
enum Color {
    Black,
    White,
}

// an base class for a chess piece
struct Piece {
    // the color of the piece
    color: Color,
    // the type of the piece
    piece_type: PieceType,
}

impl Piece {
    fn print(&self) {
        // if color is black
        if self.color == Color::Black {
            // set ansi background color to black
            print!("\x1b[38;5;0m");
        } else {
            // set ansi background color to white
            print!("\x1b[38;5;15m");
        }
        match self.piece_type {
            PieceType::Pawn => print!("P "),
            PieceType::Rook => print!("R "),
            PieceType::Knight => print!("N "),
            PieceType::Bishop => print!("B "),
            PieceType::Queen => print!("Q "),
            PieceType::King => print!("K "),
        }
    }
}

fn make_pawn(color: Color) -> Piece {
    Piece {
        color,
        piece_type: PieceType::Pawn,
    }
}

fn main() {
    // set ansi color to bright green
    // print!("\x1b[38;5;10m");

    // initilialize a 2d array of booleans
    let arr: [[Option<Piece>; 8]; 8] = [
        [None, None, None, None, None, None, None, None],
        [Some(make_pawn(Color::Black)), Some(make_pawn(Color::Black)), Some(make_pawn(Color::Black)), Some(make_pawn(Color::Black)), Some(make_pawn(Color::Black)), Some(make_pawn(Color::Black)), Some(make_pawn(Color::Black)), Some(make_pawn(Color::Black))],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [Some(make_pawn(Color::White)), Some(make_pawn(Color::White)), Some(make_pawn(Color::White)), Some(make_pawn(Color::White)), Some(make_pawn(Color::White)), Some(make_pawn(Color::White)), Some(make_pawn(Color::White)), Some(make_pawn(Color::White))],
        [None, None, None, None, None, None, None, None],
    ];


    // print the array
    let mut parity = true;
    for row in arr.iter() {
        for col in row.iter() {
            if parity {
                // set ansi background color to gray
                print!("\x1b[48;5;240m");
            } else {
                print!("\x1b[48;5;22m");
            }
            parity = !parity;
            match col {
                Some(piece) => {
                    piece.print();
                }
                None => print!("  "),
            }
        }
        parity = !parity;
        println!("");
    }
}
