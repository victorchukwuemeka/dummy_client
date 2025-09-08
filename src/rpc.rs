use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GossipPeer{
    //special identity to reconize the memember  of the peer
    pub pubkey : String,
    
    //address used to exchange cluster info 
    pub gossip: Option<String>,
    
    // the transaction processing unit , for sending transactions
    pub tpu: Option<String>,
    
    // where client can query the chain
    pub rpc: Option<String>
}


pub async  fn fetch_gossip()->Result<Vec<GossipPeer>>{
    let url = "https://api.mainnet-beta.solana.com";
    let client = reqwest::Client::new();

    let payload = serde_json::json!({
        "jsonrpc":"2.0",
        "id" : 1,
        "method":"getClusterNodes"
    });

    let res = client.post(url).json(&payload).send().await?;
    let body: serde_json::Value  = res.json().await?;
    let peer:Vec<GossipPeer> = serde_json::from_value(body["result"].clone())?;

    Ok(peer)
}





