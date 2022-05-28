//! Your Service Description here, etc.

use std::io;

#[macro_use]
extern crate log;

// Web Services
pub mod web;
// Pages
pub mod pages;

use app::Server;

pub async fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let _lock = stdout.lock();

    Server::new()
        .register_service(pages::configure)
        .register_service(web::accounts::configure)
        .register_jobs(web::accounts::jobs::configure)
        .register_service(web::dashboard::configure)
        // .register_service(web::send::configure)
        .register_service(web::upload::configure)
        .run()
        .await?
        .await
}