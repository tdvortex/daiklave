use std::collections::HashMap;

use crate::character::CharacterBuilder;
use crate::id::Id;
use crate::weapons::{EquipHand, RangeBand, Weapon, WeaponTag};
use eyre::{eyre, Context, Report, Result};
use sqlx::postgres::PgHasArrayType;

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "RANGEBAND", rename_all = "UPPERCASE")]
pub enum RangeBandPostgres {
    Close,
    Short,
    Medium,
    Long,
    Extreme,
}

impl PgHasArrayType for RangeBandPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_RANGEBAND")
    }
}

impl From<RangeBandPostgres> for RangeBand {
    fn from(range: RangeBandPostgres) -> Self {
        match range {
            RangeBandPostgres::Close => Self::Close,
            RangeBandPostgres::Short => Self::Short,
            RangeBandPostgres::Medium => Self::Medium,
            RangeBandPostgres::Long => Self::Long,
            RangeBandPostgres::Extreme => Self::Extreme,
        }
    }
}

impl From<RangeBand> for RangeBandPostgres {
    fn from(range: RangeBand) -> Self {
        match range {
            RangeBand::Close => Self::Close,
            RangeBand::Short => Self::Short,
            RangeBand::Medium => Self::Medium,
            RangeBand::Long => Self::Long,
            RangeBand::Extreme => Self::Extreme,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "WEAPONTAGTYPE", rename_all = "UPPERCASE")]
pub enum WeaponTagTypePostgres {
    Archery,
    Artifact,
    Balanced,
    Bashing,
    Brawl,
    Chopping,
    Concealable,
    Crossbow,
    Cutting,
    Disarming,
    Exceptional,
    Flame,
    Flexible,
    Grappling,
    Heavy,
    Improvised,
    Lethal,
    Light,
    MartialArts,
    Medium,
    Melee,
    Mounted,
    OneHanded,
    Natural,
    Piercing,
    Poisonable,
    Powerful,
    Reaching,
    Shield,
    Slow,
    Smashing,
    Special,
    Subtle,
    Thrown,
    TwoHanded,
    Worn,
}

impl PgHasArrayType for WeaponTagTypePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_WEAPONTAGTYPE")
    }
}

impl From<WeaponTag> for WeaponTagTypePostgres {
    fn from(tag: WeaponTag) -> Self {
        match tag {
            WeaponTag::Archery(_) => Self::Archery,
            WeaponTag::Artifact => Self::Artifact,
            WeaponTag::Balanced => Self::Balanced,
            WeaponTag::Bashing => Self::Bashing,
            WeaponTag::Brawl => Self::Brawl,
            WeaponTag::Chopping => Self::Chopping,
            WeaponTag::Concealable => Self::Concealable,
            WeaponTag::Crossbow => Self::Crossbow,
            WeaponTag::Cutting => Self::Cutting,
            WeaponTag::Disarming => Self::Disarming,
            WeaponTag::Exceptional => Self::Exceptional,
            WeaponTag::Flame => Self::Flame,
            WeaponTag::Flexible => Self::Flexible,
            WeaponTag::Grappling => Self::Grappling,
            WeaponTag::Heavy => Self::Heavy,
            WeaponTag::Improvised => Self::Improvised,
            WeaponTag::Lethal => Self::Lethal,
            WeaponTag::Light => Self::Light,
            WeaponTag::MartialArts(_) => Self::MartialArts,
            WeaponTag::Medium => Self::Medium,
            WeaponTag::Melee => Self::Melee,
            WeaponTag::Mounted => Self::Mounted,
            WeaponTag::OneHanded => Self::OneHanded,
            WeaponTag::Natural => Self::Natural,
            WeaponTag::Piercing => Self::Piercing,
            WeaponTag::Poisonable => Self::Poisonable,
            WeaponTag::Powerful => Self::Powerful,
            WeaponTag::Reaching => Self::Reaching,
            WeaponTag::Shield => Self::Shield,
            WeaponTag::Slow => Self::Slow,
            WeaponTag::Smashing => Self::Smashing,
            WeaponTag::Special => Self::Special,
            WeaponTag::Subtle => Self::Subtle,
            WeaponTag::Thrown(_) => Self::Thrown,
            WeaponTag::TwoHanded => Self::TwoHanded,
            WeaponTag::Worn => Self::Worn,
        }
    }
}

impl TryFrom<WeaponTagTypePostgres> for WeaponTag {
    type Error = eyre::Report;

    fn try_from(value: WeaponTagTypePostgres) -> Result<Self, Self::Error> {
        match value {
            WeaponTagTypePostgres::Archery => Err(eyre!("Range band missing for Archery tag")),
            WeaponTagTypePostgres::Thrown => Err(eyre!("Range band missing for Thrown tag")),
            WeaponTagTypePostgres::MartialArts => Err(eyre!("Style missing for Martial Arts tag")),
            WeaponTagTypePostgres::Artifact => Ok(Self::Artifact),
            WeaponTagTypePostgres::Balanced => Ok(Self::Balanced),
            WeaponTagTypePostgres::Bashing => Ok(Self::Bashing),
            WeaponTagTypePostgres::Brawl => Ok(Self::Brawl),
            WeaponTagTypePostgres::Chopping => Ok(Self::Chopping),
            WeaponTagTypePostgres::Concealable => Ok(Self::Concealable),
            WeaponTagTypePostgres::Crossbow => Ok(Self::Crossbow),
            WeaponTagTypePostgres::Cutting => Ok(Self::Cutting),
            WeaponTagTypePostgres::Disarming => Ok(Self::Disarming),
            WeaponTagTypePostgres::Exceptional => Ok(Self::Exceptional),
            WeaponTagTypePostgres::Flame => Ok(Self::Flame),
            WeaponTagTypePostgres::Flexible => Ok(Self::Flexible),
            WeaponTagTypePostgres::Grappling => Ok(Self::Grappling),
            WeaponTagTypePostgres::Heavy => Ok(Self::Heavy),
            WeaponTagTypePostgres::Improvised => Ok(Self::Improvised),
            WeaponTagTypePostgres::Lethal => Ok(Self::Lethal),
            WeaponTagTypePostgres::Light => Ok(Self::Light),
            WeaponTagTypePostgres::Medium => Ok(Self::Medium),
            WeaponTagTypePostgres::Melee => Ok(Self::Melee),
            WeaponTagTypePostgres::Mounted => Ok(Self::Mounted),
            WeaponTagTypePostgres::OneHanded => Ok(Self::OneHanded),
            WeaponTagTypePostgres::Natural => Ok(Self::Natural),
            WeaponTagTypePostgres::Piercing => Ok(Self::Piercing),
            WeaponTagTypePostgres::Poisonable => Ok(Self::Poisonable),
            WeaponTagTypePostgres::Powerful => Ok(Self::Powerful),
            WeaponTagTypePostgres::Reaching => Ok(Self::Reaching),
            WeaponTagTypePostgres::Shield => Ok(Self::Shield),
            WeaponTagTypePostgres::Slow => Ok(Self::Slow),
            WeaponTagTypePostgres::Smashing => Ok(Self::Smashing),
            WeaponTagTypePostgres::Special => Ok(Self::Special),
            WeaponTagTypePostgres::Subtle => Ok(Self::Subtle),
            WeaponTagTypePostgres::TwoHanded => Ok(Self::TwoHanded),
            WeaponTagTypePostgres::Worn => Ok(Self::Worn),
        }
    }
}

#[derive(Debug)]
pub struct WeaponTagRow {
    weapon_id: i32,
    tag_type: WeaponTagTypePostgres,
    max_range: Option<RangeBandPostgres>,
    martial_arts_style: Option<String>,
}

impl sqlx::Type<sqlx::Postgres> for WeaponTagRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("weapon_tags")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for WeaponTagRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let weapon_id = decoder.try_decode::<i32>()?;
        let tag_type = decoder.try_decode::<WeaponTagTypePostgres>()?;
        let max_range = decoder.try_decode::<Option<RangeBandPostgres>>()?;
        let martial_arts_style = decoder.try_decode::<Option<String>>()?;

        Ok(Self {
            weapon_id,
            tag_type,
            max_range,
            martial_arts_style,
        })
    }
}

impl TryFrom<WeaponTagRow> for WeaponTag {
    type Error = Report;

    fn try_from(value: WeaponTagRow) -> Result<Self, Self::Error> {
        match value.tag_type {
            WeaponTagTypePostgres::Archery => match value.max_range {
                Some(range) => Ok(Self::Archery(range.into())),
                None => Err(eyre!("Archery must have a range band")),
            },
            WeaponTagTypePostgres::Artifact => Ok(Self::Artifact),
            WeaponTagTypePostgres::Balanced => Ok(Self::Balanced),
            WeaponTagTypePostgres::Bashing => Ok(Self::Bashing),
            WeaponTagTypePostgres::Brawl => Ok(Self::Brawl),
            WeaponTagTypePostgres::Chopping => Ok(Self::Chopping),
            WeaponTagTypePostgres::Concealable => Ok(Self::Concealable),
            WeaponTagTypePostgres::Crossbow => Ok(Self::Crossbow),
            WeaponTagTypePostgres::Cutting => Ok(Self::Cutting),
            WeaponTagTypePostgres::Disarming => Ok(Self::Disarming),
            WeaponTagTypePostgres::Exceptional => Ok(Self::Exceptional),
            WeaponTagTypePostgres::Flame => Ok(Self::Flame),
            WeaponTagTypePostgres::Flexible => Ok(Self::Flexible),
            WeaponTagTypePostgres::Grappling => Ok(Self::Grappling),
            WeaponTagTypePostgres::Heavy => Ok(Self::Heavy),
            WeaponTagTypePostgres::Improvised => Ok(Self::Improvised),
            WeaponTagTypePostgres::Lethal => Ok(Self::Lethal),
            WeaponTagTypePostgres::Light => Ok(Self::Light),
            WeaponTagTypePostgres::MartialArts => match value.martial_arts_style {
                Some(style) => Ok(Self::MartialArts(style)),
                None => Err(eyre!("Martial arts must have a style")),
            },
            WeaponTagTypePostgres::Medium => Ok(Self::Medium),
            WeaponTagTypePostgres::Melee => Ok(Self::Melee),
            WeaponTagTypePostgres::Mounted => Ok(Self::Mounted),
            WeaponTagTypePostgres::OneHanded => Ok(Self::OneHanded),
            WeaponTagTypePostgres::Natural => Ok(Self::Natural),
            WeaponTagTypePostgres::Piercing => Ok(Self::Piercing),
            WeaponTagTypePostgres::Poisonable => Ok(Self::Poisonable),
            WeaponTagTypePostgres::Powerful => Ok(Self::Powerful),
            WeaponTagTypePostgres::Reaching => Ok(Self::Reaching),
            WeaponTagTypePostgres::Shield => Ok(Self::Shield),
            WeaponTagTypePostgres::Slow => Ok(Self::Slow),
            WeaponTagTypePostgres::Smashing => Ok(Self::Smashing),
            WeaponTagTypePostgres::Special => Ok(Self::Special),
            WeaponTagTypePostgres::Subtle => Ok(Self::Subtle),
            WeaponTagTypePostgres::Thrown => match value.max_range {
                Some(range) => Ok(Self::Thrown(range.into())),
                None => Err(eyre!("Thrown must have a range band")),
            },
            WeaponTagTypePostgres::TwoHanded => Ok(Self::TwoHanded),
            WeaponTagTypePostgres::Worn => Ok(Self::Worn),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "EQUIPHAND", rename_all = "UPPERCASE")]
pub enum EquipHandPostgres {
    Main,
    Off,
}

impl PgHasArrayType for EquipHandPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_EQUIPHAND")
    }
}

#[derive(Debug)]
pub struct WeaponRow {
    pub id: i32,
    pub name: String,
    pub book_title: Option<String>,
    pub page_number: Option<i16>,
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
        let book_title = decoder.try_decode::<Option<String>>()?;
        let page_number = decoder.try_decode::<Option<i16>>()?;
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            id,
            name,
            book_title,
            page_number,
            creator_id,
        })
    }
}

#[derive(Debug)]
pub struct WeaponEquippedRow {
    pub character_id: i32,
    pub weapon_id: i32,
    pub equip_hand: Option<EquipHandPostgres>,
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

        Ok(Self {
            character_id,
            weapon_id,
            equip_hand,
        })
    }
}

impl CharacterBuilder {
    pub(crate) fn apply_weapon_rows(
        mut self,
        weapon_rows: Option<Vec<WeaponRow>>,
        weapon_tag_rows: Option<Vec<WeaponTagRow>>,
        weapon_equipped_rows: Option<Vec<WeaponEquippedRow>>,
    ) -> Result<Self> {
        if weapon_rows.is_none() || weapon_equipped_rows.is_none() {
            return Ok(self);
        }
        let weapon_rows = weapon_rows.unwrap();
        let weapon_equipped_rows = weapon_equipped_rows.unwrap();

        let mut weapon_rows_hashmap = HashMap::new();

        for weapon_row in weapon_rows.into_iter() {
            weapon_rows_hashmap.insert(weapon_row.id, (weapon_row, Vec::new()));
        }

        if let Some(tag_rows) = weapon_tag_rows {
            for tag_row in tag_rows.into_iter() {
                let tag = match tag_row.tag_type {
                    WeaponTagTypePostgres::Archery => {
                        let range = tag_row.max_range.ok_or_else(|| {
                            eyre!(
                                "Range band missing for Archery tag on weapon {}",
                                tag_row.weapon_id
                            )
                        })?;
                        WeaponTag::Archery(range.into())
                    }
                    WeaponTagTypePostgres::Thrown => {
                        let range = tag_row.max_range.ok_or_else(|| {
                            eyre!(
                                "Range band missing for Thrown tag on weapon {}",
                                tag_row.weapon_id
                            )
                        })?;
                        WeaponTag::Thrown(range.into())
                    }
                    WeaponTagTypePostgres::MartialArts => {
                        let style = tag_row.martial_arts_style.ok_or_else(|| {
                            eyre!(
                                "Martial Arts style missing for Thrown tag on weapon {}",
                                tag_row.weapon_id
                            )
                        })?;
                        WeaponTag::MartialArts(style)
                    }
                    other_tag => other_tag
                        .try_into()
                        .wrap_err_with(|| format!("Unknown tag type: {:?}", other_tag))?,
                };

                weapon_rows_hashmap
                    .get_mut(&tag_row.weapon_id)
                    .ok_or_else(|| eyre!("Missing weapon row with id {}", tag_row.weapon_id))?
                    .1
                    .push(tag);
            }
        }

        let mut weapons_hashmap = HashMap::new();

        for (weapon_id, (weapon_row, tags)) in weapon_rows_hashmap.into_iter() {
            let mut builder = if weapon_row.book_title.is_some()
                && weapon_row.page_number.is_some()
                && weapon_row.creator_id.is_none()
            {
                Weapon::from_book(
                    weapon_row.book_title.unwrap(),
                    weapon_row.page_number.unwrap(),
                )
            } else if weapon_row.book_title.is_none()
                && weapon_row.page_number.is_none()
                && weapon_row.creator_id.is_some()
            {
                Weapon::custom(Id::Database(weapon_row.creator_id.unwrap()))
            } else {
                return Err(eyre!(
                    "Database error: inconsistent data source for weapon {}",
                    weapon_id
                ));
            };

            builder = builder.with_id(weapon_id).with_name(weapon_row.name);

            for tag in tags.into_iter() {
                builder = builder.with_tag(tag);
            }

            let weapon = builder
                .build()
                .wrap_err_with(|| format!("Could not build weapon with id {}", weapon_id))?;
            weapons_hashmap.insert(weapon_id, weapon);
        }

        let mut hands_hashmap = HashMap::new();

        for equip_row in weapon_equipped_rows.into_iter() {
            match equip_row.equip_hand {
                None => {
                    hands_hashmap.entry(equip_row.weapon_id).or_insert(None);
                }
                Some(EquipHandPostgres::Main) => {
                    let prev_hand = hands_hashmap.entry(equip_row.weapon_id).or_insert(None);
                    match prev_hand {
                        Some(EquipHand::Both) | Some(EquipHand::Main) => {
                            return Err(eyre!(
                                "Too many hands in use on character {}",
                                equip_row.character_id
                            ));
                        }
                        None => {
                            *prev_hand = Some(EquipHand::Main);
                        }
                        Some(EquipHand::Off) => {
                            *prev_hand = Some(EquipHand::Both);
                        }
                    }
                }
                Some(EquipHandPostgres::Off) => {
                    let prev_hand = hands_hashmap.entry(equip_row.weapon_id).or_insert(None);
                    match prev_hand {
                        Some(EquipHand::Both) | Some(EquipHand::Off) => {
                            return Err(eyre!(
                                "Too many hands in use on character {}",
                                equip_row.character_id
                            ));
                        }
                        None => {
                            *prev_hand = Some(EquipHand::Off);
                        }
                        Some(EquipHand::Main) => {
                            *prev_hand = Some(EquipHand::Both);
                        }
                    }
                }
            }
        }

        for (weapon_id, maybe_equip_hand) in hands_hashmap.into_iter() {
            let weapon = weapons_hashmap
                .remove(&weapon_id)
                .ok_or_else(|| eyre!("Missing weapon row with id {}", weapon_id))?;
            self = self
                .with_weapon(weapon, maybe_equip_hand)
                .wrap_err_with(|| {
                    format!("Could not apply weapon row {} to character", weapon_id)
                })?;
        }

        Ok(self)
    }
}
