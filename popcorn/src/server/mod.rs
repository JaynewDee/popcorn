use rocket::config::Config;
use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::{post, routes, Ignite, Rocket, State};

use crate::db;
mod lib;
mod routes;
use lib::{DBConn, LoginInput, CORS};
use routes::login;

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
        .attach(CORS)
        .mount("/api", routes![login])
        .mount("/", FileServer::from(relative!("./static/")))
        .manage(DBConn {
            connection: sql_pool,
        })
        .launch()
        .await?;

    Ok(rocky)
}
