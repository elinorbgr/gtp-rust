use api;

pub fn draw_board(boardsize: usize,
               black_st: &[api::Vertex], white_st: &[api::Vertex],
               black_cp: usize, white_cp: usize) -> String {
    if boardsize < 1 || boardsize > 25 {
        panic!("Invalid board size for drawing.");
    }
    let mut output: String = format!("Captured stones : {} by white and {} by black.\n",
                                        black_cp, white_cp);
    let mut board = [[0u8, 25]; 25];
    // black is 1 white is 2
    for &st in black_st.iter() {
        let (l,n) = st.to_coords();
        board[(n-1) as usize][(l-1) as usize] = 1;
    }
    for &st in white_st.iter() {
        let (l,n) = st.to_coords();
        board[(n-1) as usize][(l-1) as usize] = 2;
    }
    for n in 0..boardsize {
        output.push_str(&format!("{:2}", boardsize-n));
        for l in 0..boardsize {
            output.push_str(match board[boardsize-n-1][l] {
                1 => " B",
                2 => " W",
                _ => " ."
            });
        }
        output.push_str("\n");
    }
    output.push_str("\n  ");
    for l in 0..boardsize {
        output.push(' ');
        output.push(if l < 8 {
            (('A' as u8) + (l as u8)) as char
        } else {
            (('B' as u8) + (l as u8)) as char
        });
    }
    output
}
