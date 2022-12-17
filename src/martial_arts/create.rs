use super::MartialArtsStyle;
use eyre::{Context, Result};
use sqlx::{query, Postgres, Transaction};

pub(crate) async fn create_martial_arts_style_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    style: MartialArtsStyle,
    creator_id: Option<i32>,
) -> Result<i32> {
    query!(
        "INSERT INTO martial_arts_styles(name, description, book_title, page_number, creator_id)
        VALUES($1::VARCHAR(255), $2::TEXT, $3::VARCHAR(255), $4::SMALLINT, $5::INTEGER)
        RETURNING id
        ",
        style.name(),
        style.description.as_str(),
        style.data_source().book_title(),
        style.data_source().page_number(),
        creator_id
    ).fetch_one(&mut *transaction).await.wrap_err_with(|| format!("Database error attempting to insert martial arts style {}", style.name())).map(|record| record.id)
}

pub(crate) async fn create_martial_arts_styles_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    styles: &Vec<MartialArtsStyle>,
    creator_id: Option<i32>,
) -> Result<Vec<i32>> {
    let (names, descriptions, book_titles, page_numbers, creator_ids) = styles.iter().fold(
        (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()),
        |(mut names, mut descriptions, mut book_titles, mut page_numbers, mut creator_ids),
         style| {
            names.push(style.name());
            descriptions.push(style.description.as_str());
            book_titles.push(style.data_source().book_title());
            page_numbers.push(style.data_source().page_number());
            creator_ids.push(creator_id);
            (names, descriptions, book_titles, page_numbers, creator_ids)
        },
    );

    query!(
        "INSERT INTO martial_arts_styles(name, description, book_title, page_number, creator_id)
        SELECT
            data.name as name,
            data.description as description,
            data.book_title as book_title,
            data.page_number as page_number,
            data.creator_id as creator_id
        FROM UNNEST($1::VARCHAR(255)[], $2::TEXT[], $3::VARCHAR(255)[], $4::SMALLINT[], $5::INTEGER[]) AS data(name, description, book_title, page_number, creator_id)
        RETURNING id",
        &names as &[&str],
        &descriptions as &[&str],
        &book_titles as &[Option<&str>],
        &page_numbers as &[Option<i16>],
        &creator_ids as &[Option<i32>]
    ).fetch_all(&mut *transaction).await.wrap_err("Database error inserting martial arts styles").map(|records| records.into_iter().map(|record| record.id).collect())
}
