CREATE TABLE campaign_players (
    id BIGSERIAL PRIMARY KEY,
    campaign_id BIGSERIAL REFERENCES campaigns(id) ON DELETE CASCADE,
    player_id BIGSERIAL REFERENCES players(id) ON DELETE CASCADE
);