#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code)]

extern crate argon2;
extern crate rocket;
extern crate serde;
extern crate serde_json;
extern crate sqlx;
mod db;
mod server;

pub fn start_server() -> Result<String, String> {
    match server::main() {
        Ok(_) => Ok(format!("Rocket has liftoff.")),
        Err(e) => Err(format!("Rocket has failed to launch... ::: {}", e)),
    }
}

///
/////////////////
/// HTTP Routes
/////////////////
///
mod routes {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use rocket::fs::{relative, NamedFile};
    use rocket::http::{Cookie, CookieJar};
    use rocket::response::status::Unauthorized;
    use rocket::{get, post, State};

    use crate::models::UserAuth;
    use crate::routing::{DBConn, LoginInput, LoginSuccess};

    
    #[get("/<_path..>")]
    pub async fn client_render(_path: PathBuf) -> Result<NamedFile, std::io::Error> {
        Ok(NamedFile::open(relative!("./static/index.html"))
            .await
            .unwrap())
    }

    #[post("/login", format = "application/json", data = "<input>")]
    pub async fn login<'a>(
        sql_pool: &State<DBConn>,
        input: String,
        jar: &CookieJar<'_>,
    ) -> Result<String, Unauthorized<String>> {
        let js: LoginInput = serde_json::from_str(&input).unwrap();
        println!("{:?}", &js);

        let data = UserAuth::login_by_email(&sql_pool.connection, &js.email, &js.pw).await;

        if data.success == true {
            let user_id = get_id().to_string();

            jar.add_private(Cookie::new("user_id", String::from(&user_id)));
            let response = LoginSuccess {
                data,
                cookie_id: String::from(&user_id),
            };
            Ok(serde_json::to_string(&response).ok().unwrap())
        } else {
            Err(Unauthorized(Some(
                serde_json::to_string("Authentication failed.")
                    .ok()
                    .unwrap(),
            )))
        }
    }

    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    fn get_id() -> usize {
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}
///
////////////////////
/// Transfer Types
////////////////////
///
mod routing {
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

    #[derive(Debug, Deserialize, Serialize)]
    pub struct AuthResponse {
        pub username: String,
        pub success: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct LoginSuccess {
        pub data: AuthResponse,
        pub cookie_id: String,
    }

    pub struct DBConn {
        pub connection: Pool<MySql>,
    }

    pub struct CORS;
    #[rocket::async_trait]
    impl Fairing for CORS {
        fn info(&self) -> Info {
            Info {
                name: "CORS",
                kind: Kind::Response,
            }
        }

        async fn on_response<'r>(&self, _: &'r Request<'_>, res: &mut Response<'r>) {
            // res.set_header(Header::new("Content-Type", "application/json"));
            res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            res.set_status(Status::Ok);
        }
    }
}

///
//////////////////
///
//////////////////
///
mod models {
    use argon2::Config;
    use rand::Rng;
    use sqlx::{mysql::MySqlRow, MySql, Pool, Row};

    use crate::routing::AuthResponse;

    #[derive(Debug, PartialEq, Eq)]
    pub struct UserAuth;

    pub struct User {
        username: String,
        email: String,
        password: String,
    }

    impl User {
        pub fn create(username: String, email: String, password: String) -> User {
            let password_hash: String = UserAuth::hash_password(password);
            User {
                username,
                email,
                password: password_hash,
            }
        }
    }

    impl UserAuth {
        fn hash_password(plain_text: String) -> String {
            let salt: [u8; 32] = rand::thread_rng().gen();
            let config: Config = Config::default();

            argon2::hash_encoded(plain_text.as_bytes(), &salt, &config).unwrap()
        }

        fn verify_password(encoded: &str, plain_text: String) -> bool {
            println!("\nEncoded: {}\nPlain: {}\n", encoded, plain_text);

            argon2::verify_encoded(encoded, plain_text.as_bytes()).unwrap()
        }

        async fn get_one(sql_conn: &Pool<MySql>, email: &str) -> Result<MySqlRow, sqlx::Error> {
            let user = sqlx::query("SELECT * FROM users WHERE email = ?;")
                .bind(&email)
                .fetch_one(sql_conn)
                .await?;
            println!("{:?}", &user);
            Ok(user)
        }

        pub async fn login_by_email(sql_conn: &Pool<MySql>, email: &str, pw: &str) -> AuthResponse {
            println!("{:?}", Self);
            let user = Self::get_one(sql_conn, email).await;
            let data = user.unwrap();

            let username: String = data.get("username");
            let email: String = data.get("email");
            let pass: String = data.get("password");
            println!(
                "\nUsername: {}\nEmail: {}\nPassword: {}\n",
                username, email, pass
            );
            match Self::verify_password(data.get("password"), pw.to_string()) {
                true => AuthResponse {
                    username,
                    success: true,
                },
                false => AuthResponse {
                    username,
                    success: false,
                },
            }
        }
    }

    pub struct Seeds {
        pub users: Vec<User>,
    }

    impl Seeds {
        pub fn new() -> Seeds {
            Seeds {
                users: vec![User::create(
                    "Synthetic".to_string(),
                    "jdiehl2236@gmail.com".to_string(),
                    "supersecret".to_string(),
                )],
            }
        }
        pub async fn seed_all(sql_conn: &Pool<MySql>) {
            sqlx::query("CREATE DATABASE IF NOT EXISTS `popcorn_dev`;")
                .execute(sql_conn)
                .await
                .unwrap();

            sqlx::query(
                "CREATE TABLE IF NOT EXISTS `users` (
            username varchar(50) not null,
            email varchar(50) UNIQUE not null,
            password varchar(200) not null
        );",
            )
            .execute(sql_conn)
            .await
            .unwrap();

            let seeds = Self::new();
            let users = seeds.users;

            for user in users.iter() {
                sqlx::query(
                    "INSERT IGNORE INTO `users` (username, email, password)
                  VALUES (?, ?, ?);",
                )
                .bind(&user.username)
                .bind(&user.email)
                .bind(&user.password)
                .execute(sql_conn)
                .await
                .unwrap();
            }
        }
    }
}
