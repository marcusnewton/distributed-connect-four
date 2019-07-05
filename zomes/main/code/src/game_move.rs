use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::{Address},
    },
    holochain_json_api::{
        error::JsonError, json::JsonString,
    },
    holochain_core_types::{
        dna::entry_types::Sharing,
        validation::EntryValidationData,
        entry::Entry,
    }
};

use crate::MoveType;
use crate::game::{get_game_local_chain, get_state_local_chain};


#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct MoveInput {
	pub game: Address,
	pub move_type: MoveType,
	pub timestamp: u32,
}


#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub struct Move {
	pub game: Address,
	pub author: Address,
	pub move_type: MoveType,
	pub previous_move: Address,
	pub timestamp: u32,
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "move",
        description: "A move by an agent in an game",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },

        validation: | validation_data: hdk::EntryValidationData<Move>| {
            match validation_data {
                EntryValidationData::Create{entry, validation_data} => {
                	let mut local_chain = validation_data.package.source_chain_entries
                		.ok_or("Could not retrieve source chain")?;
                	hdk::debug(format!("{:?}", local_chain))?;

                	// load the game and game state
                	let _new_move = Move::from(entry);

                    // Sometimes the validating entry is already in the chain when validation runs,
                    // To make our state reduction work correctly this must be removed
                    local_chain.remove_item(&Entry::App("move".into() , _new_move.clone().into()));

                	let state = get_state_local_chain(local_chain.clone(), &_new_move.game)
                		.map_err(|_| "Could not load state during validation")?;
                	let game = get_game_local_chain(local_chain, &_new_move.game)
                	    .map_err(|_| "Could not load game during validation")?;
                    
                    _new_move.is_valid(game, state)
                },
                _ => {
                    Err("Cannot modify or delete a move".into())
                }
            }
        },

        links: [
        	from!(
                "game",
                link_type: "",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
        	from!(
                "move",
                link_type: "",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}
