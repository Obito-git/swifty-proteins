## TODO:

anton school pc
export LIBRARY_PATH=$LIBRARY_PATH:~/.local/Cellar/sqlite/3.46.0/lib


for deploying it in container we should install:

- curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
- command to install sqlite3 (e.g. sudo apt install libsqlite3-dev)
- cargo install diesel_cli --no-default-features --features sqlite
- in back root (or possible to put in .env) export DATABASE_URL=database/sqlite:database.db
- diesel setup