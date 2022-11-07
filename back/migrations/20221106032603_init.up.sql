create table users (
  id text primary key,
  name text unique not null,
  created_at timestamptz not null,
  updated_at timestamptz not null
);
