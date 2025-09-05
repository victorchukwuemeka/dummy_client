mod gossip;
mod rpc;

#[tokio::main]
async fn main()->anyhow::Result<()>{
    println!("Dummy client started");
    let _gossip = gossip::start().await?;
    let peers = rpc::fetch_gossip().await?;
    print!("Found peers from {}", peers.len());

    for peer in peers.iter().take(5){
        println!(
            "Peer: {} | Gossip: {:?} |Tpu: {:?} | RPC: {:?}",
            peer.pubkey, peer.gossip, peer.tpu , peer.rpc
        );
    }

    Ok(())
}