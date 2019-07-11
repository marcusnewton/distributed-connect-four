use crate::game::Game;
use crate::game_move::Move;
use super::{
    GameState,
};

use hdk::holochain_persistence_api::cas::content::Address;
use super::moves::MoveType;

/**
 *
 * To implement your own custom rule validation all you need to do is re-implement the function `is_valid` on `Move`
 * 
 * This function  takes the current game and the game state (which includes all the existing moves) 
 * and determines if a new candidate move is valid. Typically this will involve first matching on the move type
 * and then determining if the move is valid.
 * 
 * It function must return Ok(()) if a move is valid and Err("Some error string".into()) for an invalid move.
 * It is useful to provide descriptive error strings as these can be visible to the end user.
 *
 */

impl Move {
    pub fn is_valid(&self, game: Game, game_state: GameState) -> Result<(), String> {
        // Check if a move is valid given the current game and its state
        is_game_in_progress(&game_state)?;
        is_it_players_turn(self.author.clone(), &game, &game_state)?;

        match self.move_type {
            MoveType::DropPiece{column} => {
                game_state.is_column_in_bounds(column)?;
                game_state.is_column_not_full(&game_state, column)?;
            }
        }

        Ok(())
    }
}

fn is_game_in_progress(game_state: &GameState) -> Result<(), String> {
    if game_state.in_progress == false {
        Err("Game has ended".into())
    } else {
        Ok(())
    }
}

fn is_it_players_turn(player: Address, game: &Game, game_state: &GameState) -> Result<(), String> {
    let moves_history = &game_state.moves_history;

    match moves_history.last() {
        Some(last_move) => {
            if last_move.author == player {
                Err("Not your turn, must wait for other player to make a move".into())
            } else {
                Ok(())
            }
        },
        None => { 
            // if no moves in history, determine who goes first
            if game.player_2 == player {
                Ok(())
            } else {
                Err("Player 2 must make the first move".into())
            }
        }
    }
}
