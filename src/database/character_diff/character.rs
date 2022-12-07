use eyre::Result;
use sqlx::{query, Postgres, Transaction};

use crate::character::Character;

#[derive(Debug, Default)]
pub struct CharacterBaseDiff(Option<(String, Option<String>, i16, i16, i16, i16)>);

impl Character {
    pub fn compare_newer(&self, newer: &Character) -> CharacterBaseDiff {
        let mut diff = CharacterBaseDiff::default();

        if self.name != newer.name {
            diff = CharacterBaseDiff(Some((
                newer.name.to_owned(),
                newer.concept.as_ref().map(String::to_owned),
                newer.willpower.current.into(),
                newer.willpower.maximum.into(),
                // No one should ever have more than 32,767 experience
                newer
                    .experience
                    .current
                    .min(i16::MAX as u16)
                    .try_into()
                    .unwrap(),
                newer
                    .experience
                    .total
                    .min(i16::MAX as u16)
                    .try_into()
                    .unwrap(),
            )))
        }

        diff
    }
}

impl CharacterBaseDiff {
    pub async fn save(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        if self.0.is_none() {
            return Ok(());
        }

        let (
            name,
            maybe_concept,
            current_willpower,
            maximum_willpower,
            current_experience,
            total_experience,
        ) = self.0.as_ref().unwrap();

        query!("
            UPDATE characters
            SET name = $2, concept = $3, current_willpower = $4, max_willpower = $5, current_experience = $6, total_experience = $7
            WHERE id = $1",
            character_id, name, maybe_concept.as_deref(), current_willpower, maximum_willpower, current_experience, total_experience
        ).execute(&mut *transaction).await?;

        Ok(())
    }
}
