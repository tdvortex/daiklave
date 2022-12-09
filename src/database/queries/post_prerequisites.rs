use eyre::Result;
use sqlx::{query, Postgres, Transaction};

use crate::database::tables::{
    abilities::AbilityNamePostgres,
    attributes::AttributeNamePostgres,
    prerequisites::{PrerequisiteExaltTypePostgres, PrerequisiteInsert, PrerequisiteTypePostgres},
};

async fn _post_prerequisites_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    prerequisites: &[PrerequisiteInsert],
) -> Result<Vec<i32>> {
    let (
        prerequisite_types,
        ability_names,
        subskill_names,
        attribute_names,
        dots_vec,
        prerequisite_charm_ids,
        prerequisite_exalt_types,
    ) = prerequisites.iter().fold(
        (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        |(
            mut prerequisite_types,
            mut ability_names,
            mut subskill_names,
            mut attribute_names,
            mut dots_vec,
            mut prerequisite_charm_ids,
            mut prerequisite_exalt_types,
        ),
         prerequisite_insert| {
            prerequisite_types.push(prerequisite_insert.prerequisite_type);
            ability_names.push(prerequisite_insert.ability_name);
            subskill_names.push(prerequisite_insert.subskill_name.as_deref());
            attribute_names.push(prerequisite_insert.attribute_name);
            dots_vec.push(prerequisite_insert.dots);
            prerequisite_charm_ids.push(prerequisite_insert.charm_id);
            prerequisite_exalt_types.push(prerequisite_insert.exalt_type);
            (
                prerequisite_types,
                ability_names,
                subskill_names,
                attribute_names,
                dots_vec,
                prerequisite_charm_ids,
                prerequisite_exalt_types,
            )
        },
    );

    Ok(query!(
        "INSERT INTO prerequisites(prerequisite_type, ability_name, subskill_name, attribute_name, dots, charm_id, prerequisite_exalt_type)
        SELECT
            data.prerequisite_type,
            data.ability_name,
            data.subskill_name,
            data.attribute_name,
            data.dots,
            data.charm_id,
            data.prerequisite_exalt_type
        FROM UNNEST($1::PREREQUISITETYPE[], $2::ABILITYNAME[], $3::VARCHAR(255)[], $4::ATTRIBUTENAME[], $5::SMALLINT[], $6::INTEGER[], $7::PREREQUISITEEXALTTYPE[])
            AS data(prerequisite_type, ability_name, subskill_name, attribute_name, dots, charm_id, prerequisite_exalt_type)
        RETURNING id",
        &prerequisite_types as &[PrerequisiteTypePostgres],
        &ability_names as &[Option<AbilityNamePostgres>],
        &subskill_names as &[Option<&str>],
        &attribute_names as &[Option<AttributeNamePostgres>],
        &dots_vec as &[Option<i16>],
        &prerequisite_charm_ids as &[Option<i32>],
        &prerequisite_exalt_types as &[Option<PrerequisiteExaltTypePostgres>]
    ).fetch_all(&mut *transaction).await?.into_iter().map(|record| record.id).collect())
}
