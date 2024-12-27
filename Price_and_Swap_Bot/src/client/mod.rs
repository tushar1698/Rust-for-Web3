// Ethereum Client
use ethers::prelude::*;
use ethers::signers::Signer;
use ethers::signers::Wallet;
use k256::ecdsa::SigningKey;
use std::sync::Arc;

// Creating an ETH client with a provider and wallet

pub async fn create_client(
    rpc_url: String,
    private_key: String,
) -> Result<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Box<dyn std::error::Error>> {
    // Connecting to ETH node
    let provider = Provider::<Http>::try_from(rpc_url)?;

      // Parse the private key into a wallet and set chain ID
    let wallet: Wallet<SigningKey> = private_key.parse()
      .map_err(|e| format!("Invalid private key format: {}", e))?;
    let wallet = wallet.with_chain_id(11155111u64);

    //Combining the provider and wallet into a client
    let client = SignerMiddleware::new(provider, wallet);

    Ok(Arc::new(client)) //Arc is Atomic Reference count that Wraps the client in an Arc to allow shared ownership across threads or async tasks.
}

 // Initializing the Trade Executor and providing approval to the router contract -- All the SWAP LOGIC
 // Instantiate LINK ERC-20 contract
 pub async fn approve_spender(
    client: Arc<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
    token: Address,
    spender: Address,
) -> Result<H256, Box<dyn std::error::Error>> {
    let token_abi = include_bytes!("../abis/LINK_TOKEN_ABI.json");
    let abi = ethers::abi::Abi::load(token_abi.as_ref())?;
    let token_contract = Contract::new(token, abi, client.clone());

    // Approve the spender (router) to spend the tokens
    let amount_to_approve = U256::max_value(); // Approve maximum amount
    let approve_call = token_contract
        .method::<_, H256>("approve", (spender, amount_to_approve))?;
    let tx = approve_call.send().await?;
    Ok(tx.tx_hash())
}
