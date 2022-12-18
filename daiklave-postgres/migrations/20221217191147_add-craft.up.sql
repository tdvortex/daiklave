CREATE TABLE craft_abilities (
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    focus VARCHAR(255),
    dots SMALLINT NOT NULL,
    PRIMARY KEY (character_id, focus)
);

CREATE TABLE craft_ability_specialties (
    character_id INTEGER,
    focus VARCHAR(255),
    specialty VARCHAR(255),
    FOREIGN KEY (character_id, focus) REFERENCES craft_abilities(character_id, focus) ON DELETE CASCADE,
    PRIMARY KEY (character_id, focus, specialty)
);