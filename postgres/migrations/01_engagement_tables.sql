CREATE TABLE "engagement"."badges" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "name" VARCHAR(50) NOT NULL,
  "family" VARCHAR(25) NOT NULL,
  "description" VARCHAR(255) NOT NULL,
  "value" INTEGER NOT NULL,
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "engagement"."assigned_badges" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "badge_id" uuid NOT NULL,
  "user_id" uuid NOT NULL,
  "assigned_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE "engagement"."assigned_badges" ADD FOREIGN KEY ("badge_id") REFERENCES "engagement"."badges" ("id");

ALTER TABLE "engagement"."assigned_badges" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

CREATE OR REPLACE TRIGGER engagement_badges_update_last_modification_at
AFTER INSERT OR UPDATE ON "engagement"."assigned_badges"
FOR EACH STATEMENT EXECUTE FUNCTION update_last_modification_at('"EngagementBadgesChanged"');