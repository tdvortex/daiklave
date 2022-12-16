use crate::charms::tables::{CharmActionTypePostgres, CharmKeywordPostgres};

struct MartialArtsStyleRow {
    id: i32,
    name: String,
    description: String,
    book_name: Option<String>,
    page_number: Option<i16>,
    creator_id: Option<i32>,
}

struct CharacterMartialArtsRow {
    character_id: i32,
    style_id: i32,
    dots: i16,
}

struct CharacterMartialArtsSpecialtyRow {
    character_id: i32,
    style_id: i32,
    specialty: String,
}

struct MartialArtsCharmRow {
    id: i32,
    style_id: i32,
    ability_dots_required: i16,
    essence_dots_required: i16,
    name: String,
    summary: Option<String>,
    action_type: CharmActionTypePostgres,
    duration: String,
    book_name: Option<String>,
    page_number: Option<i16>,
    creator_id: Option<i32>,
}

struct MartialArtsCharmKeywordRow {
    charm_id: i32,
    charm_keyword: CharmKeywordPostgres,
}

struct MartialArtsStyleWeaponRow {
    style_id: i32,
    weapon_id: i32,
}
