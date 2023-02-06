use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::FileServer;
use rocket::{post, routes, Ignite, Rocket, State};

use super::db;
use sqlx::{MySql, Pool};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UserData {
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct LoginInput {
    pub email: String,
    pub pw: String,
}
#[derive(Serialize, Deserialize)]
pub struct LoginResponse<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub success: bool,
}

pub enum ApiResponse {
    GenericResponse,
}

pub struct DBConn {
    connection: Pool<MySql>,
}

#[post("/", format = "application/json", data = "<input>")]
async fn login(sql_pool: &State<DBConn>, input: String) -> String {
    let js: LoginInput = serde_json::from_str(&input).unwrap();
    println!("{:?}", &js);
    let is_valid = db::models::User::login_by_email(
        &sql_pool.connection,
        "jdiehl2236@gmail.com".to_string(),
        "supersecret".to_string(),
    )
    .await;

    let res: LoginResponse = LoginResponse {
        username: "Synthetic",
        email: &js.email,
        success: is_valid,
    };

    serde_json::to_string(&res).unwrap()
}
///
///
///
#[rocket::main]
pub async fn main() -> Result<Rocket<Ignite>, rocket::Error> {
    //
    // Rocket Server Handler
    //
    use rocket::Config;
    let sql_pool = db::main().await;

    let config = Config {
        port: 8000,
        ..Config::debug_default()
    };
    let rocky = rocket::custom(&config)
        .mount(
            "/",
            FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .mount("/api/login", routes![login])
        .manage(DBConn {
            connection: sql_pool,
        })
        .launch()
        .await?;
    Ok(rocky)
}
