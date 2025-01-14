use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn connect_database(host: Option<String>) -> Pool<Postgres> {
    println!("\n🚀 Connecting to database...");
    match host {
        Some(host) => match PgPoolOptions::new()
            .max_connections(20)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&host)
            .await
        {
            Ok(pool) => {
                println!("✅ Connection to the database is successful!");
                pool
            }
            Err(err) => {
                println!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        },
        None => panic!("🚫 Database host is not set"),
    }
}