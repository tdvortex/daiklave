CREATE TYPE PREREQUISITETYPE AS ENUM (
    'ABILITY',
    'ATTRIBUTE',
    'ESSENCE',
    'CHARM',
    'EXALTTYPE'
);

CREATE TYPE PREREQUISITEEXALTTYPE AS ENUM (
    'SOLAR',
    'LUNAR',
    'DRAGONBLOODED',
    'SPIRIT',
    'SPIRITORECLIPSE'
);

CREATE TABLE prerequisites (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    merit_prerequisite_set_id INTEGER REFERENCES merit_prerequisite_sets(id) ON DELETE CASCADE,
    charm_prerequisite_set_id INTEGER REFERENCES charm_prerequisite_sets(id) ON DELETE CASCADE,
    prerequisite_type PREREQUISITETYPE NOT NULL,
    ability_name ABILITYNAMEVANILLA,
    subskill_name VARCHAR(255),
    attribute_name ATTRIBUTENAME,
    dots SMALLINT CHECK (dots >= 0),
    charm_id INTEGER REFERENCES charms(id) ON DELETE CASCADE,
    prerequisite_exalt_type PREREQUISITEEXALTTYPE,
    CHECK (
        CASE
            WHEN prerequisite_type = 'ABILITY' 
                THEN (ability_name IS NOT NULL AND dots IS NOT NULL)
            WHEN prerequisite_type = 'ATTRIBUTE' 
                THEN (attribute_name IS NOT NULL AND dots IS NOT NULL)
            WHEN prerequisite_type = 'ESSENCE' THEN (dots IS NOT NULL)
            WHEN prerequisite_type = 'CHARM' THEN (charm_id IS NOT NULL)
            WHEN prerequisite_type = 'EXALTTYPE' THEN (prerequisite_exalt_type IS NOT NULL)
        END
    ),
    CHECK (
        (merit_prerequisite_set_id IS NULL AND charm_prerequisite_set_id IS NOT NULL)
        OR (merit_prerequisite_set_id IS NOT NULL AND charm_prerequisite_set_id IS NULL)
    )
);