use crate::types::Transaction;
use color_eyre::eyre::Result;
use rusqlite::{params, Connection};

pub struct BudgetService {
    connection: Connection,
}

#[derive(Debug, PartialEq)]
pub enum DBError {
    SQLError(rusqlite::Error),
}

impl From<rusqlite::Error> for DBError {
    fn from(value: rusqlite::Error) -> Self {
        DBError::SQLError(value)
    }
}

impl BudgetService {
    pub fn new(name: &str) -> Result<Self> {
        let connection = Connection::open(name)?;

        let _ = Self::create_db(&connection)?;

        Ok(Self { connection })
    }

    fn create_db(conn: &Connection) -> Result<usize> {
        Ok(conn.execute(
            "CREATE TABLE IF NOT EXISTS fin_transaction (
                transaction_id  INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp       TEXT    NULL,
                credit_acc_id   INTEGER NULL,
                debit_acc_id    INTEGER NULL,
                amount          INTEGER NULL,
                category        TEXT    NULL,
                description     TEXT    NULL
            ) STRICT",
            [],
        )?)
    }

    pub fn get_trns(&self) -> Result<Vec<Transaction>> {
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

        Ok(tr_iter.collect::<Result<Vec<Transaction>, rusqlite::Error>>()?)
    }
    pub fn del_trns(&mut self, items: &[isize]) -> Result<()> {
        let mut delete = self
            .connection
            .prepare_cached("DELETE FROM fin_transaction WHERE transaction_id = ?1")?;

        for id in items {
            let _ = delete.execute(params![id])?;
        }
        Ok(())
    }

    pub fn put_trn(&mut self, item: &Transaction) -> Result<isize> {
        let trn_id = match item.transaction_id {
            Some(transaction_id) => {
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

                let _ = update.execute(params![
                    transaction_id,
                    item.timestamp,
                    item.credit_acc_id,
                    item.debit_acc_id,
                    item.amount,
                    item.category,
                    item.description
                ]);
                transaction_id
            }
            None => {
                let mut insert = self.connection.prepare_cached(
                    "INSERT INTO fin_transaction (
                       timestamp, credit_acc_id, debit_acc_id,
                       amount, category, description
                    )
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                    RETURNING transaction_id
                    ",
                )?;
                insert.query_row(
                    params![
                        item.timestamp,
                        item.credit_acc_id,
                        item.debit_acc_id,
                        item.amount,
                        item.category,
                        item.description
                    ],
                    |row| row.get(0),
                )?
            }
        };

        Ok(trn_id)
    }

    pub fn put_trns(&mut self, data: &[Transaction]) -> Result<()> {
        for item in data {
            let _ = self.put_trn(item);
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
            credit_acc_id: Some(rng.random_range(0..u8::MAX)),
            debit_acc_id: Some(rng.random_range(0..u8::MAX)),
            timestamp: Local::now() + Duration::days(rng.random_range(0..30)),
            amount: rng.random_range(i64::MIN..i64::MAX),
            category: Some(generate_random_string(10)),
            description: Some(generate_random_string(10)),
        }
    }

    fn fake_data() -> Vec<Transaction> {
        (0..2usize).map(|_| random_trn()).collect()
    }

    #[test]
    fn create_service() -> Result<()> {
        let service = BudgetService::new(TEST_DB)?;
        let trans = service.get_trns()?;

        assert_eq!(trans, vec![]);
        Ok(())
    }

    #[test]
    fn insert_transactions() -> Result<()> {
        let fake_data = fake_data();
        let mut service = BudgetService::new(TEST_DB)?;
        if let Err(err) = service.put_trns(&fake_data) {
            println!("{err:?}");
        }

        Ok(())
    }

    #[test]
    fn update_transaction() -> Result<()> {
        let mut service = BudgetService::new(TEST_DB)?;
        let mut trn = random_trn();

        trn.transaction_id = Some(service.put_trn(&trn)?);

        let content = service.get_trns()?;
        assert_eq!(content, vec![trn]);

        Ok(())
    }

    #[test]
    fn delete_transactions() -> Result<()> {
        let mut service = BudgetService::new(TEST_DB)?;
        let mut trn1 = random_trn();
        let mut trn2 = random_trn();

        trn1.transaction_id = Some(service.put_trn(&trn1)?);
        trn2.transaction_id = Some(service.put_trn(&trn2)?);

        dbg!(&trn1, &trn2);

        assert!(service.get_trns()?.len() == 2);

        let _ = service.del_trns(&[trn1.transaction_id.unwrap()]);

        assert_eq!(service.get_trns(), Ok(vec![trn2]));
        Ok(())
    }
}
