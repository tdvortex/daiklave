CREATE TABLE martial_arts_styles (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    book_title VARCHAR(255),
    page_number SMALLINT CHECK(page_number >= 0),
    creator_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    CHECK 
        ((book_title IS NOT NULL and page_number IS NOT NULL and creator_id IS NULL) 
        OR (book_title IS NULL and page_number IS NULL and creator_id IS NOT NULL))
);

CREATE TABLE character_martial_arts (
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    style_id INTEGER REFERENCES martial_arts_styles(id) ON DELETE CASCADE,
    dots SMALLINT NOT NULL CHECK(dots >= 0 AND dots <= 255),
    PRIMARY KEY (character_id, style_id)
);

CREATE TABLE character_martial_arts_specialties (
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    style_id INTEGER REFERENCES martial_arts_styles(id) ON DELETE CASCADE,
    specialty VARCHAR(255),
    PRIMARY KEY (character_id, style_id, specialty)
);

CREATE TABLE martial_arts_charms (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    style_id INTEGER NOT NULL REFERENCES martial_arts_styles(id) ON DELETE CASCADE,
    ability_dots_required SMALLINT NOT NULL CHECK(ability_dots_required >= 0 AND ability_dots_required <= 255),
    essence_dots_required SMALLINT NOT NULL CHECK(essence_dots_required >= 0 AND essence_dots_required <= 255),
    name VARCHAR(255) NOT NULL,
    summary TEXT,
    description TEXT NOT NULL,
    action_type CHARMACTIONTYPE NOT NULL,
    duration VARCHAR(255) NOT NULL,
    book_title VARCHAR(255),
    page_number SMALLINT CHECK(page_number >= 0),
    creator_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    CHECK 
        ((book_title IS NOT NULL and page_number IS NOT NULL and creator_id IS NULL) 
        OR (book_title IS NULL and page_number IS NULL and creator_id IS NOT NULL))
);

CREATE TABLE martial_arts_charms_keywords (
    charm_id INTEGER REFERENCES martial_arts_charms(id) ON DELETE CASCADE,
    keyword CHARMKEYWORD,
    PRIMARY KEY (charm_id, keyword)
);

CREATE TABLE character_martial_arts_charms (
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    charm_id INTEGER REFERENCES martial_arts_charms(id) ON DELETE CASCADE,
    PRIMARY KEY (character_id, charm_id)
);