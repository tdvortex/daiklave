CREATE TYPE EXALTTYPE AS ENUM ('SOLAR', 'LUNAR', 'DRAGONBLOODED');

CREATE TABLE characters (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    player_id INTEGER NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    campaign_id INTEGER REFERENCES campaigns(id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    concept TEXT,
    exalt_type EXALTTYPE,
    current_willpower SMALLINT NOT NULL CHECK (current_willpower >= 0),
    max_willpower SMALLINT NOT NULL CHECK (max_willpower >= 0),
    current_experience SMALLINT NOT NULL CHECK (current_experience >= 0),
    total_experience SMALLINT NOT NULL CHECK (total_experience >= 0)
);