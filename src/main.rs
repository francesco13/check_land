//use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::{Address, H160, U256};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value ="wss://mainnet.infura.io/ws/v3/fae476cc30ff43e0813948aca32f409d")]
    wss_net: String,

    #[clap(short, long, default_value = "0x850A0521E86a63d9617112855694739dd8b23c9B")]
    account: String,

    #[clap(short, long, default_value = "0x5cc5b05a8a13e3fbdb0bb9fccd98d38e50f90c38")]
    contract: String,
    
    #[clap(short, long, default_value = "159856")]
    token_id: i32,
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    let args = Args::parse();
    let land_addr = Address::from_str(&args.contract).unwrap();
    let websocket = web3::transports::WebSocket::new(&args.wss_net).await?;
    let web3s = web3::Web3::new(websocket);
    let wei_conv: U256 = U256::exp10(18);
    let account = H160::from_str(&args.account).unwrap();
    let balance = web3s.eth().balance(account, None).await?;
    let token_contract =
        Contract::from_json(web3s.eth(), land_addr, include_bytes!("LAND_abi.json")).unwrap();
    let token_name: String = token_contract
        .query("symbol", (), None, Options::default(), None)
        .await
        .unwrap();
    let balance_of: U256 = token_contract
        .query("balanceOf", Address::from_str(&args.account).unwrap(), None, Options::default(), None)
        .await
        .unwrap(); 
     let owner_of: Address = token_contract
        .query("ownerOf", U256::from(args.token_id), None, Options::default(), None)
        .await
        .unwrap(); 
    println!("Eth balance of {:?}: {}", account,balance.checked_div(wei_conv).unwrap());
    println!("Token name: {}, Balance: {}", token_name, balance_of);
    println!("Owner of: {}, {:?}", args.token_id, owner_of);
    Ok(())
}
