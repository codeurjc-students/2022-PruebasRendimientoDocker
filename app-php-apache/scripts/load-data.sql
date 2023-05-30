DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM clothing LIMIT 1) THEN
        COPY clothing (file_name, label, size, kids)
        FROM '/var/lib/postgresql/data/pgdata/data.csv'
        DELIMITER ',' CSV HEADER;
    END IF;
END 
$$;
