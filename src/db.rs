use sqlx::{mysql::MySqlPoolOptions, postgres::PgPoolOptions, MySql, Pool, Postgres};



pub async fn new_mysql(url :String, max :u32) -> Pool<MySql> {
        return MySqlPoolOptions::new()
               .max_connections(max)
               .connect(&url)
               .await
               .unwrap();
}

pub async fn new_postgres(url :String, max :u32) -> Pool<Postgres> {
        return PgPoolOptions::new()
               .max_connections(max)
               .connect(&url)
               .await
               .unwrap();
}
