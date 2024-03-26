#!/bin/sh

set -e

TUNNEL_ID=$1
HOSTNAME=$2
SERVICE=$3

echo "Creating tunnel config for $HOSTNAME"

cat <<EOF > cloudflared-tunnel-config.yml
tunnel: $TUNNEL_ID
credentials-file: /etc/cloudflared/auth.json

ingress:
  - hostname: $HOSTNAME
    service: $SERVICE
  - service: http_status:404
EOF
