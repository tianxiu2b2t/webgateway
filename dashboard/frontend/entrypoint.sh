#!/bin/sh
set -e
: "${BACKEND_URL:=http://localhost:3000}"
export BACKEND_URL
envsubst '${BACKEND_URL}' < /etc/nginx/templates/default.conf.template > /etc/nginx/conf.d/default.conf
exec "$@"