use anyhow::Result;
use aptos_sdk::types::LocalAccount;
use rusqlite::Connection;

#[derive(Debug)]
pub struct Key {
    pub address: String,
    pub private: String,
    pub mnemonic: String,
    pub balance: String,
    pub seq: u64,
}

#[derive(Debug)]
pub struct KeyWithId {
    pub id: u64,
    pub address: String,
    pub private: String,
    pub mnemonic: String,
    pub balance: String,
    pub seq: u64,
}

#[derive(Debug)]
pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn new() -> Result<Db> {
        let conn = Connection::open("keys.db")?;
        Ok(Self { conn })
    }

    pub fn create_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE keys (
            id    INTEGER PRIMARY KEY,
            address  TEXT NOT NULL,
            private  TEXT NOT NULL,
            mnemonic TEXT NOT NULL,
            balance  TEXT NOT NULL,
            seq      INTEGER NOT NULL
        )",
            (), // empty list of parameters.
        )?;
        Ok(())
    }

    pub fn insert(&self, key: &Key) -> Result<()> {
        self.conn.execute(
            "INSERT INTO keys (address,private,mnemonic,balance,seq) VALUES (?1, ?2, ?3 ,?4,?5)",
            (
                &key.address,
                &key.private,
                &key.mnemonic,
                &key.balance,
                &key.seq,
            ),
        )?;
        Ok(())
    }

    pub fn gen_account(&self, number: &u64) -> Result<()> {
        for _ in 0..*number {
            let acc = LocalAccount::generate(&mut rand::rngs::OsRng);
            let k = Key {
                address: format!("0x{}", acc.address().to_string()),
                private: hex::encode(acc.private_key().to_bytes()),
                mnemonic: "".to_string(),
                balance: "0".to_string(),
                seq: 0,
            };
            self.insert(&k)?;
        }

        Ok(())
    }

    pub fn get_account(&self, start: u64, count: u64) -> Result<Vec<KeyWithId>> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT id,address,private,mnemonic,balance,seq FROM keys limit {},{}",
            start, count
        ))?;

        let keys_iter = stmt.query_map([], |row| {
            Ok(KeyWithId {
                id: row.get(0)?,
                address: row.get(1)?,
                private: row.get(2)?,
                mnemonic: row.get(3)?,
                balance: row.get(4)?,
                seq: row.get(5)?,
            })
        })?;

        let mut keys = vec![];

        for key in keys_iter {
            keys.push(key.unwrap());
        }

        Ok(keys)
    }

    // update seq number
    pub fn update(&self, id: u64, seq: u64) -> Result<()> {
        self.conn
            .execute("UPDATE keys set seq = ?1 where id = ?2", (seq, id))?;

        Ok(())
    }
}
