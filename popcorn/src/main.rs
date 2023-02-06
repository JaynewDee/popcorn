extern crate dotenv;

use dotenv::dotenv;

use popcorn::start_server;

fn main() {
    dotenv().ok();

    launch_app();
}

fn launch_app() -> String {
    start_server().unwrap()
}
