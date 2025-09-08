use tracing::{info, warn, error, debug};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod gossip;
mod rpc;


#[tokio::main]
async fn main()->anyhow::Result<()>{
    
    //open or create a file for logging
    let file_appender = tracing_appender::rolling::never(".", "dummy_client.log");
    let (non_blocking,_guard) = tracing_appender::non_blocking(file_appender);
    
    //define the layers
    let stdout_layer = fmt::layer().with_target(false);
    let file_layer = fmt::layer().with_writer(non_blocking);


    tracing_subscriber::registry().with(stdout_layer)
    .with(file_layer)
    .with(EnvFilter::new("info"))
    .init();
    

    println!("Dummy client started");
    debug!("This is a debug log (hidden unless you set DEBUG filter)");
    
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