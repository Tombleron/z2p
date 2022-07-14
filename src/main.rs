use actix_web;

use z2p::{run, configuration::get_configuration};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = get_configuration().expect("Error loading configuration.");

    run(&config).await
}