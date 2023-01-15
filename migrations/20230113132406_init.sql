-- Add migration script here

CREATE DOMAIN evm_address AS char(42);

CREATE TABLE "users" (
  "id" uuid PRIMARY KEY NOT NULL,
  "created_at" timestamptz NOT NULL,
  "address" evm_address UNIQUE NOT NULL
);

CREATE SCHEMA "spot";

CREATE TYPE "spot"."products_status" AS ENUM (
  'active',
  'partially_filled',
  'filled',
  'cancelled'
);

CREATE TABLE "spot"."valuts" (
  "id" uuid PRIMARY KEY NOT NULL,
  "user_id" uuid NOT NULL,
  "asset_id" uuid NOT NULL,
  "balance" money NOT NULL
);

CREATE TABLE "spot"."assets" (
  "id" uuid PRIMARY KEY NOT NULL,
  "created_at" timestamptz NOT NULL,
  "name" varchar NOT NULL,
  "symbol" varchar NOT NULL
);

CREATE TABLE "spot"."orders" (
  "id" uuid PRIMARY KEY NOT NULL,
  "created_at" timestamptz NOT NULL,
  "user_id" uuid NOT NULL,
  "status" spot.products_status NOT NULL,
  "quote_asset_id" uuid NOT NULL,
  "base_asset_id" uuid NOT NULL,
  "quote_asset_volume" money NOT NULL,
  "base_asset_price" float8 NOT NULL
);

CREATE TABLE "spot"."trades" (
  "id" uuid PRIMARY KEY NOT NULL,
  "created_at" timestamptz NOT NULL,
  "taker_id" uuid NOT NULL,
  "order_id" uuid NOT NULL,
  "taker_quote_volume" money NOT NULL,
  "taker_base_volume" money NOT NULL,
  "maker_quote_volume" money NOT NULL,
  "maker_base_volume" money NOT NULL
);

ALTER TABLE "spot"."valuts" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "spot"."valuts" ADD FOREIGN KEY ("asset_id") REFERENCES "spot"."assets" ("id");

ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("quote_asset_id") REFERENCES "spot"."assets" ("id");

ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("base_asset_id") REFERENCES "spot"."assets" ("id");

ALTER TABLE "spot"."trades" ADD FOREIGN KEY ("taker_id") REFERENCES "users" ("id");

ALTER TABLE "spot"."trades" ADD FOREIGN KEY ("order_id") REFERENCES "spot"."orders" ("id");
