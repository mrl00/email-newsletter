-- Add migration script here
CREATE TABLE
  subscriptions (
    sk_subscription uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL
  );
