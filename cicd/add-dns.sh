#!/bin/sh

set -e

ZONE_ID=$1
RECORD_NAME=$2
RECORD_COMMENT=$3
API_TOKEN=$4
TUNNEL_ID=$5

curl --request POST --silent \
  --url https://api.cloudflare.com/client/v4/zones/$ZONE_ID/dns_records \
  --header 'Content-Type: application/json' \
  --header "Authorization: Bearer $API_TOKEN" \
  --data '{
  "content": "'$TUNNEL_ID'.cfargotunnel.com",
  "name": "'$RECORD_NAME'",
  "comment": "'$RECORD_COMMENT'",
  "proxied": true,
  "type": "CNAME",
  "ttl": 1
}' \
