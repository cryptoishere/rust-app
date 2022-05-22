//! Your Service Description here, etc.

use app::actix_web;
use mainlib;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    // std::fs::create_dir_all("./tmp")?;
    mainlib::main().await
}