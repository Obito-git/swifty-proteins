INSERT INTO file_metadata (name, path)
VALUES ('001', 'database/local_storage/matilda.glb');

SELECT last_insert_rowid();

UPDATE proteins
SET file_metadata_id = last_insert_rowid()
WHERE code = '001';