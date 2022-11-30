use super::enums::{ExaltType, AbilityName, AttributeName, IntimacyType, IntimacyLevel};

pub struct CampaignRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub bot_channel: i64,
}

pub struct PlayerRow {
    pub id: i64,
    pub name: String,
}

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