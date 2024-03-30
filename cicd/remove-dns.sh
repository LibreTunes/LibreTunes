#!/bin/sh

set -e

ZONE_ID=$1
RECORD_NAME=$2
RECORD_COMMENT=$3
API_TOKEN=$4

RECORD_ID=$(
curl --request GET --silent \
  --url "https://api.cloudflare.com/client/v4/zones/$ZONE_ID/dns_records?name=$RECORD_NAME&comment=$RECORD_COMMENT" \
  --header "Content-Type: application/json" \
  --header "Authorization: Bearer $API_TOKEN" \
| jq -r '.result[0].id')

echo "Deleting DNS record ID $RECORD_ID"

curl --request DELETE --silent \
  --url "https://api.cloudflare.com/client/v4/zones/$ZONE_ID/dns_records/$RECORD_ID" \
  --header "Content-Type: application/json" \
  --header  "Authorization: Bearer $API_TOKEN"
