#!/bin/sh
set -e

if [ ! -s /tmp/supervisor.sock ]; then
  echo "Starting supervisord"
  supervisord
fi

exec "$@"
