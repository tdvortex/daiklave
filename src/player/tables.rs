use crate::character::CharacterBuilder;
use crate::player::Player;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "players")]
pub struct PlayerRow {
    pub id: i32,
    pub name: String,
}

impl CharacterBuilder {
    pub fn apply_player_row(self, player_row: PlayerRow) -> Self {
        self.with_player(Player::new(player_row.id, player_row.name))
    }
}
