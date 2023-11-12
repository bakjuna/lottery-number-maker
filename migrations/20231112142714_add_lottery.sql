create table lottery
(
    id         uuid                     default gen_random_uuid() not null
        constraint lottery_pk
            primary key,
    nth        integer                                            not null,
    numbers    jsonb,
    created_at timestamp with time zone default now()
);

alter table lottery
    owner to yacho;

create unique index lottery_id_uindex
    on lottery (id);

