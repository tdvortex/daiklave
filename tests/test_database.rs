use exalted_3e_gui::database::GetCharacter;

#[sqlx::test]
fn create_get_delete() {
    dotenvy::dotenv().unwrap();

    let url = dotenvy::var("DATABASE_URL").unwrap();

    let pool = sqlx::pool::PoolOptions::<sqlx::Postgres>::new()
        .connect(&url)
        .await
        .unwrap();

    let player_id = sqlx::query!("INSERT INTO players(name) VALUES ('test_player') RETURNING id")
        .fetch_one(&pool)
        .await
        .unwrap()
        .id;

    println!("player_id {}", player_id);

    let character_id = sqlx::query!(
        "INSERT INTO characters(player_id, name, current_willpower,
             max_willpower, current_experience, total_experience)
        VALUES($1, 'test_character', 1, 2, 3, 4)
        RETURNING id",
        player_id
    )
    .fetch_one(&pool)
    .await
    .unwrap()
    .id;

    println!("character_id {}", character_id);

    let rows_affected = sqlx::query!(
        "INSERT INTO attributes(character_id, name, dots) VALUES
            ($1, 'STRENGTH', 1),
            ($1, 'DEXTERITY', 1),
            ($1, 'STAMINA', 1),
            ($1, 'CHARISMA', 1),
            ($1, 'MANIPULATION', 1),
            ($1, 'APPEARANCE', 1),
            ($1, 'PERCEPTION', 1),
            ($1, 'INTELLIGENCE', 1),
            ($1, 'WITS', 1)",
        character_id
    )
    .execute(&pool)
    .await
    .unwrap()
    .rows_affected();

    assert!(rows_affected == 9);

    let rows_affected = sqlx::query!(
        "INSERT INTO abilities(character_id, name, dots) VALUES
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
    .execute(&pool)
    .await
    .unwrap()
    .rows_affected();

    assert!(rows_affected == 24);
    //dbg!(sqlx::query_as!(AbilityRow, r#"SELECT id, character_id, name as "name!: AbilityName", dots, subskill FROM abilities"#).fetch_all(&pool).await.unwrap());

    let rows_affected = sqlx::query!(
        "INSERT INTO health_boxes(character_id, position, wound_penalty) VALUES
            ($1, 1, 'ZERO'),
            ($1, 2, 'MINUSONE'),
            ($1, 3, 'MINUSONE'),
            ($1, 4, 'MINUSTWO'),
            ($1, 5, 'MINUSTWO'),
            ($1, 6, 'MINUSFOUR'),
            ($1, 7, 'INCAPACITATED')
            ",
        character_id
    )
    .execute(&pool)
    .await
    .unwrap()
    .rows_affected();

    assert!(rows_affected == 7);

    let maybe_unarmed_id =
        sqlx::query!("SELECT id FROM weapons WHERE weapons.name = 'Unarmed' LIMIT 1")
            .fetch_optional(&pool)
            .await
            .unwrap()
            .map(|rec| rec.id);

    let unarmed_id = if let Some(id) = maybe_unarmed_id {
        id
    } else {
        sqlx::query!(
            "INSERT INTO weapons(name, tags)
            VALUES ('Unarmed', ARRAY[
                ('BASHING', NULL, NULL),
                ('BRAWL', NULL, NULL),
                ('GRAPPLING', NULL, NULL),
                ('LIGHT', NULL, NULL),
                ('MARTIALARTS', NULL, 'Air Dragon Style'),
                ('MARTIALARTS', NULL, 'Black Claw Style'),
                ('MARTIALARTS', NULL, 'Centipede Style'),
                ('MARTIALARTS', NULL, 'Crane Style'),
                ('MARTIALARTS', NULL, 'Earth Dragon Style'),
                ('MARTIALARTS', NULL, 'Falcon Style'),
                ('MARTIALARTS', NULL, 'Fire Dragon Style'),
                ('MARTIALARTS', NULL, 'Laughing Monster Style'),
                ('MARTIALARTS', NULL, 'Snake Style'),
                ('MARTIALARTS', NULL, 'Swaying Grass Dance Style'),
                ('MARTIALARTS', NULL, 'Tiger Style'),
                ('MARTIALARTS', NULL, 'Water Dragon Style'),
                ('MARTIALARTS', NULL, 'White Reaper Style'),
                ('MARTIALARTS', NULL, 'Wood Dragon Style'),
                ('ONEHANDED', NULL, NULL),
                ('NATURAL', NULL, NULL)
            ]::WEAPONTAG[])
            RETURNING id"
        )
        .fetch_one(&pool)
        .await
        .unwrap()
        .id
    };

    let rows_affected = sqlx::query!(
        "INSERT INTO character_weapons(character_id, weapon_id) VALUES
            ($1, $2)
            ",
        character_id,
        unarmed_id
    )
    .execute(&pool)
    .await
    .unwrap()
    .rows_affected();

    assert!(rows_affected == 1);

    let get_character = GetCharacter::execute(&pool, character_id)
        .await
        .unwrap()
        .unwrap();
    dbg!(get_character);

    // Clean up by deleting player
    // Should cascade to delete everything belonging to this player and character
    sqlx::query!("DELETE FROM players WHERE id = $1", player_id)
        .execute(&pool)
        .await
        .unwrap();

    // Confirm delete
    assert!(GetCharacter::execute(&pool, character_id)
        .await
        .unwrap()
        .is_none());
}
