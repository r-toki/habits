create table users (
  id varchar(255) primary key,
  name varchar(255) unique not null,
  created_at timestamp not null,
  updated_at timestamp not null
);
