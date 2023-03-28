CREATE FUNCTION spot_valuts_notify()
RETURNS TRIGGER AS
$BODY$
DECLARE
  row_data json;
BEGIN 
  SELECT row_to_json(r) INTO row_data
  FROM (
    SELECT 
      NEW.id,
      NEW.user_id,
      NEW.asset_id,
      NEW.balance::json
    ) AS r;
  PERFORM pg_notify(TG_ARGV[0], row_data::text);
  RETURN NEW;
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION create_spot_valuts_notify_trigger_for_user_asset(trigger_name text, user_id uuid, asset_id uuid)
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
    WHEN (NEW.user_id = ''%s'' AND NEW.asset_id = ''%s'')
    EXECUTE FUNCTION spot_valuts_notify(''%s'');', 
    trigger_truncated_name, user_id::text, asset_id::text, channel_truncated_name);
END;
$BODY$ LANGUAGE plpgsql;

CREATE FUNCTION drop_spot_valuts_notify_trigger_for_user_asset(trigger_name text)
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
