CREATE TYPE network_type AS ENUM (
  'ethereum', 'polkadot', 'starknet'
);

CREATE TABLE "networks"."network" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "name" VARCHAR(50) UNIQUE
  "type" network_type
  "icon_path" VARCHAR(50) UNIQUE
);

CREATE TABLE "networks"."alchemy_websocket_provider" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "network_id" uuid NOT NULL
  "url" VARCHAR(50) NOT NULL
)

ALTER TABLE "networks"."alchemy_websocket_provider" ADD FOREIGN KEY ("network_id") REFERENCES "networks"."network" ("id");
