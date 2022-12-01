#[sqlx::test]
async fn test_workaround() {
    dotenvy::dotenv().unwrap();

    let url = dotenvy::var("DATABASE_URL").unwrap();

    let pool = sqlx::pool::PoolOptions::<sqlx::Postgres>::new().connect(&url).await.unwrap();

    let insert_campaign = sqlx::query!(
        "INSERT INTO campaigns(name, bot_channel) VALUES ('test_campaign', 123456789) RETURNING id"
    ).fetch_one(&pool);

    let insert_player = sqlx::query!(
        "INSERT INTO players(name) VALUES ('test_player') RETURNING id"
    ).fetch_one(&pool);
    
    let (campaign, player) = tokio::try_join!(insert_campaign, insert_player).unwrap();

    sqlx::query!(
        "INSERT INTO campaign_players(campaign_id, player_id) VALUES($1, $2)",
        campaign.id.clone(),
        player.id.clone()
    ).execute(&pool).await.unwrap();

    let result = exalted_3e_gui::database::workaround(&pool, player.id).await.unwrap();
    dbg!(result);

    let mut transaction = pool.begin().await.unwrap();

    sqlx::query!(
        r#"
        DELETE FROM campaigns WHERE campaigns.name = 'test_campaign'
        "#
    ).execute(&mut transaction).await.unwrap();

    sqlx::query!(
        r#"
        DELETE FROM campaigns WHERE campaigns.name = 'test_player'
        "#
    ).execute(&mut transaction).await.unwrap();

    transaction.commit().await.unwrap();
} 