CREATE TABLE "engagement"."badges" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "user_id" uuid NOT NULL,
  "badge_name" VARCHAR(50) NOT NULL
);

ALTER TABLE "engagement"."badges" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

CREATE OR REPLACE TRIGGER engagement_badges_update_last_modification_at
AFTER INSERT OR UPDATE ON "engagement"."badges"
FOR EACH STATEMENT EXECUTE FUNCTION update_last_modification_at('"EngagementBadgesChanged"');