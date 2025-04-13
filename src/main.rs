pub mod chains;
pub mod quote;

use num_format::{Locale, ToFormattedString};
use std::str::FromStr;

use anyhow::Result;
use chains::{fetch_gas_data, get_eth_cost_for_gas_limit};
use clap::Parser;
use colored::*;

fn parse_gas_limit(s: &str) -> Result<u64, String> {
    let cleaned = s.replace(['_', ','], "");
    u64::from_str(&cleaned).map_err(|e| e.to_string())
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The gas limit to use for the simulation.
    #[arg(value_name = "GAS_LIMIT", value_parser = parse_gas_limit)]
    gas_limit: u64,

    /// Specify a list of chain ids to simulate against besides mainnet. If omitted, Base, Arbitrum, and Optimism will be used.
    #[arg(value_name = "CHAIN_ID", default_values = &["42161", "8453", "10"])]
    alt_chain_ids: Vec<u64>,
}

struct GasData {
    name: String,
    eth_cost: f64,
    gas_cost_usd: f64,
    gas_fee_gwei: f64,
}

fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn print_header(gas_limit: u64, eth_price: f64) {
    let gas_limit_text = "Gas Limit";
    let eth_price_text = "ETH Price";
    let gas_limit_data_text = gas_limit.to_formatted_string(&Locale::en);
    let eth_price_data_text = format!("${:.2}", eth_price);

    // Build the plain column strings (with leading/trailing space for padding)
    let gas_limit_column_str = format!(" {}: {} ", gas_limit_text, gas_limit_data_text);
    let eth_price_column_str = format!(" {}: {} ", eth_price_text, eth_price_data_text);

    // Measure lengths (ignoring ANSI). If you use color *here*, strip them out first.
    let gltw = gas_limit_column_str.len() + 2;
    let eptw = eth_price_column_str.len() + 2;

    let gas_limit_column_str = format!(
        " {}: {} ",
        gas_limit_text,
        gas_limit_data_text.bold().yellow()
    );

    let eth_price_column_str = format!(
        " {}: {} ",
        eth_price_text,
        eth_price_data_text.bold().yellow()
    );

    // Print a title
    println!("{}", "Gas Costs".bold().cyan());

    // Top border: 2 columns
    println!(
        "┌{:─<gltw$}┬{:─<eptw$}┐",
        "", // "fill" argument for first column
        "", // "fill" argument for second column
        gltw = gltw,
        eptw = eptw
    );

    // Table row with data
    // (Here, we color the text; the measured widths above are from the plain strings.)
    println!(
        "│ {} │ {} │",
        gas_limit_column_str.bold(),
        eth_price_column_str.bold(),
    );

    // Bottom border, matching the same widths
    println!("└{:─<gltw$}┴{:─<eptw$}┘", "", "", gltw = gltw, eptw = eptw);
}

fn print_gas_table(gas_data: &[GasData], gas_limit: u64, eth_price: f64) {
    print_header(gas_limit, eth_price);
    // Calculate max widths for each column
    let chain_width = gas_data
        .iter()
        .map(|d| d.name.len())
        .max()
        .unwrap_or(8)
        .max("Chain".len());

    let eth_cost_width = gas_data
        .iter()
        .map(|d| format!("{:.8}", d.eth_cost).len())
        .max()
        .unwrap_or(10)
        .max("ETH Cost".len());

    let usd_cost_width = gas_data
        .iter()
        .map(|d| format!("{:.4}", d.gas_cost_usd).len())
        .max()
        .unwrap_or(8)
        .max("USD Cost".len());

    let gas_price_width = gas_data
        .iter()
        .map(|d| format!("{:.4}", d.gas_fee_gwei).len())
        .max()
        .unwrap_or(8)
        .max("Gas Price".len());

    // Create the top border
    let top_border = format!(
        "┌─{chain:─<chain_width$}─┬─{eth:─<eth_cost_width$}─┬─{usd:─<usd_cost_width$}─┬─{gas:─<gas_price_width$}─┐",
        chain = "",
        eth = "",
        usd = "",
        gas = "",
        chain_width = chain_width,
        eth_cost_width = eth_cost_width,
        usd_cost_width = usd_cost_width,
        gas_price_width = gas_price_width
    );

    // Create the separator
    let separator = format!(
        "├─{chain:─<chain_width$}─┼─{eth:─<eth_cost_width$}─┼─{usd:─<usd_cost_width$}─┼─{gas:─<gas_price_width$}─┤",
        chain = "",
        eth = "",
        usd = "",
        gas = "",
        chain_width = chain_width,
        eth_cost_width = eth_cost_width,
        usd_cost_width = usd_cost_width,
        gas_price_width = gas_price_width
    );

    // Create the bottom border
    let bottom_border = format!(
        "└─{chain:─<chain_width$}─┴─{eth:─<eth_cost_width$}─┴─{usd:─<usd_cost_width$}─┴─{gas:─<gas_price_width$}─┘",
        chain = "",
        eth = "",
        usd = "",
        gas = "",
        chain_width = chain_width,
        eth_cost_width = eth_cost_width,
        usd_cost_width = usd_cost_width,
        gas_price_width = gas_price_width
    );

    println!("{}", top_border);
    println!(
        "│ {:<chain_width$} │ {:<eth_cost_width$} │ {:<usd_cost_width$} │ {:<gas_price_width$} │",
        "Chain".bold(),
        "ETH Cost".bold(),
        "USD Cost".bold(),
        "Gas Price".bold(),
        chain_width = chain_width,
        eth_cost_width = eth_cost_width,
        usd_cost_width = usd_cost_width,
        gas_price_width = gas_price_width
    );
    println!("{}", separator);

    for data in gas_data {
        println!(
            "│ {:<chain_width$} │ {:>eth_cost_width$.8} │ {:>usd_cost_width$.4} │ {:>gas_price_width$.4} │",
            capitalize_first(&data.name),
            data.eth_cost,
            data.gas_cost_usd,
            data.gas_fee_gwei,
            chain_width = chain_width,
            eth_cost_width = eth_cost_width,
            usd_cost_width = usd_cost_width,
            gas_price_width = gas_price_width
        );
    }

    println!("{}", bottom_border);
}

async fn collect_gas_data(args: &Args) -> Result<(Vec<GasData>, f64)> {
    let (mainnet, alt_chains) = fetch_gas_data(&args.alt_chain_ids).await?;
    let eth_price = mainnet.eth_price.unwrap();

    let mut data = Vec::new();

    // Add mainnet data
    let (eth_cost, gas_fee_gwei) =
        get_eth_cost_for_gas_limit(mainnet.gas_per_unit_wei, args.gas_limit);
    let gas_cost_usd = eth_cost * eth_price;
    data.push(GasData {
        name: mainnet.name,
        eth_cost,
        gas_cost_usd,
        gas_fee_gwei,
    });

    // Add alt chain data
    for chain in alt_chains {
        let (eth_cost, gas_fee_gwei) =
            get_eth_cost_for_gas_limit(chain.gas_per_unit_wei, args.gas_limit);
        let gas_cost_usd = eth_cost * eth_price;
        data.push(GasData {
            name: chain.name,
            eth_cost,
            gas_cost_usd,
            gas_fee_gwei,
        });
    }

    Ok((data, eth_price))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let (gas_data, eth_price) = collect_gas_data(&args).await?;
    print_gas_table(&gas_data, args.gas_limit, eth_price);
    Ok(())
}
