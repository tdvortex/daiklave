CREATE TYPE ABILITYNAME AS ENUM ('ARCHERY', 'ATHLETICS', 'AWARENESS', 'BRAWL', 
    'BUREAUCRACY', 'CRAFT', 'DODGE', 'INTEGRITY', 'INVESTIGATION', 'LARCENY', 
    'LINGUISTICS', 'LORE', 'MARTIALARTS', 'MEDICINE', 'MELEE', 'OCCULT', 
    'PERFORMANCE', 'PRESENCE', 'RESISTANCE', 'RIDE', 'SAIL', 'SOCIALIZE', 
    'STEALTH', 'SURVIVAL', 'THROWN', 'WAR');

CREATE TABLE abilities (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    name ABILITYNAME NOT NULL,
    dots SMALLINT NOT NULL CHECK (dots >= 0),
    subskill VARCHAR(255),
    CONSTRAINT unique_abilities UNIQUE (character_id, name, subskill),
    CHECK (
        ((name != 'CRAFT' OR name != 'MARTIALARTS') AND subskill IS NOT NULL) 
        OR subskill IS NULL
        )
);

CREATE TABLE specialties (
    ability_id INTEGER REFERENCES abilities(id) ON DELETE CASCADE,
    specialty VARCHAR(255),
    PRIMARY KEY (ability_id, specialty)
);