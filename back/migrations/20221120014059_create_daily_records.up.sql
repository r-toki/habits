create table daily_records (
  id text primary key,
  comment text not null,
  recorded_on timetz not null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  user_id text not null references users(id) on delete cascade
);
