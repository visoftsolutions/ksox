CREATE FUNCTION spot_trades_notify()
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
      NEW.quote_asset_id,
      NEW.base_asset_id,
      NEW.taker_id,
      NEW.order_id,
      NEW.taker_quote_volume::json,
      NEW.taker_base_volume::json,
      NEW.maker_quote_volume::json,
      NEW.maker_base_volume::json
    ) AS r;
  PERFORM pg_notify(TG_ARGV[0], row_data::text);
  RETURN NEW;
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
    EXECUTE FUNCTION spot_trades_notify(''%s'');', 
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
    WHEN (NEW.quote_asset_id = ''%s'' AND NEW.base_asset_id = ''%s'')
    EXECUTE FUNCTION spot_trades_notify(''%s'');',
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