mod abilities;
mod armor;
mod attributes;
mod character;
mod health;
mod intimacies;
mod merits;
pub mod serde;
mod weapons;
use exalted_3e_gui::{player::Player, Character};

pub use self::serde::validate_player_serde;

use crate::fixtures::{
    character::validate_initial_base_character, weapons::validate_initial_weapons,
};

use self::{
    abilities::{
        create_intitial_abilities, modify_abilities, validate_initial_abilities,
        validate_modified_abilities,
    },
    armor::{create_initial_armor, validate_initial_armor_items, modify_armor, validate_modified_armor_items},
    attributes::{
        create_initial_attributes, modify_attributes, validate_initial_attributes,
        validate_modified_attributes,
    },
    character::{
        create_initial_base_character, modify_base_character, validate_modified_base_character,
    },
    health::{
        create_initial_health, modify_health, validate_initial_health, validate_modified_health,
    },
    intimacies::{create_initial_intimacies, validate_initial_intimacies, validate_modified_intimacies, modify_intimacies},
    merits::{create_initial_merits, validate_initial_merits},
    weapons::{create_initial_weapons, modify_weapons, validate_modified_weapons},
};

pub fn create_initial_character(player: &Player) -> Character {
    let mut builder = create_initial_base_character(player);
    builder = create_initial_attributes(builder);
    builder = create_intitial_abilities(builder);
    builder = create_initial_intimacies(builder);
    builder = create_initial_health(builder);
    builder = create_initial_armor(builder);
    builder = create_initial_weapons(builder);
    builder = create_initial_merits(builder);

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
    validate_initial_merits(&initial_character.merits, should_have_id);
}

pub fn modify_character(character: &mut Character) {
    modify_base_character(character);
    modify_abilities(&mut character.abilities);
    modify_attributes(&mut character.attributes);
    modify_health(&mut character.health);
    modify_intimacies(&mut character.intimacies);
    modify_armor(&mut character.armor);
    modify_weapons(&mut character.weapons);
}

pub fn validate_modified_character(player: &Player, modified_character: &Character) {
    validate_modified_base_character(&player, &modified_character);
    validate_modified_abilities(&modified_character.abilities);
    validate_modified_attributes(&modified_character.attributes);
    validate_modified_health(&modified_character.health);
    validate_modified_intimacies(&modified_character.intimacies);
    validate_modified_armor_items(&modified_character.armor);
    validate_modified_weapons(&modified_character.weapons);
}
