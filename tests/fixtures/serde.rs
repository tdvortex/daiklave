use exalted_3e_gui::{player::Player, Character};
use postcard::from_bytes;

use super::validate_initial_character;

pub fn validate_initial_character_serde(
    player: &Player,
    character: &Character,
    should_have_id: bool,
) -> Character {
    let send_bytes = postcard::to_allocvec(character).unwrap();
    let receive_character = from_bytes(&send_bytes).unwrap();
    validate_initial_character(player, &receive_character, should_have_id);
    receive_character
}

pub fn validate_player_serde(player: &Player) -> Player {
    let send_bytes = postcard::to_allocvec(player).unwrap();
    let receive_player: Player = from_bytes(&send_bytes).unwrap();
    assert_eq!(player.id(), receive_player.id());
    assert_eq!(player.name(), receive_player.name());

    receive_player
}
