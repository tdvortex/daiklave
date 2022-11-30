-- Add up migration script here
CREATE TYPE INTIMACYTYPE AS ENUM ('TIE', 'PRINCIPLE');
CREATE TYPE INTIMACYLEVEL AS ENUM ('MINOR', 'MAJOR', 'DEFINING');

CREATE TABLE intimacies (
    id BIGSERIAL PRIMARY KEY,
    character_id BIGSERIAL NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    intimacy_type INTIMACYTYPE NOT NULL,
    level INTIMACYLEVEL NOT NULL,
    description TEXT NOT NULL
);