#!/bin/bash

echo $ls
sqlite3 db/movie_site.db '.quit'
sqlite3 db/movie_site.db < db/clear_table.sql
sqlite3 db/movie_site.db < db/db_setup.sql
touch .env
echo 'RUST_LOG=info' >> .env
echo 'DATABASE_URL=sqlite:///$PWD/db/movie_site.db' >> .env
echo 'DEV_SECRET="Umm, just a secret for testing and stuff, but Ill delete it later"' >> .env
echo 'ACCESS_TOKEN_EXP=180000' >> .env
echo 'REFRESH_TOKEN_EXP=6000000' >> .env
echo 'CORS_ORIGIN=http://0.0.0.0:8080' >> .env
echo 'ROOT_URL=http://0.0.0.0:3030' >> .env

