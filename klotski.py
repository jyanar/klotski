#!/usr/local/bin/python3

""" klotski.py
Solving the Klotski block puzzle through random brute-force search.
Author: Jorge Yanar
"""
import time
import numpy as np


def print_board(board):
  print('')
  for i in range(5):
    print('\t', end='')
    for j in range(4):
      print(board[i,j], end=' ')
    print('')
  print('')


def get_next_states(board):
  """ Returns a list of possible next states, given a board state. """
  states = []

  # Get locations of 0s in (i,j) tuples
  zeros_idx = np.where(board == 0)
  zeros_ij = list(zip(zeros_idx[0], zeros_idx[1]))

  # For each 0, check which pieces touch it
  pieces = []
  for i, j in zeros_ij:
    if i != 0 and board[i-1, j] != 0:
      pieces.append(get_board_piece_at_ij(board, (i-1, j)))
    if i != 4 and board[i+1, j] != 0:
      pieces.append(get_board_piece_at_ij(board, (i+1, j)))
    if j != 0 and board[i, j-1] != 0:
      pieces.append(get_board_piece_at_ij(board, (i, j-1)))
    if j != 3 and board[i, j+1] != 0:
      pieces.append(get_board_piece_at_ij(board, (i, j+1)))

  # Check if pieces that were touching empty space can move
  for p in pieces:
    if is_valid_move(board, p, (-1, 0)): # UP
      states.append(move_piece(board, p, (-1, 0)))
    if is_valid_move(board, p, (1, 0)):  # DOWN
      states.append(move_piece(board, p, (1, 0)))
    if is_valid_move(board, p, (0, -1)): # LEFT
      states.append(move_piece(board, p, (0, -1)))
    if is_valid_move(board, p, (0, 1)):  # RIGHT
      states.append(move_piece(board, p, (0, 1)))
  return states


def is_valid_move(board, piece, direction):
  """ Checks if a given move is valid. """
  p, i, j = piece
  deltai, deltaj = direction
  if (i == 4 and direction == (1, 0) or
      i == 0 and direction == (-1, 0) or
      j == 0 and direction == (0, -1) or
      j == 3 and direction == (0, 1)):
    return False
  if (p == 2 and i == 3 and direction == (1, 0) or
      p == 3 and j == 2 and direction == (0, 1) or
      p == 4 and i == 3 and direction == (1, 0) or
      p == 4 and j == 2 and direction == (0, 1)):
    return False
  else:
    # Single square
    if p == 1:
      return board[i+deltai, j+deltaj] == 0

    # Vertical rectangle
    elif p == 2:
      # Left/Right
      if direction == (0, -1) or direction == (0, 1):
        return (board[i,   j+deltaj] == 0 and
                board[i+1, j+deltaj] == 0)
      # Up/Down
      elif direction == (-1, 0): # UP
        return board[i+deltai, j] == 0
      elif direction == (1, 0):  # DOWN
        return board[i+1+deltai, j] == 0

    # Horizontal rectangle
    elif p == 3:
      # Up/Down
      if direction == (-1, 0) or direction == (1, 0):
        return board[i+deltai, j] == 0 and board[i+deltai, j+1] == 0
      # Left/Right
      elif direction == (0, -1):
        return board[i, j+deltaj] == 0
      elif direction == (0, 1):
        return board[i, j+1+deltaj] == 0

    # Large square block
    elif p == 4:
      # Up/Down
      if direction == (-1, 0):   # UP
        return (board[i+deltai,   j] == 0 and
                board[i+deltai, j+1] == 0)
      elif direction == (1, 0):  # DOWN
        return (board[i+1+deltai,   j] == 0 and
                board[i+1+deltai, j+1] == 0)
      # Left/Right
      elif direction == (0, -1): # LEFT
        return (board[i,   j+deltaj] == 0 and
                board[i+1, j+deltaj] == 0)
      elif direction == (0, 1):  # RIGHT
        return (board[i,   j+1+deltaj] == 0 and
                board[i+1, j+1+deltaj] == 0)


def move_piece(board, piece, direction, check_validity=False):
  """ Moves a piece on the board in the specified direction, optionally
  checking if the move is valid prior to doing so.
  """
  board = board.copy()
  p, i, j = piece
  deltai, deltaj = direction
  if check_validity and is_valid_move(board, piece, direction) == False:
    return None
  else:
    if p == 1:
      board[i+deltai, j+deltaj] = 1
      board[i, j] = 0

    elif p == 2:
      board[i,   j] = 0
      board[i+1, j] = 0
      board[i+deltai,   j+deltaj] = 2
      board[i+1+deltai, j+deltaj] = 2

    elif p == 3:
      board[i,   j] = 0
      board[i, j+1] = 0
      board[i+deltai,   j+deltaj] = 3
      board[i+deltai, j+1+deltaj] = 3

    elif p == 4:
      board[i,     j] = 0
      board[i,   j+1] = 0
      board[i+1,   j] = 0
      board[i+1, j+1] = 0
      board[i+deltai,     j+deltaj] = 4
      board[i+deltai,   j+1+deltaj] = 4
      board[i+1+deltai,   j+deltaj] = 4
      board[i+1+deltai, j+1+deltaj] = 4
  return board


def get_board_vector_repr(board, x_type_pieces=None):
  """ Returns list of 3-tuples representing pieces and their locations
  from a matrix representation of a board state. For example, a
  vertical rectangle piece (represented by 2s) at location [1, 0] would
  be represented as (2, 1, 0). The [1, 0] refers to the i,j coordinates
  of the piece's top-left corner.

  TODO: Optimize using np.where
  """
  board = board.copy()
  pieces = []
  for i in range(5):
    for j in range(4):
      if board[i,j] == 1:
        pieces.append((1, i, j))

      if board[i,j] == 2:
        pieces.append((2, i, j))
        board[i+1,j] = 0

      if board[i,j] == 3:
        pieces.append((3, i, j))
        board[i,j+1] = 0

      if board[i,j] == 4:
        pieces.append((4, i, j))
        board[i+1,j] = 0
        board[i,j+1] = 0
        board[i+1,j+1] = 0
  return pieces


def get_board_matrix_repr(pieces):
  """ Returns the 5x4 matrix representation of the board from a list of
  3-tuples representing pieces and their locations.
  """
  board = np.zeros((5, 4))
  for p in pieces:
    if p == ():
      pass
    else:
      piece = p[0]
      i, j = p[1], p[2]
      if piece == 1:
        board[i, j] = 1
      if piece == 2:
        board[i,   j] = 2
        board[i+1, j] = 2
      if piece == 3:
        board[i, j] = 3 ; board[i, j+1] = 3
      if piece == 4:
        board[i,   j] = 4 ; board[i,   j+1] = 4
        board[i+1, j] = 4 ; board[i+1, j+1] = 4
  return board


def get_board_pieces_of_type_a(board, a):
  # TODO: Faster if we do the np.where trick?
  board = board.copy()
  pieces = []
  for i in range(5):
    for j in range(4):
      if board[i,j] == a:
        pieces.append((a, i, j))
        if a == 2: board[i+1,j] = 0
        if a == 3: board[i,j+1] = 0
        if a == 4:
          board[i+1,j] = 0
          board[i,j+1] = 0
          board[i+1,j+1] = 0
  return pieces


# Possibly merge these two ^  v ?
def get_board_piece_at_ij(board, ij_tuple):
  """ Given a board and i,j, returns the vector representation
  of the piece at that position.
  """
  i, j = ij_tuple

  if board[i,j] == 0: return ()
  elif board[i,j] == 1: return (1, i, j)
  elif board[i,j] == 2:
    if i == 0: return (2, i, j)
    elif i == 4: return (2, i-1, j)
    else:
      candidate_pieces = list(
        filter(lambda p: p[2] == j,
               get_board_pieces_of_type_a(board, 2))
      )
      for p in candidate_pieces:
        if p[1] == i:
          return (2, i, j)
        elif p[1]+1 == i:
          return (2, i-1, j)
        elif p[1]-1 == i:
          return (2, i+1, j)
  elif board[i,j] == 3:
    if j == 3:
      return (3, i, j-1)
    elif j == 0:
      return (3, i, j)
    else:
      three_idx = np.where(board == 3)
      return (3, three_idx[0][0], three_idx[1][0])
  elif board[i,j] == 4:
    return get_board_pieces_of_type_a(board, 4)[0]
  return None


def remove_cycles(state_sequence):
  """ Removes cycles from a given sequence of states. """
  states_hashed = np.array([hash(state.tostring())
                            for state in state_sequence])
  unique_states = np.unique(states_hashed)
  for i in range(len(unique_states)):
    curr_state_idxs = np.where(states_hashed == unique_states[i])[0]
    if len(curr_state_idxs) > 1:
      bad_idx = np.arange(curr_state_idxs[0]+1, curr_state_idxs[-1]+1)
      states_hashed = np.delete(states_hashed, bad_idx)
      state_sequence = np.delete(state_sequence, bad_idx, 0)
  return state_sequence


def find_solution_path(board, solns):
  """ Runs brute-force search on a board to find a path from the given
  board state to one that satisfies the piece positions found in solns.
  """
  nvisited = 1
  states_sequence = [board]
  while True:
    next_states = get_next_states(board)
    for state in next_states:
      if any([s in get_board_vector_repr(state) for s in solns]):
        print('Found solution state!')
        states_sequence.append(state) ; nvisited += 1
        states_sequence = remove_cycles(states_sequence)
        return state, states_sequence, nvisited
    board = next_states[np.random.randint(0, len(next_states))]
    states_sequence.append(board) ; nvisited += 1
    if nvisited % 10000 == 0:
      print('States visited: {}'.format(nvisited))
      states_sequence = list(remove_cycles(states_sequence))
  return None


###############################################################################

def main():

  # Starting state
  board = np.array([
    [2, 4, 4, 2],
    [2, 4, 4, 2],
    [2, 3, 3, 2],
    [2, 1, 1, 2],
    [1, 0, 0, 1],
  ])

  print('Starting position:')
  print_board(board)

  # Example state, right before solving
  # board2 = np.array([
  #   [2, 2, 2, 2],
  #   [2, 2, 2, 2],
  #   [1, 1, 4, 4],
  #   [1, 1, 4, 4],
  #   [3, 3, 0, 0]
  # ])

  solns = [(4, 3, 1)]
  solved_board, states_sequence, nvisited = find_solution_path(board, solns)

  print('Solved board:')
  print_board(solved_board)

  print('Total number of states visited: {}'.format(nvisited))
  print('Final length of state sequence, after removing cycles: {}'.format(\
                                                        len(states_sequence)))
  np.save('klotski_path.npy', states_sequence)


if __name__ == '__main__':
  main()
