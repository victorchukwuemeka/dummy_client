use anyhow::Result;
use tokio::net::UdpSocket;

pub async fn start()->Result<()>{
    println!("👂 Listening to Solana gossip ");
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    println!("socket connection is bound {:?}", socket);

    let gossip_peer = "109.94.96.51:8001";
    let msg = b"hello from victor";
    socket.send_to(msg, gossip_peer).await?;

    let mut buf = [0u8; 1024];
    if let Ok((len,addr)) = socket.recv_from(&mut buf).await  {
        println!("Got {} bytes from {}", len, addr);
        println!("Raw data: {:?}", &buf[..len]);
    }
    Ok(())
}