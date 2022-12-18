CREATE TABLE campaigns (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    bot_channel BIGINT NOT NULL CHECK (bot_channel > 0)
);