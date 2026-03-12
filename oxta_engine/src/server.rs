use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bytes::BytesMut;
use std::sync::{Arc, Mutex};
use crate::engine::GeodesicEngine;

pub struct RespServer {
    engine: Arc<Mutex<GeodesicEngine>>,
}

impl RespServer {
    pub fn new(engine: Arc<Mutex<GeodesicEngine>>) -> Self {
        Self { engine }
    }

    pub async fn run(&self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        println!("RESP Server listening on :{}", port);

        loop {
            let (socket, _) = listener.accept().await?;
            let engine = self.engine.clone();

            tokio::spawn(async move {
                if let Err(e) = process_connection(socket, engine).await {
                    eprintln!("Connection error: {}", e);
                }
            });
        }
    }
}

async fn process_connection(mut socket: TcpStream, engine: Arc<Mutex<GeodesicEngine>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = BytesMut::with_capacity(4096);

    loop {
        let n = socket.read_buf(&mut buffer).await?;
        if n == 0 { return Ok(()); }

        if let Some(command) = parse_resp(&mut buffer) {
             let response = handle_command(command, &engine);
             socket.write_all(response.as_bytes()).await?;
        }
    }
}

// Very basic RESP parser (Array of Bulk Strings only)
fn parse_resp(buffer: &mut BytesMut) -> Option<Vec<String>> {
    // Check if we have a full command... this is simplified.
    // In production we would use a proper RESP parser crate or complete state machine.
    // For now, let's assume the client sends clean commands.
    // *2\r\n$3\r\nGET\r\n$3\r\nkey\r\n

    // Quick hack for demo: convert to string and split
    // NOTE: This breaks binary safety but works for text commands.
    let s = String::from_utf8_lossy(&buffer[..]).to_string();
    if !s.contains("\r\n") { return None; }

    let parts: Vec<&str> = s.split("\r\n").collect();
    if parts.len() < 3 { return None; }

    // Consume buffer
    buffer.clear();

    let mut args = Vec::new();
    for part in parts {
        if !part.starts_with('*') && !part.starts_with('$') && !part.is_empty() {
             args.push(part.to_string());
        }
    }

    if args.is_empty() { None } else { Some(args) }
}

fn handle_command(args: Vec<String>, engine: &Arc<Mutex<GeodesicEngine>>) -> String {
    let cmd = args[0].to_uppercase();

    match cmd.as_str() {
        "PING" => "+PONG\r\n".to_string(),
        "SET" if args.len() >= 3 => {
            let key = &args[1];
            let val = args[2].as_bytes().to_vec();
            let mut eng = engine.lock().unwrap();
            match eng.write(key, val) {
                Ok(_) => "+OK\r\n".to_string(),
                Err(e) => format!("-ERR {}\r\n", e),
            }
        },
        "GET" if args.len() >= 2 => {
            let key = &args[1];
            let eng = engine.lock().unwrap();
            match eng.read_latest(key) {
                Some(node) => {
                     let s = String::from_utf8_lossy(&node.value);
                     format!("${}\r\n{}\r\n", s.len(), s)
                },
                None => "$-1\r\n".to_string()
            }
        },
        _ => "-ERR unknown command\r\n".to_string()
    }
}
