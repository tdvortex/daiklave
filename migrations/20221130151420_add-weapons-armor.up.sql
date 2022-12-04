CREATE TYPE WEAPONTAGTYPE AS ENUM ('ARCHERY', 'ARTIFACT', 'BALANCED', 
    'BASHING', 'BRAWL', 'CHOPPING', 'CONCEALABLE', 'CROSSBOW', 'CUTTING', 
    'DISARMING', 'EXCEPTIONAL', 'FLAME', 'FLEXIBLE', 'GRAPPLING', 'HEAVY',
    'IMPROVISED', 'LETHAL', 'LIGHT', 'MARTIALARTS', 'MEDIUM', 'MELEE', 
    'MOUNTED', 'ONEHANDED', 'NATURAL', 'PIERCING', 'POISONABLE', 'POWERFUL',
    'REACHING','SHIELD', 'SLOW', 'SMASHING', 'SPECIAL', 'SUBTLE', 'THROWN',
    'TWOHANDED', 'WORN');

CREATE TYPE RANGEBAND AS ENUM('CLOSE', 'SHORT', 'MEDIUM', 'LONG', 'EXTREME');

CREATE TYPE WEAPONTAG AS (
    tag_type WEAPONTAGTYPE,
    max_range RANGEBAND,
    martial_arts_style VARCHAR(255)
);

CREATE TABLE weapons (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    tags WEAPONTAG[] NOT NULL CHECK (ARRAY_LENGTH(tags, 1) > 0),
    creator_id INTEGER REFERENCES characters(id) ON DELETE CASCADE
);

CREATE TYPE ARMORTAG AS ENUM ('ARTIFACT', 'CONCEALABLE', 'HEAVY', 'LIGHT',
    'MEDIUM', 'SILENT', 'SPECIAL');

CREATE TABLE armor (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    tags ARMORTAG[] NOT NULL CHECK (ARRAY_LENGTH(tags, 1) > 0),
    creator_id INTEGER REFERENCES characters(id) ON DELETE CASCADE
);