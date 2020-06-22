//
// klotski.rs
//
// Solving the Klotski block puzzle through random brute-force search.
// Author: Jorge Yanar
//

//use std::io;
//use std::cmp::Ordering;
//use rand::Rng;
use rand::seq::SliceRandom;
use rand::Rng;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use itertools::Itertools;

use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let solns: Vec<(i32,usize,usize)> = vec![(4,3,1), (4,1,3)];
    let board: [[i32; 4]; 5] = [
        [2, 4, 4, 2],
        [2, 4, 4, 2],
        [2, 3, 3, 2],
        [2, 0, 0, 2],
        [1, 1, 1, 1],
    ];
    print_board(board);

    let ss = randomwalk(board, 10);
    for i in 0 .. ss.len() {
        print_board(ss[i]);
    }

    // let nstates = get_next_states(board);
    // for i in 0 .. nstates.len() {
    //     print_board(nstates[i]);
    //     let ps = get_board_tuple_repr(nstates[i]);
    // }
    

    // let x = rand::thread_rng().gen_range(0, 4);
    // println!("{}", x);
    // let (states_seq, nvisited) = find_solution_path(board, solns);

    // let nstates = get_next_states(board);
    // for i in 0 .. nstates.len() {
    //     print_board(nstates[i]);
    //     let ps = get_board_tuple_repr(nstates[i]);
    //     for j in 0 .. ps.len() {
    //         println!("\t{} {} {}", ps[j].0, ps[j].1, ps[j].2);
    //     }
    //     println!("");
    // }
}


fn randomwalk(board: [[i32; 4]; 5], n_steps: usize) -> Vec<[[i32; 4]; 5]> {
    let mut states_seq: Vec<[[i32; 4]; 5]> = vec![board];
    for i in 0 .. n_steps {
        let next_states = get_next_states(board);
        let board = next_states[rand::thread_rng().gen_range(0, next_states.len())];
        states_seq.push(board);
    }
    return states_seq;
}




/// Runs brute-force search on a board to find a path from the given board
/// state to one that satisfies the piece positions found in solns
fn find_solution_path(
    board: [[i32; 4]; 5],
    solns: Vec<(i32,usize,usize)>
) -> (Vec<[[i32; 4]; 5]>, usize) {

    // Keep track of number of visited states and a sequential path of all the
    // boards visited.
    let mut nvisited: usize = 1;
    let mut soln_found: bool = false;
    let mut states_seq: Vec<[[i32; 4]; 5]> = vec![board];

    // While we've not found a board state containing a piece specified in the
    // solns vec, continue a random walk through the klotski space.
    loop {
        let next_states = get_next_states(board);
        for i in 0 .. next_states.len() {
            // check 
            let ps = get_board_tuple_repr(next_states[i]);
            for j in 0 .. ps.len() {
                if solns.contains(&ps[j]) {
                    soln_found = true;
                }
            }

            if soln_found == true {
                println!("Found solution state!");
                states_seq.push(next_states[i]);
                nvisited += 1;
                states_seq = remove_cycles(states_seq);
                return (states_seq, nvisited);
            }

        }
        let board = next_states[rand::thread_rng().gen_range(0, next_states.len())];
        states_seq.push(board);
        nvisited += 1;
        if nvisited % 10000 == 0 {
            println!("States visited: {}", nvisited);
            print_board(states_seq[states_seq.len()-1]);
            // states_seq = remove_cycles(states_seq);
        }
    }
}
// fn hash_states_seq(states_seq: Vec<[[i32;4];5]>) -> Vec<u64> {
//     let mut hasher = DefaultHasher::new();
//     let sh = 
    

// }


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
    let pieces: Vec<_> = pieces.into_iter().unique().collect();


    // Check if any of the identified pieces can move. If so, add resulting
    // board state to the list of possible future states
    let mut states: Vec<[[i32; 4]; 5]> = vec![];
    for p in 0 .. pieces.len() {
        // UP
        if is_valid_move(board, pieces[p], (-1, 0)) {
            states.push(move_piece(board, pieces[p], (-1, 0)));
        }
        // DOWN
        if is_valid_move(board, pieces[p], (1, 0)) {
            states.push(move_piece(board, pieces[p], (1, 0)));
        }
        // LEFT
        if is_valid_move(board, pieces[p], (0, -1)) {
            states.push(move_piece(board, pieces[p], (0, -1)));
        }
        // RIGHT
        if is_valid_move(board, pieces[p], (0, 1)) {
            states.push(move_piece(board, pieces[p], (0, 1)));
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
                for p in 0 .. ps.len() {
                    let pi: i32 = ps[p].1 as i32;
                    let pj: i32 = ps[p].2 as i32;
                    if pi == i as i32 {
                        return (2, i, j);
                    } else if pi + 1 == i as i32 {
                        return (2, i-1, j);
                    } else if pi - 1 == i as i32{
                        return (2, i+1, j);
                    } else {
                        continue;
                    }
                }
            },
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
    // Otherwise, the general case: match based on movement type (left/right)
    // or (up/down) and then do piece-specific checks.
    match di {
        -1 => match p {
            1 => return board[i-1][j] == 0,
            2 => return board[i-1][j] == 0,
            3 => return board[i-1][j] == 0 && board[i-1][j+1] == 0,
            4 => return board[i-1][j] == 0 && board[i-1][j+1] == 0,
            _ => (),
        },
        1 => match p {
            1 => return board[i+1][j] == 0,
            2 => return board[i+2][j] == 0,
            3 => return board[i+1][j] == 0 && board[i+1][j+1] == 0,
            4 => return board[i+2][j] == 0 && board[i+2][j+1] == 0,
            _ => (),
        },
        _ => (),
    };
    match dj {
        -1 => match p {
            1 => return board[i][j-1] == 0,
            2 => return board[i][j-1] == 0 && board[i+1][j-1] == 0,
            3 => return board[i][j-1] == 0,
            4 => return board[i][j-1] == 0 && board[i+1][j-1] == 0,
            _ => (),
        },
        1 => match p {
            1 => return board[i][j+1] == 0,
            2 => return board[i][j+1] == 0 && board[i+1][j+1] == 0,
            3 => return board[i][j+2] == 0,
            4 => return board[i][j+2] == 0 && board[i+1][j+2] == 0,
            _ => (),
        },
        _ => (),

    };
    return false;
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

    /// Helper function for flexible indexing
    fn idxfn(x: usize, dx: i32) -> usize {
        return (x as i32 + dx) as usize;
    }

    match p {
        1 => {
            board[i][j] = 0;
            board[idxfn(i,di)][idxfn(j,dj)] = 1;
        },
        2 => {
            board[i ][ j] = 0;
            board[i+1][j] = 0;
            board[idxfn(i, di)][idxfn(j,dj)] = 2;
            board[idxfn(i,1+di)][idxfn(j,dj)] = 2;
        },
        3 => {
            board[i ][ j] = 0;
            board[i][j+1] = 0;
            board[idxfn(i,di)][idxfn(j , dj)] = 3;
            board[idxfn(i,di)][idxfn(j,1+dj)] = 3;
        },
        4 => {
            board[i][  j] = 0;
            board[i][j+1] = 0;
            board[i+1][  j] = 0;
            board[i+1][j+1] = 0;
            board[idxfn(i , di)][idxfn(j , dj)] = 4;
            board[idxfn(i , di)][idxfn(j,1+dj)] = 4;
            board[idxfn(i,1+di)][idxfn(j , dj)] = 4;
            board[idxfn(i,1+di)][idxfn(j,1+dj)] = 4;
        },
        _ => panic!("no such piece exists!"),
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
                },
                4 => {
                    pieces.push((4,i,j));
                    board[i+1][j] = 0;
                    board[i][j+1] = 0;
                    board[i+1][j+1] = 0;
                },
                0 => (),
                _ => panic!("no such piece exists!"),
            }
        }
    }
    return pieces;
}


/// Returns list of all pieces on the board of specified type, in tuple
/// representation (piecetype: i32, row: usize, col: usize)
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

/// Removes cycles from a given sequence of states
fn remove_cycles(states_seq: Vec<[[i32; 4]; 5]>) -> Vec<[[i32; 4]; 5]> {
    return states_seq;
}




// Code for checking is_valid_move using a match piece-first approach

    // // Otherwise, check more specific movements
    // match p {
    //     // Single square
    //     1 => return board[(i as i32 + di) as usize][(j as i32 + dj) as usize] == 0,
    //     // Vertical rectangle
    //     2 => {
    //         // Up / Down
    //         match di {
    //             -1 => return board[i-1][j] == 0,
    //              1 => return board[i+2][j] == 0,
    //              _ => (),
    //         };
    //         // Left / Right
    //         match dj {
    //             -1 => return board[i][j-1] == 0 && board[i+1][j-1] == 0,
    //              1 => return board[i][j+1] == 0 && board[i+1][j+1] == 0,
    //              _ => (),
    //         };
    //     },
    //     // Horizontal rectangle
    //     3 => {
    //         // Up/Down
    //         match di {
    //             -1 => return board[i-1][j] == 0 && board[i-1][j+1] == 0, // Up
    //              1 => return board[i+1][j] == 0 && board[i+1][j+1] == 0, // Down
    //              _ => (),
    //         };
    //         // Left/Right
    //         match dj {
    //             -1 => return board[i][j-1] == 0, // Left
    //              1 => return board[i][j+2] == 0, // Right
    //              _ => (),
    //         };
    //     },
    //     // Large square block
    //     4 => {
    //         match di { // UP / DOWN
    //             -1 => return board[i-1][j] == 0 && board[i-1][j+1] == 0 , // UP
    //              1 => return board[i+2][j] == 0 && board[i+2][j+1] == 0 , // DOWN
    //              _ => (),
    //         };
    //         match dj { // LEFT / RIGHT
    //             -1 => return board[i][j-1] == 0 && board[i+1][j-1] == 0, // LEFT
    //              1 => return board[i][j+2] == 0 && board[i+1][j+2] == 0, // RIGHT
    //              _ => (),
    //         };
    //     },
    // };
