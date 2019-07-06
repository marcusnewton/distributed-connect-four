use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};

/**
 *
 * The MoveType enum defines all the types of moves that are valid in your game and the 
 * data they carry. In Checkers you can move a piece (MovePiece) from a location to another location.
 *
 */

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub enum MoveType {
    DropPiece {
			column: u32,
		},
}

impl MoveType {
	pub fn describe() -> Vec<MoveType> {
		vec![MoveType::DropPiece{column: 0}]
	}
}
