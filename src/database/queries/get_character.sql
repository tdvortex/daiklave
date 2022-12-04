WITH player_query AS (
    SELECT
        players as player
    FROM characters INNER JOIN players ON (characters.player_id = players.id)
    WHERE characters.id = $1
), campaign_query AS (
    SELECT
        campaigns as campaign
    FROM characters INNER JOIN campaigns ON (characters.campaign_id = campaigns.id)
    WHERE characters.id = $1
), attributes_query AS (
    SELECT
        ARRAY_AGG(attributes) AS attrs
    FROM characters
        INNER JOIN attributes ON (attributes.character_id = characters.id)
    WHERE characters.id = $1
), abilities_query AS (
    SELECT
        ARRAY_AGG(abilities) AS abils
    FROM characters
        INNER JOIN abilities ON (abilities.character_id = characters.id)
    WHERE characters.id = $1
), specialties_query AS (
    SELECT
        ARRAY_AGG(specialties) AS specs
    FROM characters
        INNER JOIN abilities ON (abilities.character_id = characters.id)
        INNER JOIN specialties ON (specialties.ability_id = abilities.id)
    WHERE characters.id = $1
), intimacies_query AS (
    SELECT
        ARRAY_AGG(intimacies) AS intis
    FROM characters
        INNER JOIN intimacies ON (intimacies.character_id = characters.id)
    WHERE characters.id = $1
), health_boxes_query AS (
    SELECT
        ARRAY_AGG(health_boxes) AS hboxs
    FROM characters
        INNER JOIN health_boxes ON (health_boxes.character_id = characters.id)
    WHERE characters.id = $1
), weapons_query AS (
     SELECT
        ARRAY_AGG(weapons) as weaps
    FROM characters
        INNER JOIN character_weapons ON (characters.id = character_weapons.character_id)
        INNER JOIN weapons ON (character_weapons.weapon_id = weapons.id)
    WHERE characters.id = $1
), weapons_equipped_query AS (
    SELECT
        ARRAY_AGG(character_weapons) AS eqwps
    FROM characters
        INNER JOIN character_weapons ON (character_weapons.character_id = characters.id)
    WHERE characters.id = $1 AND character_weapons.equip_hand IS NOT NULL
), armor_query AS (
     SELECT
        ARRAY_AGG(armor) as armrs
    FROM characters
        INNER JOIN character_armor ON (characters.id = character_armor.character_id)
        INNER JOIN armor ON (character_armor.armor_id = armor.id)
    WHERE characters.id = $1
), armor_worn_query AS (
     SELECT
        ARRAY_AGG(character_armor) as wrars
    FROM characters
        INNER JOIN character_armor ON (characters.id = character_armor.character_id)
    WHERE characters.id = $1 AND character_armor.worn
), merit_templates_query AS (
    SELECT
        ARRAY_AGG(merits) AS mrtts
    FROM characters 
        INNER JOIN character_merits ON (characters.id = character_merits.character_id)
        INNER JOIN merits ON (merits.id = character_merits.merit_id)
    WHERE characters.id = $1
), merit_details_query AS (
    SELECT
        ARRAY_AGG(character_merits) AS mrtds
    FROM characters 
        INNER JOIN character_merits ON (characters.id = character_merits.character_id)
    WHERE characters.id = $1 AND character_merits.detail IS NOT NULL
), merit_prerequisite_sets_query AS (
    SELECT
        ARRAY_AGG(merit_prerequisite_sets) AS mprss
    FROM characters
        INNER JOIN character_merits ON (characters.id = character_merits.character_id)
        INNER JOIN merits ON (merits.id = character_merits.merit_id)
        INNER JOIN merit_prerequisite_sets ON (merit_prerequisite_sets.merit_id = merits.id)
    WHERE characters.id = $1
), merit_prerequisites_query AS (
    SELECT
        ARRAY_AGG(prerequisites) AS meprs
    FROM characters
        INNER JOIN character_merits ON (characters.id = character_merits.character_id)
        INNER JOIN merits ON (merits.id = character_merits.merit_id)
        INNER JOIN merit_prerequisite_sets ON (merit_prerequisite_sets.merit_id = merits.id)
        INNER JOIN prerequisites ON (merit_prerequisite_sets.prerequisite_id = prerequisites.id)
    WHERE characters.id = $1
)
SELECT
    characters AS "character!: CharacterRow",
    player AS "player!: PlayerRow",
    attrs AS "attributes!: Vec<AttributeRow>",
    abils AS "abilities!: Vec<AbilityRow>",
    hboxs AS "health_boxes!: Vec<HealthBoxRow>",
    weaps AS "weapons_owned!: Vec<WeaponRow>",
    campaign AS "campaign: CampaignRow",
    specs AS "specialties: Vec<SpecialtyRow>",
    intis AS "intimacies: Vec<IntimacyRow>",
    eqwps AS "weapons_equipped: Vec<WeaponEquippedRow>",
    armrs AS "armor_owned: Vec<ArmorRow>",
    wrars AS "armor_worn: Vec<ArmorWornRow>",
    mrtts AS "merit_templates: Vec<MeritTemplateRow>",
    mrtds AS "merit_details: Vec<MeritDetailRow>",
    mprss AS "merit_prerequisite_sets: Vec<MeritPrerequisiteSetRow>",
    meprs AS "merit_prerequisites:  Vec<PrerequisiteRow>"
FROM characters,
    player_query,
    attributes_query,
    abilities_query,
    health_boxes_query,
    weapons_query
    LEFT JOIN campaign_query ON (TRUE)
    LEFT JOIN specialties_query ON (TRUE)
    LEFT JOIN intimacies_query ON (TRUE)
    LEFT JOIN weapons_equipped_query ON (TRUE)
    LEFT JOIN armor_query ON (TRUE)
    LEFT JOIN armor_worn_query ON (TRUE)
    LEFT JOIN merit_templates_query ON (TRUE)
    LEFT JOIN merit_details_query ON (TRUE)
    LEFT JOIN merit_prerequisite_sets_query ON (TRUE)
    LEFT JOIN merit_prerequisites_query ON (TRUE)
WHERE characters.id = $1;
