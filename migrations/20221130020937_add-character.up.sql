CREATE TABLE characters (
    id BIGSERIAL PRIMARY KEY,
    campaign_player_id BIGSERIAL NOT NULL REFERENCES campaign_players(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    concept TEXT,
    exalt_type CHAR(2),
    current_willpower SMALLINT NOT NULL,
    max_willpower SMALLINT NOT NULL,
    current_experience SMALLINT NOT NULL,
    total_experience SMALLINT NOT NULL
);