WITH player_campaign_query AS (
    SELECT
        characters.id as character_id,
        players AS player,
        campaigns AS campaign
    FROM characters
        INNER JOIN campaign_players ON (characters.campaign_player_id = campaign_players.id)
        INNER JOIN players ON (players.id = campaign_players.player_id)
        INNER JOIN campaigns ON (campaigns.id = campaign_players.campaign_id)
    WHERE characters.id = $1
    LIMIT 1
),
attributes_query AS (
    SELECT
        characters.id as character_id,
        ARRAY_AGG(attributes) AS attrs
    FROM characters
        INNER JOIN attributes ON (attributes.character_id = characters.id)
    WHERE characters.id = $1
    GROUP BY 1
), abilities_query AS (
    SELECT
        characters.id as character_id,
        ARRAY_AGG(abilities) AS abils
    FROM characters
        INNER JOIN abilities ON (abilities.character_id = characters.id)
    WHERE characters.id = $1
    GROUP BY 1
), specialties_query AS (
    SELECT
        characters.id as character_id,
        ARRAY_AGG(specialties) AS specs
    FROM characters
        INNER JOIN abilities ON (abilities.character_id = characters.id)
        INNER JOIN specialties ON (specialties.ability_id = abilities.id)
    WHERE characters.id = $1
    GROUP BY 1
), 
intimacies_query AS (
    SELECT
        characters.id as character_id,
        ARRAY_AGG(intimacies) AS intis
    FROM characters
        INNER JOIN intimacies ON (intimacies.character_id = characters.id)
    WHERE characters.id = $1
    GROUP BY 1
), health_boxes_query AS (
    SELECT
        characters.id as character_id,
        ARRAY_AGG(health_boxes) AS hboxs
    FROM characters
        INNER JOIN health_boxes ON (health_boxes.character_id = characters.id)
    WHERE characters.id = $1
    GROUP BY 1
)
SELECT
    characters AS "character!: CharacterRow",
    player AS "player!: PlayerRow",
    campaign AS "campaign!: CampaignRow",
    attrs AS "attributes!: Vec<AttributeRow>",
    abils AS "abilities!: Vec<AbilityRow>",
    specs AS "specialties: Vec<SpecialtyRow>",
    intis AS "intimacies: Vec<IntimacyRow>",
    hboxs AS "health_boxes!: Vec<HealthBoxRow>"
FROM characters
    INNER JOIN player_campaign_query ON (characters.id = character_id)
    INNER JOIN attributes_query USING (character_id)
    INNER JOIN abilities_query USING (character_id)
    INNER JOIN health_boxes_query USING (character_id)
    LEFT JOIN specialties_query USING (character_id)
    LEFT JOIN intimacies_query USING (character_id)
WHERE characters.id = $1;
