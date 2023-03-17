
CREATE TABLE "users" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "address" CHAR(42) UNIQUE NOT NULL
);

CREATE TABLE "spot"."valuts" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "user_id" uuid NOT NULL,
  "asset_id" uuid NOT NULL,
  "balance" NUMERIC(78) NOT NULL CHECK ("balance" >= 0) DEFAULT 0,
  UNIQUE ("user_id", "asset_id")
);

CREATE TABLE "spot"."assets" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "name" VARCHAR NOT NULL,
  "symbol" VARCHAR NOT NULL,
  "maker_fee" fraction NOT NULL,
  "taker_fee" fraction NOT NULL
);

CREATE TABLE "spot"."orders" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "user_id" uuid NOT NULL,
  "is_active" BOOLEAN NOT NULL DEFAULT true,
  "quote_asset_id" uuid NOT NULL,
  "base_asset_id" uuid NOT NULL,
  "quote_asset_volume" NUMERIC(78) NOT NULL CHECK ("quote_asset_volume" >= 0),
  "base_asset_volume" NUMERIC(78) NOT NULL CHECK ("base_asset_volume" >= 0),
  "quote_asset_volume_left" NUMERIC(78) NOT NULL CHECK ("quote_asset_volume_left" >= 0) CHECK ("quote_asset_volume_left" <= "quote_asset_volume"),
  "maker_fee" fraction NOT NULL
);

CREATE TABLE "spot"."trades" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "taker_id" uuid NOT NULL,
  "order_id" uuid NOT NULL,
  "taker_quote_volume" NUMERIC(78) NOT NULL CHECK ("taker_quote_volume" >= 0),
  "taker_base_volume" NUMERIC(78) NOT NULL CHECK ("taker_base_volume" >= 0),
  "maker_quote_volume" NUMERIC(78) NOT NULL CHECK ("maker_quote_volume" >= 0),
  "maker_base_volume" NUMERIC(78) NOT NULL CHECK ("maker_base_volume" >= 0)
);

CREATE TABLE "spot"."candlesticks" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "quote_asset_id" uuid NOT NULL,
  "base_asset_id" uuid NOT NULL,
  "kind" candlestick_type NOT NULL,
  "topen" TIMESTAMPTZ NOT NULL,
  "tclose" TIMESTAMPTZ NOT NULL,
  "open" fraction NOT NULL,
  "high" fraction NOT NULL,
  "low" fraction NOT NULL,
  "close" fraction NOT NULL,
  "span" NUMERIC(78) NOT NULL CHECK ("span" >= 0),
  "taker_quote_volume" NUMERIC(78) NOT NULL CHECK ("taker_quote_volume" >= 0),
  "taker_base_volume" NUMERIC(78) NOT NULL CHECK ("taker_base_volume" >= 0),
  "maker_quote_volume" NUMERIC(78) NOT NULL CHECK ("maker_quote_volume" >= 0),
  "maker_base_volume" NUMERIC(78) NOT NULL CHECK ("maker_base_volume" >= 0)
);

ALTER TABLE "spot"."valuts" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "spot"."valuts" ADD FOREIGN KEY ("asset_id") REFERENCES "spot"."assets" ("id");

ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("quote_asset_id") REFERENCES "spot"."assets" ("id");

ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("base_asset_id") REFERENCES "spot"."assets" ("id");

ALTER TABLE "spot"."trades" ADD FOREIGN KEY ("taker_id") REFERENCES "users" ("id");

ALTER TABLE "spot"."trades" ADD FOREIGN KEY ("order_id") REFERENCES "spot"."orders" ("id");

ALTER TABLE "spot"."candlesticks" ADD FOREIGN KEY ("quote_asset_id") REFERENCES "spot"."assets" ("id");

ALTER TABLE "spot"."candlesticks" ADD FOREIGN KEY ("base_asset_id") REFERENCES "spot"."assets" ("id");
