// Representation of the game
// The board has a 5x5 size
// Positions in the board follow the sense of lecture (e.g. 0 is considered top-left, 24 is at the bottom-right)
// Red starts on the board top side, green on the bottom side.
pub struct Board {
    pub red: i32,
    pub bobail: i32,
    pub green: i32,
}

pub fn init_board() -> Board {
   return Board {
       red: 31,
       bobail: 4096,
       green: 32505856,
   };
}

