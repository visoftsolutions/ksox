CREATE TABLE "assets"."asset" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "name" VARCHAR(50) UNIQUE
  "network_id" uuid NOT NULL
  "icon_path" VARCHAR(50) UNIQUE
);

ALTER TABLE "assets"."asset" ADD FOREIGN KEY ("network_id") REFERENCES "networks"."network" ("id");
