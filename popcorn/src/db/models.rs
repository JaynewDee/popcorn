use argon2::Config;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, MySql, Pool, Row};

#[derive(Debug, PartialEq, Eq)]
pub struct UserAuth;

pub struct User {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthResponse {
    pub username: String,
    pub success: bool,
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
