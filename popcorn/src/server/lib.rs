use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::http::Status;
use rocket::{Request, Response};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

#[derive(Serialize, Deserialize)]
struct UserData {
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginInput {
    pub email: String,
    pub pw: String,
}

pub struct DBConn {
    pub connection: Pool<MySql>,
}

pub struct CORS;
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS FAIRING",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _: &'r Request<'_>, res: &mut Response<'r>) {
        // res.set_header(Header::new("Content-Type", "application/json"));
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_status(Status::Ok);
    }
}
