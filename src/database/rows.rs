use sqlx::postgres::PgHasArrayType;
use eyre::{eyre, Report};

use crate::character::traits::prerequisite::{Prerequisite, AbilityPrerequisite, AttributePrerequisite, PrerequisiteType};

use super::composites::{CharmCostPostgres, WeaponTagPostgres};
use super::enums::{
    AbilityNamePostgres, ArmorTagPostgres, AttributeNamePostgres, CharmActionTypePostgres, CharmDurationTypePostgres, CharmKeywordPostgres,
    DamageTypePostgres, EquipHandPostgres, ExaltTypePostgres, IntimacyLevelPostgres, IntimacyTypePostgres, MeritTypePostgres,
    PrerequisiteExaltTypePostgres, PrerequisiteTypePostgres, WoundPenaltyPostgres,
};

#[derive(Debug)]
pub struct CampaignRow {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub bot_channel: i64,
}

impl sqlx::Type<sqlx::Postgres> for CampaignRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("campaigns")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for CampaignRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let name = decoder.try_decode::<String>()?;
        let description = decoder.try_decode::<Option<String>>()?;
        let bot_channel = decoder.try_decode::<i64>()?;

        Ok(Self {
            id,
            name,
            description,
            bot_channel,
        })
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "players")]
pub struct PlayerRow {
    pub id: i32,
    pub name: String,
}

#[derive(Debug)]
pub struct CharacterRow {
    pub id: i32,
    pub player_id: i32,
    pub campaign_id: Option<i32>,
    pub name: String,
    pub concept: Option<String>,
    pub exalt_type: Option<ExaltTypePostgres>,
    pub current_willpower: i16,
    pub max_willpower: i16,
    pub current_experience: i16,
    pub total_experience: i16,
}

impl sqlx::Type<sqlx::Postgres> for CharacterRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("characters")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for CharacterRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let player_id = decoder.try_decode::<i32>()?;
        let campaign_id = decoder.try_decode::<Option<i32>>()?;
        let name = decoder.try_decode::<String>()?;
        let concept = decoder.try_decode::<Option<String>>()?;
        let exalt_type = decoder.try_decode::<Option<ExaltTypePostgres>>()?;
        let current_willpower = decoder.try_decode::<i16>()?;
        let max_willpower = decoder.try_decode::<i16>()?;
        let current_experience = decoder.try_decode::<i16>()?;
        let total_experience = decoder.try_decode::<i16>()?;

        Ok(Self {
            id,
            player_id,
            campaign_id,
            name,
            concept,
            exalt_type,
            current_willpower,
            max_willpower,
            current_experience,
            total_experience,
        })
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "attributes")]
pub struct AttributeRow {
    pub character_id: i32,
    pub name: AttributeNamePostgres,
    pub dots: i16,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "abilities")]
pub struct AbilityRow {
    pub id: i32,
    pub character_id: i32,
    pub name: AbilityNamePostgres,
    pub dots: i16,
    pub subskill: Option<String>,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "specialties")]
pub struct SpecialtyRow {
    pub id: i32,
    pub ability_id: i32,
    pub specialty: String,
}

impl PgHasArrayType for SpecialtyRow {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_specialties")
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "intimacies")]
pub struct IntimacyRow {
    pub id: i32,
    pub character_id: i32,
    pub intimacy_type: IntimacyTypePostgres,
    pub level: IntimacyLevelPostgres,
    pub description: String,
}

impl PgHasArrayType for IntimacyRow {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_intimacies")
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "health_boxes")]
pub struct HealthBoxRow {
    pub character_id: i32,
    pub position: i16,
    pub wound_penalty: WoundPenaltyPostgres,
    pub damage: Option<DamageTypePostgres>,
}

#[derive(Debug)]
pub struct WeaponRow {
    pub id: i32,
    pub name: String,
    pub tags: Vec<WeaponTagPostgres>,
    pub creator_id: Option<i32>,
}

impl sqlx::Type<sqlx::Postgres> for WeaponRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("weapons")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for WeaponRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let name = decoder.try_decode::<String>()?;
        let tags = decoder.try_decode::<Vec<WeaponTagPostgres>>()?;
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            id,
            name,
            tags,
            creator_id,
        })
    }
}

#[derive(Debug)]
pub struct WeaponEquippedRow {
    pub character_id: i32,
    pub weapon_id: i32,
    pub equip_hand: Option<EquipHandPostgres>,
    pub creator_id: Option<i32>,
}

impl sqlx::Type<sqlx::Postgres> for WeaponEquippedRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("character_weapons")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for WeaponEquippedRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let character_id = decoder.try_decode::<i32>()?;
        let weapon_id = decoder.try_decode::<i32>()?;
        let equip_hand = decoder.try_decode::<Option<EquipHandPostgres>>()?;
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            character_id,
            weapon_id,
            equip_hand,
            creator_id,
        })
    }
}

#[derive(Debug)]
pub struct ArmorRow {
    pub id: i32,
    pub name: String,
    pub tags: Vec<ArmorTagPostgres>,
    pub creator_id: Option<i32>,
}

impl sqlx::Type<sqlx::Postgres> for ArmorRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("armor")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for ArmorRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let name = decoder.try_decode::<String>()?;
        let tags = decoder.try_decode::<Vec<ArmorTagPostgres>>()?;
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            id,
            name,
            tags,
            creator_id,
        })
    }
}

impl TryFrom<ArmorRow> for crate::character::traits::armor::ArmorItem {
    type Error = eyre::Report;

    fn try_from(value: ArmorRow) -> Result<Self, Self::Error> {
        Self::new(
            value.name,
            value.tags.into_iter().map(|tag| tag.into()).collect(),
            Some(value.id)
        )
    }
}

#[derive(Debug)]
pub struct ArmorWornRow {
    pub character_id: i32,
    pub armor_id: i32,
    pub worn: bool,
    pub creator_id: Option<i32>,
}

impl sqlx::Type<sqlx::Postgres> for ArmorWornRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("character_armor")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for ArmorWornRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let character_id = decoder.try_decode::<i32>()?;
        let armor_id = decoder.try_decode::<i32>()?;
        let worn = decoder.try_decode::<bool>()?;
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            character_id,
            armor_id,
            worn,
            creator_id,
        })
    }
}

#[derive(Debug)]
pub struct CharmRow {
    pub id: i32,
    pub name: String,
    pub costs: Vec<CharmCostPostgres>,
    pub action_type: CharmActionTypePostgres,
    pub keywords: Vec<CharmKeywordPostgres>,
    pub duration: CharmDurationTypePostgres,
    pub special_duration: Option<String>,
    pub book_name: Option<String>,
    pub page_number: Option<i32>,
    pub creator_id: Option<i32>,
    pub summary: String,
    pub description: String,
}

#[derive(Debug)]
pub struct CharmPrerequisiteSetRow {
    pub id: i32,
    pub charm_id: i32,
    pub prerequisite_id: i32,
}

#[derive(Debug)]
pub struct PrerequisiteRow {
    pub id: i32,
    pub prerequisite_type: PrerequisiteTypePostgres,
    pub ability_name: Option<AbilityNamePostgres>,
    pub subskill_name: Option<String>,
    pub attribute_name: Option<AttributeNamePostgres>,
    pub dots: Option<i16>,
    pub prerequisite_charm_id: Option<i32>,
    pub prerequisite_exalt_type: Option<PrerequisiteExaltTypePostgres>,
}

impl sqlx::Type<sqlx::Postgres> for PrerequisiteRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("prerequisites")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for PrerequisiteRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let prerequisite_type = decoder.try_decode::<PrerequisiteTypePostgres>()?;
        let ability_name = decoder.try_decode::<Option<AbilityNamePostgres>>()?;
        let subskill_name = decoder.try_decode::<Option<String>>()?;
        let attribute_name = decoder.try_decode::<Option<AttributeNamePostgres>>()?;
        let dots = decoder.try_decode::<Option<i16>>()?;
        let prerequisite_charm_id = decoder.try_decode::<Option<i32>>()?;
        let prerequisite_exalt_type = decoder.try_decode::<Option<PrerequisiteExaltTypePostgres>>()?;

        Ok(Self {
            id,
            prerequisite_type,
            ability_name,
            subskill_name,
            attribute_name,
            dots,
            prerequisite_charm_id,
            prerequisite_exalt_type,
        })
    }
}

impl TryInto<Prerequisite> for PrerequisiteRow {
    type Error = Report;

    fn try_into(self) -> Result<Prerequisite, Self::Error> {
        match self.prerequisite_type {
            PrerequisiteTypePostgres::Ability => {
                if self.ability_name.is_none() {
                    return Err(eyre!("ability name must be specified for ability prerequisite"));
                }

                if self.dots.is_none() {
                    return Err(eyre!("dots level must be specified for ability prerequisite")); 
                }

                Ok(Prerequisite::new(
                    PrerequisiteType::Ability(AbilityPrerequisite {
                    ability_name: self.ability_name.unwrap().into(),
                    subskill: self.subskill_name,
                    dots: self.dots.unwrap().try_into()?
                }), 
                Some(self.id)))
            }
            PrerequisiteTypePostgres::Attribute => {
                if self.attribute_name.is_none() {
                    return Err(eyre!("attribute name must be specified for attribute prerequisite"));
                }

                if self.dots.is_none() {
                    return Err(eyre!("dots level must be specified for attribute prerequisite")); 
                }

                Ok(Prerequisite::new(
                    PrerequisiteType::Attribute(AttributePrerequisite {
                        attribute_name: self.attribute_name.unwrap().into(),
                    dots: self.dots.unwrap().try_into()?
                }), 
                Some(self.id)))
            }
            PrerequisiteTypePostgres::Essence => {
                if self.dots.is_none() {
                    return Err(eyre!("dots level must be specified for essence prerequisite"));
                }

                Ok(Prerequisite::new(
                    PrerequisiteType::Essence(self.dots.unwrap().try_into()?), 
                Some(self.id)))
            }
            PrerequisiteTypePostgres::Charm => {
                if self.prerequisite_charm_id.is_none() {
                    return Err(eyre!("charm id must be specified for charm prerequisite"));
                }

                Ok(Prerequisite::new(
                    PrerequisiteType::Charm(self.prerequisite_charm_id.unwrap()), 
                Some(self.id)))
            }
            PrerequisiteTypePostgres::ExaltType => {
                if self.prerequisite_exalt_type.is_none() {
                    return Err(eyre!("exalt type must be specified for exalt type prerequisite"));
                }

                Ok(Prerequisite::new(
                    PrerequisiteType::ExaltType(self.prerequisite_exalt_type.unwrap().into()), 
                Some(self.id)))
            }
        }
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "merits")]
pub struct MeritTemplateRow {
    pub id: i32,
    pub name: String,
    pub dots: i16,
    pub merit_type: MeritTypePostgres,
    pub description: String,
    pub requires_detail: bool,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "merit_prerequisite_sets")]
pub struct MeritPrerequisiteSetRow {
    pub id: i32,
    pub merit_id: i32,
    pub prerequisite_id: i32,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "merit_prerequisite_sets")]
pub struct MeritDetailRow {
    pub character_id: i32,
    pub merit_id: i32,
    pub detail: String,
}