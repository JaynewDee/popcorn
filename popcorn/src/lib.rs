#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code)]

extern crate argon2;
extern crate rocket;
extern crate serde;
extern crate serde_json;
extern crate sqlx;
extern crate tokio;
mod db;
mod server;

pub fn start_server() -> Result<String, String> {
    match server::main() {
        Ok(_) => Ok(format!("Rocket has liftoff.")),
        Err(e) => Err(format!("Rocket has failed to launch... ::: {}", e)),
    }
}
