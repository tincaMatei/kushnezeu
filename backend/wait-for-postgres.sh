#!/bin/sh
until PGPASSWORD=password psql -h "database" -U "postgres" -c '\q'; do
    sleep 1
done

DATABASE_URL=postgres://postgres:password@database:5432 MY_ADDRESS=0.0.0.0:8080 ./backend
