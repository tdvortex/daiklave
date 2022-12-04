use super::{
    traits::{
        abilities::{Abilities, AbilityName, AbilityNameNoFocus},
        attributes::{AttributeName, Attributes},
        campaign::Campaign,
        experience::ExperiencePoints,
        player::Player,
        willpower::Willpower, intimacies::{Intimacies, Intimacy},
    },
    Character,
};
use eyre::{eyre, Result};

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
}

impl CharacterBuilder {
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

    pub fn with_ability(&mut self, ability_name: AbilityNameNoFocus, value: u8) -> &mut Self {
        self.abilities
            .get_mut(&ability_name.into())
            .unwrap()
            .set_dots(value);
        self
    }

    pub fn with_craft(&mut self, craft_focus: &str, value: u8) -> &mut Self {
        self.abilities.add_craft(craft_focus.to_owned()).set_dots(value);
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
        ability_name: AbilityNameNoFocus,
        specialty: String,
    ) -> Result<&mut Self> {
        self.abilities
            .get_mut(&ability_name.into())
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
        })
    }
}

pub fn create_character() -> CharacterBuilder {
    CharacterBuilder::default()
}