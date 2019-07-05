use std::convert::TryFrom;
use hdk::{
    utils,
    entry_definition::ValidatingEntryType,
    error::{ZomeApiResult, ZomeApiError},
    holochain_persistence_api::{
        cas::content::{AddressableContent, Address},
    },
    holochain_json_api::{
        error::JsonError, json::JsonString,
    },
    holochain_core_types::{
        dna::entry_types::Sharing,
        validation::EntryValidationData,
        entry::Entry,
        link::LinkMatch,
    }
};

use crate::game_move::Move;
use crate::GameState;

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Game {
    pub player_1: Address,
    pub player_2: Address,
    pub created_at: u32,
}

/*=====================================
=            DHT Functions            =
=====================================*/

/// Traverse the linked list rooted at a game to find all the moves
pub fn get_moves(game_address: &Address) -> ZomeApiResult<Vec<Move>> {
    match hdk::get_links(game_address, LinkMatch::Any, LinkMatch::Any)?.addresses().into_iter().next() {
        Some(first_move) => {
            let mut move_addresses = vec![first_move];
            let mut more = true;
            while more {
                more = match hdk::get_links(move_addresses.last().unwrap(), LinkMatch::Any, LinkMatch::Any)?.addresses().into_iter().next() {
                    Some(addr) => {
                        move_addresses.push(addr.clone());
                        true
                    },
                    None => {
                        false
                    },
                }
            }
            let moves: Vec<Move> = move_addresses.iter().map(|addr| {
                let move_entry = hdk::get_entry(addr).unwrap().unwrap();
                if let Entry::App(_, move_struct) = move_entry {
                    Move::try_from(move_struct).expect("Entry at address is type other than Move")
                } else {
                    panic!("Not an app entry!")
                }
            }).collect();
            Ok(moves)
        },
        None => {
            Ok(Vec::new())
        }
    }
}

pub fn get_state(game_address: &Address) -> ZomeApiResult<GameState> {
    let moves = get_moves(game_address)?;
    let game = get_game(game_address)?;
    let new_state = moves.iter().fold(GameState::initial(), |state, new_move| state.evolve(game.clone(), new_move));
    Ok(new_state)
}

pub fn get_game(game_address: &Address) -> ZomeApiResult<Game> {
    utils::get_as_type(game_address.to_owned())
}

/*=====  End of DHT Functions  ======*/



/*=============================================
=            Local chain functions            =
=============================================*/

pub fn get_game_local_chain(local_chain: Vec<Entry>, game_address: &Address) -> ZomeApiResult<Game> {
    local_chain
        .iter()
        .filter(|entry| {
            entry.address() == game_address.to_owned()
        })
        .filter_map(|entry| {
            if let Entry::App(_, entry_data) = entry {
                Some(Game::try_from(entry_data.clone()).unwrap())
            } else {
                None
            }
        })
        .next()
        .ok_or(ZomeApiError::HashNotFound)
}

pub fn get_moves_local_chain(local_chain: Vec<Entry>, game_address: &Address) -> ZomeApiResult<Vec<Move>> {
    Ok(local_chain
        .iter()
        .filter_map(|entry| {
            if let Entry::App(entry_type, entry_data) = entry {
                if entry_type.to_string() == "move" {
                    Some(Move::try_from(entry_data.clone()).unwrap())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .filter(|game_move| {
            game_move.game == game_address.to_owned()
        })
        .rev()
        .collect())
}

pub fn get_state_local_chain(local_chain: Vec<Entry>, game_address: &Address) -> ZomeApiResult<GameState> {
    let moves = get_moves_local_chain(local_chain.clone(), game_address)?;
    let game = get_game_local_chain(local_chain, game_address)?;
    let new_state = moves.iter().fold(GameState::initial(), move |state, new_move| state.evolve(game.clone(), new_move));
    Ok(new_state)
}


/*=====  End of Local chain functions  ======*/




pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "game",
        description: "Represents an occurence of a game between several agents",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | validation_data: hdk::EntryValidationData<Game>| {
            match validation_data {
                EntryValidationData::Create{entry, validation_data: _} => {
                    let game = entry as Game;
                    if game.player_1 == game.player_2 {
                        return Err("Player 1 and Player 2 must be different agents.".into())
                    }
                    Ok(())
                },
                _ => {
                    Err("Cannot modify or delete a game".into())
                }
            }
        }
    )
}
