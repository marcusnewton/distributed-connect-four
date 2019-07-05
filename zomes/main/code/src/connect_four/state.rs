use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};

use crate::game_move::Move;
use crate::game::Game;


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
    // <<DEVCAMP-TODO>>
    // pub moves: Vec<Move>, // each state entry contains history of moves
    // Implement your own game state
    // May be helpful to split this into state for each player
}


impl GameState {
    pub fn initial() -> Self {
        // <<DEVCAMP-TODO>> return an initial state of a game
        Self{}
    }

    pub fn render(&self) -> String {
        // <<DEVCAMP-TODO>> return a pretty formatting string representation
        "".to_string()
    }

    pub fn evolve(&self, _game: Game, _next_move: &Move) -> GameState {
        // <<DEVCAMP-TODO>>
        // given a current state, a game and a move, compute the next state
        // You can assume all moves are valid
        self.clone()
    }

}
