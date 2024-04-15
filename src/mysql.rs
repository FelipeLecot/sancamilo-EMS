use std::env;
use mysql::{prelude::Queryable, Pool, PooledConn};

async fn establish_connection() -> anyhow::Result<Pool> {
    let database_url = env::var("DB_URL").expect("DB_URL not set");
    
    let pool = Pool::new(database_url)?;
    Ok(pool)
}

async fn execute_query(pool: &Pool) -> anyhow::Result<()> {
    let mut conn = pool.get_conn()?;
    
    let query = "SELECT * FROM dxts_users LIMIT 1;";
    
    let rows = conn.query_map(query, |(id, name): (i32, String)| (id, name))?;
    
    for row in rows {
        let (id, name) = row?;
        println!("ID: {}, Name: {}", id, name);
    }
    
    Ok(())
}