use std::ops::Deref;

use super::{
    traits::{
        abilities::{Abilities, AbilityName, AbilityNameNoSubskill},
        armor::{Armor, ArmorItem},
        attributes::{AttributeName, Attributes},
        campaign::Campaign,
        experience::ExperiencePoints,
        health::{Health, WoundPenalty},
        intimacies::{Intimacies, Intimacy},
        merits::{Merit, MeritTemplate, Merits},
        player::Player,
        prerequisite::{ExaltTypePrerequisite, Prerequisite, PrerequisiteSet, PrerequisiteType},
        range_bands::RangeBand,
        weapons::{EquipHand, Weapon, WeaponTag, Weapons},
        willpower::Willpower,
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

#[derive(Debug, Default)]
pub struct WeaponBuilder {
    id: Option<i32>,
    name: Option<String>,
    two_handed: bool,
    is_lethal: bool,
    weight_class_tag: Option<WeaponTag>,
    attack_tags: Vec<WeaponTag>,
    other_tags: Vec<WeaponTag>,
    creator_id: Option<i32>,
}

pub fn create_book_weapon() -> WeaponBuilder {
    WeaponBuilder::default()
}

pub fn create_custom_weapon(_character_id: i32) -> WeaponBuilder {
    WeaponBuilder {
        id: Default::default(),
        name: Default::default(),
        two_handed: Default::default(),
        is_lethal: Default::default(),
        weight_class_tag: Default::default(),
        attack_tags: Default::default(),
        other_tags: Default::default(),
        creator_id: Some(_character_id),
    }
}

impl WeaponBuilder {
    pub fn with_id(&mut self, id: i32) -> &mut WeaponBuilder {
        self.id = Some(id);
        self
    }

    pub fn with_name(&mut self, name: String) -> &mut WeaponBuilder {
        self.name = Some(name);
        self
    }

    pub fn dealing_bashing(&mut self) -> &mut WeaponBuilder {
        self.is_lethal = false;
        self
    }

    pub fn dealing_lethal(&mut self) -> &mut WeaponBuilder {
        self.is_lethal = true;
        self
    }

    pub fn as_one_handed(&mut self) -> &mut WeaponBuilder {
        self.two_handed = false;
        self
    }

    pub fn as_two_handed(&mut self) -> &mut WeaponBuilder {
        self.two_handed = true;
        self
    }

    pub fn as_archery_with_range(&mut self, max_range: RangeBand) -> &mut WeaponBuilder {
        self.attack_tags = std::mem::take(&mut self.attack_tags)
            .into_iter()
            .filter_map(|tag| match tag {
                WeaponTag::MartialArts(style) => Some(WeaponTag::MartialArts(style)),
                _ => None,
            })
            .collect();

        self.attack_tags.push(WeaponTag::Archery(max_range));
        self
    }

    pub fn as_brawl(&mut self) -> &mut WeaponBuilder {
        self.attack_tags = std::mem::take(&mut self.attack_tags)
            .into_iter()
            .filter_map(|tag| match tag {
                WeaponTag::MartialArts(style) => Some(WeaponTag::MartialArts(style)),
                _ => None,
            })
            .collect();

        self.attack_tags.push(WeaponTag::Brawl);
        self
    }

    pub fn as_melee(&mut self) -> &mut WeaponBuilder {
        self.attack_tags = std::mem::take(&mut self.attack_tags)
            .into_iter()
            .filter_map(|tag| match tag {
                WeaponTag::MartialArts(style) => Some(WeaponTag::MartialArts(style)),
                WeaponTag::Thrown(range) => Some(WeaponTag::Thrown(range)),
                _ => None,
            })
            .collect();

        self.attack_tags.push(WeaponTag::Melee);
        self
    }

    pub fn with_thrown_range(&mut self, max_range: RangeBand) -> &mut WeaponBuilder {
        self.attack_tags = std::mem::take(&mut self.attack_tags)
            .into_iter()
            .filter_map(|tag| match tag {
                WeaponTag::MartialArts(style) => Some(WeaponTag::MartialArts(style)),
                WeaponTag::Melee => Some(WeaponTag::Melee),
                _ => None,
            })
            .collect();

        self.attack_tags.push(WeaponTag::Thrown(max_range));
        self
    }

    pub fn with_martial_arts(&mut self, style: String) -> &mut WeaponBuilder {
        self.attack_tags.push(WeaponTag::MartialArts(style));
        self
    }

    pub fn as_light(&mut self) -> &mut WeaponBuilder {
        self.weight_class_tag = Some(WeaponTag::Light);
        self
    }

    pub fn as_medium(&mut self) -> &mut WeaponBuilder {
        self.weight_class_tag = Some(WeaponTag::Medium);
        self
    }

    pub fn as_heavy(&mut self) -> &mut WeaponBuilder {
        self.weight_class_tag = Some(WeaponTag::Heavy);
        self
    }

    pub fn as_artifact(&mut self) -> &mut WeaponBuilder {
        self.other_tags.push(WeaponTag::Artifact);
        self
    }

    pub fn with_tag(&mut self, tag: WeaponTag) -> &mut WeaponBuilder {
        match tag {
            WeaponTag::Archery(range) => self.as_archery_with_range(range),
            WeaponTag::Artifact => self.as_artifact(),
            WeaponTag::Bashing => self.dealing_bashing(),
            WeaponTag::Brawl => self.as_brawl(),
            WeaponTag::Heavy => self.as_heavy(),
            WeaponTag::Lethal => self.dealing_lethal(),
            WeaponTag::Light => self.as_light(),
            WeaponTag::MartialArts(style) => self.with_martial_arts(style),
            WeaponTag::Medium => self.as_medium(),
            WeaponTag::Melee => self.as_melee(),
            WeaponTag::OneHanded => self.as_one_handed(),
            WeaponTag::Thrown(range) => self.with_thrown_range(range),
            WeaponTag::TwoHanded => self.as_two_handed(),
            other_tag => {
                self.other_tags.push(other_tag);
                self
            }
        }
    }

    pub fn build(self) -> Result<Weapon> {
        if self.name.is_none() {
            return Err(eyre!("weapon name is required"));
        }

        let mut tags = Vec::new();
        tags.push(
            self.weight_class_tag
                .ok_or_else(|| eyre!("weapons must be exactly one of Light, Medium, or Heavy"))?,
        );

        if self.two_handed {
            tags.push(WeaponTag::TwoHanded)
        } else {
            tags.push(WeaponTag::OneHanded)
        };

        if self.is_lethal {
            tags.push(WeaponTag::Lethal)
        } else {
            tags.push(WeaponTag::Bashing)
        };

        tags.extend(self.attack_tags.into_iter());
        tags.extend(self.other_tags.into_iter());

        Weapon::new(
            self.name.unwrap(),
            tags.into_iter().collect(),
            self.id,
            self.creator_id,
        )
    }
}
