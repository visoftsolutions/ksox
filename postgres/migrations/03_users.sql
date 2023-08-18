CREATE TABLE "users"."user" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "username" CHAR(50) UNIQUE NOT NULL
);

CREATE TABLE "users"."addresses" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "network_id" uuid NOT NULL
  "address" CHAR(42) UNIQUE NOT NULL
);

ALTER TABLE "users"."addresses" ADD FOREIGN KEY ("network_id") REFERENCES "networks"."network" ("id");

CREATE TABLE "users"."discord_usernames" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "user_id" uuid NOT NULL
  "username" CHAR(50) UNIQUE NOT NULL
);

ALTER TABLE "users"."discord_usernames" ADD FOREIGN KEY ("user_id") REFERENCES "users"."user" ("id");

CREATE TABLE "users"."telegram_usernames" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "last_modification_at" TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "user_id" uuid NOT NULL
  "username" CHAR(50) UNIQUE NOT NULL
);

ALTER TABLE "users"."telegram_usernames" ADD FOREIGN KEY ("user_id") REFERENCES "users"."user" ("id");
