CREATE TABLE "spot"."order" (
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
ALTER TABLE "spot"."order" ADD FOREIGN KEY ("maker_id") REFERENCES "users"."user" ("id");
ALTER TABLE "spot"."order" ADD FOREIGN KEY ("quote_asset_id") REFERENCES "assets"."asset" ("id");
ALTER TABLE "spot"."order" ADD FOREIGN KEY ("base_asset_id") REFERENCES "assets"."asset" ("id");
CREATE OR REPLACE FUNCTION spot_order_changed_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"SpotOrder"';
  PERFORM notify_worker(val);
  PERFORM notify_engagement(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER spot_order_changed
AFTER INSERT OR UPDATE ON "networks"."family"
FOR EACH STATEMENT EXECUTE FUNCTION spot_order_changed_trigger();

CREATE TABLE "spot"."trade" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "quote_asset_id" uuid NOT NULL,
  "base_asset_id" uuid NOT NULL,
  "taker_id" uuid NOT NULL,
  "maker_id" uuid NOT NULL,
  "order_id" uuid NOT NULL,
  "taker_presentation" BOOLEAN NOT NULL DEFAULT false,
  "price" fraction NOT NULL,
  "taker_quote_volume" fraction NOT NULL,
  "maker_quote_volume" fraction NOT NULL,
  "taker_quote_volume_transfer_id" uuid NOT NULL,
  "maker_quote_volume_transfer_id" uuid NOT NULL
);
ALTER TABLE "spot"."trade" ADD FOREIGN KEY ("taker_id") REFERENCES "users"."user" ("id");
ALTER TABLE "spot"."trade" ADD FOREIGN KEY ("order_id") REFERENCES "spot"."order" ("id");
ALTER TABLE "spot"."trade" ADD FOREIGN KEY ("quote_asset_id") REFERENCES "assets"."asset" ("id");
ALTER TABLE "spot"."trade" ADD FOREIGN KEY ("base_asset_id") REFERENCES "assets"."asset" ("id");
ALTER TABLE "spot"."trade" ADD FOREIGN KEY ("taker_quote_volume_transfer_id") REFERENCES "transfers"."transfer" ("id");
ALTER TABLE "spot"."trade" ADD FOREIGN KEY ("maker_quote_volume_transfer_id") REFERENCES "transfers"."transfer" ("id");
CREATE OR REPLACE FUNCTION spot_trade_changed_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"SpotTrade"';
  PERFORM notify_worker(val);
  PERFORM notify_engagement(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER spot_trade_changed
AFTER INSERT OR UPDATE ON "networks"."family"
FOR EACH STATEMENT EXECUTE FUNCTION spot_trade_changed_trigger();

CREATE TABLE "spot"."candlestick" (
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
ALTER TABLE "spot"."candlestick" ADD FOREIGN KEY ("quote_asset_id") REFERENCES "assets"."asset" ("id");
ALTER TABLE "spot"."candlestick" ADD FOREIGN KEY ("base_asset_id") REFERENCES "assets"."asset" ("id");
CREATE OR REPLACE FUNCTION spot_candlestick_changed_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"SpotCandlestick"';
  PERFORM notify_worker(val);
  PERFORM notify_engagement(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER spot_candlestick_changed
AFTER INSERT OR UPDATE ON "networks"."family"
FOR EACH STATEMENT EXECUTE FUNCTION spot_candlestick_changed_trigger();
