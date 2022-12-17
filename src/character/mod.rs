pub(crate) mod create;
pub(crate) mod destroy;
pub(crate) mod retrieve;
pub(crate) mod update;
pub use create::create_character;
pub use destroy::destroy_character;
pub use retrieve::retrieve_character;
pub use update::update_character;
pub use update::CharacterBaseDiff;
pub(crate) mod tables;
use eyre::{eyre, Result};
use std::ops::Deref;

use crate::abilities::Ability;
use crate::abilities::{Abilities, AbilityNameNoSubskill};
use crate::armor::{Armor, ArmorItem};
use crate::attributes::{AttributeName, Attributes};
use crate::campaign::Campaign;
use crate::charms::MartialArtsCharm;
use crate::exalt_type::ExaltType;
use crate::health::{Health, WoundPenalty};
use crate::id::Id;
use crate::intimacies::Intimacies;
use crate::intimacies::Intimacy;
use crate::martial_arts::MartialArtistTraits;
use crate::martial_arts::MartialArtsStyle;
use crate::merits::Merits;
use crate::merits::{Merit, MeritTemplate};
use crate::player::Player;
use crate::prerequisite::{ExaltTypePrerequisite, Prerequisite, PrerequisiteSet, PrerequisiteType};
use crate::sorcery::SorcererTraits;
use crate::weapons::{EquipHand, Weapon, Weapons};
use serde::{Deserialize, Serialize};

/// The basic Character object, representing a full player character.
/// This represents the state of a valid character at a given instant of a game.
/// It is also the serialization format to be moved back and forth between client and server.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Character {
    id: Id,
    player: Player,
    campaign: Option<Campaign>,
    pub name: String,
    pub concept: Option<String>,
    pub willpower: Willpower,
    pub experience: ExperiencePoints,
    pub attributes: Attributes,
    abilities: Abilities,
    pub intimacies: Intimacies,
    pub health: Health,
    pub weapons: Weapons,
    pub armor: Armor,
    pub merits: Merits,
    pub exalt_type: ExaltType,
    pub sorcery: Option<SorcererTraits>,
    martial_arts_styles: MartialArtistTraits,
}

impl Character {
    pub fn builder(placeholder_id: i32) -> CharacterBuilder {
        CharacterBuilder {
            id: Id::Placeholder(placeholder_id),
            ..Default::default()
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn campaign(&self) -> &Option<Campaign> {
        &self.campaign
    }

    pub fn get_ability(
        &self,
        ability_name: AbilityNameNoSubskill,
        subskill: Option<&str>,
    ) -> Option<Ability> {
        if ability_name == AbilityNameNoSubskill::MartialArts {
            self.martial_arts_styles.get_ability(subskill?)
        } else {
            self.abilities.get(ability_name, subskill)
        }
    }

    pub fn set_ability_dots(
        &mut self,
        ability_name: AbilityNameNoSubskill,
        subskill: Option<&str>,
        dots: u8,
    ) -> Result<()> {
        if ability_name == AbilityNameNoSubskill::MartialArts {
            return Err(eyre!("TODO: fix this"));
        } else {
            self.abilities.set_dots(ability_name, subskill, dots)
        }
    }

    pub fn add_specialty(
        &mut self,
        ability_name: AbilityNameNoSubskill,
        subskill: Option<&str>,
        specialty: String,
    ) -> Result<()> {
        if ability_name == AbilityNameNoSubskill::MartialArts {
            return Err(eyre!("TODO: fix this"));
        } else {
            self.abilities
                .add_specialty(ability_name, subskill, specialty)
        }
    }

    pub fn remove_specialty(
        &mut self,
        ability_name: AbilityNameNoSubskill,
        subskill: Option<&str>,
        specialty: &str,
    ) -> Result<()> {
        if ability_name == AbilityNameNoSubskill::MartialArts {
            return Err(eyre!("TODO: fix this"));
        } else {
            self.abilities
                .remove_specialty(ability_name, subskill, specialty)
        }
    }
}

#[derive(Debug, Default)]
pub struct CharacterBuilder {
    id: Id,
    player: Option<Player>,
    campaign: Option<Campaign>,
    name: Option<String>,
    concept: Option<String>,
    willpower: Willpower,
    experience: ExperiencePoints,
    attributes: Attributes,
    abilities: Abilities,
    intimacies: Vec<Intimacy>,
    health: Health,
    weapons: Weapons,
    armor: Armor,
    merits: Vec<Merit>,
    exalt_type: Option<ExaltType>,
    sorcery: Option<SorcererTraits>,
    martial_arts_styles: MartialArtistTraits,
}

impl CharacterBuilder {
    pub fn id(&self) -> Id {
        self.id
    }

    fn meets_prerequisite(&self, prerequisite: &Prerequisite) -> bool {
        match prerequisite.deref() {
            PrerequisiteType::Ability(ability_prerequisite) => {
                self.abilities.meets_prerequisite(ability_prerequisite)
            }
            PrerequisiteType::Attribute(attribute_prerequisite) => {
                self.attributes.meets_prerequisite(attribute_prerequisite)
            }
            PrerequisiteType::Essence(_) => false,
            PrerequisiteType::Charm(_) => false,
            PrerequisiteType::ExaltType(exalt_type) => match exalt_type {
                ExaltTypePrerequisite::Solar => false,
                ExaltTypePrerequisite::Lunar => false,
                ExaltTypePrerequisite::DragonBlooded => false,
                ExaltTypePrerequisite::Spirit => false,
                ExaltTypePrerequisite::SpiritOrEclipse => false,
            },
        }
    }

    fn meets_prerequisite_set(&self, prerequisite_set: &PrerequisiteSet) -> bool {
        prerequisite_set
            .iter()
            .all(|prerequisite| self.meets_prerequisite(prerequisite))
    }

    pub fn meets_any_prerequisite_set(&self, prerequisite_sets: &[PrerequisiteSet]) -> bool {
        prerequisite_sets.is_empty()
            || prerequisite_sets
                .iter()
                .any(|prerequisite_set| self.meets_prerequisite_set(prerequisite_set))
    }

    pub fn with_database_id(mut self, id: i32) -> Self {
        self.id = Id::Database(id);
        self
    }

    pub fn with_player(mut self, player: Player) -> Self {
        self.player = Some(player);
        self
    }

    pub fn with_campaign(mut self, campaign: Campaign) -> Self {
        self.campaign = Some(campaign);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_concept(mut self, concept: String) -> Self {
        self.concept = Some(concept);
        self
    }

    pub fn with_willpower(mut self, willpower: Willpower) -> Self {
        self.willpower = willpower;
        self
    }

    pub fn with_experience(mut self, experience: ExperiencePoints) -> Self {
        self.experience = experience;
        self
    }

    pub fn with_attribute(mut self, attribute_name: AttributeName, value: u8) -> Result<Self> {
        self.attributes.set(attribute_name, value)?;
        Ok(self)
    }

    pub fn with_ability(mut self, ability_name: AbilityNameNoSubskill, value: u8) -> Result<Self> {
        self.abilities.set_dots(ability_name, None, value)?;
        Ok(self)
    }

    pub fn with_craft(mut self, craft_focus: &str, value: u8) -> Self {
        self.abilities
            .set_dots(AbilityNameNoSubskill::Craft, Some(craft_focus), value)
            .unwrap();
        self
    }

    pub fn with_martial_arts_style(mut self, style: MartialArtsStyle, dots: u8) -> Result<Self> {
        self.martial_arts_styles.add_style(style, dots)?;
        Ok(self)
    }

    pub fn with_specialty(
        mut self,
        ability_name: AbilityNameNoSubskill,
        specialty: String,
    ) -> Result<Self> {
        self.abilities
            .add_specialty(ability_name, None, specialty)?;
        Ok(self)
    }

    pub fn with_craft_specialty(mut self, craft_focus: &str, specialty: String) -> Result<Self> {
        self.abilities
            .add_specialty(AbilityNameNoSubskill::Craft, Some(craft_focus), specialty)?;
        Ok(self)
    }

    pub fn with_martial_arts_specialty(mut self, style_id: Id, specialty: String) -> Result<Self> {
        self.martial_arts_styles
            .add_specialty(style_id, specialty)?;
        Ok(self)
    }

    pub fn with_intimacy(mut self, intimacy: Intimacy) -> Self {
        self.intimacies.push(intimacy);
        self
    }

    pub fn with_wound_penalties(mut self, wound_penalties: Vec<WoundPenalty>) -> Self {
        let (bashing, lethal, aggravated) = self.health.damage();
        self.health = Health::empty();
        for wound_penalty in wound_penalties.into_iter() {
            self.health.add_health_box(wound_penalty);
        }
        self.health.set_damage(bashing, lethal, aggravated);

        self
    }

    pub fn with_damage(mut self, bashing: u8, lethal: u8, aggravated: u8) -> Self {
        self.health.set_damage(bashing, lethal, aggravated);
        self
    }

    pub fn with_weapon(mut self, weapon: Weapon, equipped: Option<EquipHand>) -> Result<Self> {
        let key = self.weapons.add_weapon(weapon);

        if let Some(hand) = equipped {
            self.weapons.equip(key, hand)?;
        }

        Ok(self)
    }

    pub fn with_armor(mut self, armor_item: ArmorItem, worn: bool) -> Self {
        let index = self.armor.add_armor_item(armor_item);
        if worn {
            self.armor.equip_armor_item(index).unwrap();
        }
        self
    }

    pub(crate) fn with_merit_ignore_prerequisites(
        mut self,
        template: MeritTemplate,
        dots: u8,
        detail: Option<String>,
        id: Id,
    ) -> Result<Self> {
        let merit = Merit::from_template(template, dots, detail, id)?;
        self.merits.push(merit);
        Ok(self)
    }

    pub fn with_merit(
        self,
        template: MeritTemplate,
        dots: u8,
        detail: Option<String>,
        id: Id,
    ) -> Result<Self> {
        if self.meets_any_prerequisite_set(template.prerequisites()) {
            self.with_merit_ignore_prerequisites(template, dots, detail, id)
        } else {
            Err(eyre!("prerequisites not met for merit {}", template.name()))
        }
    }

    pub fn with_martial_arts_charm(
        mut self,
        style_id: Id,
        charm: MartialArtsCharm,
    ) -> Result<Self> {
        self.martial_arts_styles.add_charm(style_id, charm)?;
        Ok(self)
    }

    pub fn build(self) -> Result<Character> {
        if self.player.is_none() {
            return Err(eyre!("player must be specified"));
        }

        if self.name.is_none() {
            return Err(eyre!("name must be specified"));
        }

        let exalt_type = if let Some(exalt) = self.exalt_type {
            exalt
        } else {
            ExaltType::Mortal
        };

        Ok(Character {
            id: self.id,
            player: self.player.unwrap(),
            campaign: self.campaign,
            name: self.name.unwrap(),
            concept: self.concept,
            willpower: self.willpower,
            experience: self.experience,
            attributes: self.attributes,
            abilities: self.abilities,
            intimacies: Intimacies::new(self.intimacies),
            health: self.health,
            weapons: self.weapons,
            armor: self.armor,
            merits: Merits::new(self.merits),
            exalt_type,
            sorcery: self.sorcery,
            martial_arts_styles: self.martial_arts_styles,
        })
    }
}

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
