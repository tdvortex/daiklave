CREATE TYPE EXALTTYPE AS ENUM ('SOLAR', 'LUNAR', 'DRAGONBLOODED');

CREATE TABLE characters (
    id BIGSERIAL PRIMARY KEY,
    campaign_player_id BIGSERIAL NOT NULL REFERENCES campaign_players(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    concept TEXT,
    exalt_type EXALTTYPE,
    current_willpower SMALLINT NOT NULL CHECK (current_willpower >= 0),
    max_willpower SMALLINT NOT NULL CHECK (max_willpower >= 0),
    current_experience SMALLINT NOT NULL CHECK (current_experience >= 0),
    total_experience SMALLINT NOT NULL CHECK (total_experience >= 0)
);