#!/usr/bin/env bash
# Stream preconfirmations that touch Jupiter v6 from Frankfurt.
#   MANKA_API_KEY=... ./examples/grpcurl.sh
# Keys in JSON are base64 of the 32 key bytes, not base58.
set -euo pipefail
cd "$(dirname "$0")/.."
grpcurl -proto proto/sugar.proto \
  -H "x-api-key: ${MANKA_API_KEY:?set MANKA_API_KEY}" \
  -d '{"filters": {"jupiter": {"accounts": ["BHnVW/IxwG7udMVuzmgVB/2xst6j9I5RArHNola8E48="]}}}' \
  fra-preconf.manka.wtf:12349 sugar.v1.Sugar/Subscribe
