CREATE TABLE "vaults"."vault" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "user_id" uuid NOT NULL,
  "asset_id" uuid NOT NULL,
  "balance" text NOT NULL,
  UNIQUE ("user_id", "asset_id")
);
ALTER TABLE "vaults"."vault" ADD FOREIGN KEY ("user_id") REFERENCES "users"."user" ("id");
ALTER TABLE "vaults"."vault" ADD FOREIGN KEY ("asset_id") REFERENCES "assets"."asset" ("id");
CREATE OR REPLACE FUNCTION vaults_vault_changed_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"VaultsVault"';
  PERFORM notify_worker(val);
  PERFORM notify_engagement(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER vaults_vault_changed
AFTER INSERT OR UPDATE ON "vaults"."vault"
FOR EACH STATEMENT EXECUTE FUNCTION vaults_vault_changed_trigger();
