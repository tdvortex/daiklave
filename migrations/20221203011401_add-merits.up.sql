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
    merit_type MERITTYPE NOT NULL,
    description TEXT NOT NULL,
    book_title VARCHAR(255),
    page_number SMALLINT,
    creator_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    CHECK 
        ((book_title IS NOT NULL and page_number IS NOT NULL and creator_id IS NULL) 
        OR (book_title IS NULL and page_number IS NULL and creator_id IS NOT NULL))
);

CREATE TABLE character_merits (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    merit_id INTEGER REFERENCES merits(id) ON DELETE CASCADE,
    dots SMALLINT NOT NULL CHECK (dots >= 0),
    detail TEXT
);

CREATE TABLE merit_prerequisite_sets (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    merit_id INTEGER REFERENCES merits(id) ON DELETE CASCADE
);