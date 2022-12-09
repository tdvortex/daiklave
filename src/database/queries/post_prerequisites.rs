use eyre::Result;
use sqlx::{query, Postgres, Transaction};

use crate::database::tables::{
    abilities::AbilityNamePostgres,
    attributes::AttributeNamePostgres,
    prerequisites::{PrerequisiteExaltTypePostgres, PrerequisiteInsert, PrerequisiteTypePostgres},
};

pub async fn post_prerequisites_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    prerequisites: &[PrerequisiteInsert],
) -> Result<Vec<i32>> {
    let (
        merit_prerequisite_ids,
        charm_prerequisite_ids,
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
            Vec::new(),
            Vec::new(),
        ),
        |(
            mut merit_prerequisite_ids,
            mut charm_prerequisite_ids,
            mut prerequisite_types,
            mut ability_names,
            mut subskill_names,
            mut attribute_names,
            mut dots_vec,
            mut prerequisite_charm_ids,
            mut prerequisite_exalt_types,
        ),
         prerequisite_insert| {
            merit_prerequisite_ids.push(prerequisite_insert.merit_prerequisite_set_id);
            charm_prerequisite_ids.push(prerequisite_insert.charm_prerequisite_set_id);
            prerequisite_types.push(prerequisite_insert.prerequisite_type);
            ability_names.push(prerequisite_insert.ability_name);
            subskill_names.push(prerequisite_insert.subskill_name.as_deref());
            attribute_names.push(prerequisite_insert.attribute_name);
            dots_vec.push(prerequisite_insert.dots);
            prerequisite_charm_ids.push(prerequisite_insert.charm_id);
            prerequisite_exalt_types.push(prerequisite_insert.exalt_type);
            (
                merit_prerequisite_ids,
                charm_prerequisite_ids,
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
        "INSERT INTO prerequisites(merit_prerequisite_set_id, charm_prerequisite_set_id, prerequisite_type, ability_name, subskill_name, attribute_name, dots, charm_id, prerequisite_exalt_type)
        SELECT
            data.merit_prerequisite_set_id, 
            data.charm_prerequisite_set_id, 
            data.prerequisite_type,
            data.ability_name,
            data.subskill_name,
            data.attribute_name,
            data.dots,
            data.charm_id,
            data.prerequisite_exalt_type
        FROM UNNEST($1::INTEGER[], $2::INTEGER[], $3::PREREQUISITETYPE[], $4::ABILITYNAME[], $5::VARCHAR(255)[], $6::ATTRIBUTENAME[], $7::SMALLINT[], $8::INTEGER[], $9::PREREQUISITEEXALTTYPE[])
            AS data(merit_prerequisite_set_id, charm_prerequisite_set_id, prerequisite_type, ability_name, subskill_name, attribute_name, dots, charm_id, prerequisite_exalt_type)
        RETURNING id",
        &merit_prerequisite_ids as &[Option<i32>],
        &charm_prerequisite_ids as &[Option<i32>],
        &prerequisite_types as &[PrerequisiteTypePostgres],
        &ability_names as &[Option<AbilityNamePostgres>],
        &subskill_names as &[Option<&str>],
        &attribute_names as &[Option<AttributeNamePostgres>],
        &dots_vec as &[Option<i16>],
        &prerequisite_charm_ids as &[Option<i32>],
        &prerequisite_exalt_types as &[Option<PrerequisiteExaltTypePostgres>]
    ).fetch_all(&mut *transaction).await?.into_iter().map(|record| record.id).collect())
}
