#!/bin/sh
# wait-for-postgres.sh

set -e

until diesel setup; do
  >&2 echo "diesel setup failed - database may be down - sleeping"
  sleep 1
done
  
>&2 echo "Database is up - continuing"
