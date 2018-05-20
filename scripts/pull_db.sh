LOCAL_DATABASE_URL=postgres://reing:reing@127.0.0.1:5432/reing
psql $LOCAL_DATABASE_URL -c "
drop schema public cascade;
create schema public;
"
PGSSLMODE=disable heroku pg:pull DATABASE_URL $LOCAL_DATABASE_URL --remote heroku
