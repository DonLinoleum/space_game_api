-- Add migration script here
create table scores(
    id serial primary key,
    name varchar(100),
    level integer not null,
    scores integer not null,
    created timestamptz default now()
);