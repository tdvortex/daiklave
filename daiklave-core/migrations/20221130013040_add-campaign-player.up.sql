CREATE TABLE campaign_players (
    id INTEGER GENERATED ALWAYS AS IDENTITY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    player_id INTEGER NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    PRIMARY KEY (campaign_id, player_id)
);