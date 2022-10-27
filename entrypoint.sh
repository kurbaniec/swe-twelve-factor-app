#!/bin/sh
# Entrypoint for docker container execution
# -----------------------------------------
# Tensorflow path
export LD_LIBRARY_PATH=/usr/src/dogorcat-app/tf/lib
# Default environment flags
export DATABASE_URL="${DATABASE_URL:=postgres://postgres:postgres@localhost/app-db}"
export ROCKET_LOG_LEVEL="${ROCKET_LOG_LEVEL:=NORMAL}"
export ROCKET_LIMITS="${ROCKET_LIMITS:=\{form=100000000,forms=100000000,data-form=100000000,file=100000000\}}"
# Run migration when flag is set to true
if [ "$RUN_MIGRATION" = "true" ]
then
  echo "💾 Running Database Migration:"
  diesel migration run
  echo "   >> Migration successful"
fi
# Start dogorcat-service (also knows at the "twelve factor app")
twelve-factor-app