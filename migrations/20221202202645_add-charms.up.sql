CREATE TYPE CHARMCOSTTYPE AS ENUM (
    'MOTES',
    'WILLPOWER',
    'BASHINGHEALTH',
    'LETHALHEALTH',
    'AGGRAVATEDHEALTH',
    'ANIMALEVELS',
    'INITIAIVE',
    'EXPERIENCE',
    'SILVERCRAFTEXPERIENCE',
    'GOLDCRAFTEXPERIENCE',
    'WHITECRAFTEXPERIENCE',
    'SORCEROUSMOTES'
);

CREATE TYPE CHARMCOST AS (
    cost_type CHARMCOSTTYPE,
    amount SMALLINT
);

CREATE TYPE CHARMACTIONTYPE AS ENUM (
    'SIMPLE',
    'SUPPLEMENTAL',
    'REFLEXIVE',
    'PERMANENT'
);

CREATE TYPE CHARMKEYWORD AS ENUM (
    'AIR',
    'AGGRAVATED',
    'ARCHETYPE',
    'AURA',
    'BALANCED',
    'BRIDGE',
    'CLASH',
    'COUNTERATTACK',
    'DECISIVEONLY',
    'DUAL',
    'EXCELLENCY',
    'FIRE',
    'EARTH',
    'MUTE',
    'PILOT',
    'PROTEAN',
    'PSYCHE',
    'PERILOUS',
    'SALIENT',
    'SIGNATURE',
    'STACKABLE',
    'UNIFORM',
    'WATER',
    'WITHERINGONLY',
    'WOOD',
    'WRITTENONLY'
);

CREATE TYPE CHARMDURATIONTYPE AS ENUM (
    'INSTANT',
    'TICK',
    'TURN',
    'ROUND',
    'SCENE',
    'INDEFINITE',
    'PERMANENT',
    'SPECIAL'
);

CREATE TABLE charms (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    costs CHARMCOSTTYPE[] NOT NULL,
    action_type CHARMACTIONTYPE NOT NULL,
    keywords CHARMKEYWORD[] NOT NULL,
    duration CHARMDURATIONTYPE NOT NULL,
    special_duration VARCHAR(255),
    book_name VARCHAR(255),
    page_number INTEGER CHECK (page_number >= 0),
    creator_id BIGINT REFERENCES characters(id) ON DELETE CASCADE,
    summary TEXT NOT NULL,
    description TEXT NOT NULL,
    CHECK (
            (book_name IS NULL AND page_number IS NULL and creator_id IS NOT NULL) 
            OR
            (book_name IS NOT NULL AND page_number IS NOT NULL and creator_id IS NULL)
    ),
    CHECK (duration != 'SPECIAL' OR special_duration IS NOT NULL)
);