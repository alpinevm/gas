# gas

Compare gas costs across Ethereum networks. See how much your transaction will cost on different chains.

## Install

```bash
cargo install --path .
```

## Use it

```bash
# Check gas cost for a standard eth transfer (21k gas)
gas 21000

# Compare costs on specific chains (by id)
gas 21000 42161 8453 10
```

## Example

```bash
$ gas 100000
Gas Costs
┌──────────────────────┬───────────────────────┐
│  Gas Limit: 100,000  │  ETH Price: $1627.07  │
└──────────────────────┴───────────────────────┘
┌──────────┬────────────┬──────────┬───────────┐
│ Chain    │ ETH Cost   │ USD Cost │ Gas Price │
├──────────┼────────────┼──────────┼───────────┤
│ Ethereum │ 0.00004080 │   0.0664 │    0.4080 │
│ Arbitrum │ 0.00000100 │   0.0016 │    0.0100 │
│ Base     │ 0.00000018 │   0.0003 │    0.0018 │
│ Optimism │ 0.00000020 │   0.0003 │    0.0020 │
└──────────┴────────────┴──────────┴───────────┘
```

### Technical Notes 
- Uniswap V3 for price data
- RPCs taken from Chainlist for each chain