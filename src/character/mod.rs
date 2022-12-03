mod mortal;
pub mod traits;

use traits::campaign::Campaign;
use traits::experience::ExperiencePoints;
use traits::player::Player;
use traits::willpower::Willpower;

use eyre::{eyre, Result};

#[derive(Debug)]
pub struct Character {
    pub id: Option<i32>,
    pub player: Player,
    pub campaign: Option<Campaign>,
    pub name: String,
    pub concept: Option<String>,
    pub willpower: Willpower,
    pub experience: ExperiencePoints,
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

    pub fn build(self) -> Result<Character> {
        if let None = self.player {
            return Err(eyre!("player must be specified"));
        }

        if let None = self.name {
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
        })
    }
}

pub fn create_character() -> CharacterBuilder {
    CharacterBuilder::default()
}
