-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "address" CHAR(42) UNIQUE NOT NULL
);

CREATE SCHEMA "spot";

CREATE TABLE "spot"."valuts" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "user_id" uuid NOT NULL,
  "asset_id" uuid NOT NULL,
  "balance" NUMERIC(78) NOT NULL CHECK ("balance" >= 0) DEFAULT 0,
  UNIQUE ("user_id", "asset_id")
);

CREATE TABLE "spot"."assets" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "name" VARCHAR NOT NULL,
  "symbol" VARCHAR NOT NULL,
  "maker_fee_num" NUMERIC(78) NOT NULL CHECK ("maker_fee_num" >= 0) CHECK ("maker_fee_num" <= "maker_fee_denum") DEFAULT 0,
  "maker_fee_denum" NUMERIC(78) NOT NULL CHECK ("maker_fee_denum" > 0) DEFAULT 1,
  "taker_fee_num" NUMERIC(78) NOT NULL CHECK ("taker_fee_num" >= 0) CHECK ("taker_fee_num" <= "taker_fee_denum") DEFAULT 0,
  "taker_fee_denum" NUMERIC(78) NOT NULL CHECK ("taker_fee_denum" > 0) DEFAULT 1
);

CREATE TABLE "spot"."orders" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "user_id" uuid NOT NULL,
  "is_active" BOOLEAN NOT NULL DEFAULT true,
  "quote_asset_id" uuid NOT NULL,
  "base_asset_id" uuid NOT NULL,
  "quote_asset_volume" NUMERIC(78) NOT NULL CHECK ("quote_asset_volume" >= 0),
  "base_asset_volume" NUMERIC(78) NOT NULL CHECK ("base_asset_volume" >= 0),
  "quote_asset_volume_left" NUMERIC(78) NOT NULL CHECK ("quote_asset_volume_left" >= 0) CHECK ("quote_asset_volume_left" <= "quote_asset_volume"),
  "maker_fee_num" NUMERIC(78) NOT NULL CHECK ("maker_fee_num" >= 0) CHECK ("maker_fee_num" <= "maker_fee_denum") DEFAULT 0,
  "maker_fee_denum" NUMERIC(78) NOT NULL CHECK ("maker_fee_denum" > 0) DEFAULT 1
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

CREATE FUNCTION notify()
RETURNS TRIGGER AS 
$BODY$
DECLARE
  row_data json;
BEGIN
  SELECT json_object_agg(key, CASE WHEN json_typeof(value) = 'number'::text THEN to_json(value::text) ELSE to_json(value) END) INTO row_data FROM json_each(to_json(NEW));
  PERFORM pg_notify(TG_ARGV[0], row_data::text);
  RETURN NEW;
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION create_spot_valuts_notify_trigger_for_user(trigger_name text, user_id uuid)
RETURNS VOID AS
$BODY$
DECLARE
  trigger_truncated_name text := LEFT(format('t_%s', trigger_name), 63);
  channel_truncated_name text := LEFT(format('c_%s', trigger_name), 63);
BEGIN
  EXECUTE format('
    CREATE OR REPLACE TRIGGER %s
    AFTER INSERT OR UPDATE ON spot.valuts
    FOR EACH ROW
    WHEN (NEW.user_id = ''%s'')
    EXECUTE FUNCTION notify(''%s'');', 
    trigger_truncated_name, user_id::text, channel_truncated_name);
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION drop_spot_valuts_notify_trigger_for_user(trigger_name text)
RETURNS VOID AS
$BODY$
DECLARE
  listener_count integer;
  trigger_truncated_name text := LEFT(format('t_%s', trigger_name), 63);
  channel_truncated_name text := LEFT(format('c_%s', trigger_name), 63);
BEGIN
  SELECT count(*) INTO listener_count FROM pg_stat_activity WHERE lower(query) LIKE '%listen%'|| channel_truncated_name ||'%';
  IF listener_count = 0 THEN
    EXECUTE format('
      DROP TRIGGER %s ON spot.valuts;', 
      trigger_truncated_name);
  END IF;
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION create_spot_orders_notify_trigger_for_user(trigger_name text, user_id uuid)
RETURNS VOID AS
$BODY$
DECLARE
  trigger_truncated_name text := LEFT(format('t_%s', trigger_name), 63);
  channel_truncated_name text := LEFT(format('c_%s', trigger_name), 63);
BEGIN
  EXECUTE format('
    CREATE OR REPLACE TRIGGER %s
    AFTER INSERT OR UPDATE ON spot.orders
    FOR EACH ROW
    WHEN (NEW.user_id = ''%s'')
    EXECUTE FUNCTION notify(''%s'');', 
    trigger_truncated_name, user_id::text, channel_truncated_name);
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION drop_spot_orders_notify_trigger_for_user(trigger_name text)
RETURNS VOID AS
$BODY$
DECLARE
  listener_count integer;
  trigger_truncated_name text := LEFT(format('t_%s', trigger_name), 63);
  channel_truncated_name text := LEFT(format('c_%s', trigger_name), 63);
BEGIN
  SELECT count(*) INTO listener_count FROM pg_stat_activity WHERE lower(query) LIKE '%listen%'|| channel_truncated_name ||'%';
  IF listener_count = 0 THEN
    EXECUTE format('
      DROP TRIGGER %s ON spot.orders;', 
      trigger_truncated_name);
  END IF;
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION create_spot_orders_notify_trigger_active_for_user(trigger_name text, user_id uuid)
RETURNS VOID AS
$BODY$
DECLARE
  trigger_truncated_name text := LEFT(format('t_%s', trigger_name), 63);
  channel_truncated_name text := LEFT(format('c_%s', trigger_name), 63);
BEGIN
  EXECUTE format('
    CREATE OR REPLACE TRIGGER %s
    AFTER INSERT OR UPDATE ON spot.orders
    FOR EACH ROW
    WHEN (NEW.user_id = ''%s'' AND OLD.is_active = true)
    EXECUTE FUNCTION notify(''%s'');', 
    trigger_truncated_name, user_id::text, channel_truncated_name);
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION drop_spot_orders_notify_trigger_active_for_user(trigger_name text)
RETURNS VOID AS
$BODY$
DECLARE
  listener_count integer;
  trigger_truncated_name text := LEFT(format('t_%s', trigger_name), 63);
  channel_truncated_name text := LEFT(format('c_%s', trigger_name), 63);
BEGIN
  SELECT count(*) INTO listener_count FROM pg_stat_activity WHERE lower(query) LIKE '%listen%'|| channel_truncated_name ||'%';
  IF listener_count = 0 THEN
    EXECUTE format('
      DROP TRIGGER %s ON spot.orders;', 
      trigger_truncated_name);
  END IF;
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION create_spot_orders_notify_trigger_for_asset_pair(trigger_name text, quote_asset_id uuid, base_asset_id uuid)
RETURNS VOID AS
$BODY$
DECLARE
  trigger_truncated_name text := LEFT(format('t_%s', trigger_name), 63);
  channel_truncated_name text := LEFT(format('c_%s', trigger_name), 63);
BEGIN
  EXECUTE format('
    CREATE OR REPLACE TRIGGER %s
    AFTER INSERT OR UPDATE ON spot.orders
    FOR EACH ROW
    WHEN (NEW.quote_asset_id = ''%s'' AND NEW.base_asset_id = ''%s'')
    EXECUTE FUNCTION notify(''%s'');', 
    trigger_truncated_name, quote_asset_id::text, base_asset_id::text, channel_truncated_name);
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION drop_spot_orders_notify_trigger_for_asset_pair(trigger_name text)
RETURNS VOID AS
$BODY$
DECLARE
  listener_count integer;
  trigger_truncated_name text := LEFT(format('t_%s', trigger_name), 63);
  channel_truncated_name text := LEFT(format('c_%s', trigger_name), 63);
BEGIN
  SELECT count(*) INTO listener_count FROM pg_stat_activity WHERE lower(query) LIKE '%listen%'|| channel_truncated_name ||'%';
  IF listener_count = 0 THEN
    EXECUTE format('
      DROP TRIGGER %s ON spot.orders;', 
      trigger_truncated_name);
  END IF;
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION orderbook_notify()
RETURNS TRIGGER AS
$BODY$
DECLARE
  row_data json;
BEGIN
  SELECT json_agg(row_to_json(sub.*)) INTO row_data
      FROM (
        SELECT orderbook.price::float AS price, SUM(orderbook.volume)::text AS volume FROM 
          (SELECT 
            spot.orders.base_asset_volume/spot.orders.quote_asset_volume AS price,
            spot.orders.quote_asset_volume_left AS volume
          FROM spot.orders
          WHERE spot.orders.is_active = true
            AND spot.orders.quote_asset_id = NEW.quote_asset_id
            AND spot.orders.base_asset_id = NEW.base_asset_id
          ) AS orderbook
        GROUP BY orderbook.price
    ) sub;

  PERFORM pg_notify(TG_ARGV[0], row_data::text);
  RETURN NEW;
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION create_spot_orders_notify_trigger_for_orderbook(trigger_name text, quote_asset_id uuid, base_asset_id uuid)
RETURNS VOID AS
$BODY$
DECLARE
  trigger_truncated_name text := LEFT(format('t_%s', trigger_name), 63);
  channel_truncated_name text := LEFT(format('c_%s', trigger_name), 63);
BEGIN
  EXECUTE format('
    CREATE OR REPLACE TRIGGER %s
    AFTER INSERT OR UPDATE ON spot.orders
    FOR EACH ROW
    WHEN (NEW.quote_asset_id = ''%s'' AND NEW.base_asset_id = ''%s'')
    EXECUTE FUNCTION orderbook_notify(''%s'');', 
    trigger_truncated_name, quote_asset_id::text, base_asset_id::text, channel_truncated_name);
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION drop_spot_orders_notify_trigger_for_orderbook(trigger_name text)
RETURNS VOID AS
$BODY$
DECLARE
  listener_count integer;
  trigger_truncated_name text := LEFT(format('t_%s', trigger_name), 63);
  channel_truncated_name text := LEFT(format('c_%s', trigger_name), 63);
BEGIN
  SELECT count(*) INTO listener_count FROM pg_stat_activity WHERE lower(query) LIKE '%listen%'|| channel_truncated_name ||'%';
  IF listener_count = 0 THEN
    EXECUTE format('
      DROP TRIGGER %s ON spot.orders;', 
      trigger_truncated_name);
  END IF;
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION create_spot_trades_notify_trigger_for_taker(trigger_name text, taker_id uuid)
RETURNS VOID AS
$BODY$
DECLARE
  trigger_truncated_name text := LEFT(format('t_%s', trigger_name), 63);
  channel_truncated_name text := LEFT(format('c_%s', trigger_name), 63);
BEGIN
  EXECUTE format('
    CREATE OR REPLACE TRIGGER %s
    AFTER INSERT OR UPDATE ON spot.trades
    FOR EACH ROW
    WHEN (NEW.taker_id = ''%s'')
    EXECUTE FUNCTION notify(''%s'');', 
    trigger_truncated_name, taker_id::text, channel_truncated_name);
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION drop_spot_trades_notify_trigger_for_taker(trigger_name text)
RETURNS VOID AS
$BODY$
DECLARE
  listener_count integer;
  trigger_truncated_name text := LEFT(format('t_%s', trigger_name), 63);
  channel_truncated_name text := LEFT(format('c_%s', trigger_name), 63);
BEGIN
  SELECT count(*) INTO listener_count FROM pg_stat_activity WHERE lower(query) LIKE '%listen%'|| channel_truncated_name ||'%';
  IF listener_count = 0 THEN
    EXECUTE format('
      DROP TRIGGER %s ON spot.trades;', 
      trigger_truncated_name);
  END IF;
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION trade_check (p_order_id uuid, p_quote_asset_id uuid, p_base_asset_id uuid)
RETURNS BOOLEAN AS 
$BODY$
BEGIN
  RETURN EXISTS (
    SELECT 1 FROM spot.orders 
    WHERE spot.orders.id = p_order_id 
    AND spot.orders.quote_asset_id = p_quote_asset_id 
    AND spot.orders.base_asset_id = p_base_asset_id);
END;
$BODY$ LANGUAGE plpgsql;


CREATE FUNCTION create_spot_trades_notify_trigger_for_asset_pair(trigger_name text, quote_asset_id uuid, base_asset_id uuid)
RETURNS VOID AS
$BODY$
DECLARE
  trigger_truncated_name text := LEFT(format('t_%s', trigger_name), 63);
  channel_truncated_name text := LEFT(format('c_%s', trigger_name), 63);
BEGIN
  EXECUTE format('
    CREATE OR REPLACE TRIGGER %s
    AFTER INSERT OR UPDATE ON spot.trades
    FOR EACH ROW
    WHEN (trade_check(NEW.order_id,''%s'',''%s''))
    EXECUTE FUNCTION notify(''%s'');',
    trigger_truncated_name, quote_asset_id::text, base_asset_id::text, channel_truncated_name);
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION drop_spot_trades_notify_trigger_for_asset_pair(trigger_name text)
RETURNS VOID AS
$BODY$
DECLARE
  listener_count integer;
  trigger_truncated_name text := LEFT(format('t_%s', trigger_name), 63);
  channel_truncated_name text := LEFT(format('c_%s', trigger_name), 63);
BEGIN
  SELECT count(*) INTO listener_count FROM pg_stat_activity WHERE lower(query) LIKE '%listen%'|| channel_truncated_name ||'%';
  IF listener_count = 0 THEN
    EXECUTE format('
      DROP TRIGGER %s ON spot.trades;', 
      trigger_truncated_name);
  END IF;
END;
$BODY$ LANGUAGE plpgsql;