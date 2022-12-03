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
    prerequisite_type PREREQUISITETYPE NOT NULL,
    ability_name ABILITYNAME,
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
    )
);

CREATE TABLE charm_prerequisite_sets (
    id INTEGER NOT NULL,
    charm_id INTEGER REFERENCES charms(id) ON DELETE CASCADE,
    prerequisite_id INTEGER REFERENCES prerequisites(id) ON DELETE CASCADE,
    PRIMARY KEY (id, charm_id, prerequisite_id)
)