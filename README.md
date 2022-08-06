# Ethereum Gas CLI Tool
*Get current gas prices, the ETH/USD exchange rate, and cost to execute aribitrary transactions*
## Install
Install the Rust toolchain then
```shell
git clone https://github.com/c4syner/gas.git && cd gas
cargo install --bin gas --path .
```
## Usage
To calculate the cost to send a transaction that uses 100000 gas
```shell
gas 100000
```