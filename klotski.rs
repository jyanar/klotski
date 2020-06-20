//
// klotski.rs
//
// Solving the Klotski block puzzle through random brute-force search.
// Author: Jorge Yanar
//

//use std::io;
//use std::cmp::Ordering;
//use rand::Rng;


fn main() {
    println!("Running!");
    let board: [[i32; 4]; 5] = [
        [2, 4, 4, 2],
        [2, 4, 4, 2],
        [2, 3, 3, 2],
        [2, 1, 1, 2],
        [1, 0, 0, 1],
    ];
    print_board(board);

    //let ps = get_board_pieces_of_type(board, 4);
    // for p in ps.iter() {
    //     println!("{} {} {}", p.0, p.1, p.2);
    // }
    // let p = get_board_piece_at_ij(board, 2, 0);
    // println!("{} {} {}", p.0, p.1, p.2);


    // get_next_states(board);
    
}


/// Prints the board to stdout
fn print_board(board: [[i32; 4]; 5]) {
    println!();
    for i in 0..5 {
        print!("\t");
        for j in 0..4 {
            print!("{} ", board[i][j]);
        }
        println!();
    }
    println!();
}


/// Returns a list of possible next board states, given a board state
fn get_next_states(board: [[i32; 4]; 5]) -> Vec<[[i32; 4]; 5]> {
    // Find indices of empty board spaces
    let mut zeros_idx: Vec<(usize,usize)> = vec![];
    for i in 0..5 {
        for j in 0..4 {
            if board[i][j] == 0 {
                zeros_idx.push((i, j));
            }
        }
    }
    // For each empty space, check which pieces touch it
    let mut pieces: Vec<(i32,usize,usize)> = vec![];
    for (i,j) in zeros_idx.into_iter() {
        if i != 0 && board[i-1][j] != 0 {
            pieces.push(get_board_piece_at_ij(board, i-1, j));
        }
        if i != 4 && board[i+1][j] != 0 {
            pieces.push(get_board_piece_at_ij(board, i+1, j));
        }
        if j != 0 && board[i][j-1] != 0 {
            pieces.push(get_board_piece_at_ij(board, i, j-1));
        }
        if j != 3 && board[i][j+1] != 0 {
            pieces.push(get_board_piece_at_ij(board, i, j+1));
        }
    }
    // Check if any of the identified pieces can move. If so, add resulting
    // board state to the list of possible future states
    let mut states: Vec<[[i32; 4]; 5]> = vec![];
    for p in pieces.iter() {
        // UP
        if is_valid_move(board, *p, (-1, 0)) {
            states.push(move_piece(board, *p, (-1, 0)));
        }
        // DOWN
        if is_valid_move(board, *p, (1, 0)) {
            states.push(move_piece(board, *p, (1, 0)));
        }
        // LEFT
        if is_valid_move(board, *p, (0, -1)) {
            states.push(move_piece(board, *p, (0, -1)));
        }
        // RIGHT
        if is_valid_move(board, *p, (-1, 0)) {
            states.push(move_piece(board, *p, (0, 1)));
        }
    }
    return states;
}


/// Returns the board piece, in (i32, i32, i32) form, located
/// at index i, j
fn get_board_piece_at_ij(
    board: [[i32; 4]; 5],
    i: usize,
    j: usize,
) -> (i32,usize,usize) {
    println!("PIECE AT IJ: {}", board[i][j]);
    if board[i][j] == 0 {
        return (0, i, j);
    } else if board[i][j] == 1 {
        return (1, i, j);
    } else if board[i][j] == 2 {
        match i {
            0 => return (2, i, j),
            4 => return (2, i-1, j),
            _ => {
                let ps = get_board_pieces_of_type(board, 2);
                for p in ps.iter() {
                    if p.1 == i {
                        return (2, i, j);
                    } else if p.1 + 1 == i {
                        return (2, i-1, j);
                    } else if p.1 - 1 == i {
                        return (2, i+1, j);
                    } else {
                        continue;
                    }
                }
            }
        };
        return (0, 0 as usize, 0 as usize);
    } else if board[i][j] == 3 {
        match j {
            0 => return (3, i, j),
            3 => return (3, i, j-1),
            _ => return get_board_pieces_of_type(board, 3)[0],
        };
    } else if board[i][j] == 4 {
        return get_board_pieces_of_type(board, 4)[0];
    } else {
        return (0, 0 as usize, 0 as usize);
    }
}


/// Checks to see if moving a given piece in a specified direction is valid
fn is_valid_move(
    board: [[i32; 4]; 5],
    piece: (i32,usize,usize),
    dir:   (i32,i32),
) -> bool {
    let p = piece.0;
    let i = piece.1;
    let j = piece.2;
    let di = dir.0;
    let dj = dir.1;

    // Easy boundary conditions
    if (i == 4 && di == 1)  ||
       (i == 0 && di == -1) ||
       (j == 0 && dj == -1) ||
       (j == 3 && dj == 1) {
        return false;
    }

    // Piece-specific boundary conditions
    if (p == 2 && i == 3 && di == 1) ||
       (p == 3 && j == 2 && dj == 1) ||
       (p == 4 && i == 3 && di == 1) ||
       (p == 4 && j == 2 && dj == 1) {
        return false;
    }

    // Otherwise, check more specific movements
    match p {
        // Single square
        1 => return board[i+di][j+dj] == 0,
        // Vertical rectangle
        2 => {
            // Left / Right
            if dj == -1 || dj == 1 {
                return board[i][j+dj] == 0 &&
                       board[i+1][j+dj] == 0;
            }
            // Up / Down
            else if di == -1 {
                return board[i+di][j] == 0; // Up
            } else if di == 1 {
                return board[i+1+di][j] == 0; // Down
            }
        },
        // Horizontal rectangle
        3 => {
            // Up/down
            if di == -1 || di == 1 {
                return board[i+di][j] == 0 &&
                       board[i+di][j+1] == 0;
            }
            // Left/right
            else if dj == -1 {
                return board[i][j+dj] == 0;
            }
            else if dj == 1 {
                return board[i][j+1+dj] == 0;
            }
        },
        // Large square block
        4 => {
            // Up/down
            if di == -1 { // Up
                return board[i+di][j]   == 0 &&
                       board[i+di][j+1] == 0;
            } else if di == 1 { // Down
                return board[i+1+di][j]   == 0 &&
                       board[i+1+di][j+1] == 0;
            }
            // Left/right
            if dj == -1 {
                return board[i][j+dj]   == 0 &&
                       board[i+1][j+dj] == 0;
            } else if dj == 1 {
                return board[i][j+1+dj]   == 0 &&
                       board[i+1][j+1+dj] == 0;
            }
        },
    }
}


fn move_piece(
    board: [[i32; 4]; 5],
    piece: (i32,usize,usize),
    dir:   (i32,i32),
) -> [[i32; 4]; 5] {
    let mut board = board;
    let p = piece.0;
    let i = piece.1;
    let j = piece.2;
    let di = dir.0;
    let dj = dir.1;

    match p {
        1 => {
            board[i][j] = 0;
            board[i+di][j+dj] = 1;
        },
        2 => {
            board[i ][ j] = 0;
            board[i+1][j] = 0;
            board[i + di][j+dj] = 2;
            board[i+1+di][j+dj] = 2;
        },
        3 => {
            board[i ][ j] = 0;
            board[i][j+1] = 0;
            board[i+di][j + dj] = 3;
            board[i+di][j+1+dj] = 3;
        },
        4 => {
            board[i  ][  j] = 0;
            board[i  ][j+1] = 0;
            board[i+1][  j] = 0;
            board[i+1][j+1] = 0;
            board[i + di][j + dj] = 4;
            board[i + di][j+1+dj] = 4;
            board[i+1+di][j + dj] = 4;
            board[i+1+di][j+1+dj] = 4;
        },
    }
    return board;
}


fn get_board_tuple_repr(board: [[i32; 4]; 5]) -> Vec<(i32,usize,usize)> {
    let mut board = board;
    let mut pieces: Vec<(i32,usize,usize)> = vec![];
    for i in 0..5 {
        for j in 0..4 {
            match board[i][j] {
                1 => pieces.push((1,i,j)),
                2 => {
                    pieces.push((2,i,j));
                    board[i+1][j] = 0;
                },
                3 => {
                    pieces.push((3,i,j));
                    board[i][j+1] = 0;
                }
                4 => {
                    pieces.push((4,i,j));
                    board[i+1][j] = 0;
                    board[i][j+1] = 0;
                    board[i+1][j+1] = 0;
                }
                _ => panic!("crash and burn"),
            }
        }
    }
    return pieces;
}


// Returns list of all pieces on the board of specified type, in tuple
// representation (piecetype: i32, row: usize, col: usize)
fn get_board_pieces_of_type(
    board: [[i32; 4]; 5],
    piece_type: i32
) -> Vec<(i32,usize,usize)> {
    let mut board = board;
    let mut pieces: Vec<(i32,usize,usize)> = vec![];
    for i in 0..5 {
        for j in 0..4 {
            if board[i][j] == piece_type {
                pieces.push((piece_type, i, j));
                match board[i][j] {
                    2 => board[i+1][j] = 0,
                    3 => board[i][j+1] = 0,
                    4 => {
                        board[i+1][j] = 0;
                        board[i][j+1] = 0;
                        board[i+1][j+1] = 0;
                    }
                    _ => continue,
                }
            }
        }
    }
    return pieces;
}



