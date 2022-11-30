#[derive(sqlx::Type)]
#[sqlx(type_name = "EXALTTYPE", rename_all = "UPPERCASE")]
pub enum ExaltType {
    Solar,
    Lunar,
    DragonBlooded,
}


#[derive(sqlx::Type)]
#[sqlx(type_name = "ATTRIBUTENAME", rename_all = "UPPERCASE")]
pub enum AttributeName {
    Strength,
    Dexterity,
    Stamina,
    Charisma,
    Manipulation,
    Appearance,
    Perception,
    Intelligence,
    Wits,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "ABILITYNAME", rename_all = "UPPERCASE")]
pub enum AbilityName {
    Archery,
    Athletics,
    Awareness,
    Brawl,
    Bureaucracy,
    Craft,
    Dodge,
    Integrity,
    Investigation,
    Larcency,
    Linguistics,
    Lore,
    MartialArts,
    Medicine,
    Melee,
    Occult,
    Performance,
    Presence,
    Resistance,
    Ride,
    Sail,
    Socialize,
    Stealth,
    Survival,
    Thrown,
    War,
}