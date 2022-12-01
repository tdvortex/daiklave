use super::enums::{ExaltType, WoundPenalty, DamageType, AbilityName, AttributeName, IntimacyType, IntimacyLevel, WeaponTag};

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

        Ok(Self { id, name, description, bot_channel })
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name="_campaigns")]
pub struct CampaignRows(Vec<CampaignRow>);

// impl sqlx::Type<Postgres> for CampaignRows {
//     fn type_info() -> sqlx::postgres::PgTypeInfo {
//         sqlx::postgres::PgTypeInfo::with_name("_campaigns")
//     }
// }

// impl<'r> sqlx::Decode<'r, Postgres> for CampaignRows {
//     fn decode(
//         value: sqlx::postgres::PgValueRef<'r>,
//     ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
//         Ok(Self(Vec::<CampaignRow>::decode(value)?))
//     }
// }

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name="players")]
pub struct PlayerRow {
    pub id: i64,
    pub name: String,
}

// impl sqlx::Type<sqlx::Postgres> for PlayerRow {
//     fn type_info() -> sqlx::postgres::PgTypeInfo {
//         sqlx::postgres::PgTypeInfo::with_name("players")
//     }
// }

// impl<'r> sqlx::Decode<'r, Postgres> for PlayerRow {
//     fn decode(
//         value: sqlx::postgres::PgValueRef<'r>,
//     ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
//         let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
//         let id = decoder.try_decode::<i64>()?;
//         let name = decoder.try_decode::<String>()?;

//         Ok(Self { id, name})
//     }
// }

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

pub struct AttributeRow {
    pub character_id: i64,
    pub name: AttributeName,
    pub dots: i16,
}

pub struct AbilityRow {
    pub id: i64,
    pub character_id: i64,
    pub name: AbilityName,
    pub dots: i16,
    pub subskill: Option<String>,
}

pub struct SpecialtyRow {
    pub id: i64,
    pub ability_id: i64,
    pub specialty: String,
}

pub struct IntimacyRow {
    pub id: i64,
    pub character_id: i64,
    pub intimacy_type: IntimacyType,
    pub level: IntimacyLevel,
    pub description: String,
}

pub struct HealthBoxRow {
    pub character_id: i64,
    pub position: i16,
    pub wound_level: WoundPenalty,
    pub damage: Option<DamageType>,
}

pub struct WeaponRow {
    pub id: i64,
    pub name: String,
    pub tags: Vec<WeaponTag>,
}