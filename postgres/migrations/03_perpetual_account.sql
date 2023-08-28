INSERT INTO "users" ("id", "address", "name") VALUES
('00000000-0000-0000-0000-000000000000', '0x0000000000000000000000000000000000000000', 'External');

DO $$
DECLARE
   asset record;
BEGIN
   FOR asset IN SELECT * FROM assets
   LOOP
      INSERT INTO "valuts" ("user_id", "asset_id", "balance")
         VALUES ('00000000-0000-0000-0000-000000000000', asset.id, '{"Infinite":"Positive"}')
         ON CONFLICT (user_id, asset_id)
         DO UPDATE SET
            asset_id = EXCLUDED.asset_id,
            balance = EXCLUDED.balance;
   END LOOP;
END; $$;

INSERT INTO "users" ("id", "address") VALUES ('ce3876ba-15b7-4409-8cf2-035fcc9d8687', '0x27ca37670fe706fe9c9ba97a672a1d2d3f49efc6');

DO $$
DECLARE
   asset record;
BEGIN
   FOR asset IN SELECT * FROM assets
   LOOP
      INSERT INTO "valuts" ("user_id", "asset_id", "balance")
         VALUES ('ce3876ba-15b7-4409-8cf2-035fcc9d8687', asset.id, '{"Finite":{"numer":"0","denom":"1"}}')
         ON CONFLICT (user_id, asset_id)
         DO UPDATE SET
            asset_id = EXCLUDED.asset_id,
            balance = EXCLUDED.balance;
   END LOOP;
END; $$;
