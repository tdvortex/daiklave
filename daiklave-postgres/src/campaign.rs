use daiklave_core::{campaign::Campaign, character::CharacterBuilder};

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

pub fn apply_campaign_row(
    builder: CharacterBuilder,
    campaign_row: Option<CampaignRow>,
) -> CharacterBuilder {
    if let Some(campaign) = campaign_row {
        builder.with_campaign(Campaign::new(
            campaign.id,
            campaign.name,
            campaign.bot_channel,
            campaign.description,
        ))
    } else {
        builder
    }
}