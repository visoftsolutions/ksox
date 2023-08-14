INSERT INTO "users" ("id", "address") VALUES ('00000000-0000-0000-0000-000000000000', '0x0000000000000000000000000000000000000000');

DO $$
DECLARE
   asset record;
BEGIN
   FOR asset IN SELECT * FROM assets
   LOOP
      INSERT INTO "valuts" ("user_id","asset_id","balance") VALUES ('00000000-0000-0000-0000-000000000000', asset.id, '{"Infinite":"Positive"}');
   END LOOP;
END; $$
