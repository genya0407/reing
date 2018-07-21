PRODUCTION_DATABASE_URL=postgresql://postgres:postgres@127.0.0.1:5432/reing
LOCAL_DATABASE_URL=postgres://reing:reing@127.0.0.1:5432/reing
psql $LOCAL_DATABASE_URL -c "
drop schema public cascade;
create schema public;
"
ssh kuminecraft.xyz pg_dump -Ft $PRODUCTION_DATABASE_URL | pg_restore -d $LOCAL_DATABASE_URL
