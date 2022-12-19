use daiklave_core::{id::Id, player::Player};

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "players")]
pub struct PlayerRow {
    pub id: i32,
    pub name: String,
}

impl From<PlayerRow> for Player {
    fn from(row: PlayerRow) -> Self {
        Player::new(Id::Database(row.id), row.name)
    }
}
