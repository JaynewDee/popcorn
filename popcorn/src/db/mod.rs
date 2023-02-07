use sqlx::{MySql, MySqlPool, Pool};
pub mod models;

use models::Seeds;
pub async fn main() -> Pool<MySql> {
    //
    // Main SQL Connection Handler
    //
    let url: &str = "mysql://root:root@localhost:3306/popcorn_dev";

    let pool = MySqlPool::connect(url).await.unwrap();

    Seeds::new();
    Seeds::seed_all(&pool).await;

    println!("Yay!");

    pool
}
