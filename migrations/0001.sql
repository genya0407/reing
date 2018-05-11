CREATE TABLE questions (
    id serial primary key,
    body text not null,
    ip_address text not null,
    hidden boolean not null default 'f',
    created_at timestamp with time zone not null default CURRENT_TIMESTAMP
);