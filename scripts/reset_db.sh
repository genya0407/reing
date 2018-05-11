. ./.env

psql $DATABASE_URL -c "
DROP schema public cascade;
CREATE schema public;
"

for MIGRATION_NAME in `ls migrations`
do
    FILENAME="migrations/$MIGRATION_NAME"
    psql $DATABASE_URL < $FILENAME
done