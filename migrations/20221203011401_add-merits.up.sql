CREATE TYPE MERITTYPE AS ENUM (
    'INNATE',
    'SUPERNATURAL',
    'STORY',
    'PURCHASED'
);

CREATE TABLE merits (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    requires_detail BOOLEAN NOT NULL,
    dots SMALLINT NOT NULL CHECK (dots >= 0),
    merit_type MERITTYPE NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE character_merits (
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    merit_id INTEGER REFERENCES merits(id) ON DELETE CASCADE,
    detail TEXT,
    PRIMARY KEY (character_id, merit_id)
);

CREATE TABLE merit_prerequisite_sets (
    id INTEGER,
    merit_id INTEGER REFERENCES merits(id) ON DELETE CASCADE,
    prerequisite_id INTEGER REFERENCES prerequisites(id) ON DELETE CASCADE,
    PRIMARY KEY (id, merit_id, prerequisite_id)
);