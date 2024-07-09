use ethers::prelude::*;
use std::convert::TryFrom;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load Ethereum node URL and wallet private key from environment variables
    let node_url = env::var("ETH_NODE_URL").expect("ETH_NODE_URL not set");
    let wallet_private_key = env::var("WALLET_PRIVATE_KEY").expect("WALLET_PRIVATE_KEY not set");

    // Connect to the Ethereum network
    let provider = Provider::<Http>::try_from(node_url.as_str())?;

    // Create a wallet
    let wallet: LocalWallet = wallet_private_key.parse()?;
    let wallet = wallet.with_chain_id(1u64); // Mainnet chain id

    // Create a client
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let client = Arc::new(client);

    // Query account balance
    let address = wallet.address();
    let balance = client.get_balance(address, None).await?;
    println!("Balance of {:?}: {} ETH", address, ethers::utils::format_units(balance, "ether")?);

    // Send a transaction (optional, uncomment to use)
    // let tx = TransactionRequest::new()
    //     .to("recipient_address".parse::<Address>()?)
    //     .value(ethers::utils::parse_units("0.01", "ether")?);
    // let pending_tx = client.send_transaction(tx, None).await?;
    // let receipt = pending_tx.confirmations(3).await?;
    // println!("Transaction receipt: {:?}", receipt);

    Ok(())
}
