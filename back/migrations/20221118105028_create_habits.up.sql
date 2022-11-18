create table habits (
  id text primary key,
  name text not null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  archived_at timestamptz,
  user_id text not null references users(id) on delete cascade
);
