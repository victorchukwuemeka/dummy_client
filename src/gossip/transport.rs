use std::net::UdpSocket;
use std::io;
use crate::gossip::message::Message;


#[derive(Debug)]
pub struct Transport{
    socket : UdpSocket,
} 

impl Transport {
    
    pub fn new(bind_addr:&str)->io::Result<Self>{
        let  socket =UdpSocket::bind(bind_addr)?;
        socket.set_nonblocking(true)?;
        Ok(Self { socket })
    }

    pub fn send(&self, message:&Message, target:&str)->io::Result<usize>{
        let message_bytes = message.to_bytes().map_err(
            |e|io::Error::new(io::ErrorKind::InvalidData, e)
        )?;
        self.socket.send_to(&message_bytes, target)
    }

    pub fn receive(&self)->io::Result<(Message, String)>{
        let mut buf = [0u8; 65535];
        match self.socket.recv_from(&mut buf){
            Ok((size, sender_addr)) => {
                let message = Message::from_bytes(&buf[..size])
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                
                Ok((message, sender_addr.to_string()))
            }
            Err(e) => Err(e)
        }
    }

    pub fn local_addr(&self) -> io::Result<std::net::SocketAddr> {
        self.socket.local_addr()
    }
}

