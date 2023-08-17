drop table if exists course;


create table course (
                        id serial primary key,
                        teacher_id INT not null,
                        name varchar(140) not null,
                        time TIMESTAMP default now()
);


insert into course (id, teacher_id, name, time)
values(1,
       1,
       'First course',
       '2022-01-17 05:40:00');


insert into course (id, teacher_id, name, time)
values(2,
       1,
       'Second course',
       '2022-01-18 05:45:00');

-- ////////////////////////////////

drop table if exists teacher;


create table teacher (
                         id serial primary key,
                         name varchar(140) not null,
                         picture_url varchar(140) not null,
                         profile varchar(140)
);


insert into teacher (id, name, picture_url, profile)
values(1,
       'name test',
       'http://xxx.png',
       'profile_test1');


insert into teacher (id, name, picture_url, profile)
values(2,
       'name test',
       'http://xxx.png',
       'profile_test2');