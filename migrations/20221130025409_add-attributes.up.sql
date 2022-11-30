CREATE TABLE attributes (
    character_id BIGSERIAL NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    name CHAR(3),
    dots SMALLINT NOT NULL,
    PRIMARY KEY (character_id, name)
);