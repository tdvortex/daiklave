use sqlx::postgres::PgHasArrayType;

use super::enums::{
    AbilityName, AttributeName, DamageType, ExaltType, IntimacyLevel, IntimacyType,
    WoundPenalty, ArmorTag, EquipHand, CharmKeyword, CharmDurationType, CharmActionType,
};
use super::composites::{WeaponTag, CharmCost};

#[derive(Debug)]
pub struct CampaignRow {
    pub id: i64,
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
        let id = decoder.try_decode::<i64>()?;
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
    pub id: i64,
    pub name: String,
}

#[derive(Debug)]
pub struct CharacterRow {
    pub id: i64,
    pub campaign_player_id: i64,
    pub name: String,
    pub concept: Option<String>,
    pub exalt_type: Option<ExaltType>,
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
        let id = decoder.try_decode::<i64>()?;
        let campaign_player_id = decoder.try_decode::<i64>()?;
        let name = decoder.try_decode::<String>()?;
        let concept = decoder.try_decode::<Option<String>>()?;
        let exalt_type = decoder.try_decode::<Option<ExaltType>>()?;
        let current_willpower = decoder.try_decode::<i16>()?;
        let max_willpower = decoder.try_decode::<i16>()?;
        let current_experience = decoder.try_decode::<i16>()?;
        let total_experience = decoder.try_decode::<i16>()?;

        Ok(Self {
            id,
            campaign_player_id,
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
    pub character_id: i64,
    pub name: AttributeName,
    pub dots: i16,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "abilities")]
pub struct AbilityRow {
    pub id: i64,
    pub character_id: i64,
    pub name: AbilityName,
    pub dots: i16,
    pub subskill: Option<String>,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "specialties")]
pub struct SpecialtyRow {
    pub id: i64,
    pub ability_id: i64,
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
    pub id: i64,
    pub character_id: i64,
    pub intimacy_type: IntimacyType,
    pub level: IntimacyLevel,
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
    pub character_id: i64,
    pub position: i16,
    pub wound_penalty: WoundPenalty,
    pub damage: Option<DamageType>,
}

#[derive(Debug)]
pub struct WeaponRow {
    pub id: i64,
    pub name: String,
    pub tags: Vec<WeaponTag>,
    pub creator_id: Option<i64>,
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
        let id = decoder.try_decode::<i64>()?;
        let name = decoder.try_decode::<String>()?;
        let tags = decoder.try_decode::<Vec<WeaponTag>>()?;
        let creator_id = decoder.try_decode::<Option<i64>>()?;

        Ok(Self { id, name, tags, creator_id })
    }
}

#[derive(Debug)]
pub struct WeaponEquippedRow {
    pub character_id: i64,
    pub weapon_id: i64,
    pub equip_hand: Option<EquipHand>,
    pub creator_id: Option<i64>,
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
        let character_id = decoder.try_decode::<i64>()?;
        let weapon_id = decoder.try_decode::<i64>()?;
        let equip_hand = decoder.try_decode::<Option<EquipHand>>()?;
        let creator_id = decoder.try_decode::<Option<i64>>()?;

        Ok(Self { character_id, weapon_id, equip_hand, creator_id})
    }
}

#[derive(Debug)]
pub struct ArmorRow {
    pub id: i64,
    pub name: String,
    pub tags: Vec<ArmorTag>,
    pub creator_id: Option<i64>,
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
        let id = decoder.try_decode::<i64>()?;
        let name = decoder.try_decode::<String>()?;
        let tags = decoder.try_decode::<Vec<ArmorTag>>()?;
        let creator_id = decoder.try_decode::<Option<i64>>()?;

        Ok(Self { id, name, tags, creator_id})
    }
}


#[derive(Debug)]
pub struct ArmorWornRow {
    pub character_id: i64,
    pub armor_id: i64,
    pub worn: bool,
    pub creator_id: Option<i64>,
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
        let character_id = decoder.try_decode::<i64>()?;
        let armor_id = decoder.try_decode::<i64>()?;
        let worn = decoder.try_decode::<bool>()?;
        let creator_id = decoder.try_decode::<Option<i64>>()?;

        Ok(Self { character_id, armor_id, worn, creator_id})
    }
}

#[derive(Debug)]
pub struct CharmRow {
    pub id: i64,
    pub name: String,
    pub costs: Vec<CharmCost>,
    pub action_type: CharmActionType,
    pub keywords: Vec<CharmKeyword>,
    pub duration: CharmDurationType,
    pub special_duration: Option<String>,
    pub book_name: Option<String>,
    pub page_number: Option<i32>,
    pub creator_id: Option<i64>,
    pub summary: String,
    pub description: String,
}