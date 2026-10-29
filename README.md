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

Each region streams the leaders connected to it. Subscribe to all three for full coverage.

## Quick start

With grpcurl:

```bash
MANKA_API_KEY=... ./examples/grpcurl.sh          # everything touching Jupiter v6
MANKA_API_KEY=... ./examples/grpcurl-actions.sh  # new tokens, pump.fun and PumpSwap buys
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

The transaction may be legacy, v0 or v1. A third of mainnet traffic is v1 now; in Rust it needs `solana-message` 4.3 or newer.

## Filters

At least one, up to 16 per stream and 1,024 keys across them. A filter matches a transaction that

- writes one of `writes`,
- mentions one of `accounts`,
- is signed by one of `signers`,
- has a top-level instruction that calls one of `instructions`' programs with data starting with its `discriminator` (up to 32 bytes, empty for any instruction of the program),
- or has a top-level instruction named in `actions`.

Keys, programs and discriminators are raw bytes; in JSON they are base64. Every entry of every list counts as one key.

Only top-level instructions are matched. A trade a router or your own program makes through a cross-program invocation is not in the transaction yet; match the venue's program in `accounts` instead.

```json
{"filters": {
  "launches": {"actions": ["create"]},
  "my_program": {"instructions": [{"program": "<base64 program id>", "discriminator": "<base64 first bytes of the data>"}]}
}}
```

### Actions

A name is `<venue>.<action>` (`pump_fun.buy`), an action on every venue (`buy`) or a whole venue (`pump_swap`). An unknown name is refused with `INVALID_ARGUMENT`.

| Action | Meaning |
|---|---|
| `create` | a new token: a launchpad's create, or a mint or metadata created by hand |
| `buy` | quote in, token out, on a venue with its own buy instruction |
| `sell` | token in, quote out, likewise |
| `swap` | a swap whose direction is set by the accounts, so it is neither `buy` nor `sell` |
| `create_pool` | a new pool |
| `migrate` | a bonding curve graduating to a pool |

A pump.fun launch usually carries the creator's first buy in the same transaction, so it matches
`pump_fun.create` and `pump_fun.buy`. Moonit and Boop are left out: neither made a single call in 500
mainnet blocks on 2026-10-29. Every Anchor discriminator below is checked against
`sha256("global:<instruction>")` by the tests; the others are the program's one byte tag.

| Venue | Program | Action | Instructions |
|---|---|---|---|
| `pump_fun` | `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P` | `create` | `create`, `create_v2` |
| | | `buy` | `buy`, `buy_v2`, `buy_v3`, `buy_exact_sol_in`, `buy_exact_quote_in_v2`, `buy_exact_quote_in_v3` |
| | | `sell` | `sell`, `sell_v2`, `sell_v3` |
| | | `swap` | `multi_hop_curve_swap` |
| | | `migrate` | `migrate`, `migrate_v2` |
| `pump_swap` | `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA` | `buy` | `buy`, `buy_v2`, `buy_exact_quote_in`, `buy_exact_quote_in_v2` |
| | | `sell` | `sell`, `sell_v2` |
| | | `swap` | `multi_hop_swap` |
| | | `create_pool` | `create_pool` |
| `raydium_amm_v4` | `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8` | `swap` | `swap_base_in`, `swap_base_out`, `swap_base_in_v2`, `swap_base_out_v2` |
| | | `create_pool` | `initialize2` |
| `raydium_cpmm` | `CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C` | `swap` | `swap_base_input`, `swap_base_output` |
| | | `create_pool` | `initialize`, `initialize_with_permission` |
| `raydium_clmm` | `CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK` | `swap` | `swap`, `swap_v2`, `swap_router_base_in` |
| | | `create_pool` | `create_pool`, `create_customizable_pool` |
| `raydium_launchlab` | `LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj` | `create` | `initialize`, `initialize_v2`, `initialize_with_token_2022` |
| | | `buy` | `buy_exact_in`, `buy_exact_out` |
| | | `sell` | `sell_exact_in`, `sell_exact_out` |
| | | `migrate` | `migrate_to_amm`, `migrate_to_cpswap` |
| `meteora_dbc` | `dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN` | `create` | `initialize_virtual_pool_with_spl_token`, `initialize_virtual_pool_with_token2022`, `initialize_virtual_pool_with_token2022_transfer_hook` |
| | | `swap` | `swap`, `swap2`, `swap2_with_transfer_hook` |
| | | `migrate` | `migration_damm_v2`, `migrate_meteora_damm` |
| `meteora_damm_v2` | `cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG` | `swap` | `swap`, `swap2` |
| | | `create_pool` | `initialize_pool`, `initialize_customizable_pool`, `initialize_pool_with_dynamic_config` |
| `meteora_damm_v1` | `Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB` | `swap` | `swap` |
| | | `create_pool` | `initialize_permissionless_pool`, `initialize_permissionless_pool_with_fee_tier`, `initialize_permissionless_constant_product_pool_with_config`, `initialize_permissionless_constant_product_pool_with_config2`, `initialize_customizable_permissionless_constant_product_pool` |
| `meteora_dlmm` | `LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo` | `swap` | `swap`, `swap2`, `swap_exact_out`, `swap_exact_out2`, `swap_with_price_impact`, `swap_with_price_impact2` |
| | | `create_pool` | `initialize_lb_pair`, `initialize_lb_pair2`, `initialize_customizable_permissionless_lb_pair`, `initialize_customizable_permissionless_lb_pair2` |
| `orca_whirlpool` | `whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc` | `swap` | `swap`, `swap_v2`, `two_hop_swap`, `two_hop_swap_v2` |
| | | `create_pool` | `initialize_pool`, `initialize_pool_v2` |
| `jupiter` | `JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4` | `swap` | `route`, `route_v2`, `route_with_token_ledger`, `shared_accounts_route`, `shared_accounts_route_v2`, `shared_accounts_route_with_token_ledger`, `shared_accounts_exact_out_route` |
| `spl_token` | `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` | `create` | `initialize_mint`, `initialize_mint2` |
| `token_2022` | `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` | `create` | `initialize_mint`, `initialize_mint2` |
| `token_metadata` | `metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s` | `create` | `create_metadata_account_v3`, `create` |

