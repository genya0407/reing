initdb ./database --username=reing -A md5 --pwfile=./scripts/database_password
postgres -D ./database -c unix_socket_directories=/tmp
