use std::env;
use dotenv::dotenv;
use mysql::{prelude::Queryable, Pool};

pub async fn establish_connection() -> anyhow::Result<Pool> {
    dotenv().ok();
    let database_url = env::var("DB_URL").expect("DB_URL not set");
    
    let pool = Pool::new(database_url)?;
    Ok(pool)
}

pub async fn execute_query(pool: &Pool) -> anyhow::Result<Vec<(i32, String)>> {
    let mut conn = pool.get_conn()?;
    
    let query = "SELECT id, display_name FROM dxts_users;";
    
    let rows = conn.query_map(query, |(id, name): (i32, String)| (id, name))?;
    
    Ok(rows)
}

pub async fn getOneUser(pool: &Pool) -> anyhow::Result<String> {
    let mut conn = pool.get_conn()?;
    
    let query = "SELECT display_name FROM dxts_users LIMIT 1;";
    
    let rows = conn.query_map(query, |name: String | name)?;

    Ok(rows.get(0).unwrap().to_string())
}