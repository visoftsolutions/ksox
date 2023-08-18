CREATE TABLE "valuts"."valut" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "user_id" uuid NOT NULL,
  "asset_id" uuid NOT NULL,
  "balance" text NOT NULL,
  UNIQUE ("user_id", "asset_id")
);

ALTER TABLE "valuts" ADD FOREIGN KEY ("user_id") REFERENCES "users"."user" ("id");
ALTER TABLE "valuts" ADD FOREIGN KEY ("asset_id") REFERENCES "assets"."asset" ("id");
