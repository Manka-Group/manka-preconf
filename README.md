# manka-preconf

The gRPC definition and examples for Manka preconfirmations: transactions the current leader has taken into its block, streamed before the block is shredded.

Full documentation: [docs.manka.wtf](https://docs.manka.wtf).

## Endpoints

| Region | Address |
| --- | --- |
| Frankfurt | `fra-preconf.manka.wtf:12349` |
| Amsterdam | `ams-preconf.manka.wtf:12349` |
| New York | `nyc-preconf.manka.wtf:12349` |

gRPC over TLS with a public certificate. Send your Manka API key in the `x-api-key` header. The server has no reflection, so use [`proto/sugar.proto`](proto/sugar.proto).

## Quick start

With grpcurl:

```bash
MANKA_API_KEY=... ./examples/grpcurl.sh
```

With Rust (needs `protoc`):

```bash
cd examples/rust
MANKA_API_KEY=... cargo run
```

## What you receive

- `SlotOpened` once per covered slot, before its first transaction, with the slot's leader.
- `Transaction` with the slot, its order of arrival in the slot, the leader, the signature, the serialized transaction, the names of your filters it matched and when we received it.

A preconfirmation carries no success or failed status: the leader sends it before running the transaction. Check the outcome at `processed` or later.

## Filters

At least one, up to 16 per stream and 1,024 keys across them. A filter matches a transaction that writes one of `writes`, mentions one of `accounts`, or is signed by one of `signers`. Keys are 32 raw bytes; in JSON they are base64.
