#!/usr/local/bin/julia

#= klotski.jl
Solving the Klotski block puzzle through random brute-force search.
Author: Jorge Yanar
=#
using JLD

function print_board(board)
  println()
  for i = 1 : 5
    print("\t")
    for j = 1 : 4
      print(board[i,j] , " ")
    end
    println()
  end
  println()
end


function get_next_states(board)
  # Returns a list of possible next states, given board state.
  states = [];
  # Get locations of 0s in (i,j) tuples
  zeros_idx = findall(x->x==0, board);
  # For each 0, check which pieces touch it
  pieces = [];
  for idx in zeros_idx
    i = idx[1] ; j = idx[2];
    if i != 1 && board[i-1, j] != 0
      push!(pieces , get_board_piece_at_ij(board, [i-1, j]));
    end
    if i != 5 && board[i+1, j] != 0
      push!(pieces , get_board_piece_at_ij(board, [i+1, j]));
    end
    if j != 1 && board[i, j-1] != 0
      push!(pieces , get_board_piece_at_ij(board, [i, j-1]));
    end
    if j != 4 && board[i, j+1] != 0
      push!(pieces , get_board_piece_at_ij(board, [i, j+1]));
    end
  end
  pieces = unique(pieces);

  # Check if pieces that were touching empty space can move
  for p in pieces
    if is_valid_move(board, p, [-1, 0]) # UP
      push!(states, move_piece(board, p, (-1, 0)));
    end
    if is_valid_move(board, p, [1, 0])  # DOWN
      push!(states, move_piece(board, p, (1,  0)));
    end
    if is_valid_move(board, p, [0, -1]) # LEFT
      push!(states, move_piece(board, p, (0, -1)));
    end
    if is_valid_move(board, p, [0, 1])  # RIGHT
      push!(states, move_piece(board, p, (0,  1)));
    end
  end
  return states
end


function is_valid_move(board, piece, direction)
  # Checks if a given move is valid
  p, i, j = piece;
  di, dj = direction;
  # Check if movement is out of bounds
  if (i == 5 && di == 1)  ||
     (i == 1 && di == -1) ||
     (j == 1 && dj == -1) ||
     (j == 4 && dj == 1)
    return false
  end
  # Check if movement is out of bounds for specific piece type
  if (p == 2 && i == 4 && di == 1) ||
     (p == 3 && j == 3 && dj == 1) ||
     (p == 4 && i == 4 && di == 1) ||
     (p == 4 && j == 3 && dj == 1)
    return false
  else
    # Single square
    if p == 1
      return board[i+di, j+dj] == 0

    # Vertical rectangle
    elseif p == 2
      # Up/Down
      if     di == -1 return board[i+di,   j] == 0
      elseif di ==  1 return board[i+1+di, j] == 0
      # Left/Right
      elseif dj == -1 || dj == 1
        return board[i, j+dj] == 0 && board[i+1, j+dj] == 0
      end

    # Horizontal rectangle
    elseif p == 3
      # Up/Down
      if di != 0 return board[i+di, j] == 0 && board[i+di, j+1] == 0 end
      # Left/Right
      if     dj == -1 return board[i, j+dj]   == 0
      elseif dj ==  1 return board[i, j+1+dj] == 0
      end

    # Large square block
    elseif p == 4
      # Up/Down
      if     di == -1 return board[i+di,   j] == 0 && board[i+di,   j+1] == 0
      elseif di ==  1 return board[i+1+di, j] == 0 && board[i+1+di, j+1] == 0
      # Left/Right
      elseif dj == -1 return board[i,   j+dj] == 0 && board[i+1,   j+dj] == 0
      elseif dj ==  1 return board[i, j+1+dj] == 0 && board[i+1, j+1+dj] == 0
      end
    end
  end
end


function move_piece(board, piece, direction, check_validity=false)
  # Moves a piece on the board in the specified direction, optionally
  # checking if the move is valid prior to doing so.
  board = copy(board);
  p, i, j = piece;
  di, dj = direction;
  if check_validity && is_valid_move(board, piece, direction) == false
    return nothing
  else
    if p == 1
      board[i+di, j+dj] = 1;
      board[i, j] = 0;

    elseif p == 2
      board[i,   j] = 0;
      board[i+1, j] = 0;
      board[i+di,   j+dj] = 2;
      board[i+1+di, j+dj] = 2;

    elseif p == 3
      board[i,   j] = 0;
      board[i, j+1] = 0;
      board[i+di,   j+dj] = 3;
      board[i+di, j+1+dj] = 3;

    elseif p == 4
      board[i,     j] = 0;
      board[i,   j+1] = 0;
      board[i+1,   j] = 0;
      board[i+1, j+1] = 0;
      board[i+di,     j+dj] = 4;
      board[i+di,   j+1+dj] = 4;
      board[i+1+di,   j+dj] = 4;
      board[i+1+di, j+1+dj] = 4;
    end
  end
  return board
end


function get_board_vector_repr(board)
  board = copy(board);
  pieces = [];
  for i = 1 : 5
    for j = 1 : 4
      if board[i,j] == 1
        push!(pieces, (1, i, j));
      end
      if board[i,j] == 2
        push!(pieces, (2, i, j));
        board[i+1,j] = 0;
      end
      if board[i,j] == 3
        push!(pieces, (3, i, j));
        board[i,j+1] = 0;
      end
      if board[i,j] == 4
        push!(pieces, (4, i, j));
        board[i+1,  j] = 0;
        board[i,  j+1] = 0;
        board[i+1,j+1] = 0;
      end
    end
  end
  return pieces
end


function get_board_matrix_repr(pieces)
  board = zeros((5, 4));
  for p in pieces
    piece = p[1];
    i, j = p[2], p[3];
    if piece == 1
      board[i, j] = 1;
    elseif piece == 2
      board[i,   j] = 2;
      board[i+1, j] = 2;
    elseif piece == 3
      board[i,   j] = 3;
      board[i, j+1] = 3;
    elseif piece == 4
      board[i,   j] = 4; board[i,   j+1] = 4;
      board[i+1, j] = 4; board[i+1, j+1] = 4;
    end
  end
  return board
end


function get_board_pieces_of_type_a(board, a)
  board = copy(board);
  pieces = [];
  for i = 1 : 5
    for j = 1 : 4
      if board[i,j] == a
        push!(pieces, (a, i, j));
        if a == 2 board[i+1,j] = 0 end
        if a == 3 board[i,j+1] = 0 end
        if a == 4
          board[i+1,   j] = 0;
          board[i,   j+1] = 0;
          board[i+1, j+1] = 0;
        end
      end
    end
  end
  return pieces
end


function get_board_piece_at_ij(board, ij_tuple)
  i, j = ij_tuple;
  if     board[i,j] == 0 return ()
  elseif board[i,j] == 1 return (1, i, j)
  elseif board[i,j] == 2
    if     i == 1 return (2, i, j)
    elseif i == 5 return (2, i-1, j)
    else
      # Need to figure out which 2-piece this belongs to
      candidates_2s = filter(p->p[3] == j, get_board_pieces_of_type_a(board, 2));
      for p in candidates_2s
        if     p[2]   == i return (2, i, j)
        elseif p[2]+1 == i return (2, i-1, j)
        elseif p[2]-1 == i return (2, i+1, j)
        end
      end
    end
  elseif board[i,j] == 3
    if     j == 4 return (3, i, j-1)
    elseif j == 1 return (3, i, j)
    else
      three_idx = findall(x->x==3, board);
      return (3, three_idx[1][1], three_idx[1][2])
    end
  elseif board[i,j] == 4
    return get_board_pieces_of_type_a(board, 4)[1]
  end
  return nothing
end


function remove_cycles(states_sequence)
  # """ Removes cycles from a given sequence of states. """
  states_hashed = map(v -> hash(v), states_sequence);
  unique_states = unique(states_hashed);
  for i = 1 : length(unique_states)
    curr_state_idxs = findall(hv->hv==unique_states[i], states_hashed);
    if length(curr_state_idxs) > 1
      # Get rid of all repeats
      bad_idx = curr_state_idxs[1]+1 : curr_state_idxs[end];
      deleteat!(states_hashed, bad_idx);
      deleteat!(states_sequence, bad_idx);
    end
  end
  return states_sequence
end


function find_solution_path(board, solns)
  nvisited = 1;
  states_seq = [board];
  while true
    # Generate list of possible next states
    next_states = get_next_states(board);

    # For each state, check whether it contains a piece placed
    # in a winning configuration
    for state in next_states
      pieces = get_board_vector_repr(state);
      if any(s -> s in solns, pieces)
        println("Found solution state!");
        push!(states_seq, state);
        nvisited += 1;
        states_seq = remove_cycles(states_seq);
        return state, states_seq, nvisited
      end
    end

    # None found, randomly select one of the next_states
    board = rand(next_states);
    push!(states_seq, board);
    nvisited += 1;

    # Trim cycles every 10,000 steps to reduce computational load
    if nvisited % 10000 == 0
      print("States visited: ", nvisited);
      states_seq = remove_cycles(states_seq);
      println(" current pathlength: ", length(states_seq));
    end
  end
end

###############################################################################

function main()
  
  # Starting state
  board = [
    2 4 4 2
    2 4 4 2
    2 3 3 2
    2 1 1 2
    1 0 0 1
  ];

  println("Starting position:");
  print_board(board)

  solns = [(4, 4, 2)];
  solved_board, states_sequence, nvisited = find_solution_path(board, solns);

  println("Solved board:")
  print_board(solved_board)

  println("Total number of states visited: ", nvisited);
  println("Final length of state sequence, after removing cycles: ", length(states_sequence));

  save("klotski_path.jld", "states_sequence", states_sequence,
                              "solved_board", solved_board,
                                  "nvisited", nvisited);
end

main()
