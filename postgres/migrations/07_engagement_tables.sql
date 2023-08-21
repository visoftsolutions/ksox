CREATE TABLE "engagement"."badge" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "user_id" uuid NOT NULL,
  "badge_name" VARCHAR(50) NOT NULL
);
ALTER TABLE "engagement"."badge" ADD FOREIGN KEY ("user_id") REFERENCES "users"."user" ("id");
CREATE OR REPLACE FUNCTION engagement_badge_changed_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"EngagementBadge"';
  PERFORM notify_worker(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER engagement_badge_changed
AFTER INSERT OR UPDATE ON "engagement"."badge"
FOR EACH STATEMENT EXECUTE FUNCTION engagement_badge_changed_trigger();
