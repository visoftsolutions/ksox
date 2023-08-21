CREATE TABLE "fees"."revenue" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "payee_id" uuid NOT NULL,
  "maker" fraction NOT NULL,
  "taker" fraction NOT NULL,
  "transfer" fraction NOT NULL
);
ALTER TABLE "fees"."revenue" ADD FOREIGN KEY ("payee_id") REFERENCES "users"."user" ("id");
CREATE OR REPLACE FUNCTION fees_revenue_changed_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"FeesRevenue"';
  PERFORM notify_worker(val);
  PERFORM notify_engagement(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER fees_revenue_changed
AFTER INSERT OR UPDATE ON "fees"."revenue"
FOR EACH STATEMENT EXECUTE FUNCTION fees_revenue_changed_trigger();
