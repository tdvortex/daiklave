use crate::character::{builder::CharacterBuilder, traits::player::Player};

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "players")]
pub struct PlayerRow {
    pub id: i32,
    pub name: String,
}

impl CharacterBuilder {
    pub fn apply_player_row(&mut self, player_row: PlayerRow) -> &mut Self {
        self.with_player(Player::new(player_row.id, player_row.name))
    }
}
