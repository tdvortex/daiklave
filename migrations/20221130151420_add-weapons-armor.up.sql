CREATE TYPE WEAPONTAGTYPE AS ENUM ('ARCHERY', 'ARTIFACT', 'BALANCED', 
    'BASHING', 'BRAWL', 'CHOPPING', 'CONCEALABLE', 'CROSSBOW', 'CUTTING', 
    'DISARMING', 'EXCEPTIONAL', 'FLAME', 'FLEXIBLE', 'GRAPPLING', 'HEAVY',
    'IMPROVISED', 'LETHAL', 'LIGHT', 'MARTIALARTS', 'MEDIUM', 'MELEE', 
    'MOUNTED', 'ONEHANDED', 'NATURAL', 'PIERCING', 'POISONABLE', 'POWERFUL',
    'REACHING','SHIELD', 'SLOW', 'SMASHING', 'SPECIAL', 'SUBTLE', 'THROWN',
    'TWOHANDED', 'WORN');

CREATE TYPE RANGEBAND AS ENUM('CLOSE', 'SHORT', 'MEDIUM', 'LONG', 'EXTREME');

CREATE TABLE weapons (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    book_title VARCHAR(255),
    page_number SMALLINT,
    creator_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    CHECK 
        ((book_title IS NOT NULL and page_number IS NOT NULL and creator_id IS NULL) 
        OR (book_title IS NULL and page_number IS NULL and creator_id IS NOT NULL))
);

CREATE TABLE weapon_tags (
    weapon_id INTEGER NOT NULL REFERENCES weapons(id) ON DELETE CASCADE,
    tag_type WEAPONTAGTYPE NOT NULL,
    max_range RANGEBAND,
    martial_arts_style VARCHAR(255),
    CHECK (
        CASE
            WHEN tag_type = 'ARCHERY' THEN max_range IS NOT NULL AND martial_arts_style IS NULL
            WHEN tag_type = 'THROWN' THEN max_range IS NOT NULL AND martial_arts_style IS NULL
            WHEN tag_type = 'MARTIALARTS' then martial_arts_style IS NOT NULL AND max_range IS NULL
            ELSE max_range IS NULL AND martial_arts_style IS NULL
        END
    )
);

CREATE TYPE ARMORTAGTYPE AS ENUM ('ARTIFACT', 'CONCEALABLE', 'HEAVY', 'LIGHT',
    'MEDIUM', 'SILENT', 'SPECIAL');

CREATE TABLE armor (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    book_title VARCHAR(255),
    page_number SMALLINT,
    creator_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    CHECK 
        ((book_title IS NOT NULL and page_number IS NOT NULL and creator_id IS NULL) 
        OR (book_title IS NULL and page_number IS NULL and creator_id IS NOT NULL))
);

CREATE TABLE armor_tags (
    armor_id INTEGER NOT NULL REFERENCES armor(id) ON DELETE CASCADE,
    tag_type ARMORTAGTYPE NOT NULL
);