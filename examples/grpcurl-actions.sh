#!/usr/bin/env bash
# Stream new tokens on every launchpad and every pump.fun and PumpSwap buy from Frankfurt.
#   MANKA_API_KEY=... ./examples/grpcurl-actions.sh
# Actions are plain names, so nothing needs encoding. See the catalog in README.md.
set -euo pipefail
cd "$(dirname "$0")/.."
grpcurl -proto proto/sugar.proto \
  -H "x-api-key: ${MANKA_API_KEY:?set MANKA_API_KEY}" \
  -d '{"filters": {"launches": {"actions": ["create"]}, "pump_buys": {"actions": ["pump_fun.buy", "pump_swap.buy"]}}}' \
  fra-preconf.manka.wtf:12349 sugar.v1.Sugar/Subscribe
