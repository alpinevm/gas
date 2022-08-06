use reqwest;
use serde_json;
use clap::Parser;
use serde::{Deserialize, Serialize};
use colored::*;
/// Get current gas prices, the ETH/USD exchange rate, and cost to execute aribitrary transactions
#[derive(Parser, Debug)]
struct Cli {
    /// Number of gas units to compute costs for, DEFAULT: 21000 (transfer)
    gas_units: Option<i128>,
}

/// Statically typed response
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityFees {
    pub slow: f64,
    pub medium: f64,
    pub fast: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasApi {
    pub base_fee: f64,
    pub priority_fees: PriorityFees,
    #[serde(rename = "ether_usd")]
    pub ether_usd: f64,
    #[serde(rename = "update_time")]
    pub update_time: f64,
}

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}

fn main() {
    // Using an API for this is a bit unnecessary, but allows us to not have to interact directly w/ the blockchain
    const GAS_API: &str = "https://egas.c4syner.repl.co/gas";
    const GWEI_TO_ETH: f64 = 10i128.pow(9) as f64;

    let args: Cli = Cli::parse();

    // Prep Args
    let data = reqwest::blocking::get(GAS_API)
        .unwrap()
        .text()
        .unwrap()
        .to_string();

    let mut gas_units: i128 = 21000;
    if !args.gas_units.is_none(){
        gas_units = args.gas_units.unwrap();
    }

    let gas_response: GasApi = serde_json::from_str(&data).unwrap();
    let gas_cost_gwei: f64 = (gas_response.base_fee + gas_response.priority_fees.fast) * (gas_units as f64);
    let gas_cost_eth: f64 = gas_cost_gwei/GWEI_TO_ETH;
    let gas_cost_usd: f64 = gas_cost_eth * gas_response.ether_usd;
    // General Ethereum Network Info
    println!(
        "----------------------\n{}\n  Slow:   {}\n  Medium: {}\n  Fast:   {}\n{}  {}\n----------------------",
        "Gas Costs".bold(),
        round(gas_response.base_fee + gas_response.priority_fees.slow, 2).to_string().green(),
        round(gas_response.base_fee + gas_response.priority_fees.medium, 2).to_string().yellow(),
        round(gas_response.base_fee + gas_response.priority_fees.fast, 2).to_string().red(),
        "ETH/USD:".bold(),
        (&round(gas_response.ether_usd, 2).to_string()).cyan()
    );
    println!(
        "{}\n  Ether: {} \n  USD:   {}\n----------------------",
        "Transaction Cost".bold(),
        gas_cost_eth.to_string().green(),
        round(gas_cost_usd, 2).to_string().green()
    );
}
