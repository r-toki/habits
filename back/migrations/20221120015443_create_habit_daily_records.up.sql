create table habit_daily_records (
  id text primary key,
  done boolean not null,
  recorded_on timetz not null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  habit_id text not null references habits(id) on delete cascade,
  daily_record_id text not null references daily_records(id) on delete cascade
);
