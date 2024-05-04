use std::path::Path;

use redb::{AccessGuard, Database, TableDefinition};

const TABLE: TableDefinition<&str, u32> = TableDefinition::new("click7_data");

pub fn load_db_if_exist(db_path: &Path) -> Result<Database, redb::Error> {
    if db_path.exists() {
        /* error coersion */
        return Ok(Database::open(db_path)?);
    }
    let db = Database::create(db_path)?;

    /* create empty table */
    let write_txn = db.begin_write()?;
    write_txn.open_table(TABLE)?;
    write_txn.commit()?;

    Ok(db)
}

pub fn read_and_increment(db: &Database, key: &str) -> Result<u32, redb::Error> {
    /* read number (inced +1 for currrent user) */
    let num = {
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;
        table
            .get(key)?
            .map(|arg0: AccessGuard<'_, u32>| AccessGuard::value(&arg0) + 1)
            .unwrap_or(1)
    };

    /* inc */
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(TABLE)?;
        table.insert(key, num)?;
    }
    write_txn.commit()?;

    Ok(num)
}
