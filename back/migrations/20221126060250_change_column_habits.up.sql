begin;
alter table habits
alter column sort_number type bigint;

update habits
set sort_number = extract('epoch' from habits.created_at)::bigint;
commit;
