use std::ops::Deref;
use eyre::{eyre, Result};

/// Contains all of the individual traits that describe a character.
pub mod traits;

use traits::campaign::Campaign;
use traits::experience::ExperiencePoints;
use traits::player::Player;
use traits::willpower::Willpower;

use self::traits::abilities::{Abilities, AbilityNameNoSubskill, AbilityName};
use self::traits::armor::{Armor, ArmorItem};
use self::traits::attributes::{Attributes, AttributeName};
use self::traits::health::{Health, WoundPenalty};
use self::traits::intimacies::{Intimacies, Intimacy};
use self::traits::merits::{Merits, MeritTemplate, Merit};
use self::traits::prerequisite::{Prerequisite, PrerequisiteType, ExaltTypePrerequisite, PrerequisiteSet};
use self::traits::weapons::{Weapons, Weapon, EquipHand};

/// The basic Character object, representing a full player character.
/// This represents the state of a valid character at a given instant of a game.
/// It is also the serialization format to be moved back and forth between client and server.
#[derive(Debug)]
pub struct Character {
    pub id: Option<i32>,
    pub player: Player,
    pub campaign: Option<Campaign>,
    pub name: String,
    pub concept: Option<String>,
    pub willpower: Willpower,
    pub experience: ExperiencePoints,
    pub attributes: Attributes,
    pub abilities: Abilities,
    pub intimacies: Intimacies,
    pub health: Health,
    pub weapons: Weapons,
    pub armor: Armor,
    pub merits: Merits,
}

#[derive(Debug, Default)]
pub struct CharacterBuilder {
    id: Option<i32>,
    player: Option<Player>,
    campaign: Option<Campaign>,
    name: Option<String>,
    concept: Option<String>,
    willpower: Willpower,
    experience: ExperiencePoints,
    attributes: Attributes,
    abilities: Abilities,
    intimacies: Intimacies,
    health: Health,
    weapons: Weapons,
    armor: Armor,
    merits: Merits,
}

impl CharacterBuilder {
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
        prerequisite_sets
            .iter()
            .any(|prerequisite_set| self.meets_prerequisite_set(prerequisite_set))
    }

    pub fn with_id(&mut self, id: i32) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn with_player(&mut self, player: Player) -> &mut Self {
        self.player = Some(player);
        self
    }

    pub fn with_campaign(&mut self, campaign: Campaign) -> &mut Self {
        self.campaign = Some(campaign);
        self
    }

    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn with_concept(&mut self, concept: String) -> &mut Self {
        self.concept = Some(concept);
        self
    }

    pub fn with_willpower(&mut self, willpower: Willpower) -> &mut Self {
        self.willpower = willpower;
        self
    }

    pub fn with_experience(&mut self, experience: ExperiencePoints) -> &mut Self {
        self.experience = experience;
        self
    }

    pub fn with_attribute(
        &mut self,
        attribute_name: AttributeName,
        value: u8,
    ) -> Result<&mut Self> {
        self.attributes.set(attribute_name, value)?;
        Ok(self)
    }

    pub fn with_ability(
        &mut self,
        ability_name: AbilityNameNoSubskill,
        value: u8,
    ) -> Result<&mut Self> {
        self.abilities
            .get_mut(&ability_name.try_into()?)
            .unwrap()
            .set_dots(value);
        Ok(self)
    }

    pub fn with_craft(&mut self, craft_focus: &str, value: u8) -> &mut Self {
        self.abilities
            .add_craft(craft_focus.to_owned())
            .set_dots(value);
        self
    }

    pub fn with_martial_arts(&mut self, martial_arts_style: &str, value: u8) -> &mut Self {
        self.abilities
            .add_martial_arts(martial_arts_style.to_owned())
            .set_dots(value);
        self
    }

    pub fn with_specialty(
        &mut self,
        ability_name: AbilityNameNoSubskill,
        specialty: String,
    ) -> Result<&mut Self> {
        self.abilities
            .get_mut(&ability_name.try_into()?)
            .unwrap()
            .add_specialty(specialty)?;
        Ok(self)
    }

    pub fn with_craft_specialty(
        &mut self,
        craft_focus: &str,
        specialty: String,
    ) -> Result<&mut Self> {
        self.abilities
            .get_mut(&AbilityName::Craft(craft_focus.to_owned()))
            .ok_or_else(|| eyre!("craft focus {} not found", craft_focus))
            .and_then(|mut craft| craft.add_specialty(specialty))?;
        Ok(self)
    }

    pub fn with_martial_arts_specialty(
        &mut self,
        martial_arts_style: &str,
        specialty: String,
    ) -> Result<&mut Self> {
        self.abilities
            .get_mut(&AbilityName::MartialArts(martial_arts_style.to_owned()))
            .ok_or_else(|| eyre!("martial arts style {} not found", martial_arts_style))
            .and_then(|mut ma| ma.add_specialty(specialty))?;
        Ok(self)
    }

    pub fn with_intimacy(&mut self, intimacy: Intimacy) -> &mut Self {
        self.intimacies.push(intimacy);
        self
    }

    pub fn with_wound_penalties(&mut self, wound_penalties: Vec<WoundPenalty>) -> &mut Self {
        let (bashing, lethal, aggravated) = self.health.damage();
        self.health = Health::empty();
        for wound_penalty in wound_penalties.into_iter() {
            self.health.add_health_box(wound_penalty);
        }
        self.health.set_damage(bashing, lethal, aggravated);

        self
    }

    pub fn with_damage(&mut self, bashing: u8, lethal: u8, aggravated: u8) -> &mut Self {
        self.health.set_damage(bashing, lethal, aggravated);
        self
    }

    pub fn with_weapon(
        &mut self,
        weapon: Weapon,
        equipped: Option<EquipHand>,
    ) -> Result<&mut Self> {
        let key = self.weapons.add_weapon(weapon);

        if let Some(hand) = equipped {
            self.weapons.equip(key, hand)?;
        }

        Ok(self)
    }

    pub fn with_armor(&mut self, armor_item: ArmorItem, worn: bool) -> Result<&mut Self> {
        let key = self.armor.add_armor_item(armor_item);

        if worn {
            self.armor.equip_armor_item(key)?;
        }

        Ok(self)
    }

    fn with_merit_ignore_prerequisites(
        &mut self,
        template: MeritTemplate,
        detail: Option<String>,
        id: Option<i32>,
    ) -> Result<&mut Self> {
        let merit = Merit::from_template(template, detail, id)?;
        self.merits.push(merit);
        Ok(self)
    }

    pub fn with_merit(
        &mut self,
        template: MeritTemplate,
        detail: Option<String>,
        id: Option<i32>,
    ) -> Result<&mut Self> {
        if self.meets_any_prerequisite_set(template.prerequisites()) {
            self.with_merit_ignore_prerequisites(template, detail, id)
        } else {
            Err(eyre!("prerequisites not met"))
        }
    }

    pub fn build(self) -> Result<Character> {
        if self.player.is_none() {
            return Err(eyre!("player must be specified"));
        }

        if self.name.is_none() {
            return Err(eyre!("name must be specified"));
        }

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
            intimacies: self.intimacies,
            health: self.health,
            weapons: self.weapons,
            armor: self.armor,
            merits: self.merits,
        })
    }
}

pub fn create_character() -> CharacterBuilder {
    CharacterBuilder::default()
}