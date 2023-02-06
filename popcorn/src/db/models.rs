use argon2::Config;
use rand::Rng;
use sqlx::{query, query_as, MySql, Pool};

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn create(username: String, email: String, password: String) -> User {
        let password_hash: String = Self::hash_password(password);
        print!("{}", password_hash);
        User {
            username,
            email,
            password: password_hash,
        }
    }

    fn hash_password(plain_text: String) -> String {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config: Config = Config::default();

        argon2::hash_encoded(plain_text.as_bytes(), &salt, &config).unwrap()
    }

    fn verify_password(encoded: &str, plain_text: String) -> bool {
        match argon2::verify_encoded(encoded, plain_text.as_bytes()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    async fn get_one(sql_conn: &Pool<MySql>, email: String) {
        sqlx::query("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .execute(sql_conn)
            .await
            .unwrap();
    }

    pub async fn login_by_email(sql_conn: &Pool<MySql>, email: String, pw: String) -> bool {
        Self::get_one(sql_conn, email).await;
        // - Get hash from db by email
        // - Compare hash to user input w/ func
        // - if true, redirect
        // - if false, throw error
        true
    }
}

pub struct Seeder {
    pub users: Vec<User>,
}

impl Seeder {
    pub fn new() -> Seeder {
        Seeder {
            users: vec![User::create(
                "Synthetic".to_string(),
                "jdiehl2236@gmail.com".to_string(),
                "supersecret".to_string(),
            )],
        }
    }
    pub async fn seed_all(sql_conn: &Pool<MySql>) {
        sqlx::query("CREATE DATABASE IF NOT EXISTS users")
            .execute(sql_conn)
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
            username varchar(50) not null,
            email varchar(50) not null,
            password varchar(200) not null
        );",
        )
        .execute(sql_conn)
        .await
        .unwrap();

        let seeder = Self::new();
        let users = seeder.users;

        for user in users.iter() {
            sqlx::query(
                "INSERT INTO users (username, email, password)
                  VALUES (?, ?, ?)",
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
