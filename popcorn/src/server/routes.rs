use crate::db;

use super::lib::{DBConn, LoginInput};
use rocket::response::status::Unauthorized;
use rocket::{post, State};

///
/// HTTP Routes
///

#[post("/login", format = "application/json", data = "<input>")]
pub async fn login(
    sql_pool: &State<DBConn>,
    input: String,
) -> Result<String, Unauthorized<String>> {
    let js: LoginInput = serde_json::from_str(&input).unwrap();
    println!("{:?}", &js);

    let data = db::models::UserAuth::login_by_email(&sql_pool.connection, &js.email, &js.pw).await;

    if data.success == true {
        Ok(serde_json::to_string(&data).ok().unwrap())
    } else {
        Err(Unauthorized(Some(
            serde_json::to_string("Authentication failed.")
                .ok()
                .unwrap(),
        )))
    }
}
