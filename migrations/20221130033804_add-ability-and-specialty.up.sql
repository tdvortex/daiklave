CREATE TYPE ABILITYNAME AS ENUM ('ARCHERY', 'ATHLETICS', 'AWARENESS', 'BRAWL', 
    'BUREAUCRACY', 'DODGE', 'INTEGRITY', 'INVESTIGATION', 'LARCENY', 
    'LINGUISTICS', 'LORE', 'MEDICINE', 'MELEE', 'OCCULT', 'PERFORMANCE',
    'PRESENCE', 'RESISTANCE', 'RIDE', 'SAIL', 'SOCIALIZE', 'STEALTH',
    'SURVIVAL', 'THROWN', 'WAR');

CREATE TABLE abilities (
    character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    name ABILITYNAME NOT NULL,
    dots SMALLINT NOT NULL CHECK (dots >= 0),
    PRIMARY KEY (character_id, name)
);

CREATE TABLE specialties (
    character_id INTEGER,
    ability_name ABILITYNAME,
    specialty VARCHAR(255),
    FOREIGN KEY (character_id, ability_name) REFERENCES abilities(character_id, name) ON DELETE CASCADE,
    PRIMARY KEY (character_id, ability_name, specialty)
);