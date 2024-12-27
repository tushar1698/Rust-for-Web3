
# Price Tracking and Trading Bot

This bot is a decentralized finance (DeFi) tool built in **Rust**. It leverages the **Ethers** crate to interact with Ethereum-compatible blockchains and uses **Uniswap V2/V3 routers** for executing token swaps. It also integrates with Chainlink oracles to fetch live price feeds. The bot is designed to be modular, secure, and developer-friendly.

---
## Disclaimer
This project is provided for educational purposes only. Use it at your own risk. The author assumes no responsibility for any financial loss, damages, or other issues arising from the use of this code. Always review and test the code thoroughly before deploying it in a live environment.

---

## Features

- Fetch live price feeds (e.g., LINK/ETH, ETH/USD) using Chainlink Oracles.
- Execute token swaps (ETH ↔ LINK) on Ethereum-compatible networks.
- Dynamically configure swap settings (amount, slippage, direction).
- Modular architecture:
  - Easily add or modify price feeds.
  - Support additional tokens and swap paths with minimal changes.
- Centralized ABI management for simplicity.
- Secure handling of sensitive data using environment variables.

---

## Prerequisites

Ensure the following are installed on your system:
1. **Rust Toolchain**: Install Rust from [https://rustup.rs/](https://rustup.rs/).
2. **Node Provider**: Access to an Ethereum-compatible RPC node (e.g., Alchemy, Infura).
3. **Git**: Install Git for version control.

---

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/price-trading-bot.git
   cd price-trading-bot
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Create a `.env` file in the root directory to securely store sensitive data:
   ```plaintext
   RPC_URL=<your-ethereum-node-rpc-url>
   PRIVATE_KEY=<your-private-key>
   LINK_ETH_PRICE_FEED=<Chainlink LINK/ETH price feed address>
   ETH_USD_PRICE_FEED=<Chainlink ETH/USD price feed address>
   ```

4. Add ABI files to the `abis/` directory:
   - `price_feed.json`: ABI for Chainlink price feed contracts.
   - `router.json`: ABI for Uniswap router contracts.
   - Ensure these files are formatted correctly as JSON.
  
  **Note**: Currently the ABIs set for the Smart Contracts are from the Sepolia Testnet so adjust accordingly

---

## Usage

Run the bot using the following command:
```bash
cargo run
```

### Workflow
1. **Select a Price Feed**:
   - Choose between LINK/ETH and ETH/USD price feeds.
2. **Configure Swap Settings**:
   - Specify the swap direction (ETH → LINK or LINK → ETH).
   - Enter the amount to swap and the slippage tolerance.
3. **Execute the Swap**:
   - The bot will execute the trade using the Uniswap router.

---

## Extending Functionality

### Adding New Price Feeds
1. **Update `config/mod.rs`**:
   ```rust
   impl PriceFeed {
       pub fn new_feed() -> Self {
           PriceFeed {
               name: "NEW/FEED",
               env_var: "NEW_FEED_PRICE_FEED",
               decimals: 18,
           }
       }
   }
   ```
2. **Add to `.env`**:
   - Include the price feed contract address in the `.env` file:
     ```plaintext
     NEW_FEED_PRICE_FEED=<address>
     ```

### Supporting Additional Tokens
1. Update the **swap logic** in `trade/mod.rs` to handle new tokens or paths.
2. Add environment variables for the new tokens (e.g., WETH, DAI) in the `.env` file.

### Adding ABIs
1. Place new ABI files in the `abis/` directory.
2. Reference them in the `config/mod.rs` using `load_abi`:
   ```rust
   pub fn load_custom_abi(name: &str) -> Abi {
       load_abi(name)
   }
   ```

---

## Project Structure

```plaintext
price-trading-bot/
├── src/
│   ├── config/
│   │   └── mod.rs       # Configuration for RPC URL, private key, ABIs, and user input
│   ├── client/
│   │   └── mod.rs       # Ethereum client creation and wallet setup
│   ├── price_feed/
│   │   └── mod.rs       # Logic to fetch live price feeds
│   ├── trade/
│   │   └── mod.rs       # Token swap logic
│   └── main.rs          # Main entry point for the bot
├── abis/
│   ├── price_feed.json  # ABI for Chainlink price feed contracts
│   ├── router.json      # ABI for Uniswap router contracts
├── .env                 # Environment variables (not included in version control)
├── .gitignore           # Files and directories to ignore in Git
├── Cargo.toml           # Rust dependencies and metadata
└── README.md            # Project documentation
```

---

## Security Best Practices

1. **Protect Private Keys**:
   - Use a secure `.env` file to store your private key and RPC URL.
   - Add `.env` to `.gitignore` to prevent accidental exposure.

2. **Test on Testnets**:
   - Start with Ethereum testnets like Sepolia or Goerli before deploying to mainnet.

3. **Review External Dependencies**:
   - Ensure the ABIs and external libraries (like `ethers`) are verified.

---

## Contributing

Contributions are welcome! Open an issue or submit a pull request for any improvements or new features.

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
