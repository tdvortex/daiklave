-- Add up migration script here
CREATE TYPE WOUNDPENALTY AS ENUM ('ZERO', 'MINUSONE', 'MINUSTWO', 'MINUSFOUR', 'INCAPACITATED');
CREATE TYPE DAMAGETYPE AS ENUM ('BASHING', 'LETHAL', 'AGGRAVATED');

CREATE TABLE health_boxes (
    character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    position SMALLINT NOT NULL,
    wound_penalty WOUNDPENALTY NOT NULL,
    current_damage DAMAGETYPE,
    PRIMARY KEY (character_id, position)
)