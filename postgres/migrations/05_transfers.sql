CREATE TABLE "transfers"."transfer" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "from_vault_id" uuid NOT NULL,
  "to_vault_id" uuid NOT NULL,
  "fee_harvester_user_id" uuid NOT NULL,
  "asset_id" uuid NOT NULL,
  "amount" fraction NOT NULL,
  "fee" fraction NOT NULL
);
ALTER TABLE "transfers"."transfer" ADD FOREIGN KEY ("from_vault_id") REFERENCES "vaults"."vault" ("id");
ALTER TABLE "transfers"."transfer" ADD FOREIGN KEY ("to_vault_id") REFERENCES "vaults"."vault" ("id");
ALTER TABLE "transfers"."transfer" ADD FOREIGN KEY ("fee_harvester_user_id") REFERENCES "users"."user" ("id");
ALTER TABLE "transfers"."transfer" ADD FOREIGN KEY ("asset_id") REFERENCES "assets"."asset" ("id");
CREATE OR REPLACE FUNCTION transfers_transfer_changed_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"TransfersTransfer"';
  PERFORM notify_worker(val);
  PERFORM notify_engagement(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER spot_order_changed
AFTER INSERT OR UPDATE ON "transfers"."transfer"
FOR EACH STATEMENT EXECUTE FUNCTION transfers_transfer_changed_trigger();
