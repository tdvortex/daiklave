use super::{ExperiencePoints, Willpower};
use crate::{
    abilities::{Abilities, AbilityNameVanilla, AbilityNameNoSubskill},
    armor::{Armor, ArmorItem},
    attributes::{AttributeName, Attributes},
    campaign::Campaign,
    craft::CraftAbilities,
    exalt_type::{ExaltTypeBuilder},
    health::{Health, WoundPenalty},
    id::Id,
    intimacies::{Intimacies, Intimacy},
    martial_arts::{MartialArtistTraits, MartialArtsStyle},
    merits::{Merit, MeritTemplate, Merits},
    player::Player,
    weapons::{EquipHand, Weapon, Weapons},
    Character, charms::{MartialArtsCharm, SolarCharm, Spell}, anima::AnimaLevel, solar::{DawnTraits, ZenithTraits, TwilightTraits, NightTraits, EclipseTraits}, sorcery::ShapingRitual,
};
use eyre::{eyre, Result};

#[derive(Debug, Default)]
pub struct CharacterBuilder {
    id: Id,
    player: Player,
    campaign: Option<Campaign>,
    name: String,
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
    exalt_type: ExaltTypeBuilder,
    craft_abilities: CraftAbilities,
    martial_arts_styles: MartialArtistTraits,
}

impl CharacterBuilder {
    pub fn id(&self) -> Id {
        self.id
    }

    pub fn with_placeholder_id(mut self, id: i32) -> Self {
        self.id = Id::Placeholder(id);
        self
    }

    pub fn with_database_id(mut self, id: i32) -> Self {
        self.id = Id::Database(id);
        self
    }

    pub fn with_player(mut self, player: Player) -> Self {
        self.player = player;
        self
    }

    pub fn with_campaign(mut self, campaign: Campaign) -> Self {
        self.campaign = Some(campaign);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
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

    pub fn with_ability(mut self, ability_name: AbilityNameVanilla, dots: u8) -> Self {
        self.abilities.set_dots(ability_name, dots);
        self
    }

    pub fn with_craft(mut self, craft_focus: &str, dots: u8) -> Self {
        self.craft_abilities.set_dots(craft_focus, dots);
        self
    }

    pub fn with_martial_arts_style(mut self, style: MartialArtsStyle, dots: u8) -> Result<Self> {
        self.martial_arts_styles.add_style(style, dots)?;
        Ok(self)
    }

    pub fn with_specialty(
        mut self,
        ability_name: AbilityNameVanilla,
        specialty: String,
    ) -> Result<Self> {
        self.abilities.add_specialty(ability_name, specialty)?;
        Ok(self)
    }

    pub fn with_craft_specialty(mut self, craft_focus: &str, specialty: String) -> Result<Self> {
        self.craft_abilities.add_specialty(craft_focus, specialty)?;
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

    pub fn with_merit_ignore_prerequisites(
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

    pub fn with_martial_arts_charm(mut self, charm: MartialArtsCharm) -> Result<Self> {
        let essence_rating = self.exalt_type.essence().map_or(0, |ess| ess.rating());
        if essence_rating < charm.essence_requirement() {
            if let ExaltTypeBuilder::Solar(solar_builder) = &self.exalt_type {
                if solar_builder.supernal() != Some(AbilityNameNoSubskill::MartialArts) {
                    return Err(eyre!("Essence requirement not met, need {} (or Supernal) but only have {}", charm.essence_requirement(), essence_rating));
                }
            } else {
                return Err(eyre!("Essence requirement not met, need {} but only have {}", charm.essence_requirement(), essence_rating));
            }
        } 

        self.martial_arts_styles.add_charm(charm)?;
        Ok(self)
    }

    pub fn into_solar(mut self) -> Self {
        self.exalt_type = self.exalt_type.into_solar();
        self
    }

    pub fn with_essence_rating(mut self, rating: u8) -> Result<Self> {
        self.exalt_type = self.exalt_type.with_essence_rating(rating)?;
        Ok(self)
    }

    pub fn with_limit(mut self, limit_trigger: String, track: u8) -> Result<Self> {
        self.exalt_type = self.exalt_type.with_limit(limit_trigger, track)?;
        Ok(self)
    }

    pub fn with_anima_level(mut self, anima_level: AnimaLevel) -> Result<Self> {
        self.exalt_type = self.exalt_type.with_anima_level(anima_level)?;
        Ok(self)
    }

    pub fn into_dawn(mut self, dawn_traits: DawnTraits) -> Result<Self> {
        self.exalt_type = self.exalt_type.into_dawn(dawn_traits)?;
        Ok(self)
    }

    pub fn into_zenith(mut self, zenith_traits: ZenithTraits) -> Result<Self> {
        self.exalt_type = self.exalt_type.into_zenith(zenith_traits)?;
        Ok(self)
    }

    pub fn into_twilight(mut self, twilight_traits: TwilightTraits) -> Result<Self> {
        self.exalt_type = self.exalt_type.into_twilight(twilight_traits)?;
        Ok(self)
    }

    pub fn into_night(mut self, night_traits: NightTraits) -> Result<Self> {
        self.exalt_type = self.exalt_type.into_night(night_traits)?;
        Ok(self)
    }

    pub fn into_eclipse(mut self, eclipse_traits: EclipseTraits) -> Result<Self> {
        self.exalt_type = self.exalt_type.into_eclipse(eclipse_traits)?;
        Ok(self)
    }

    pub fn with_favored_ability(mut self, ability: AbilityNameNoSubskill) -> Result<Self> {
        self.exalt_type = self.exalt_type.with_favored_ability(ability)?;
        Ok(self)
    }

    pub fn with_solar_charm(mut self, charm: SolarCharm) -> Result<Self> {
        let meets_ability_requirement = match charm.ability_requirement() {
            (AbilityNameNoSubskill::Craft, dots) => {
                self.craft_abilities.iter().map(|ability| ability.dots()).max().unwrap_or(0) >= dots
            }
            (AbilityNameNoSubskill::MartialArts, _) => {
                return Err(eyre!("Solar charms may not be Martial Arts based"));
            }
            (vanilla_ability, dots) => {
                self.abilities.get(vanilla_ability.try_into()?).dots() >= dots
            }
        };

        if !meets_ability_requirement {
            return Err(eyre!("Minimum ability requirement not met"));
        }

        self.exalt_type = self.exalt_type.with_solar_charm(charm)?;
        Ok(self)
    }

    pub fn with_terrestrial_circle_sorcery(mut self, shaping_ritual: ShapingRitual, control_spell: Spell) -> Result<Self> {
        if self.abilities.get(AbilityNameVanilla::Occult).dots() < 3 {
            Err(eyre!("Occult 3 requirement not met, only have {} dots", self.abilities.get(AbilityNameVanilla::Occult).dots()))
        } else {
            self.exalt_type = self.exalt_type.with_terrestrial_circle_sorcery(shaping_ritual, control_spell)?;
            Ok(self)
        }
    }

    pub fn with_celestial_circle_sorcery(mut self, shaping_ritual: ShapingRitual, control_spell: Spell) -> Result<Self> {
        if self.abilities.get(AbilityNameVanilla::Occult).dots() < 4 {
            Err(eyre!("Occult 4 requirement not met, only have {} dots", self.abilities.get(AbilityNameVanilla::Occult).dots()))
        } else {
            self.exalt_type = self.exalt_type.with_celestial_circle_sorcery(shaping_ritual, control_spell)?;
            Ok(self)
        }
    }

    pub fn with_solar_circle_sorcery(mut self, shaping_ritual: ShapingRitual, control_spell: Spell) -> Result<Self> {
        if self.abilities.get(AbilityNameVanilla::Occult).dots() < 5 {
            Err(eyre!("Occult 5 requirement not met, only have {} dots", self.abilities.get(AbilityNameVanilla::Occult).dots()))
        } else {
            self.exalt_type = self.exalt_type.with_solar_circle_sorcery(shaping_ritual, control_spell)?;
            Ok(self)
        }
    }

    pub fn with_spell(mut self, spell: Spell) -> Result<Self> {
        self.exalt_type = self.exalt_type.with_spell(spell)?;
        Ok(self)
    }

    pub fn build(self) -> Result<Character> {
        Ok(Character {
            id: self.id,
            player: self.player,
            campaign: self.campaign,
            name: self.name,
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
            exalt_type: self.exalt_type.build()?,
            craft_abilities: self.craft_abilities,
            martial_arts_styles: self.martial_arts_styles,
        })
    }
}
