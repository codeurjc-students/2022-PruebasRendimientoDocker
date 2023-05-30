CREATE TABLE IF NOT EXISTS clothing (
    id SERIAL PRIMARY KEY,
    file_name UUID UNIQUE,
    label VARCHAR(255),
    size VARCHAR(10),
    kids BOOLEAN
);