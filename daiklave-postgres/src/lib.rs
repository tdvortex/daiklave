use abilities::{apply_abilities_and_specialties_rows, update_abilities, AbilityRow, SpecialtyRow};
use armor::{apply_armor_rows, update_armor, ArmorRow, ArmorTagRow, ArmorWornRow};
use attributes::{apply_attribute_rows, update_attributes, AttributeRow};
use campaign::{apply_campaign_row, CampaignRow};
use character::{apply_character_row, update_base_character, CharacterRow};
use craft::{apply_craft, update_craft, CraftAbilityRow, CraftAbilitySpecialtyRow};
use daiklave_core::Character;
use eyre::{eyre, Result, WrapErr};
use health::{apply_health_box_rows, update_health, HealthBoxRow};
use intimacies::{apply_intimacy_rows, update_intimacies, IntimacyRow};
use martial_arts::{
    apply_martial_arts, update_martial_arts, AllMartialArtsRows, CharacterMartialArtsRow,
    CharacterMartialArtsSpecialtyRow, MartialArtsCharmCostRow, MartialArtsCharmKeywordRow,
    MartialArtsCharmRow, MartialArtsCharmTreeRow, MartialArtsStyleRow,
};
use merits::{
    apply_merits_rows, update_merits, MeritDetailRow, MeritPrerequisiteSetRow, MeritTemplateRow,
    PrerequisiteRow,
};
use player::PlayerRow;
use sqlx::{query, PgPool, Postgres, Transaction};
use weapons::{apply_weapon_rows, update_weapons, WeaponEquippedRow, WeaponRow, WeaponTagRow};
mod abilities;
mod armor;
mod attributes;
mod campaign;
mod character;
mod craft;
mod health;
mod intimacies;
mod martial_arts;
mod merits;
mod player;
mod weapons;

pub async fn destroy_character(pool: &PgPool, id: i32) -> Result<()> {
    query!(
        "DELETE FROM characters
        WHERE id = $1",
        id as i32
    )
    .execute(pool)
    .await
    .wrap_err_with(|| format!("Database error deleting character {}", id))?;

    Ok(())
}

#[derive(Debug)]
struct GetCharacter {
    character: CharacterRow,
    player: PlayerRow,
    campaign: Option<CampaignRow>,
    attributes: Vec<AttributeRow>,
    abilities: Vec<AbilityRow>,
    specialties: Option<Vec<SpecialtyRow>>,
    intimacies: Option<Vec<IntimacyRow>>,
    health_boxes: Vec<HealthBoxRow>,
    weapons_owned: Option<Vec<WeaponRow>>,
    weapon_tags: Option<Vec<WeaponTagRow>>,
    weapons_equipped: Option<Vec<WeaponEquippedRow>>,
    armor_owned: Option<Vec<ArmorRow>>,
    armor_tags: Option<Vec<ArmorTagRow>>,
    armor_worn: Option<Vec<ArmorWornRow>>,
    merit_templates: Option<Vec<MeritTemplateRow>>,
    merit_details: Option<Vec<MeritDetailRow>>,
    merit_prerequisite_sets: Option<Vec<MeritPrerequisiteSetRow>>,
    merit_prerequisites: Option<Vec<PrerequisiteRow>>,
    martial_arts_styles: Option<Vec<MartialArtsStyleRow>>,
    character_martial_arts_styles: Option<Vec<CharacterMartialArtsRow>>,
    martial_arts_specialties: Option<Vec<CharacterMartialArtsSpecialtyRow>>,
    martial_arts_charms: Option<Vec<MartialArtsCharmRow>>,
    martial_arts_charm_keywords: Option<Vec<MartialArtsCharmKeywordRow>>,
    martial_arts_charms_costs: Option<Vec<MartialArtsCharmCostRow>>,
    martial_arts_charm_tree: Option<Vec<MartialArtsCharmTreeRow>>,
    craft_abilities: Option<Vec<CraftAbilityRow>>,
    craft_specialties: Option<Vec<CraftAbilitySpecialtyRow>>,
}

pub async fn retrieve_character(pool: &PgPool, character_id: i32) -> Result<Option<Character>> {
    let mut transaction = pool
        .begin()
        .await
        .wrap_err("Error attempting to start transaction")?;

    let maybe_character = retrieve_character_transaction(&mut transaction, character_id).await?;

    transaction
        .commit()
        .await
        .wrap_err("Error attempting to commit transaction")?;
    Ok(maybe_character)
}

async fn retrieve_character_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<Option<Character>> {
    let maybe_get_character = sqlx::query_file_as!(GetCharacter, "src/retrieve.sql", character_id)
        .fetch_optional(&mut *transaction)
        .await
        .wrap_err_with(|| {
            format!(
                "Database error trying to retrieve character with id: {}",
                character_id
            )
        })?;

    if let Some(get_character) = maybe_get_character {
        Ok(Some(get_character.try_into().wrap_err_with(|| {
            format!(
                "Error attempting to convert database output to Character for character_id {}",
                character_id
            )
        })?))
    } else {
        // Valid query with no character
        Ok(None)
    }
}

impl TryInto<Character> for GetCharacter {
    type Error = eyre::Report;

    fn try_into(self) -> Result<Character, Self::Error> {
        let mut builder = Character::builder(self.character.id, self.player.into());
        builder = apply_campaign_row(builder, self.campaign);
        builder = apply_character_row(builder, self.character)
            .wrap_err("Could not apply character row")?;
        builder = apply_attribute_rows(builder, self.attributes)
            .wrap_err("Could not apply attribute rows")?;
        builder = apply_abilities_and_specialties_rows(builder, self.abilities, self.specialties)
            .wrap_err("Could not apply ability and specialty rows")?;
        builder = apply_craft(builder, self.craft_abilities, self.craft_specialties)
            .wrap_err("Could not apply craft rows")?;
        builder = apply_intimacy_rows(builder, self.intimacies);
        builder = apply_health_box_rows(builder, self.health_boxes);
        builder = apply_weapon_rows(
            builder,
            self.weapons_owned,
            self.weapon_tags,
            self.weapons_equipped,
        )
        .wrap_err("Could not apply weapon rows")?;
        builder = apply_armor_rows(builder, self.armor_owned, self.armor_tags, self.armor_worn)
            .wrap_err("Could not apply armor rows")?;
        builder = apply_merits_rows(
            builder,
            self.merit_templates,
            self.merit_details,
            self.merit_prerequisite_sets,
            self.merit_prerequisites,
        )
        .wrap_err("Could not apply merit rows")?;
        builder = apply_martial_arts(
            builder,
            AllMartialArtsRows {
                style_rows: self.martial_arts_styles,
                character_style_rows: self.character_martial_arts_styles,
                specialty_rows: self.martial_arts_specialties,
                martial_arts_charm_rows: self.martial_arts_charms,
                charm_keyword_rows: self.martial_arts_charm_keywords,
                charm_cost_rows: self.martial_arts_charms_costs,
                charm_tree_rows: self.martial_arts_charm_tree,
            },
        )
        .wrap_err("Could not apply martial arts rows")?;

        builder.build()
    }
}

async fn create_character_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: i32,
) -> Result<Character> {
    // Insert character placeholder and get an ID
    let character_id = query!(
        "
        INSERT INTO characters(player_id, name, current_willpower, max_willpower, current_experience, total_experience)
        VALUES($1, 'New Character', 0, 0, 0, 0)
        RETURNING id
        ",
        player_id
    ).fetch_one(&mut *transaction).await.wrap_err_with(|| format!("Initial character insert failed for player id {}", player_id))?.id;

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
    .await
    .wrap_err_with(|| {
        format!(
            "New attributes insert failed for character_id {}",
            character_id
        )
    })?;

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
    .await
    .wrap_err_with(|| {
        format!(
            "New abilities insert failed for character_id {}",
            character_id
        )
    })?;

    // Add health boxes
    query!(
        "
        INSERT INTO health_boxes(character_id, position, wound_penalty)
        VALUES
            ($1, 0, 'ZERO'),
            ($1, 1, 'MINUSONE'),
            ($1, 2, 'MINUSONE'),
            ($1, 3, 'MINUSTWO'),
            ($1, 4, 'MINUSTWO'),
            ($1, 5, 'MINUSFOUR'),
            ($1, 6, 'INCAPACITATED')
        ",
        character_id
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| {
        format!(
            "New health boxes insert failed for character_id {}",
            character_id
        )
    })?;

    // Get the character that was just inserted
    retrieve_character_transaction(transaction, character_id)
        .await
        .wrap_err_with(|| {
            format!(
                "Database error retrieving new inserted character with id {}",
                character_id
            )
        })?
        .ok_or_else(|| {
            eyre!(
                "No results returned retrieving inserted character with id {}",
                character_id
            )
        })
}

pub async fn update_character(pool: &PgPool, character: &Character) -> Result<Character> {
    let mut transaction = pool.begin().await.wrap_err("Failed to start transaction")?;

    let old_character = if character.id().is_placeholder() {
        create_character_transaction(&mut transaction, *character.player().id())
            .await
            .wrap_err_with(|| {
                format!("Failed to create initial character from: {:#?}", character)
            })?
    } else {
        retrieve_character_transaction(&mut transaction, *character.id())
            .await
            .wrap_err_with(|| {
                format!(
                    "Database error on retrieving pre-update character_id: {}",
                    *character.id()
                )
            })?
            .ok_or_else(|| eyre!("No character found with id {}", *character.id()))?
    };

    let character_id = if old_character.id().is_placeholder() {
        return Err(eyre!(
            "Missing character id for character with name {}",
            old_character.name
        ));
    } else {
        *old_character.id()
    };

    let diff = old_character.compare_newer(&character);

    update_abilities(diff.abilities_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating abilities")?;
    update_craft(diff.craft_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating craft abilities")?;
    update_attributes(diff.attributes_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating attributes")?;
    update_base_character(diff.base_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating base character")?;
    update_health(diff.health_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating health")?;
    update_intimacies(diff.intimacies_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating intimacies")?;
    update_weapons(diff.weapons_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating weapons")?;
    update_armor(diff.armor_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating armor")?;
    update_merits(diff.merits_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating merits")?;
    update_martial_arts(diff.martial_arts_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating martial arts")?;

    let character = retrieve_character_transaction(&mut transaction, character_id)
        .await
        .wrap_err_with(|| {
            format!(
                "Database error on retrieving post-update character_id: {}",
                character_id
            )
        })?
        .ok_or_else(|| {
            eyre!(
                "Could not retrieve post-update character with id {}",
                character_id
            )
        })?;

    transaction
        .commit()
        .await
        .wrap_err("Error trying to commit character update transaction")?;

    Ok(character)
}
