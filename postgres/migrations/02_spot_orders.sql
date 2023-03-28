CREATE FUNCTION spot_orders_notify()
RETURNS TRIGGER AS
$BODY$
DECLARE
  row_data json;
BEGIN 
  SELECT row_to_json(r) INTO row_data
  FROM (
    SELECT 
      NEW.id, 
      NEW.created_at, 
      NEW.user_id, 
      NEW.is_active, 
      NEW.quote_asset_id, 
      NEW.base_asset_id, 
      NEW.quote_asset_volume::json, 
      NEW.base_asset_volume::json, 
      NEW.quote_asset_volume_left::json, 
      NEW.maker_fee::json
    ) AS r;
  PERFORM pg_notify(TG_ARGV[0], row_data::text);
  RETURN NEW;
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
    EXECUTE FUNCTION spot_orders_notify(''%s'');', 
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
    EXECUTE FUNCTION spot_orders_notify(''%s'');', 
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
    EXECUTE FUNCTION spot_orders_notify(''%s'');', 
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

CREATE OR REPLACE FUNCTION spot_orders_orderbook_notify()
RETURNS TRIGGER AS
$BODY$
DECLARE
  row_data json;
BEGIN
  SELECT json_agg(row_to_json(sub.*)) INTO row_data
      FROM (
        SELECT
            ROUND(CAST(base_asset_volume/quote_asset_volume AS NUMERIC), CAST(TG_ARGV[1] AS INTEGER))::float as price,
            SUM(quote_asset_volume_left)::text as volume
        FROM spot.orders
        WHERE quote_asset_id = NEW.quote_asset_id
        AND base_asset_id = NEW.base_asset_id
        AND is_active = true
        GROUP BY price
        ORDER BY price
        LIMIT CAST(TG_ARGV[2] AS BIGINT)
    ) sub;

  PERFORM pg_notify(TG_ARGV[0], row_data::text);
  RETURN NEW;
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION create_spot_orders_notify_trigger_for_orderbook(trigger_name text, quote_asset_id uuid, base_asset_id uuid, precission INTEGER, _limit BIGINT)
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
    EXECUTE FUNCTION spot_orders_orderbook_notify(''%s'', %s, %s);', 
    trigger_truncated_name, quote_asset_id::text, base_asset_id::text, channel_truncated_name, precission, _limit);
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
