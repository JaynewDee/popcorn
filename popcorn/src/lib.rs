#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate argon2;
extern crate sqlx;
mod db;
mod server;

pub async fn start_server() {
    server::main().await;
}
