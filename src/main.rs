mod board;
mod print;

fn main() {
    let board: board::Board = board::init_board();
    print::print_board(&board);
    print::print_red(&board);
    print::print_bobail(&board);
    print::print_green(&board);
}
