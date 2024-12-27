//Fetch Prices
use ethers::prelude::*;
use std::sync::Arc;

pub async fn get_latest_price(
    client: Arc<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
    price_feed_address: Address,
    feed_type: &str,
) -> Result<i128, Box<dyn std::error::Error>> {
    //Defining the ABI of the Chainlink price feed depending on the user choice
    let abi = if feed_type == "LINK/ETH" {
        let link_abi = include_bytes!("../abis/Price_Feed_LINK_ETH_ABI.json");
        ethers::abi::Abi::load(link_abi.as_ref())?
    }
    else if feed_type == "ETH/USD" {
        let eth_abi = include_bytes!("../abis/Price_Feed_ETH_USD.json");
        ethers::abi::Abi::load(eth_abi.as_ref())?
    }
    else {
        return Err("Unsupported feed Type".into());
    };
    

    //Create a contract instance,
    // from_abi binds our Rust code to a deployed contract using its address, ABI, and client, enabling seamless interaction.
    let contract = Contract::new(price_feed_address, abi, client); //Contract from ethers crate provides this method for calling contract functions, querying state, and handling events.

    // Fetch the Price
    let price: i128 = contract.method("latestAnswer", ())?.call().await?;
    Ok(price)
}
