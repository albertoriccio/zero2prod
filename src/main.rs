use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};

use std::env;

#[allow(unused_variables)]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let dir = env::current_dir().unwrap();
    println!("{}", dir.display());

    let config = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port");
    run(listener)?.await
}
