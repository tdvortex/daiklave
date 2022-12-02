use exalted_3e_gui::database::GetCharacter;

#[sqlx::test]
fn create_get_delete() {
    dotenvy::dotenv().unwrap();

    let url = dotenvy::var("DATABASE_URL").unwrap();

    let pool = sqlx::pool::PoolOptions::<sqlx::Postgres>::new()
        .connect(&url)
        .await
        .unwrap();

    let insert_campaign = sqlx::query!(
        "INSERT INTO campaigns(name, bot_channel) VALUES ('test_campaign', 123456789) RETURNING id"
    )
    .fetch_one(&pool);

    let insert_player =
        sqlx::query!("INSERT INTO players(name) VALUES ('test_player') RETURNING id")
            .fetch_one(&pool);

    let (campaign, player) = tokio::try_join!(insert_campaign, insert_player).unwrap();
    let campaign_id = campaign.id;
    println!("campaign_id {}", campaign_id);
    let player_id = player.id;
    println!("player_id {}", player_id);

    let campaign_player_id = sqlx::query!(
        "INSERT INTO campaign_players(campaign_id, player_id) VALUES($1, $2) RETURNING id",
        campaign_id,
        player_id
    )
    .fetch_one(&pool)
    .await
    .unwrap()
    .id;

    println!("campaign_player_id {}", campaign_player_id);

    let character_id = sqlx::query!(
        "INSERT INTO characters(campaign_player_id, name, current_willpower,
             max_willpower, current_experience, total_experience)
        VALUES($1, 'test_character', 1, 2, 3, 4)
        RETURNING id",
        campaign_player_id
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
    //dbg!(sqlx::query_as!(AttributeRow, r#"SELECT character_id, name as "name!: AttributeName", dots FROM attributes"#).fetch_all(&pool).await.unwrap());

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

    let get_character = GetCharacter::execute(&pool, character_id)
        .await
        .unwrap()
        .unwrap();
    dbg!(get_character);

    // Clean up by deleting campaign and player
    // Should cascade to delete everything else
    sqlx::query!("DELETE FROM campaigns WHERE id = $1", campaign_id)
        .execute(&pool)
        .await
        .unwrap();

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
