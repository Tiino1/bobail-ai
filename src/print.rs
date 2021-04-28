use crate::board;

static RED_SYMBOL: &'static str = "r";
static BOBAIL_SYMBOL: &'static str = "B";
static GREEN_SYMBOL: &'static str = "g";
static EMPTY_SYMBOL: &'static str = ".";

pub fn print_board(board: &board::Board) {
    let mut mask: i32 = 1;
    for i in 1..26 {
        if board.red & mask != 0 {
            print!(" {}", RED_SYMBOL);
        } else if board.bobail & mask != 0 {
            print!(" {}", BOBAIL_SYMBOL);
        } else if board.green & mask != 0 {
            print!(" {}", GREEN_SYMBOL);
        } else {
            print!(" {}", EMPTY_SYMBOL);
        }
        if i % 5 == 0 {
            println!("");
        }
        mask = mask << 1;
    }
    println!("");
}

pub fn print_red(board: &board::Board) {
    println!("Red piece board is:");
    print_piece_board(board.red, RED_SYMBOL, EMPTY_SYMBOL);
}

pub fn print_bobail(board: &board::Board) {
    println!("Bobail piece board is:");
    print_piece_board(board.bobail, BOBAIL_SYMBOL, EMPTY_SYMBOL);
}

pub fn print_green(board: &board::Board) {
    println!("Green piece board is:");
    print_piece_board(board.green, GREEN_SYMBOL, EMPTY_SYMBOL);
}

pub fn print_piece_board(piece_board: i32, piece_symbol: &str, empty_symbol: &str) {
    let mut mask: i32 = 1;
    for i in 1..26 {
        if piece_board & mask != 0 {
            print!(" {}", piece_symbol);
        } else {
            print!(" {}", empty_symbol);
        }
        if i % 5 == 0 {
            println!("");
        }
        mask = mask << 1;
    }
    println!("");
}

