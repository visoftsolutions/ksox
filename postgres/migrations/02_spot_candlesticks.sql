CREATE OR REPLACE FUNCTION candlesticks_notify() 
RETURNS TRIGGER AS 
$BODY$
DECLARE
  f_quote_asset_id uuid := TG_ARGV[1];
  f_base_asset_id uuid := TG_ARGV[2];
  f_kind candlestick_type := TG_ARGV[3];
  f_span INTEGER := TG_ARGV[4];
  row_data json;
BEGIN
  -- Check if the new row has the latest date compared to all other rows
  IF NEW.topen >= (
    SELECT MAX(spot.candlesticks.topen) 
    FROM spot.candlesticks
    WHERE spot.candlesticks.quote_asset_id = f_quote_asset_id
    AND spot.candlesticks.base_asset_id = f_base_asset_id
    AND spot.candlesticks.kind = f_kind
    AND spot.candlesticks.span = f_span
    ) THEN
    
    SELECT row_to_json(r) INTO row_data
    FROM (
        SELECT 
        NEW.id,
        NEW.quote_asset_id,
        NEW.base_asset_id,
        NEW.kind::json,
        NEW.topen,
        NEW.tclose,
        NEW.open::json,
        NEW.high::json,
        NEW.low::json,
        NEW.close::json,
        NEW.span::json,
        NEW.taker_quote_volume::json,
        NEW.taker_base_volume::json,
        NEW.maker_quote_volume::json,
        NEW.maker_base_volume::json
        ) AS r;
    PERFORM pg_notify(TG_ARGV[0], row_data::text);

  END IF;
  RETURN NEW;
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION create_spot_candlesticks_notify_trigger_asset_pair(trigger_name text, quote_asset_id uuid, base_asset_id uuid, kind candlestick_type, span INTEGER)
RETURNS VOID AS
$BODY$
DECLARE
  trigger_truncated_name text := LEFT(format('t_%s', trigger_name), 63);
  channel_truncated_name text := LEFT(format('c_%s', trigger_name), 63);
BEGIN
  EXECUTE format('
    CREATE OR REPLACE TRIGGER %s
    AFTER INSERT OR UPDATE ON spot.candlesticks
    FOR EACH ROW
    EXECUTE FUNCTION candlesticks_notify(''%s'', ''%s'', ''%s'', ''%s'', ''%s'');', 
    trigger_truncated_name, channel_truncated_name, quote_asset_id, base_asset_id, kind, span);
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION drop_spot_candlesticks_notify_trigger_asset_pair(trigger_name text)
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
      DROP TRIGGER %s ON spot.candlesticks;', 
      trigger_truncated_name);
  END IF;
END;
$BODY$ LANGUAGE plpgsql;