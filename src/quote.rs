/// Fetch ETH->USD price from Uniswap v3 on Ethereum mainnet
use crate::chains::{RpcProvider, MAINNET_CHAIN_ID};
use alloy::{
    primitives::{address, Address, U160},
    sol,
};
pub const UNISWAP_V3_USDC_ETH_POOL_ADDRESS: Address =
    address!("0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");

sol!(
    #[sol(rpc)]
    IUniswapV3PoolState,
    r#"[{
        "inputs": [],
        "name": "slot0",
        "outputs": [
        {
            "internalType": "uint160",
            "name": "sqrtPriceX96",
            "type": "uint160"
        },
        {
            "internalType": "int24",
            "name": "tick",
            "type": "int24"
        },
        {
            "internalType": "uint16",
            "name": "observationIndex",
            "type": "uint16"
        },
        {
            "internalType": "uint16",
            "name": "observationCardinality",
            "type": "uint16"
        },
        {
            "internalType": "uint16",
            "name": "observationCardinalityNext",
            "type": "uint16"
        },
        {
            "internalType": "uint8",
            "name": "feeProtocol",
            "type": "uint8"
        },
        {
            "internalType": "bool",
            "name": "unlocked",
            "type": "bool"
        }
        ],
        "stateMutability": "view",
        "type": "function"
  }]"#
);

#[async_trait::async_trait]
pub trait FetchEthPrice {
    async fn fetch_eth_price(&self) -> anyhow::Result<f64>;
}

#[async_trait::async_trait]
impl FetchEthPrice for RpcProvider {
    async fn fetch_eth_price(&self) -> anyhow::Result<f64> {
        let pool = IUniswapV3PoolState::new(UNISWAP_V3_USDC_ETH_POOL_ADDRESS, self);

        let slot0 = pool.slot0().call().await?;

        let sqrt_price_float = q64_96_to_float(slot0.sqrtPriceX96);
        let price_float = sqrt_price_float * sqrt_price_float;
        let price_per_eth = 10f64.powi(12) / price_float;
        Ok(price_per_eth)
    }
}

fn q64_96_to_float(num: U160) -> f64 {
    let limbs = num.into_limbs();
    let lo = limbs[0] as f64; // bits 0..64
    let mid = limbs[1] as f64 * 2f64.powi(64); // bits 64..128
    let hi = limbs[2] as f64 * 2f64.powi(128); // bits 128..160 (top 32 bits used)

    let full = lo + mid + hi;
    full / 2f64.powi(96)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q64_96_to_float() {
        let num = U160::from(1506673274302120988651364689808458u128);
        let float = q64_96_to_float(num);
        println!("Float: {}", float);
    }
}
