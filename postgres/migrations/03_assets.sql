CREATE TABLE "assets"."asset" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "network_id" uuid NOT NULL,
  "icon_path" VARCHAR(50) UNIQUE NOT NULL,
  "address" VARCHAR(256) UNIQUE NOT NULL,
);
ALTER TABLE "assets"."asset" ADD FOREIGN KEY ("network_id") REFERENCES "networks"."network" ("id");
CREATE OR REPLACE FUNCTION assets_asset_changed_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"AssetsAsset"';
  PERFORM notify_worker(val);
  PERFORM notify_engagement(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER assets_asset_changed
AFTER INSERT OR UPDATE ON "assets"."asset"
FOR EACH STATEMENT EXECUTE FUNCTION assets_asset_changed_trigger();

CREATE TABLE "assets"."deposit" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "owner" CHAR(42) NOT NULL,
  "spender" CHAR(42) NOT NULL,
  "asset" CHAR(42) NOT NULL,
  "amount" fraction NOT NULL,
  "tx_hash" CHAR(66) NOT NULL,
  "confirmations" fraction NOT NULL
);
ALTER TABLE "assets"."deposit" ADD FOREIGN KEY ("spender") REFERENCES "users"."user" ("address");
ALTER TABLE "assets"."deposit" ADD FOREIGN KEY ("asset") REFERENCES "assets"."assets" ("address");
CREATE OR REPLACE FUNCTION assets_deposit_changed_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"AssetsDeposit"';
  PERFORM notify_worker(val);
  PERFORM notify_engagement(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER assets_deposit_changed
AFTER INSERT OR UPDATE ON "assets"."deposit"
FOR EACH STATEMENT EXECUTE FUNCTION assets_deposit_changed_trigger();

CREATE TABLE "assets"."withdraw" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "owner" CHAR(42) NOT NULL,
  "spender" CHAR(42) NOT NULL,
  "asset" CHAR(42) NOT NULL,
  "amount" fraction NOT NULL,
  "nonce"  fraction NOT NULL,
  "deadline" TIMESTAMP(6) WITH TIME ZONE NOT NULL
);
ALTER TABLE "assets"."withdraw" ADD FOREIGN KEY ("spender") REFERENCES "users" ("address");
ALTER TABLE "assets"."withdraw" ADD FOREIGN KEY ("asset") REFERENCES "assets" ("address");
CREATE OR REPLACE FUNCTION assets_withdraw_changed_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"AssetsWithdraw"';
  PERFORM notify_worker(val);
  PERFORM notify_engagement(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER assets_withdraw_changed
AFTER INSERT OR UPDATE ON "assets"."withdraw"
FOR EACH STATEMENT EXECUTE FUNCTION assets_withdraw_changed_trigger();

CREATE TABLE "assets"."evm"."metadata" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "asset_id" uuid NOT NULL,
  "name" VARCHAR(128),
  "symbol" VARCHAR(8),
  "decimals" text
);
ALTER TABLE "assets"."evm"."metadata" ADD FOREIGN KEY ("asset_id") REFERENCES "assets"."asset" ("id");
CREATE OR REPLACE FUNCTION assets_evm_metadata_changed_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"AssetsEvmMetadata"';
  PERFORM notify_worker(val);
  PERFORM notify_engagement(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER assets_evm_metadata_changed
AFTER INSERT OR UPDATE ON "assets"."evm"."metadata"
FOR EACH STATEMENT EXECUTE FUNCTION assets_evm_metadata_changed_trigger();
