use crate::types::Transaction;
use rusqlite::{params, Connection, Result};

pub struct BudgetService {
    connection: Connection,
}

impl BudgetService {
    pub fn new(name: &str) -> Result<Self> {
        let connection = Connection::open(name)?;

        let _ = Self::create_db(&connection)?;

        Ok(Self { connection })
    }

    fn create_db(conn: &Connection) -> Result<usize> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS fin_transaction (
                transaction_id  INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp       TEXT NOT NULL,
                credit_acc_id   INTEGER NOT NULL,
                debit_acc_id    INTEGER NOT NULL,
                amount          DECIMAL(23,2) NOT NULL,
                category        TEXT,
                description     TEXT
            )",
            [],
        )
    }

    pub fn get_transactions(&self) -> Result<Vec<Transaction>> {
        let mut stmt = self.connection.prepare(
            "SELECT 
                transaction_id, timestamp, credit_acc_id, debit_acc_id,
                amount, category, description
             FROM fin_transaction",
        )?;

        let tr_iter = stmt.query_map([], |row| {
            Ok(Transaction {
                transaction_id: row.get(0)?,
                timestamp: row.get(1)?,
                credit_acc_id: row.get(2)?,
                debit_acc_id: row.get(3)?,
                amount: row.get(4)?,
                category: row.get(5)?,
                description: row.get(6)?,
            })
        })?;

        tr_iter.collect()
    }

    pub fn put_transaction(&mut self, item: &Transaction) -> Result<()> {
        let mut insert = self.connection.prepare(
            "INSERT INTO fin_transaction (
                timestamp, credit_acc_id, debit_acc_id,
                amount, category, description
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ",
        )?;

        insert.execute(params!(
            item.timestamp,
            item.credit_acc_id,
            item.debit_acc_id,
            item.amount,
            item.category,
            item.description
        ))?;
        Ok(())
    }

    pub fn put_transactions(&mut self, data: &[Transaction]) -> Result<()> {
        for item in data {
            let _ = self.put_transaction(item);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use chrono::{Duration, Local};
    use rand::Rng;
    use rstest::{fixture, rstest};

    const TEST_DB: &str = ":memory:";

    fn generate_random_string(length: usize) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::rng();

        (0..length)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    #[fixture]
    fn fake_data() -> Vec<Transaction> {
        let mut rng = rand::rng();

        (0..2usize)
            .map(|_| Transaction {
                transaction_id: rng.random_range(0..usize::MAX),
                credit_acc_id: rng.random_range(0..u16::MAX),
                debit_acc_id: rng.random_range(0..u16::MAX),
                timestamp: Local::now() + Duration::days(rng.random_range(0..30)),
                amount: rng.random_range(0..usize::MAX),
                category: generate_random_string(10),
                description: generate_random_string(10),
            })
            .collect()
    }

    #[rstest]
    fn create_service() -> Result<()> {
        let service = BudgetService::new(TEST_DB)?;
        let trans = service.get_transactions()?;

        assert_eq!(trans, vec![]);
        Ok(())
    }

    #[rstest]
    fn put_transactions(fake_data: Vec<Transaction>) -> Result<()> {
        let mut service = BudgetService::new(TEST_DB)?;
        if let Err(err) = service.put_transactions(&fake_data) {
            println!("{err:?}");
        }

        Ok(())
    }
}
