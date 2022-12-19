use daiklave_core::Character;
use sqlx::PgPool;
use eyre::Result;

pub async fn destroy_character(pool: &PgPool, id: i32) -> Result<()> {
    todo!()
}


pub async fn retrieve_character(pool: &PgPool, character_id: i32) -> Result<Option<Character>> {
    todo!()
}


pub async fn update_character(pool: &PgPool, character: &Character) -> Result<Character> {
    todo!()
}

