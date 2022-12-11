mod abilities;
mod armor;
mod attributes;
mod character;
mod health;
mod intimacies;
pub mod serde;
mod weapons;
use exalted_3e_gui::{ player::Player,Character,};

pub use self::serde::validate_player_serde;

use crate::fixtures::{
    character::validate_initial_base_character,
    weapons::validate_initial_weapons,
};

use self::{
    abilities::{create_intitial_abilities, validate_initial_abilities},
    attributes::{create_initial_attributes, validate_initial_attributes},
    health::{validate_initial_health, create_initial_health},
    intimacies::{create_initial_intimacies, validate_initial_intimacies},
    weapons::create_initial_weapons, character::create_initial_base_character, armor::{create_initial_armor, validate_initial_armor_items},
};

pub fn create_initial_character(player: &Player) -> Character {
    let mut builder = create_initial_base_character(player);
    builder = create_initial_attributes(builder);
    builder = create_intitial_abilities(builder);
    builder = create_initial_intimacies(builder);
    builder = create_initial_health(builder);
    builder = create_initial_armor(builder);
    builder = create_initial_weapons(builder);

    builder.build().unwrap()
}

pub fn validate_initial_character(
    player: &Player,
    initial_character: &Character,
    should_have_id: bool,
) {
    validate_initial_base_character(player, initial_character, should_have_id);
    validate_initial_attributes(&initial_character.attributes);
    validate_initial_abilities(&initial_character.abilities);
    validate_initial_intimacies(&initial_character.intimacies, should_have_id);
    validate_initial_health(&initial_character.health);
    validate_initial_armor_items(&initial_character.armor, should_have_id);
    validate_initial_weapons(&initial_character.weapons, should_have_id);
}