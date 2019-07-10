use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};

use crate::game_move::Move;
use crate::game::Game;
use super::MoveType;

const ROWS: usize = 6;
const COLUMNS: usize = 7;

pub type Grid = [[u8; ROWS]; COLUMNS];

/**
 *
 * As a game author you get to decide what the State object of your game looks like.
 * Most of the time you want it to include all of the previous moves as well.
 * 
 * To customize the game state implement your own GameState struct. This must have a function called `initial()`
 * which returns the initial state.
 *
 */

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct PlayerState {
    pub resigned: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct GameState {
    pub grid: Grid, 
    pub moves_history: Vec<Move>,
    pub player_1: PlayerState,
    pub player_2: PlayerState,
    pub in_progress: bool,
}

impl PlayerState {
    pub fn initial() -> Self {
        Self {
            resigned: false,
        }
    }
}

impl GameState {
    pub fn initial() -> Self {
        Self {
            grid: [[0; ROWS]; COLUMNS], // create 2D array of zeroes
            moves_history: Vec::new(), // flexible size vector
            player_1: PlayerState::initial(),
            player_2: PlayerState::initial(),
            in_progress: true,
        }
    }

    pub fn render(&self) -> String {
        // <<DEVCAMP-TODO>> return a pretty formatting string representation
        "".to_string()
    }

    pub fn evolve(&self, game: Game, next_move: &Move) -> GameState {
        // given a current state, a game and a move, compute the next state
        // You can assume all moves are valid

        // Prepare variables for evolution
        let mut grid = self.grid.clone();
        let mut moves_history = self.moves_history.clone();
        let mut player_1 = self.player_1.clone();
        let mut player_2 = self.player_2.clone();
        let in_progress: bool;

        // Record move
        moves_history.push(next_move.clone());

        // Mutate state
        match next_move.move_type {
            MoveType::DropPiece{column} => {
                if game.player_1 == next_move.author {
                    grid = self.drop_piece(grid, 1, column as usize);
                    // Check for player win and force opponent to resign if so
                    player_2.resigned = self.check_win(grid, 1);
                } else {
                    grid = self.drop_piece(grid, 2, column as usize);
                    // Check for player win and force opponent to resign if so
                    player_1.resigned = self.check_win(grid, 2);
                }

                // Check for finish
                if player_1.resigned || player_2.resigned {
                    in_progress = false;
                } else {
                    // If draw, must stop progression
                    in_progress = !self.check_draw(grid);
                }
            }
        }

        // Return mutated state
        GameState {
            grid,
            moves_history,
            player_1,
            player_2,
            in_progress,
        }
    }

    fn drop_piece(&self, mut grid: Grid, player: u8, column: usize) -> Grid {
        // Get column from grid reference
        let col_array = &grid[column];

        // Make gravity happen
        for el in 0..(col_array.len()-1) {  // begin at bottom of column and work upwards
            if col_array[el] == 0 {         // until unoccupied spot
                grid[column][el as usize] = player;
                break;
            }
        }

        grid
    }

    fn check_draw(&self, grid: Grid) -> bool {
        // If all columns full
        // This is called if neither player has resigned after a drop
        for col in 0..(COLUMNS-1) {
            // check if vacant spot in top row
            if grid[col][ROWS-1] == 0 {
                return true;
            }
        }

        return false;
    }

    fn check_win(&self, grid: Grid, player: u8) -> bool {    
        self.check_column_win(grid, player) || 
        self.check_row_win(grid, player) || 
        self.check_diagonal_win(grid, player)
    }

    fn check_column_win(&self, grid: Grid, player: u8) -> bool {
        for col in 0..(grid.len()-1) {
            let mut count = 0;

            for row in 0..(grid[col].len()-1) {
                // check if piece is the player's
                if grid[col][row] == player {
                    // then increment counter
                    count = count + 1;

                    // connect 4!
                    if count == 4 {
                        return true;
                    }

                } else { 
                    // reset counter
                    count = 0;
                }
            }
        }

        return false;
    }

    fn check_row_win(&self, grid: Grid, player: u8) -> bool {
        for row in 0..(grid[0].len()-1) {
            let mut count = 0;

            for col in 0..(grid.len()-1) {
                // check if piece is the player's
                if grid[col][row] == player {
                    // then increment counter
                    count = count + 1;

                    // connect 4!
                    if count == 4 {
                        return true;
                    }

                } else { // reset counter
                    count = 0;
                }
            }
        }

        return false;
    }

    fn check_diagonal_win(&self, grid: Grid, player: u8) -> bool {
        for col in 0..(grid.len()-4) {
            // Up and to the right diagonal
            for row in 0..(grid[0].len()-4) {
                if 
                    grid[col][row]      == player &&
                    grid[col+1][row+1]  == player &&
                    grid[col+2][row+2]  == player &&
                    grid[col+3][row+3]  == player
                    {
                        return true;
                    }
            }

            // Down and to the right diagonal
            for row in (grid[0].len()-1)..3 {
                if 
                    grid[col][row]      == player &&
                    grid[col+1][row-1]  == player &&
                    grid[col+2][row-2]  == player &&
                    grid[col+3][row-3]  == player
                    {
                        return true;
                    }
            }
        }

        return false;
    }

    // Validation fragment
    pub fn is_column_in_bounds(&self, column: u32) -> Result<(), String> {
        if column < (COLUMNS - 1) as u32 {
            Ok(())
        } else {
            Err("Column out of bounds".into())
        }
    }

    // Validation fragment. Borrow GameState so that other fragments can use it after.
    pub fn is_column_not_full(&self, game_state: &GameState, column: u32) -> Result<(), String> {
        // If the top row of the column is 0, then it can receive a piece
        if game_state.grid[column as usize][ROWS-1] == 0 {
            Ok(())
        } else {
            Err("Column is full".into())
        }
    }
}
