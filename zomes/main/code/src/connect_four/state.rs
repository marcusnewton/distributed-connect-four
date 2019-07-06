use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};

use crate::game_move::Move;
use crate::game::Game;
use super::MoveType;

const ROWS: usize = 6;
const COLUMNS: usize = 7;

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
pub struct GameState {
    pub in_progress: bool,
    pub grid: [[u32; ROWS]; COLUMNS], 
    pub moves_history: Vec<Move>,
    pub player_1_resigned: bool,
    pub player_2_resigned: bool,
}

impl GameState {
    pub fn initial() -> Self {
        Self {
            in_progress: true,
            grid: [[0; ROWS]; COLUMNS], // create 2D array of zeroes 
            moves_history: Vec::new(), // flexible size vector
            player_1_resigned: false,
            player_2_resigned: false,
        }
    }

    pub fn render(&self) -> String {
        // <<DEVCAMP-TODO>> return a pretty formatting string representation
        "".to_string()
    }

    pub fn evolve(&self, game: Game, next_move: &Move) -> GameState {
        // <<DEVCAMP-TODO>>
        // given a current state, a game and a move, compute the next state
        // You can assume all moves are valid

        let mut moves_history = self.moves_history.clone();
        let mut grid = self.grid.clone();

        moves_history.push(next_move.clone()); // just records Move entries, doesn't reduce state of grid

        match next_move.move_type {
            MoveType::DropPiece{column} => {
                if game.player_1 == next_move.author {
                    // TODO: perform piece drop for player 1, returning new grid
                } else {
                    // TODO: perform piece drop for player 2, returning new grid
                }
            }
        }

        self.clone()
    }
}
