CREATE TABLE campaign_players (
    id BIGSERIAL NOT NULL UNIQUE,
    campaign_id BIGSERIAL NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    player_id BIGSERIAL NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    PRIMARY KEY (campaign_id, player_id)
);