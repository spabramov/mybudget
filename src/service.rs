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
                amount          INTEGER NOT NULL,
                category        TEXT,
                description     TEXT
            ) STRICT",
            [],
        )
    }

    pub fn get_transactions(&self) -> Result<Vec<Transaction>> {
        let mut stmt = self.connection.prepare_cached(
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
    pub fn delete_transactions(&mut self, items: &[isize]) -> Result<()> {
        let mut delete = self
            .connection
            .prepare_cached("DELETE FROM fin_transaction WHERE transaction_id = ?1")?;

        for id in items {
            let _ = delete.execute(params![id])?;
        }
        Ok(())
    }

    pub fn put_transaction(&mut self, item: &Transaction) -> Result<isize> {
        let mut insert = self.connection.prepare_cached(
            "INSERT INTO fin_transaction (
               timestamp, credit_acc_id, debit_acc_id,
               amount, category, description
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            RETURNING transaction_id
            ",
        )?;

        let mut update = self.connection.prepare_cached(
            "UPDATE fin_transaction 
             SET 
                timestamp     = ?2,
                credit_acc_id = ?3,
                debit_acc_id  = ?4,
                amount        = ?5,
                category      = ?6,
                description   = ?7
            WHERE
                transaction_id = ?1
            ",
        )?;

        match item.transaction_id {
            Some(transaction_id) => {
                let _ = update.execute(params![
                    transaction_id,
                    item.timestamp,
                    item.credit_acc_id,
                    item.debit_acc_id,
                    item.amount,
                    item.category,
                    item.description
                ]);
                Ok(transaction_id)
            }
            None => insert.query_row(
                params![
                    item.timestamp,
                    item.credit_acc_id,
                    item.debit_acc_id,
                    item.amount,
                    item.category,
                    item.description
                ],
                |row| row.get(0),
            ),
        }
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

    fn random_trn() -> Transaction {
        let mut rng = rand::rng();
        Transaction {
            transaction_id: None,
            credit_acc_id: rng.random_range(0..u8::MAX),
            debit_acc_id: rng.random_range(0..u8::MAX),
            timestamp: Local::now() + Duration::days(rng.random_range(0..30)),
            amount: rng.random_range(i64::MIN..i64::MAX),
            category: generate_random_string(10),
            description: generate_random_string(10),
        }
    }

    fn fake_data() -> Vec<Transaction> {
        (0..2usize).map(|_| random_trn()).collect()
    }

    #[test]
    fn create_service() -> Result<()> {
        let service = BudgetService::new(TEST_DB)?;
        let trans = service.get_transactions()?;

        assert_eq!(trans, vec![]);
        Ok(())
    }

    #[test]
    fn insert_transactions() -> Result<()> {
        let fake_data = fake_data();
        let mut service = BudgetService::new(TEST_DB)?;
        if let Err(err) = service.put_transactions(&fake_data) {
            println!("{err:?}");
        }

        Ok(())
    }

    #[test]
    fn update_transaction() -> Result<()> {
        let mut service = BudgetService::new(TEST_DB)?;
        let mut trn = random_trn();

        trn.transaction_id = Some(service.put_transaction(&trn)?);

        let content = service.get_transactions()?;
        assert_eq!(content, vec![trn]);

        Ok(())
    }

    #[test]
    fn delete_transactions() -> Result<()> {
        let mut service = BudgetService::new(TEST_DB)?;
        let mut trn1 = random_trn();
        let mut trn2 = random_trn();

        trn1.transaction_id = Some(service.put_transaction(&trn1)?);
        trn2.transaction_id = Some(service.put_transaction(&trn2)?);

        dbg!(&trn1, &trn2);

        assert!(service.get_transactions()?.len() == 2);

        let _ = service.delete_transactions(&[trn1.transaction_id.unwrap()]);

        assert_eq!(service.get_transactions(), Ok(vec![trn2]));
        Ok(())
    }
}
