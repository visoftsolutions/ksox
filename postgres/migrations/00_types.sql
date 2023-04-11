-- Set max connections
ALTER SYSTEM SET max_connections = 50;

-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE fraction AS (
    numerator NUMERIC(78),
    denominator NUMERIC(78)
);

CREATE TYPE candlestick_type AS ENUM (
  'interval', 'tick'
);

-- Define a JSON cast for the fraction type
CREATE OR REPLACE FUNCTION to_json(fraction)
RETURNS json AS $$
  SELECT json_build_object(
    'numerator', $1.numerator::text,
    'denominator', $1.denominator::text
  );
$$ LANGUAGE sql IMMUTABLE;

-- Define a JSON cast for the numeric type
CREATE OR REPLACE FUNCTION to_json(numeric)
RETURNS json AS $$
  SELECT to_json($1::text)
$$ LANGUAGE sql IMMUTABLE;

CREATE CAST (fraction AS json) WITH FUNCTION to_json(fraction) AS ASSIGNMENT;

CREATE CAST (numeric AS json) WITH FUNCTION to_json(numeric) AS ASSIGNMENT;

CREATE SCHEMA "spot";
