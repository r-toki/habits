create table users (
  id text primary key,
  display_name text unique not null,
  created_at timestamptz not null,
  updated_at timestamptz not null
);
