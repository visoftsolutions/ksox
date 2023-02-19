-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "address" CHAR(42) UNIQUE NOT NULL
);

CREATE SCHEMA "spot";

CREATE TYPE "spot"."products_status" AS ENUM (
  'active',
  'partially_filled',
  'filled',
  'cancelled'
);

CREATE TABLE "spot"."valuts" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "user_id" uuid NOT NULL,
  "asset_id" uuid NOT NULL,
  "balance" NUMERIC(78) NOT NULL CHECK ("balance" >= 0) DEFAULT 0
);

CREATE TABLE "spot"."assets" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "name" VARCHAR NOT NULL,
  "symbol" VARCHAR NOT NULL,
  "maker_fee_num" NUMERIC(78) NOT NULL CHECK ("maker_fee_num" >= 0) DEFAULT 0,
  "maker_fee_denum" NUMERIC(78) NOT NULL CHECK ("maker_fee_denum" > 0) DEFAULT 1,
  "taker_fee_num" NUMERIC(78) NOT NULL CHECK ("taker_fee_num" >= 0) DEFAULT 0,
  "taker_fee_denum" NUMERIC(78) NOT NULL CHECK ("taker_fee_denum" > 0) DEFAULT 1
);

CREATE TABLE "spot"."orders" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "user_id" uuid NOT NULL,
  "status" spot.products_status NOT NULL,
  "quote_asset_id" uuid NOT NULL,
  "base_asset_id" uuid NOT NULL,
  "quote_asset_volume" NUMERIC(78) NOT NULL CHECK ("quote_asset_volume" >= 0),
  "base_asset_volume" NUMERIC(78) NOT NULL CHECK ("base_asset_volume" >= 0)
);

CREATE TABLE "spot"."trades" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "taker_id" uuid NOT NULL,
  "order_id" uuid NOT NULL,
  "taker_quote_volume" NUMERIC(78) NOT NULL CHECK ("taker_quote_volume" >= 0),
  "taker_base_volume" NUMERIC(78) NOT NULL CHECK ("taker_base_volume" >= 0),
  "maker_quote_volume" NUMERIC(78) NOT NULL CHECK ("maker_quote_volume" >= 0),
  "maker_base_volume" NUMERIC(78) NOT NULL CHECK ("maker_base_volume" >= 0)
);

ALTER TABLE "spot"."valuts" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "spot"."valuts" ADD FOREIGN KEY ("asset_id") REFERENCES "spot"."assets" ("id");

ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("quote_asset_id") REFERENCES "spot"."assets" ("id");

ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("base_asset_id") REFERENCES "spot"."assets" ("id");

ALTER TABLE "spot"."trades" ADD FOREIGN KEY ("taker_id") REFERENCES "users" ("id");

ALTER TABLE "spot"."trades" ADD FOREIGN KEY ("order_id") REFERENCES "spot"."orders" ("id");

CREATE OR REPLACE FUNCTION notify()
RETURNS TRIGGER AS $$
BEGIN
  PERFORM pg_notify(TG_ARGV[0], row_to_json(NEW)::text);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION create_trades_notify_trigger(id uuid)
RETURNS VOID AS
$BODY$
BEGIN
  EXECUTE format('
    CREATE OR REPLACE TRIGGER trades_notify_trigger_id_%s
    AFTER INSERT OR UPDATE ON spot.trades
    FOR EACH ROW
    WHEN (NEW.taker_id = ''%s'')
    EXECUTE FUNCTION notify(''trades_notify_channel_id_%s'');', 
    replace(id::text, '-'::text,'_'::text), id::text, id::text);
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION drop_trades_notify_trigger(id uuid)
RETURNS VOID AS
$BODY$
DECLARE
  listener_count integer;
BEGIN
  SELECT count(*) INTO listener_count FROM pg_stat_activity WHERE lower(query) LIKE '%listen%trades_notify_trigger_id_'|| id::text ||'%';
  IF listener_count = 0 THEN
    EXECUTE format('
      DROP TRIGGER trades_notify_trigger_id_%s ON spot.trades;', 
      replace(id::text, '-'::text,'_'::text));
  END IF;
END;
$BODY$ LANGUAGE plpgsql;