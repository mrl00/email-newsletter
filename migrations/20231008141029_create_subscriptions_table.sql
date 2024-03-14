-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
  subscriptions (
    sk_subscription UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    tx_email TEXT NOT NULL UNIQUE,
    tx_name TEXT NOT NULL,
    dh_subscribed_at timestamptz NOT NULL
  );
