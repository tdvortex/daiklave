use ::eyre::{eyre, Result};
use sqlx::{query, PgPool, Postgres, Transaction};

use crate::character::Character;
use crate::player::Player;
use crate::character::retrieve::retrieve_character_transaction;

pub async fn create_character(pool: &PgPool, player: Player) -> Result<Character> {
    let mut transaction = pool.begin().await?;

    let character = create_character_transaction(&mut transaction, player).await?;

    transaction.commit().await?;

    Ok(character)
}

pub(crate) async fn create_character_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    player: Player,
) -> Result<Character> {
    // Insert character placeholder and get an ID
    let character_id = query!(
        "
        INSERT INTO characters(player_id, name, current_willpower, max_willpower, current_experience, total_experience)
        VALUES($1, 'New Character', 0, 0, 0, 0)
        RETURNING id
        ",
        player.id()
    ).fetch_one(&mut *transaction).await?.id;

    // Insert attributes
    query!(
        "
        INSERT INTO attributes(character_id, name, dots)
        VALUES
            ($1, 'STRENGTH', 1),
            ($1, 'DEXTERITY', 1),
            ($1, 'STAMINA', 1),
            ($1, 'CHARISMA', 1),
            ($1, 'MANIPULATION', 1),
            ($1, 'APPEARANCE', 1),
            ($1, 'PERCEPTION', 1),
            ($1, 'INTELLIGENCE', 1),
            ($1, 'WITS', 1)
        ",
        character_id
    )
    .execute(&mut *transaction)
    .await?;

    // Insert abilities
    query!(
        "
        INSERT INTO abilities(character_id, name, dots)
        VALUES
            ($1, 'ARCHERY', 0),
            ($1, 'ATHLETICS', 0),
            ($1, 'AWARENESS', 0),
            ($1, 'BRAWL', 0),
            ($1, 'BUREAUCRACY', 0),
            ($1, 'DODGE', 0),
            ($1, 'INTEGRITY', 0),
            ($1, 'INVESTIGATION', 0),
            ($1, 'LARCENY', 0),
            ($1, 'LINGUISTICS', 0),
            ($1, 'LORE', 0),
            ($1, 'MEDICINE', 0),
            ($1, 'MELEE', 0),
            ($1, 'OCCULT', 0),
            ($1, 'PERFORMANCE', 0),
            ($1, 'PRESENCE', 0),
            ($1, 'RESISTANCE', 0),
            ($1, 'RIDE', 0),
            ($1, 'SAIL', 0),
            ($1, 'SOCIALIZE', 0),
            ($1, 'STEALTH', 0),
            ($1, 'SURVIVAL', 0),
            ($1, 'THROWN', 0),
            ($1, 'WAR', 0)
        ",
        character_id
    )
    .execute(&mut *transaction)
    .await?;

    // Add health boxes
    query!(
        "
        INSERT INTO health_boxes(character_id, position, wound_penalty)
        VALUES
            ($1, 0, 'ZERO'),
            ($1, 0, 'MINUSONE'),
            ($1, 0, 'MINUSONE'),
            ($1, 0, 'MINUSTWO'),
            ($1, 0, 'MINUSTWO'),
            ($1, 0, 'MINUSFOUR'),
            ($1, 0, 'INCAPACITATED')
        ",
        character_id
    )
    .execute(&mut *transaction)
    .await?;

    // Get the character that was just inserted
    if let Some(character) = retrieve_character_transaction(transaction, character_id).await? {
        Ok(character)
    } else {
        Err(eyre!(
            "could not retrieve inserted character with id {}",
            character_id
        ))
    }
}
