use clap::{Parser,Subcommand};

#[derive(Parser)]
#[command(name = "dc", about = "Dummy Client CLI")]

pub struct Cli{
     #[command(subcommand)]
     pub command : Commands,
}


#[derive(Subcommand)]
pub enum Commands{
    Gossip{
        #[command(subcommand)]
        gossip_cmd: GossipCommands,
    },
}


#[derive(Subcommand)]
pub enum GossipCommands {
    Peers,
    Slots,
}