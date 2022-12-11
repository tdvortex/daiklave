mod fixtures;

#[sqlx::test]
fn lifecycle() {
    fixtures::lifecycle().await;
}
