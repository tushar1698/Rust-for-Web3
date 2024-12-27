// Environment Variables

use std::env; //You can get, set, or remove environment variables using std::env functions, this is a standard module in rust
use std::io;
use ethers::prelude::*;

//Load the ETH RPC URL from env var


pub fn load_rpc_url() -> String {
    env::var("RPC_URL").expect("Missing RPC_URL in .env file")

}

// Load the private key from environment variables.
pub fn load_private_key() -> String {
    env::var("PRIVATE_KEY").expect("Missing PRIVATE_KEY in .env file")
}

//Make changes here if you are adding more choices for the Price Feed
pub fn get_user_selected_feed() -> Result<PriceFeed, Box<dyn std::error::Error>> {
    println!("Select the Price Feed to Fetch:\n");
    println!("1: LINK/ETH \n");
    println!("2: ETH/USD \n");
    println!("Enter Your Choice (1 or 2): \n");
    io::Write::flush(&mut io::stdout())?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    match input.trim() {
        "1" => Ok(PriceFeed::link_eth()),
        "2" => Ok(PriceFeed::eth_usd()),
        _ => Err("Invalid choice".into()),
    }
}

pub fn get_user_swap_settings() -> Result<SwapSettings, Box<dyn std::error::Error>> {
    println!("Choose Swap Direction:");
    println!("1: ETH → LINK");
    println!("2: LINK → ETH");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let swap_direction = match input.trim() {
        "1" => SwapDirection::EthToLink,
        "2" => SwapDirection::LinkToEth,
        _ => {
            println!("Invalid choice, defaulting to ETH → LINK");
            SwapDirection::EthToLink
        }
    };

    println!("Enter the amount to swap (e.g., 0.01 for ETH):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed To Read Line or Invalid Input");
    let amount: f64 = input.trim().parse().expect("Invalid Input Amount");
    let amount_in = U256::from((amount * 10f64.powi(18)) as u128);

    println!("Enter the slippage percentage (e.g., 1 for 1%):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let slippage_percent: f64 = input.trim().parse().expect("Invalid slippage input");
    let slippage = U256::from((slippage_percent * 100.0) as u128); // Convert to basis points (e.g., 1% → 100)

    Ok(SwapSettings {
        amount_in,
        swap_direction,
        slippage,
    })

}

#[derive(Debug)]
pub enum SwapDirection {
    LinkToEth,
    EthToLink,
}

#[derive(Debug)]
pub struct SwapSettings {
    pub amount_in: U256,
    pub swap_direction: SwapDirection,
    pub slippage: U256, //saved as basis points
}

//Load Price Feed Structs for more modularity from env
pub struct PriceFeed{
    pub name: &'static str,
    pub env_var: &'static str,
    pub decimals: u32, 
}

impl PriceFeed {
    pub fn load_price_feed(&self) -> String{
        env::var(self.env_var).expect(&format!("Missing variable in env"))
    }
}

// Add more price feeds in this implementation if needed or edit the existing ones
impl PriceFeed {
    pub fn link_eth() -> Self {
        PriceFeed {
            name: "LINK/ETH",
            env_var:"LINK_ETH_PRICE_FEED",
            decimals:18,
        }
    }

    pub fn eth_usd() -> Self {
        PriceFeed {
            name: "ETH/USD",
            env_var:"ETH_USD_PRICE_FEED",
            decimals: 8,
        }
    }


}
