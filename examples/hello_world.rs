#[macro_use] extern crate logkit;

fn main() {
    info!(version = "0.1.0", commit = "3291cc60"; "server is started");
    info!(address = "127.0.0.1", port = 3000; "listen and serve");
    error!("address already in use {}:{}", "127.0.0.1", 3000);
    info!("graceful shutdown completed successfully");
}