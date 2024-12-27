//Main Entrypoint of the code
mod client;
mod config;
mod price_feed;
use ethers::prelude::*;
use std::io;
mod trade;
use std::sync::Arc;
use config::{SwapSettings, SwapDirection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok(); //Loading the .env crate
    let rpc_url = config::load_rpc_url();
    let private_key = config::load_private_key();

    //Creating ETH client
    let client = client::create_client(rpc_url, private_key).await?;

    // Fetch and Print Client's Address
    let address = client.address();
    println!("Client Address: {:?} \n", address);

    // Query the balance of the wallet
    let balance = client.provider().get_balance(address, None).await?;
    println!("Wallet ETH Balance: {} \n", ethers::utils::format_ether(balance));

    //Fetching the feed that the user has selected from config program

    let selected_feed = config::get_user_selected_feed()?;

    //Get the Price
    let price_feed_address: Address = selected_feed.load_price_feed().parse()?;
    let price: i128 = price_feed::get_latest_price(client.clone(), price_feed_address, selected_feed.name).await?;

    let price_f64 = price as f64 / 10f64.powi(selected_feed.decimals.try_into().unwrap());
    println!("Latest {} Price: {:.5} ",selected_feed.name, price_f64);

    ///////////////////////////////////////////////
    /// /////  SWAP LOGIC ////////////////////////
    /// /////////////////////////////////////////

    // Initializing the Trade Executor and providing approval to the router contract -- All the SWAP LOGIC
   
  
    // Set Addresses
    let router_address: Address = "0xeE567Fe1712Faf6149d80dA1E6934E354124CfE3".parse()?;
    let link_token: Address = "0x779877A7B0D9E8603169DdbD7836e478b4624789".parse()?;

    let swap_settings = config::get_user_swap_settings()?;

    println!(
        "Swap Settings - Direction: {:?}, Amount: {}, Slippage: {}%",
        swap_settings.swap_direction,
        ethers::utils::format_units(swap_settings.amount_in, 18)?,
        swap_settings.slippage / 100
    );

    // swap path and settings based on `SwapSettings` provided by the user
    let (token_in, token_out, is_eth_input, is_eth_output) = match swap_settings.swap_direction {
        SwapDirection::LinkToEth => (link_token, Address::zero(), false, true),
        SwapDirection::EthToLink => (Address::zero(), link_token, true, false),
    };

   
    // Approve the router to spend LINK tokens
    if !is_eth_input {

        let approval_tx = client::approve_spender(client.clone(), link_token, router_address).await?;
        println!("Approval transaction hash: {:?}", approval_tx);
    
    };
    
    // Creating instance for TradeExecutor Struct
    let recipient =  client.address();
    let trade_executor = trade::TradeExecutor::new(client.clone(), router_address, recipient);
    
    // Executing the SWAP by calling execute swap impl
    let tx = trade_executor
    .execute_swap(token_in, token_out, swap_settings.amount_in, is_eth_input, is_eth_output, swap_settings.slippage) 
    .await?;

    println!("Trade executed. Transaction hash: {:?}", tx);

    Ok(())
}