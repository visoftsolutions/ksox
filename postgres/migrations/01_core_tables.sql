CREATE TABLE "users" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "address" CHAR(42) UNIQUE NOT NULL,
  "name" VARCHAR(50) UNIQUE,
  "phone" VARCHAR(50) UNIQUE,
  "email" VARCHAR(100) UNIQUE
);

CREATE TABLE "assets" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "name" VARCHAR NOT NULL,
  "address" CHAR(42) UNIQUE NOT NULL,
  "decimals" fraction NOT NULL,
  "symbol" VARCHAR NOT NULL,
  "maker_fee" fraction NOT NULL,
  "taker_fee" fraction NOT NULL
);

CREATE TABLE valuts (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "user_id" uuid NOT NULL,
  "asset_id" uuid NOT NULL,
  "balance" fraction NOT NULL,
  UNIQUE ("user_id", "asset_id")
);

ALTER TABLE "valuts" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");
ALTER TABLE "valuts" ADD FOREIGN KEY ("asset_id") REFERENCES "assets" ("id");

CREATE TABLE "deposits" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "user_id" uuid NOT NULL,
  "asset_id" uuid NOT NULL,
  "tx_hash" CHAR(66) UNIQUE NOT NULL,
  "amount" fraction NOT NULL,
  "confirmations" fraction NOT NULL
);

ALTER TABLE "deposits" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");
ALTER TABLE "deposits" ADD FOREIGN KEY ("asset_id") REFERENCES "assets" ("id");

CREATE TABLE "withdraws" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "user_id" uuid NOT NULL,
  "asset_id" uuid NOT NULL,
  "tx_hash" CHAR(66) UNIQUE NOT NULL,
  "amount" fraction NOT NULL,
  "confirmations" fraction NOT NULL
);

ALTER TABLE "withdraws" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");
ALTER TABLE "withdraws" ADD FOREIGN KEY ("asset_id") REFERENCES "assets" ("id");

CREATE TABLE "transfers" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "maker_id" uuid NOT NULL,
  "taker_id" uuid NOT NULL,
  "asset_id" uuid NOT NULL,
  "amount" fraction NOT NULL
);

ALTER TABLE "transfers" ADD FOREIGN KEY ("maker_id") REFERENCES "users" ("id");
ALTER TABLE "transfers" ADD FOREIGN KEY ("taker_id") REFERENCES "users" ("id");
ALTER TABLE "transfers" ADD FOREIGN KEY ("asset_id") REFERENCES "assets" ("id");

CREATE TABLE "spot"."orders" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "maker_id" uuid NOT NULL,
  "maker_presentation" BOOLEAN NOT NULL DEFAULT false,
  "is_active" BOOLEAN NOT NULL DEFAULT true,
  "quote_asset_id" uuid NOT NULL,
  "base_asset_id" uuid NOT NULL,
  "price" fraction NOT NULL,
  "quote_asset_volume" fraction NOT NULL,
  "quote_asset_volume_left" fraction NOT NULL,
  "maker_fee" fraction NOT NULL
);

ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("maker_id") REFERENCES "users" ("id");
ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("quote_asset_id") REFERENCES "assets" ("id");
ALTER TABLE "spot"."orders" ADD FOREIGN KEY ("base_asset_id") REFERENCES "assets" ("id");

CREATE TABLE "spot"."trades" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "quote_asset_id" uuid NOT NULL,
  "base_asset_id" uuid NOT NULL,
  "order_id" uuid NOT NULL,
  "taker_id" uuid NOT NULL,
  "taker_presentation" BOOLEAN NOT NULL DEFAULT false,
  "price" fraction NOT NULL,
  "taker_quote_volume" fraction NOT NULL,
  "taker_base_volume" fraction NOT NULL,
  "maker_quote_volume" fraction NOT NULL,
  "maker_base_volume" fraction NOT NULL
);

ALTER TABLE "spot"."trades" ADD FOREIGN KEY ("taker_id") REFERENCES "users" ("id");
ALTER TABLE "spot"."trades" ADD FOREIGN KEY ("order_id") REFERENCES "spot"."orders" ("id");
ALTER TABLE "spot"."trades" ADD FOREIGN KEY ("quote_asset_id") REFERENCES "assets" ("id");
ALTER TABLE "spot"."trades" ADD FOREIGN KEY ("base_asset_id") REFERENCES "assets" ("id");

CREATE TABLE "spot"."candlesticks" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "quote_asset_id" uuid NOT NULL,
  "base_asset_id" uuid NOT NULL,
  "kind" candlestick_type NOT NULL,
  "topen" TIMESTAMP(6) WITH TIME ZONE NOT NULL,
  "tclose" TIMESTAMP(6) WITH TIME ZONE NOT NULL,
  "open" fraction NOT NULL,
  "high" fraction NOT NULL,
  "low" fraction NOT NULL,
  "close" fraction NOT NULL,
  "span" BIGINT NOT NULL CHECK ("span" >= 0),
  "taker_quote_volume" fraction NOT NULL,
  "taker_base_volume" fraction NOT NULL,
  "maker_quote_volume" fraction NOT NULL,
  "maker_base_volume" fraction NOT NULL
);

ALTER TABLE "spot"."candlesticks" ADD FOREIGN KEY ("quote_asset_id") REFERENCES "assets" ("id");
ALTER TABLE "spot"."candlesticks" ADD FOREIGN KEY ("base_asset_id") REFERENCES "assets" ("id");


CREATE OR REPLACE FUNCTION update_last_modification_at()
RETURNS TRIGGER AS $$
BEGIN
   PERFORM pg_notify('notifications', TG_ARGV[0]::text);
   RETURN NEW;
END;
$$ language 'plpgsql';

CREATE OR REPLACE TRIGGER users_update_last_modification_at
AFTER INSERT OR UPDATE ON "users"
FOR EACH STATEMENT EXECUTE FUNCTION update_last_modification_at('"UsersChanged"');

CREATE OR REPLACE TRIGGER spot_assets_update_last_modification_at
AFTER INSERT OR UPDATE ON "assets"
FOR EACH STATEMENT EXECUTE FUNCTION update_last_modification_at('"SpotAssetsChanged"');

CREATE OR REPLACE TRIGGER spot_valuts_update_last_modification_at
AFTER INSERT OR UPDATE ON "valuts"
FOR EACH STATEMENT EXECUTE FUNCTION update_last_modification_at('"SpotValutsChanged"');

CREATE OR REPLACE TRIGGER transfers_update_last_modification_at
AFTER INSERT OR UPDATE ON "transfers"
FOR EACH STATEMENT EXECUTE FUNCTION update_last_modification_at('"TransfersChanged"');

CREATE OR REPLACE TRIGGER spot_orders_update_last_modification_at
AFTER INSERT OR UPDATE ON "spot"."orders"
FOR EACH STATEMENT EXECUTE FUNCTION update_last_modification_at('"SpotOrdersChanged"');

CREATE OR REPLACE TRIGGER spot_trades_update_last_modification_at
AFTER INSERT OR UPDATE ON "spot"."trades"
FOR EACH STATEMENT EXECUTE FUNCTION update_last_modification_at('"SpotTradesChanged"');

CREATE OR REPLACE TRIGGER spot_candlesticks_update_last_modification_at
AFTER INSERT OR UPDATE ON "spot"."candlesticks"
FOR EACH STATEMENT EXECUTE FUNCTION update_last_modification_at('"SpotCandlesticksChanged"');

CREATE OR REPLACE TRIGGER deposits_update_last_modification_at
AFTER INSERT OR UPDATE ON "deposits"
FOR EACH STATEMENT EXECUTE FUNCTION update_last_modification_at('"DepositsChanged"');

CREATE OR REPLACE TRIGGER withdraws_update_last_modification_at
AFTER INSERT OR UPDATE ON "withdraws"
FOR EACH STATEMENT EXECUTE FUNCTION update_last_modification_at('"WithdrawsChanged"');
