
use anyhow::Result;

use rusqlite::{params, Connection};

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
            "CREATE TABLE IF NOT EXIST blocks(
                slot  INTEGER PRIMARY KEY,
                parent_slot INTEGER,
                block_hash TEXT,
                timestamp INTEGER 
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXIST transaction(
                tx_signature TEXT PRIMARY KEY,
                slot INTEGER,
                signers TEXT, 
                program TEXT,
                status TEXT 
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXIST  accounts(
                pubkey TEXT PRIMARY KEY, 
                balance INTEGER ,
                owner_program TEXT, 
                data BLOB
            )",
            [],
        )?;
        Ok(conn)
    }


    //i'm adding the rpc for getting the ledger here .
}