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
    FROM attributes
    WHERE attributes.character_id = $1
), abilities_query AS (
    SELECT
        ARRAY_AGG(abilities) AS abils
    FROM abilities
    WHERE abilities.character_id = $1
), specialties_query AS (
    SELECT
        ARRAY_AGG(specialties) AS specs
    FROM specialties
    WHERE specialties.character_id = $1
), intimacies_query AS (
    SELECT
        ARRAY_AGG(intimacies) AS intis
    FROM intimacies
    WHERE intimacies.character_id = $1
), health_boxes_query AS (
    SELECT
        ARRAY_AGG(health_boxes) AS hboxs
    FROM health_boxes
    WHERE health_boxes.character_id = $1
), weapons_query AS (
     SELECT
        ARRAY_AGG(weapons) as weaps
    FROM character_weapons
        INNER JOIN weapons ON (character_weapons.weapon_id = weapons.id)
    WHERE character_weapons.character_id = $1
), weapon_tags_query AS (
     SELECT
        ARRAY_AGG(weapon_tags) as wepts
    FROM character_weapons
        INNER JOIN weapons ON (character_weapons.weapon_id = weapons.id)
        INNER JOIN weapon_tags ON (weapon_tags.weapon_id = weapons.id)
    WHERE character_weapons.character_id = $1
), weapons_equipped_query AS (
    SELECT
        ARRAY_AGG(character_weapons) AS eqwps
    FROM character_weapons
    WHERE character_weapons.character_id = $1
), armor_query AS (
     SELECT
        ARRAY_AGG(armor) as armrs
    FROM character_armor INNER JOIN armor ON (character_armor.armor_id = armor.id)
    WHERE character_armor.character_id = $1
), armor_tags_query AS (
    SELECT
        ARRAY_AGG(armor_tags) as armts
    FROM character_armor 
        INNER JOIN armor ON (character_armor.armor_id = armor.id)
        INNER JOIN armor_tags ON (armor_tags.armor_id = armor.id)
    WHERE character_armor.character_id = $1
), character_armor_query AS (
     SELECT
        ARRAY_AGG(character_armor) as wrars
    FROM character_armor
    WHERE character_armor.character_id = $1
), merit_templates_query AS (
    SELECT
        ARRAY_AGG(merits) AS mrtts
    FROM character_merits INNER JOIN merits ON (merits.id = character_merits.merit_id)
    WHERE character_merits.character_id = $1
), merit_details_query AS (
    SELECT
        ARRAY_AGG(character_merits) AS mrtds
    FROM character_merits
    WHERE character_merits.character_id = $1
), merit_prerequisite_sets_query AS (
    SELECT
        ARRAY_AGG(merit_prerequisite_sets) AS mprss
    FROM character_merits INNER JOIN merit_prerequisite_sets ON (merit_prerequisite_sets.merit_id = character_merits.merit_id)
    WHERE character_merits.character_id = $1
), merit_prerequisites_query AS (
    SELECT
        ARRAY_AGG(prerequisites) AS meprs
    FROM character_merits 
        INNER JOIN merit_prerequisite_sets ON (merit_prerequisite_sets.merit_id = character_merits.merit_id)
        INNER JOIN prerequisites ON (prerequisites.merit_prerequisite_set_id = merit_prerequisite_sets.id)
    WHERE character_merits.character_id = $1
), martial_arts_styles_query AS (
    SELECT
        ARRAY_AGG(martial_arts_styles) as masts
    FROM character_martial_arts INNER JOIN martial_arts_styles ON (character_martial_arts.style_id = martial_arts_styles.id)
    WHERE character_martial_arts.character_id = $1
), character_martial_arts_query AS (
    SELECT
        ARRAY_AGG(character_martial_arts) as chmas
    FROM character_martial_arts
    WHERE character_martial_arts.character_id = $1
), character_martial_arts_specialties AS (
    SELECT
        ARRAY_AGG(character_martial_arts_specialties) as cmass
    FROM character_martial_arts_specialties
    WHERE character_martial_arts_specialties.character_id = $1
), martial_arts_charms_query AS (
    SELECT
        ARRAY_AGG(martial_arts_charms) as machs
    FROM character_martial_arts_charms INNER JOIN martial_arts_charms ON (character_martial_arts_charms.charm_id = martial_arts_charms.id)
    WHERE character_martial_arts_charms.character_id = $1
), martial_arts_charms_keywords_query AS (
    SELECT
        ARRAY_AGG(martial_arts_charms_keywords) as makws
    FROM character_martial_arts_charms INNER JOIN martial_arts_charms_keywords ON (character_martial_arts_charms.charm_id = martial_arts_charms_keywords.charm_id)
    WHERE character_martial_arts_charms.character_id = $1
), martial_arts_charms_costs_query AS (
    SELECT
        ARRAY_AGG(martial_arts_charms_costs) as macos
    FROM character_martial_arts_charms INNER JOIN martial_arts_charms_costs ON (character_martial_arts_charms.charm_id = martial_arts_charms_costs.charm_id)
    WHERE character_martial_arts_charms.character_id = $1
), martial_arts_charm_tree_query AS (
    SELECT
        ARRAY_AGG(martial_arts_charm_tree) as matre
    FROM character_martial_arts_charms INNER JOIN martial_arts_charm_tree ON (character_martial_arts_charms.charm_id = martial_arts_charm_tree.child_id)
    WHERE character_martial_arts_charms.character_id = $1
), craft_abilities_query AS (
    SELECT
        ARRAY_AGG(craft_abilities) as cftas
    FROM craft_abilities
    WHERE craft_abilities.character_id = $1
), craft_specialties_query AS (
    SELECT
        ARRAY_AGG(craft_ability_specialties) as cftss
    FROM craft_ability_specialties
    WHERE craft_ability_specialties.character_id = $1
)
SELECT
    characters AS "character!: CharacterRow",
    player AS "player!: PlayerRow",
    attrs AS "attributes!: Vec<AttributeRow>",
    abils AS "abilities!: Vec<AbilityRow>",
    hboxs AS "health_boxes!: Vec<HealthBoxRow>",
    campaign AS "campaign: CampaignRow",
    specs AS "specialties: Vec<SpecialtyRow>",
    intis AS "intimacies: Vec<IntimacyRow>",
    weaps AS "weapons_owned: Vec<WeaponRow>",
    wepts AS "weapon_tags: Vec<WeaponTagRow>",
    eqwps AS "weapons_equipped: Vec<WeaponEquippedRow>",
    armrs AS "armor_owned: Vec<ArmorRow>",
    armts AS "armor_tags: Vec<ArmorTagRow>",
    wrars AS "armor_worn: Vec<ArmorWornRow>",
    mrtts AS "merit_templates: Vec<MeritTemplateRow>",
    mrtds AS "merit_details: Vec<MeritDetailRow>",
    mprss AS "merit_prerequisite_sets: Vec<MeritPrerequisiteSetRow>",
    meprs AS "merit_prerequisites:  Vec<PrerequisiteRow>",
    masts AS "martial_arts_styles: Vec<MartialArtsStyleRow>",
    chmas AS "character_martial_arts_styles: Vec<CharacterMartialArtsRow>",
    cmass AS "martial_arts_specialties: Vec<CharacterMartialArtsSpecialtyRow>",
    machs AS "martial_arts_charms: Vec<MartialArtsCharmRow>",
    makws AS "martial_arts_charm_keywords: Vec<MartialArtsCharmKeywordRow>",
    macos AS "martial_arts_charms_costs: Vec<MartialArtsCharmCostRow>",
    matre as "martial_arts_charm_tree: Vec<MartialArtsCharmTreeRow>",
    cftas AS "craft_abilities: Vec<CraftAbilityRow>",
    cftss AS "craft_specialties: Vec<CraftAbilitySpecialtyRow>"
FROM characters,
    player_query,
    attributes_query,
    abilities_query,
    health_boxes_query
    LEFT JOIN campaign_query ON (TRUE)
    LEFT JOIN specialties_query ON (TRUE)
    LEFT JOIN intimacies_query ON (TRUE)
    LEFT JOIN weapons_query ON (TRUE)
    LEFT JOIN weapon_tags_query ON (TRUE)
    LEFT JOIN weapons_equipped_query ON (TRUE)
    LEFT JOIN armor_query ON (TRUE)
    LEFT JOIN armor_tags_query ON (TRUE)
    LEFT JOIN character_armor_query ON (TRUE)
    LEFT JOIN merit_templates_query ON (TRUE)
    LEFT JOIN merit_details_query ON (TRUE)
    LEFT JOIN merit_prerequisite_sets_query ON (TRUE)
    LEFT JOIN merit_prerequisites_query ON (TRUE)
    LEFT JOIN martial_arts_styles_query ON (TRUE)
    LEFT JOIN character_martial_arts_query ON (TRUE)
    LEFT JOIN character_martial_arts_specialties ON (TRUE)
    LEFT JOIN martial_arts_charms_query ON (TRUE)
    LEFT JOIN martial_arts_charms_keywords_query ON (TRUE)
    LEFT JOIN martial_arts_charms_costs_query ON (TRUE)
    LEFT JOIN martial_arts_charm_tree_query ON (TRUE)
    LEFT JOIN craft_abilities_query ON (TRUE)
    LEFT JOIN craft_specialties_query ON (TRUE)
WHERE characters.id = $1;
