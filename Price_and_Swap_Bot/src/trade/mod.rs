//Handle Trades
use ethers::prelude::*;
use std::sync::Arc;

 /* Execute a token swap using a DEX.

 # Arguments:
    - `client`: Ethereum client for signing and sending transactions.
    - `router_address`: Address of the DEX router contract.
    - `token_in`: Address of the token being sold.
    - `token_out`: Address of the token being bought.
    - `amount_in`: Amount of `token_in` to swap (in smallest units).
    - `recipient`: Address to receive the output tokens.
 # Returns:
    - Transaction hash of the trade.
 */

 pub struct TradeExecutor {
    pub client: Arc<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
    pub router_address: Address,
    pub recipient: Address,
}

impl TradeExecutor {
    pub fn new(
        client: Arc<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
        router_address: Address,
        recipient: Address,
    ) -> Self {
        Self {
            client,
            router_address,
            recipient,
        }
    }

    pub async fn execute_swap(
        &self,
        token_in: Address,
        token_out: Address,
        amount_in: U256,
        is_eth_input: bool,
        is_eth_output: bool,
        slippage_percentage:U256,
    ) -> Result<H256, Box<dyn std::error::Error>> {
        let abi = include_bytes!("../abis/Uniswap_Router_ABI.json");
        let abi = ethers::abi::Abi::load(abi.as_ref())?;

        // Contract instance for the router
        let contract = Contract::new(self.router_address, abi, self.client.clone());

        // Deadline for the swap (5 minutes from now)
        let deadline = U256::from(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs()
                + 300,
        );

        let min_amount_out = amount_in.checked_mul(U256::from(10_000).saturating_sub(slippage_percentage))
        .ok_or("Slippage calculation overflow")?
        .checked_div(U256::from(10_000))
        .ok_or("Slippage calculation division error")?;

        // Execute based on swap type
        let tx_hash = if is_eth_input {
            self.swap_eth_to_token(&contract, token_out, amount_in, min_amount_out, deadline)
                .await?
        } else if is_eth_output {
            self.swap_token_to_eth(&contract, token_in, amount_in, min_amount_out, deadline)
                .await?
        } else {
            self.swap_token_to_token(&contract, token_in, token_out, amount_in, min_amount_out, deadline)
                .await?
        };

        Ok(tx_hash)
    }

    async fn swap_eth_to_token(
        &self,
        contract: &Contract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
        token_out: Address,
        amount_in: U256,
        min_out: U256,
        deadline: U256,
    ) -> Result<H256, Box<dyn std::error::Error>> {
            // Sepolia WETH address
            let weth_address: Address = "0xfFf9976782d46CC05630D1f6eBAb18b2324d6B14".parse()?;
        
            // Construct the path: WETH -> LINK
            let path = vec![weth_address, token_out];
        let method_call = contract.method::<_, H256>(
            "swapExactETHForTokens",
            (
                min_out,     // Minimum amount of tokens
                path,  // Path: ETH → Token
                self.recipient,   // Recipient address
                deadline,         // Deadline
            ),
        )?;
        let prepared_call = method_call.value(amount_in); // Store the intermediate result to increase the lifetime
        let tx = prepared_call.send().await?;
        Ok(tx.tx_hash())
    }

    async fn swap_token_to_eth(
        &self,
        contract: &Contract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
        token_in: Address,
        amount_in: U256,
        min_out: U256,
        deadline: U256,
    ) -> Result<H256, Box<dyn std::error::Error>> {
            // Sepolia WETH address
        let weth_address: Address = "0xfFf9976782d46CC05630D1f6eBAb18b2324d6B14".parse()?;
        
        // Construct the path: LINK -> WETH
        let path = vec![token_in, weth_address];

        // Get current gas price and increase it  to ensure the transaction goes through
        let provider = contract.client();
        let sender_address = provider.address();
        let nonce = provider.get_transaction_count(sender_address, None).await?;
    
        // Get current gas price and substantially increase it
        let base_gas_price = provider.get_gas_price().await?;
        let adjusted_gas_price = {
            let multiplier = U256::from(200u64); // Increase by the number of percentage you like
            let divisor = U256::from(100u64);
            base_gas_price
                .checked_mul(multiplier)
                .ok_or("Gas price multiplication overflow")?
                .checked_div(divisor)
                .ok_or("Gas price division error")?
        };

        let method_call = contract.method::<_, H256>(
            "swapExactTokensForETH",
            (
                amount_in,      // Amount of Token to swap
                min_out,   // Minimum ETH output
                path, // Path: Token → WETH
                self.recipient, // Recipient address
                deadline,       // Deadline
            ),
        )?;
      
        let prepared_call = method_call.gas_price(adjusted_gas_price).nonce(nonce);
        let tx = prepared_call.send().await?;
        Ok(tx.tx_hash())
    }

    async fn swap_token_to_token(
        &self,
        contract: &Contract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
        token_in: Address,
        token_out: Address,
        amount_in: U256,
        min_out: U256,
        deadline: U256,
    ) -> Result<H256, Box<dyn std::error::Error>> {
        let method_call = contract.method::<_, H256>(
            "swapExactTokensForTokens",
            (
                amount_in,             // Amount of input token
                min_out,          // Slippage Protection is avoided here as this code is not production-ready
                vec![token_in, token_out], // Path
                self.recipient,        // Recipient address
                deadline,              // Deadline
            ),
        )?;
        let prepared_call = method_call.value(amount_in); //  Store the intermediate result to increase the lifetime
        let tx = prepared_call.send().await?;
        Ok(tx.tx_hash())
    }
}
