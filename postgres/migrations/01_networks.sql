CREATE TABLE "networks"."family" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "name" VARCHAR(50) UNIQUE
);
CREATE OR REPLACE FUNCTION networks_family_changed_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"NetworksFamily"';
  PERFORM notify_worker(val);
  PERFORM notify_engagement(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER networks_family_changed
AFTER INSERT OR UPDATE ON "networks"."family"
FOR EACH STATEMENT EXECUTE FUNCTION networks_family_changed_trigger();

CREATE TABLE "networks"."network" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "family_id" uuid NOT NULL,
  "name" VARCHAR(50) UNIQUE,
  "icon_path" VARCHAR(50) UNIQUE,
  "contract_address" VARCHAR(50)
);
ALTER TABLE "networks"."network" ADD FOREIGN KEY ("family_id") REFERENCES "networks"."family" ("id");
CREATE OR REPLACE FUNCTION networks_network_changed_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"NetworksNetwork"';
  PERFORM notify_worker(val);
  PERFORM notify_engagement(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER networks_network_changed
AFTER INSERT OR UPDATE ON "networks"."network"
FOR EACH STATEMENT EXECUTE FUNCTION networks_network_changed_trigger();

CREATE TABLE "networks"."provider" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "name" VARCHAR(50) UNIQUE,
  "network_id" uuid NOT NULL,
  "url" VARCHAR(50) NOT NULL
);
ALTER TABLE "networks"."provider" ADD FOREIGN KEY ("network_id") REFERENCES "networks"."network" ("id");
CREATE OR REPLACE FUNCTION networks_provider_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"NetworksProvider"';
  PERFORM notify_worker(val);
  PERFORM notify_engagement(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER networks_provider_changed
AFTER INSERT OR UPDATE ON "networks"."provider"
FOR EACH STATEMENT EXECUTE FUNCTION networks_provider_trigger();
