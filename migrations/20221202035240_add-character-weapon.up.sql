CREATE TYPE EQUIPHAND AS ENUM('MAIN', 'OFF');

CREATE TABLE character_weapons (
    character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    weapon_id INTEGER NOT NULL REFERENCES weapons(id) ON DELETE CASCADE,
    equip_hand EQUIPHAND,
    UNIQUE (character_id, equip_hand)
);

CREATE TABLE character_armor (
    character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    armor_id INTEGER NOT NULL REFERENCES armor(id) ON DELETE CASCADE,
    worn BOOLEAN NOT NULL
);