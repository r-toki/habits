begin;
alter table habits add column sort_number integer not null default 0;

update habits
set sort_number = sort_numbers.sort_number
from
  (
    select
      h1.id,
      (
        select count(*) from habits h2
        where h1.user_id = h2.user_id and h1.created_at < h2.created_at
      ) sort_number
    from habits h1
  ) sort_numbers
where habits.id = sort_numbers.id;
commit;
