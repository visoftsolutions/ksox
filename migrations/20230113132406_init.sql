-- Add migration script here

CREATE TABLE "users" (
  "id" uuid PRIMARY KEY,
  "created_at" timestamptz NOT NULL,
  "address" char(42) UNIQUE NOT NULL
);

CREATE SCHEMA "spot";

CREATE TYPE "spot"."products_status" AS ENUM (
  'active',
  'partially_filled',
  'filled',
  'cancelled'
);

CREATE TABLE "spot"."valuts" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid,
  "asset_id" uuid,
  "balance" decimal
);

CREATE TABLE "spot"."assets" (
  "id" uuid PRIMARY KEY,
  "created_at" timestamp,
  "name" varchar,
  "symbol" varchar
);

CREATE TABLE "spot"."orders" (
  "id" uuid PRIMARY KEY,
  "created_at" timestamp,
  "user_id" uuid,
  "status" spot.products_status,
  "quote_asset_id" uuid,
  "base_asset_id" uuid,
  "quote_asset_volume" decimal,
  "base_asset_price" float8
);

CREATE TABLE "spot"."trades" (
  "id" uuid PRIMARY KEY,
  "created_at" timestamp,
  "taker_id" uuid,
  "order_id" uuid,
  "taker_quote_volume" decimal,
  "taker_base_volume" decimal,
  "maker_quote_volume" decimal,
  "maker_base_volume" decimal
);

ALTER TABLE "spot"."valuts" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "spot"."valuts" ADD FOREIGN KEY ("asset_id") REFERENCES "spot"."assets" ("id");

ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("quote_asset_id") REFERENCES "spot"."assets" ("id");

ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("base_asset_id") REFERENCES "spot"."assets" ("id");

ALTER TABLE "spot"."trades" ADD FOREIGN KEY ("taker_id") REFERENCES "users" ("id");

ALTER TABLE "spot"."trades" ADD FOREIGN KEY ("order_id") REFERENCES "spot"."orders" ("id");
