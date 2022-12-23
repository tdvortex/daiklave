mod builder;
mod command;
mod diff;
// mod guided; // Not sure about this; maybe Yew-only?
pub use builder::CharacterBuilder;
pub use diff::{CharacterBaseDiff, CharacterDiff};
use eyre::{eyre, Result};

use crate::abilities::Abilities;
use crate::abilities::Ability;
use crate::abilities::AbilityName;
use crate::abilities::AbilityNameVanilla;
use crate::armor::Armor;
use crate::attributes::Attributes;
use crate::campaign::Campaign;
use crate::charms::MartialArtsCharm;
use crate::craft::CraftAbilities;
use crate::exalt_type::ExaltType;
use crate::health::Health;
use crate::id::CharacterId;
use crate::id::MartialArtsStyleId;
use crate::initiative::Initiative;
use crate::intimacies::Intimacies;
use crate::martial_arts::MartialArtistTraits;
use crate::martial_arts::MartialArtsStyle;
use crate::merits::Merits;
use crate::player::Player;
use crate::sorcery::Sorcerer;
use crate::weapons::Weapons;
use serde::{Deserialize, Serialize};

/// The basic Character object, representing a full player character.
/// This represents the state of a valid character at a given instant of a game.
/// It is also the serialization format to be moved back and forth between client and server.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Character {
    id: CharacterId,
    player: Player,
    campaign: Option<Campaign>,
    pub name: String,
    pub concept: Option<String>,
    pub willpower: Willpower,
    pub experience: ExperiencePoints,
    pub initiative: Initiative,
    pub attributes: Attributes,
    abilities: Abilities,
    pub intimacies: Intimacies,
    pub health: Health,
    pub weapons: Weapons,
    pub armor: Armor,
    pub merits: Merits,
    exalt_type: ExaltType,
    craft_abilities: CraftAbilities,
    martial_arts_styles: MartialArtistTraits,
}

impl Character {
    pub fn blank(placeholder_id: i32, player: Player) -> Character {
        Character::builder(placeholder_id, player)
            .build()
            .expect("Default CharacterBuilder should not error")
    }

    pub fn builder(placeholder_id: i32, player: Player) -> CharacterBuilder {
        CharacterBuilder::default()
            .with_placeholder_id(placeholder_id)
            .with_player(player)
            .with_name("New Character".to_owned())
    }

    pub fn id(&self) -> CharacterId {
        self.id
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn campaign(&self) -> &Option<Campaign> {
        &self.campaign
    }

    pub fn get_ability(&self, ability_name_vanilla: AbilityNameVanilla) -> Ability {
        self.abilities.get(ability_name_vanilla)
    }

    pub fn get_craft_ability(&self, focus: &str) -> Option<Ability> {
        self.craft_abilities
            .iter()
            .find(|a| *a.name() == AbilityName::Craft(focus))
    }

    pub fn get_martial_arts_ability(&self, style_id: MartialArtsStyleId) -> Option<Ability> {
        self.martial_arts_styles.get_ability(style_id)
    }

    pub fn martial_arts_iter(
        &self,
    ) -> impl Iterator<Item = (&MartialArtsStyle, Ability, &Vec<MartialArtsCharm>)> + '_ {
        self.martial_arts_styles.iter()
    }

    pub fn set_ability_dots(&mut self, ability_name: AbilityNameVanilla, dots: u8) {
        self.abilities.set_dots(ability_name, dots);
    }

    pub fn set_craft_ability_dots(&mut self, focus: &str, dots: u8) {
        self.craft_abilities.set_dots(focus, dots);
    }

    pub fn set_martial_arts_ability_dots(
        &mut self,
        style_id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<()> {
        self.martial_arts_styles.set_dots(style_id, dots)
    }

    pub fn add_specialty(
        &mut self,
        ability_name: AbilityNameVanilla,
        specialty: String,
    ) -> Result<()> {
        self.abilities.add_specialty(ability_name, specialty)
    }

    pub fn add_craft_specialty(&mut self, focus: &str, specialty: String) -> Result<()> {
        self.craft_abilities.add_specialty(focus, specialty)
    }

    pub fn add_martial_arts_specialty(
        &mut self,
        style_id: MartialArtsStyleId,
        specialty: String,
    ) -> Result<()> {
        self.martial_arts_styles.add_specialty(style_id, specialty)
    }

    pub fn remove_specialty(
        &mut self,
        ability_name: AbilityNameVanilla,
        specialty: &str,
    ) -> Result<()> {
        self.abilities.remove_specialty(ability_name, specialty)
    }

    pub fn remove_craft_specialty(&mut self, focus: &str, specialty: &str) -> Result<()> {
        self.craft_abilities.remove_specialty(focus, specialty)
    }

    pub fn remove_martial_arts_specialty(
        &mut self,
        style_id: MartialArtsStyleId,
        specialty: &str,
    ) -> Result<()> {
        self.martial_arts_styles
            .remove_specialty(style_id, specialty)
    }
}

impl Sorcerer for Character {
    fn shaping_rituals(&self) -> Option<Vec<&crate::sorcery::ShapingRitual>> {
        self.exalt_type.shaping_rituals()
    }

    fn spells(&self) -> Option<Vec<(&crate::charms::Spell, bool)>> {
        self.exalt_type.spells()
    }
}

// TODO: refactor to current + spent with total() method
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExperiencePoints {
    pub current: u16,
    pub total: u16,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CraftingExperience {
    pub _silver: u16,
    pub _gold: u16,
    pub _white: u16,
    pub _major_slots: u16,
}

pub(crate) enum _CraftingExperienceType {
    Silver,
    Gold,
    White,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Willpower {
    pub current: u8,
    pub maximum: u8,
}

impl Default for Willpower {
    fn default() -> Self {
        Self {
            current: 5,
            maximum: 5,
        }
    }
}

impl Willpower {
    pub fn recover_all(&mut self) {
        self.current = self.current.max(self.maximum);
    }

    pub fn recover_one(&mut self) {
        self.current = self.maximum.min(self.current + 1);
    }

    pub fn gain_one(&mut self) {
        self.current += 1;
    }

    pub fn spend_one(&mut self) -> Result<()> {
        if self.current == 0 {
            Err(eyre!("Cannot spend willpower while at zero"))
        } else {
            self.current -= 1;
            Ok(())
        }
    }
}
