
use std::time::SystemTimeError;

use anyhow::Result;

use rusqlite::{Connection};
use crate::rpc::{fetch_gossip, GossipPeer};
use crate::network::Network;
use crate::cli::NetworkOpt;


pub struct Ledger{
    pub rpc_url : String,
    pub db_path : String 
}

impl Ledger{
    pub fn new(rpc_url: String, db_path: String)->Self{
        Self { rpc_url, db_path }
    }

    pub fn init_db(&self)->Result<Connection>{
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS blocks(
                slot  INTEGER PRIMARY KEY,
                parent_slot INTEGER,
                block_hash TEXT,
                timestamp INTEGER 
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS transactions(
                tx_signature TEXT PRIMARY KEY,
                slot INTEGER,
                signers TEXT, 
                program TEXT,
                status TEXT 
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS  accounts(
                pubkey TEXT PRIMARY KEY, 
                balance INTEGER ,
                owner_program TEXT, 
                data BLOB
            )",
            [],
        )?;
        Ok(conn)
    }

    pub async fn fetch_block(&self, slot:u64)->Result<serde_json::Value>{
        let client  = reqwest::Client::new();
        let playload = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getBlock",
            "params": [
                slot,
                { "encoding": "json", "transactionDetails": "full", "rewards": false }
            ]
        });

        let res = client.post(&self.rpc_url).json(&payload).send().await?;
        let body: serde_json::Value = res.json().await?;
        Ok(body["result"].clone())
    }



    //i'm adding the rpc for getting the ledger here .
    pub async  fn fetch_peers(&self, network_opt: NetworkOpt, custom_url: Option<String>)->Result<Vec<GossipPeer>>
    {
        let peers = fetch_gossip(network_opt, custom_url).await?;
        Ok(peers)
    }

    pub async fn store_block(&self, conn: &Connection, block: &serde_json::Value) -> Result<()> {
        if let Some(slot) = block["slot"].as_u64() {
            let parent_slot = block["parentSlot"].as_u64().unwrap_or(0);
            let block_hash = block["blockhash"].as_str().unwrap_or_default();
            let timestamp  = block["blockTime"].as_i64().unwrap_or(0);

            conn.execute(
                "INSERT OR REPLACE INTO blocks (slot, parent_slot, block_hash, timestamp) VALUES (?1, ?2, ?3, ?4)",
                params![slot, parent_slot, block_hash, timestamp],
            )?;

            // Transactions would be handled in a loop here
        }
        Ok(())
    }
}