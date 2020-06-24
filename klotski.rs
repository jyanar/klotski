//
// klotski.rs
//
// Solving the Klotski block puzzle through random brute-force search.
// Author: Jorge Yanar
//

use rand::Rng;
use rand::seq::SliceRandom;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use itertools::Itertools;

use std::any::type_name;


fn main() {
    let solns: Vec<(i32,usize,usize)> = vec![(4,3,1)];
    let board: [[i32; 4]; 5] = [
        [2, 4, 4, 2],
        [2, 4, 4, 2],
        [2, 3, 3, 2],
        [2, 1, 1, 2],
        [1, 0, 0, 1],
    ];
    println!("Starting board: ");
    print_board(&board);
    let (states_seq, nvisited) = find_solution_path(board, solns);
    println!("Number of states visited: {}", nvisited);
    println!("Final board state: ");
    print_board(&states_seq.last().unwrap());
}


fn take_step(board: [[i32;4];5]) -> [[i32;4];5] {
    let next_states = get_next_states(board);
    next_states[rand::thread_rng().gen_range(0, next_states.len())]
}


fn randomwalk(board: [[i32; 4]; 5], n_steps: usize) -> Vec<[[i32; 4]; 5]> {
    let mut states_seq: Vec<[[i32; 4]; 5]> = vec![board];
    for i in 0 .. n_steps {
        // let board = take_step(board);
        states_seq.push(take_step(states_seq[states_seq.len()-1]));
    }
    return states_seq;
}


/// Runs brute-force search on a board to find a path from the given board
/// state to one that satisfies the piece positions found in solns
fn find_solution_path(
    board: [[i32; 4]; 5],
    solns: Vec<(i32,usize,usize)>
) -> (Vec<[[i32; 4]; 5]>, usize) {
    let mut nvisited: usize = 1;
    let mut states_seq: Vec<[[i32; 4]; 5]> = vec![board];

    loop {
        let next_states = get_next_states(states_seq.last().unwrap().clone());
        for i in 0 .. next_states.len() {
            let ps = get_board_tuple_repr(&next_states[i]);
            let soln_found = ps.iter().any(|&p| solns.contains(&p));
            if soln_found {
                println!("Found solution state!");
                states_seq.push(next_states[i].clone());
                nvisited += 1;
                // states_seq = remove_cycles(states_seq);
                return (states_seq, nvisited);
            }
        }
        states_seq.push(next_states[rand::thread_rng().gen_range(0, next_states.len())].clone());
        nvisited += 1;
        if nvisited % 10000 == 0 {
            println!("States visited: {}", nvisited);
            // states_seq = remove_cycles(states_seq);
        }
    }
}


/// Removes cycles from a given sequence of states
fn remove_cycles(states_seq: Vec<[[i32; 4]; 5]>) -> Vec<[[i32; 4]; 5]> {
    return states_seq;
}


// fn hash_states_seq(states_seq: Vec<[[i32;4];5]>) -> Vec<u64> {
//     let mut hasher = DefaultHasher::new();
//     let sh = 
// }


/// Prints the board to stdout
fn print_board(board: &[[i32; 4]; 5]) {
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
            pieces.push(get_board_piece_at_ij(&board, i-1, j));
        }
        if i != 4 && board[i+1][j] != 0 {
            pieces.push(get_board_piece_at_ij(&board, i+1, j));
        }
        if j != 0 && board[i][j-1] != 0 {
            pieces.push(get_board_piece_at_ij(&board, i, j-1));
        }
        if j != 3 && board[i][j+1] != 0 {
            pieces.push(get_board_piece_at_ij(&board, i, j+1));
        }
    }
    // Remove duplicate pieces, as well as "empty" pieces
    let mut pieces: Vec<_> = pieces.into_iter().unique().collect();
    // for p in 0 .. pieces.len() {
    //     println!("{}", pieces[p].0);
    // }
    pieces.retain(|&p| p.0 != 0);

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


/// Returns the board piece, in (i32, usize, usize) form, located
/// at index i, j
fn get_board_piece_at_ij(
    board: &[[i32; 4]; 5],
    i: usize,
    j: usize,
) -> (i32,usize,usize) {
    match board[i][j] {
        // Pieces
        0 => return (0, i, j),
        1 => return (1, i, j),
        2 => match i {
            0 => return (2, i, j),
            4 => return (2, i-1, j),
            _ => {
                let candidates: Vec<(i32,usize,usize)> = get_board_pieces_of_type(&board, 2)
                                                                    .into_iter()
                                                                    .filter(|&p| p.2 == j)
                                                                    .collect();
                for p in 0 .. candidates.len() {
                    let pi = candidates[p].1; let pj = candidates[p].2;
                    if pi == i {
                        return (2, i, j);
                    } else if pi + 1 == i {
                        return (2, (i as i32 - 1 as i32) as usize, j);
                    } else if (pi as i32 - 1 as i32) as usize == i {
                        return (2, i+1, j);
                    }
                }
                panic!("2 piece not found!");
            },
        },
        3 => match j {
            0 => return (3, i, j),
            3 => return (3, i, j-1),
            _ => return get_board_pieces_of_type(&board, 3)[0],
        },
        4 => return get_board_pieces_of_type(&board, 4)[0],
        _ => panic!("piece not found!"),
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

    let mut board = board.clone();
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


fn get_board_tuple_repr(board: &[[i32; 4]; 5]) -> Vec<(i32,usize,usize)> {
    let mut board = board.clone();
    let mut pieces: Vec<(i32,usize,usize)> = vec![];
    for i in 0 .. 5 {
        for j in 0 .. 4 {
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
    board: &[[i32; 4]; 5],
    piece_type: i32
) -> Vec<(i32,usize,usize)> {
    let mut board = board.clone();
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


fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

