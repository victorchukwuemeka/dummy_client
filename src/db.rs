use anyhow::Result;
use rusqlite::{Connection, params};

pub struct Database{
    pub path : String,
}


impl Database {

    pub fn new(path : String)-> Self{
        Self { path }
    }
    
    pub fn init_db(&self)->Result<Connection>{
        let conn = Connection::open(&self.path)?;
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


}

