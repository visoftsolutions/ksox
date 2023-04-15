-- Set max connections
ALTER SYSTEM SET max_connections = 50;

-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE fraction AS (
    numer NUMERIC(78),
    denom NUMERIC(78)
);

CREATE TYPE candlestick_type AS ENUM (
  'interval', 'tick'
);

-- Define a JSON cast for the fraction type
CREATE OR REPLACE FUNCTION to_json(fraction)
RETURNS json AS $$
  SELECT json_build_object(
    'numer', $1.numer::text,
    'denom', $1.denom::text
  );
$$ LANGUAGE sql IMMUTABLE;

CREATE FUNCTION fraction_le(fraction, fraction) RETURNS boolean AS $$
  SELECT ($1.numer * $2.denom) <= ($2.numer * $1.denom);
$$ LANGUAGE SQL;

CREATE FUNCTION fraction_ge(fraction, fraction) RETURNS boolean AS $$
  SELECT ($1.numer * $2.denom) >= ($2.numer * $1.denom);
$$ LANGUAGE SQL;

CREATE OPERATOR <= (
  PROCEDURE = fraction_le,
  LEFTARG = fraction,
  RIGHTARG = fraction
);

CREATE OPERATOR >= (
  PROCEDURE = fraction_ge,
  LEFTARG = fraction,
  RIGHTARG = fraction
);

-- Define a JSON cast for the numeric type
CREATE OR REPLACE FUNCTION to_json(numeric)
RETURNS json AS $$
  SELECT to_json($1::text)
$$ LANGUAGE sql IMMUTABLE;

CREATE CAST (fraction AS json) WITH FUNCTION to_json(fraction) AS ASSIGNMENT;

CREATE CAST (numeric AS json) WITH FUNCTION to_json(numeric) AS ASSIGNMENT;

CREATE SCHEMA "spot";
