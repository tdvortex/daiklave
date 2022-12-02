CREATE TABLE campaign_players (
    id BIGSERIAL NOT NULL UNIQUE,
    campaign_id BIGINT NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    player_id BIGINT NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    PRIMARY KEY (campaign_id, player_id)
);