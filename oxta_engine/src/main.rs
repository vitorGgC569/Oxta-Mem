use geodesic_engine::engine::GeodesicEngine;
use geodesic_engine::server::RespServer;
use std::sync::{Arc, Mutex};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 6379)]
    port: u16,

    #[arg(short, long, default_value = "geodesic.db")]
    db_path: String,

    #[arg(short, long, default_value_t = 100)]
    size_mb: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("--- Geodesic Engine Starting ---");
    println!("Store: {} ({} MB)", args.db_path, args.size_mb);

    let engine = GeodesicEngine::new(&args.db_path, args.size_mb)?;
    let shared_engine = Arc::new(Mutex::new(engine));

    let server = RespServer::new(shared_engine);
    server.run(args.port).await?;

    Ok(())
}
