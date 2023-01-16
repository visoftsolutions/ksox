-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE DOMAIN evm_address AS CHAR(42);
CREATE DOMAIN evm_decimal AS NUMERIC(78, 18) CHECK (VALUE >= 0);

CREATE TABLE "users" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4 (),
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
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
  "balance" evm_decimal NOT NULL
);

CREATE TABLE "spot"."assets" (
  "id" uuid PRIMARY KEY NOT NULL,
  "created_at" TIMESTAMPTZ NOT NULL,
  "name" varchar NOT NULL,
  "symbol" varchar NOT NULL
);

CREATE TABLE "spot"."orders" (
  "id" uuid PRIMARY KEY NOT NULL,
  "created_at" TIMESTAMPTZ NOT NULL,
  "user_id" uuid NOT NULL,
  "status" spot.products_status NOT NULL,
  "quote_asset_id" uuid NOT NULL,
  "base_asset_id" uuid NOT NULL,
  "quote_asset_volume" evm_decimal NOT NULL,
  "base_asset_price" float8 NOT NULL
);

CREATE TABLE "spot"."trades" (
  "id" uuid PRIMARY KEY NOT NULL,
  "created_at" TIMESTAMPTZ NOT NULL,
  "taker_id" uuid NOT NULL,
  "order_id" uuid NOT NULL,
  "taker_quote_volume" evm_decimal NOT NULL,
  "taker_base_volume" evm_decimal NOT NULL,
  "maker_quote_volume" evm_decimal NOT NULL,
  "maker_base_volume" evm_decimal NOT NULL
);

ALTER TABLE "spot"."valuts" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "spot"."valuts" ADD FOREIGN KEY ("asset_id") REFERENCES "spot"."assets" ("id");

ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("quote_asset_id") REFERENCES "spot"."assets" ("id");

ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("base_asset_id") REFERENCES "spot"."assets" ("id");

ALTER TABLE "spot"."trades" ADD FOREIGN KEY ("taker_id") REFERENCES "users" ("id");

ALTER TABLE "spot"."trades" ADD FOREIGN KEY ("order_id") REFERENCES "spot"."orders" ("id");
