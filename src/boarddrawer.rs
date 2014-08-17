use api;

pub fn draw_board(boardsize: uint,
               black_st: &[api::Vertex], white_st: &[api::Vertex],
               black_cp: uint, white_cp: uint) -> String {
    if boardsize < 1 || boardsize > 25 {
        fail!("Invalid board size for drawing.");
    }
    let mut output: String = format!("Captured stones : {:u} by white and {:u} by black.\n",
                                        black_cp, white_cp);
    let mut board = [[0u8, ..25], ..25];
    // black is 1 white is 2
    for &st in black_st.iter() {
        let (l,n) = st.to_coords();
        board[(n-1) as uint][(l-1) as uint] = 1;
    }
    for &st in white_st.iter() {
        let (l,n) = st.to_coords();
        board[(n-1) as uint][(l-1) as uint] = 2;
    }
    for n in range(0, boardsize){
        output = output.append(format!("{:2u}", boardsize-n).as_slice());
        for l in range(0, boardsize) {
            output = output.append(match board[boardsize-n-1][l] {
                1 => " B",
                2 => " W",
                _ => " ."
            });
        }
        output = output.append("\n");
    }
    output = output.append("\n  ");
    for l in range(0, boardsize) {
        output.grow(1, ' ');
        output.grow(1, if l < 8 {
            (('A' as u8) + (l as u8)) as char
        } else {
            (('B' as u8) + (l as u8)) as char
        });
    }
    output
}
