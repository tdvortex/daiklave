#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "MERITTYPE", rename_all = "UPPERCASE")]
pub enum MeritTypePostgres {
    Innate,
    Supernatural,
    Story,
    Purchased,
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
