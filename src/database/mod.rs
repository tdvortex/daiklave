use std::collections::{HashMap, HashSet};
use eyre::Report;

use crate::character::{
    builder::create_character,
    builder::CharacterBuilder,
    traits::{
        armor::ArmorItem, campaign::Campaign, experience::ExperiencePoints, intimacies::Intimacy,
        player::Player, weapons::Weapon, willpower::Willpower,
    },
    Character,
};

use self::{
    enums::AbilityNamePostgres,
    rows::*,
};
use eyre::{eyre, Result};
use sqlx::PgPool;

pub mod composites;
pub mod enums;
pub mod rows;

#[derive(Debug)]
pub struct GetCharacter {
    character: CharacterRow,
    player: PlayerRow,
    campaign: Option<CampaignRow>,
    attributes: Vec<AttributeRow>,
    abilities: Vec<AbilityRow>,
    specialties: Option<Vec<SpecialtyRow>>,
    intimacies: Option<Vec<IntimacyRow>>,
    health_boxes: Vec<HealthBoxRow>,
    weapons_owned: Vec<WeaponRow>,
    weapons_equipped: Option<Vec<WeaponEquippedRow>>,
    armor_owned: Option<Vec<ArmorRow>>,
    armor_worn: Option<Vec<ArmorWornRow>>,
    pub merit_templates: Option<Vec<MeritTemplateRow>>,
    pub merit_details: Option<Vec<MeritDetailRow>>,
    pub merit_prerequisite_sets: Option<Vec<MeritPrerequisiteSetRow>>,
    pub merit_prerequisites: Option<Vec<PrerequisiteRow>>,
}

pub async fn get_character(pool: &PgPool, character_id: i32) -> Result<Option<GetCharacter>> {
    Ok(
        sqlx::query_file_as!(GetCharacter, "src/database/get_character.sql", character_id)
            .fetch_optional(pool)
            .await?,
    )
}

impl CharacterBuilder {
    fn apply_player_row(&mut self, player_row: PlayerRow) -> &mut Self {
        self.with_player(Player::new(player_row.id, player_row.name))
    }

    fn apply_campaign_row(&mut self, campaign_row: Option<CampaignRow>) -> &mut Self {
        if let Some(campaign) = campaign_row {
            self.with_campaign(Campaign::new(
                campaign.id,
                campaign.name,
                campaign.bot_channel,
                campaign.description,
            ))
        } else {
            self
        }
    }

    fn apply_character_row(&mut self, character_row: CharacterRow) -> Result<&mut Self> {
        let willpower = Willpower {
            current: character_row.current_willpower.try_into()?,
            maximum: character_row.max_willpower.try_into()?,
        };

        let experience = ExperiencePoints {
            current: character_row.current_experience.try_into()?,
            total: character_row.total_experience.try_into()?,
        };

        let applied = self
            .with_id(character_row.id)
            .with_name(character_row.name)
            .with_willpower(willpower)
            .with_experience(experience);

        Ok(applied)
    }

    fn apply_attribute_row(&mut self, attribute_row: AttributeRow) -> Result<&mut Self> {
        let attribute_name = attribute_row.name.into();
        let value = attribute_row.dots.try_into()?;

        self.with_attribute(attribute_name, value)
    }

    fn apply_attribute_rows(&mut self, attribute_rows: Vec<AttributeRow>) -> Result<&mut Self> {
        attribute_rows.into_iter().fold(Ok(self), |output, attribute_row| {
            output.and_then(|character| character.apply_attribute_row(attribute_row))
        })
    }

    fn apply_ability_with_specialties_rows(
        &mut self,
        ability_row: AbilityRow,
        specialty_rows: Vec<SpecialtyRow>,
    ) -> Result<&mut Self> {
        let dots: u8 = ability_row.dots.try_into()?;

        match ability_row.name {
            AbilityNamePostgres::Craft => {
                let craft_focus = ability_row
                    .subskill
                    .ok_or(eyre!("craft abilities must have a focus"))?;
                self.with_craft(craft_focus.as_str(), dots);
                specialty_rows
                    .into_iter()
                    .fold(Ok(self), |character_result, specialty_row| {
                        character_result.and_then(|character| {
                            character
                                .with_craft_specialty(craft_focus.as_str(), specialty_row.specialty)
                        })
                    })
            }
            AbilityNamePostgres::MartialArts => {
                let martial_arts_style = ability_row
                    .subskill
                    .ok_or(eyre!("martial arts abilities must have a style"))?;
                self.with_martial_arts(martial_arts_style.as_str(), dots);
                specialty_rows
                    .into_iter()
                    .fold(Ok(self), |character_result, specialty_row| {
                        character_result.and_then(|character| {
                            character.with_martial_arts_specialty(
                                martial_arts_style.as_str(),
                                specialty_row.specialty,
                            )
                        })
                    })
            }
            other_ability => {
                let ability_name = other_ability.try_into()?;
                self.with_ability(ability_name, dots)?;
                specialty_rows
                    .into_iter()
                    .fold(Ok(self), |character_result, specialty_row| {
                        character_result.and_then(|character| {
                            character.with_specialty(ability_name, specialty_row.specialty)
                        })
                    })
            }
        }
    }

    fn apply_abilities_and_specialties_rows(&mut self, abilities_rows: Vec<AbilityRow>, specialty_rows: Option<Vec<SpecialtyRow>>) -> Result<&mut Self> {
        let mut abilities_hashmap =
            abilities_rows
                .into_iter()
                .fold(HashMap::new(), |mut map, ability| {
                    map.insert(ability.id, (ability, Vec::<SpecialtyRow>::new()));
                    map
                });

        if let Some(specialties) = specialty_rows {
            specialties.into_iter().fold(
                Ok(&mut abilities_hashmap),
                |map: Result<&mut HashMap<i32, (AbilityRow, Vec<SpecialtyRow>)>, eyre::Report>,
                 specialty: SpecialtyRow| {
                    map.and_then(|m| {
                        m.get_mut(&specialty.ability_id)
                            .ok_or_else(|| eyre!("ability {} not found", specialty.ability_id))
                            .map(|tup| tup.1.push(specialty))?;
                        Ok(m)
                    })
                },
            )?;
        };

        abilities_hashmap.into_iter().fold(
            Ok(self),
            |character_result: Result<&mut CharacterBuilder, Report>, (_, (ability_row, specialty_rows))| {
                character_result.and_then(|character| {
                    character.apply_ability_with_specialties_rows(ability_row, specialty_rows)
                })
            },
        )
    }

    fn apply_intimacy_rows(&mut self, intimacy_rows: Option<Vec<IntimacyRow>>) -> &mut Self {
        if let Some(rows) = intimacy_rows {
            rows.into_iter().fold(self, |s, intimacy_row| {
                s.with_intimacy(Intimacy {
                    intimacy_level: intimacy_row.level.into(),
                    intimacy_type: intimacy_row.intimacy_type.into(),
                    description: intimacy_row.description,
                })
            })
        } else {
            self
        }
    }

    fn apply_health_box_rows(&mut self, health_box_rows: Vec<HealthBoxRow>) -> &mut Self {
        use crate::character::traits::health::WoundPenalty;
        let (mut bashing, mut lethal, mut aggravated) = (0, 0, 0);
        let mut wound_penalties = Vec::new();

        for health_box_row in health_box_rows.into_iter() {
            wound_penalties.push(match health_box_row.wound_penalty {
                enums::WoundPenaltyPostgres::Zero => WoundPenalty::Zero,
                enums::WoundPenaltyPostgres::MinusOne => WoundPenalty::MinusOne,
                enums::WoundPenaltyPostgres::MinusTwo => WoundPenalty::MinusTwo,
                enums::WoundPenaltyPostgres::MinusFour => WoundPenalty::MinusFour,
                enums::WoundPenaltyPostgres::Incapacitated => WoundPenalty::Incapacitated,
            });

            match health_box_row.damage {
                Some(enums::DamageTypePostgres::Bashing) => {
                    bashing += 1;
                }
                Some(enums::DamageTypePostgres::Lethal) => {
                    lethal += 1;
                }
                Some(enums::DamageTypePostgres::Aggravated) => {
                    aggravated += 1;
                }
                None => {}
            }
        }
        self.with_wound_penalties(wound_penalties);
        self.with_damage(bashing, lethal, aggravated);
        self
    }

    fn apply_weapon_rows(
        &mut self,
        weapon_rows: Vec<WeaponRow>,
        weapon_equipped_rows: Option<Vec<WeaponEquippedRow>>,
    ) -> Result<&mut Self> {
        use crate::character::traits::weapons::EquipHand as TraitsEquipHand;
        let mut weapons_hashmap = HashMap::new();

        for weapon_row in weapon_rows.into_iter() {
            let mut tags = HashSet::new();
            for tag in weapon_row.tags {
                tags.insert(tag.try_into()?);
            }
            let weapon = Weapon::new(weapon_row.name, tags, Some(weapon_row.id))?;
            weapons_hashmap.insert(weapon_row.id, (weapon, None));
        }

        if weapon_equipped_rows.is_none() {
            return Ok(self);
        }

        let equips = weapon_equipped_rows.unwrap();

        for weapon_equipped_row in equips.into_iter() {
            if weapon_equipped_row.equip_hand.is_none() {
                continue;
            }

            let (_, equipped) = weapons_hashmap
                .get_mut(&weapon_equipped_row.weapon_id)
                .ok_or_else(|| {
                    eyre!(
                        "cannot equip weapon {} which is not owned",
                        weapon_equipped_row.weapon_id
                    )
                })?;

            *equipped = match (&equipped, weapon_equipped_row.equip_hand.unwrap()) {
                (None, enums::EquipHandPostgres::Main) => Some(TraitsEquipHand::Main),
                (None, enums::EquipHandPostgres::Off) => Some(TraitsEquipHand::Off),
                (Some(TraitsEquipHand::Main), enums::EquipHandPostgres::Main) => {
                    return Err(eyre!("cannot equip two weapons in Main hand"));
                }
                (Some(TraitsEquipHand::Off), enums::EquipHandPostgres::Off) => {
                    return Err(eyre!("cannot equip two weapons in Off hand"));
                }
                (Some(TraitsEquipHand::Both), enums::EquipHandPostgres::Main) => {
                    return Err(eyre!("cannot equip two weapons in Main hand"));
                }
                (Some(TraitsEquipHand::Both), enums::EquipHandPostgres::Off) => {
                    return Err(eyre!("cannot equip two weapons in Off hand"));
                }
                (Some(TraitsEquipHand::Main), enums::EquipHandPostgres::Off) => Some(TraitsEquipHand::Both),
                (Some(TraitsEquipHand::Off), enums::EquipHandPostgres::Main) => Some(TraitsEquipHand::Both),
            };
        }

        let mut applied = Ok(self);

        for (_, (weapon, maybe_equip_hand)) in weapons_hashmap.into_iter() {
            applied = applied.and_then(|character| character.with_weapon(weapon, maybe_equip_hand));
        }

        applied
    }

    fn apply_armor_rows(
        &mut self,
        armor_owned: Option<Vec<ArmorRow>>,
        armor_worn: Option<Vec<ArmorWornRow>>,
    ) -> Result<&mut Self> {
        if armor_owned.is_none() {
            if armor_worn.is_none() {
                return Ok(self);
            } else {
                return Err(eyre!("cannot wear armor that is not owned"));
            }
        }

        let mut armor_hashmap = HashMap::new();

        for armor_row in armor_owned.unwrap().into_iter() {
            let tags = armor_row.tags.into_iter().map(|tag| tag.into()).collect();
            let armor_item = ArmorItem::new(armor_row.name, tags, Some(armor_row.id))?;
            armor_hashmap.insert(armor_row.id, (armor_item, false));
        }

        if let Some(armor_worn_rows) = armor_worn {
            for armor_worn_row in armor_worn_rows.into_iter() {
                if armor_worn_row.worn {
                    let (_, worn) =
                        armor_hashmap
                            .get_mut(&armor_worn_row.armor_id)
                            .ok_or_else(|| {
                                eyre!(
                                    "cannot equip unowned armor item {}",
                                    armor_worn_row.armor_id
                                )
                            })?;
                    *worn = true;
                }
            }
        }

        let mut applied = Ok(self);

        for (_, (armor_item, worn)) in armor_hashmap.into_iter() {
            applied = applied.and_then(|character| character.with_armor(armor_item, worn));
        }

        applied
    }
}

impl TryInto<Character> for GetCharacter {
    type Error = eyre::Report;

    fn try_into(self) -> Result<Character, Self::Error> {
        let mut character = create_character();
        character.apply_player_row(self.player);
        character.apply_campaign_row(self.campaign);
        character.apply_character_row(self.character)?;
        character.apply_attribute_rows(self.attributes)?;
        character.apply_abilities_and_specialties_rows(self.abilities, self.specialties)?;
        character.apply_intimacy_rows(self.intimacies);
        character.apply_health_box_rows(self.health_boxes);
        character.apply_weapon_rows(self.weapons_owned, self.weapons_equipped)?;
        character.apply_armor_rows(self.armor_owned, self.armor_worn)?;

        character.build()
    }
}
