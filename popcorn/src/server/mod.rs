use std::io::Cursor;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::{
    http::{Header, Method},
    request::Request,
    response::Response,
    Outcome,
};
use rocket::{post, routes, State};
use rocket_contrib::json::{Json, JsonError, JsonValue};
use rocket_contrib::serve::{Options, StaticFiles};

use sqlx::{MySql, Pool};

use super::db;

use serde::{Deserialize, Serialize};
struct UserData {
    username: String,
    email: String,
}

pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

pub struct LoginInput {
    pub email: String,
    pub pw: String,
}

pub struct LoginResponse {
    pub username: String,
    pub email: String,
    pub success: bool,
}
impl<'r> rocket::response::Responder<'r> for LoginResponse {
    fn respond_to(self, req: &rocket::Request) -> rocket::response::Result<'static> {
        let json = serde_json::to_string(&self).unwrap();
        let res = Response::build().sized_body(Cursor::new(json)).ok();
        res
    }
}
pub enum ApiResponse {
    GenericResponse,
}

pub struct DBConn {
    connection: Pool<MySql>,
}

struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS header to responses",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options
            || response.content_type().map_or(false, |ct| ct.is_known())
        {
            response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:5173"));
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "GET, POST, PUT, DELETE, OPTIONS",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        }
    }
}

#[post("/", format = "application/json", data = "<input>")]
pub async fn login(sql_pool: State<DBConn, '_>, input: Json<LoginInput>) -> Json<String> {
    let input = input.into_inner();
    println!("{:?}", input);
    let is_valid = db::models::User::login_by_email(
        &sql_pool.connection,
        "jdiehl2236@gmail.com".to_string(),
        "supersecret".to_string(),
    )
    .await;
    let response = LoginResponse {
        username: "userguy".to_string(),
        email: "email@gmail.com".to_string(),
        success: is_valid,
    };
    Json(serde_json::to_string(&input).unwrap())
}

pub async fn main() {
    //
    // Rocket Server Handler
    //

    let options = Options::Index | Options::DotFiles;

    let sql_pool = db::main().await;

    rocket::ignite()
        .attach(CORS())
        .mount("/api/login", routes![login])
        .mount(
            "/",
            StaticFiles::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static"), options),
        )
        .manage(DBConn {
            connection: sql_pool,
        })
        .launch();
}
