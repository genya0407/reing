CREATE TABLE answers (
    id serial primary key,
    question_id int references questions (id) not null,
    body text not null,
    created_at timestamp with time zone not null default CURRENT_TIMESTAMP
);