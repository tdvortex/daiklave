CREATE TABLE abilities (
    id BIGSERIAL PRIMARY KEY,
    character_id BIGSERIAL NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    name CHAR(2) NOT NULL,
    dots SMALLINT NOT NULL,
    subskill VARCHAR(255),
    UNIQUE(character_id, name, subskill)
);

CREATE TABLE specialties (
    id BIGSERIAL PRIMARY KEY,
    ability_id BIGSERIAL NOT NULL REFERENCES abilities(id) ON DELETE CASCADE,
    specialty VARCHAR(255) NOT NULL,
    UNIQUE(ability_id, specialty)
);