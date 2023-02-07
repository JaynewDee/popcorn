use rocket::config::Config;
use rocket::{routes, Ignite, Rocket};

use crate::{
    db,
    routes::{client_render, login},
    routing::DBConn,
};

///
/// SERVER Handler
///

#[rocket::main]
pub async fn main() -> Result<Rocket<Ignite>, rocket::Error> {
    let sql_pool = db::main().await;

    let config = Config {
        port: 8000,
        ..Config::debug_default()
    };

    let rocky = rocket::custom(&config)
        // .attach(CORS)
        .mount("/api", routes![login])
        .mount("/", routes![client_render])
        .manage(DBConn {
            connection: sql_pool,
        })
        .launch()
        .await?;

    Ok(rocky)
}
