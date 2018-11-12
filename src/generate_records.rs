use rusqlite::{Connection, Error};
use serde_json;

pub fn get_column_names(conn: &Connection) -> Vec<String> {
    let mut columns_query_stmt = conn.prepare("PRAGMA table_info(users)").unwrap();
    let result: Result<Vec<String>, Error> = columns_query_stmt
        .query_map(&[], |_row| _row.get(1))
        .unwrap()
        .collect();
    result.unwrap()
}

pub fn convert_records(conn: &Connection) -> String {
    let mut stmt = conn.prepare("SELECT * FROM users").unwrap();
    let result: Result<Vec<Vec<String>>, Error> = stmt
        .query_map(&[], |_row| {
            let mut vec: Vec<String> = Vec::new();
            let count = _row.column_count();
            for i in 1..count {
                vec.push(_row.get(i));
            }
            vec
        })
        .unwrap()
        .collect();

    match result {
        Ok(res) => {
            let mut new_vec: Vec<String> = Vec::new();
            for i in 0..res.len() {
                for j in 0..res.get(i).unwrap().len() {
                    new_vec.push(res.get(i).unwrap().get(j).unwrap().to_string());
                }
            }
            return serde_json::to_string(&new_vec).unwrap();
        }
        Err(err) => {
            panic!("{}", err);
        }
    }
}

// Could use std::any::Any; to implement 'dynamic types'
pub fn generate_records(conn: &Connection, record_count: i32) {
    for _ in 0..record_count {
        conn.execute(
            "INSERT INTO users (name, data)
            VALUES (?1, ?2)",
            &[&fake!(Name.name), &"bar"],
        )
        .unwrap();
    }
}

pub fn create_conn() -> Connection {
    Connection::open_in_memory().unwrap()
}

pub fn create_table(conn: &Connection) {
    let _ = conn.execute(
        "
        CREATE TABLE users (
            id              INTEGER PRIMARY KEY,
            name            TEXT NOT NULL,
            data            BLOB
        )",
        &[],
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_column_names() {
        let conn = create_conn();
        get_column_names(&conn);
    }

    #[bench]
    fn bench_generate_records(bench: &mut Bencher) {
        let conn = create_conn();
        create_table(&conn);
        bench.iter(|| convert_records(&conn));
    }
}
