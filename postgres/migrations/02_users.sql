CREATE TABLE "users"."user" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "comission_id" uuid NOT NULL
);
ALTER TABLE "users"."user" ADD FOREIGN KEY ("comission_id") REFERENCES "fees"."revenue" ("id");
CREATE OR REPLACE FUNCTION users_user_changed_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"UsersUser"';
  PERFORM notify_worker(val);
  PERFORM notify_engagement(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER users_user_changed
AFTER INSERT OR UPDATE ON "users"."user"
FOR EACH STATEMENT EXECUTE FUNCTION users_user_changed_trigger();

CREATE TABLE "users"."address" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "network_id" uuid NOT NULL
  "address" VARCHAR(256) UNIQUE NOT NULL
);
ALTER TABLE "users"."address" ADD FOREIGN KEY ("network_id") REFERENCES "networks"."network" ("id");
CREATE OR REPLACE FUNCTION users_address_changed_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"UsersAddress"';
  PERFORM notify_worker(val);
  PERFORM notify_engagement(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER users_address_changed
AFTER INSERT OR UPDATE ON "users"."address"
FOR EACH STATEMENT EXECUTE FUNCTION users_address_changed_trigger();

CREATE TABLE "users"."username" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "user_id" uuid NOT NULL
  "username" CHAR(50) UNIQUE NOT NULL
);
ALTER TABLE "users"."username" ADD FOREIGN KEY ("user_id") REFERENCES "users"."user" ("id");
CREATE OR REPLACE FUNCTION users_username_changed_trigger() 
RETURNS TRIGGER AS $$
DECLARE
  val text;
BEGIN
  val := '"UsersUsername"';
  PERFORM notify_worker(val);
  PERFORM notify_engagement(val);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER users_username_changed
AFTER INSERT OR UPDATE ON "users"."username"
FOR EACH STATEMENT EXECUTE FUNCTION users_username_changed_trigger();
