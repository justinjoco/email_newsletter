#!/bin/sh

SQL_HOST="localhost"
SQL_PORT="5432"
echo "Waiting for postgres..."

while ! nc -z $SQL_HOST $SQL_PORT; do
  sleep 0.1
done

echo "PostgreSQL started"
