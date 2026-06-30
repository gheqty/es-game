use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

const HTML: &str = include_str!("index.html");
const WASM: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/target/client-wasm/wasm32-unknown-unknown/release/client.wasm"
));

const ADDR: &str = "127.0.0.1:12080";

fn main() {
    let listener = TcpListener::bind(ADDR).unwrap_or_else(|e| {
        eprintln!("failed to bind {ADDR}: {e}");
        std::process::exit(1);
    });
    println!("snake server listening on http://{ADDR}");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                std::thread::spawn(move || {
                    let _ = s.set_read_timeout(Some(Duration::from_secs(5)));
                    let _ = s.set_write_timeout(Some(Duration::from_secs(5)));
                    handle_connection(s);
                });
            }
            Err(e) => eprintln!("accept error: {e}"),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() || request_line.is_empty() {
        return;
    }

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/");

    loop {
        let mut header = String::new();
        if reader.read_line(&mut header).is_err() {
            return;
        }
        if header == "\r\n" || header == "\n" || header.is_empty() {
            break;
        }
    }

    let (body, content_type, cache) = match (method, path) {
        ("GET", "/") => (HTML.as_bytes(), "text/html; charset=utf-8", "no-cache"),
        ("GET", "/game.wasm") => (WASM, "application/wasm", "no-cache"),
        ("GET", "/favicon.ico") => (&b""[..], "image/x-icon", "max-age=86400"),
        _ => {
            let msg = b"not found\n";
            let resp = format!(
                "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                msg.len()
            );
            let _ = stream.write_all(resp.as_bytes());
            let _ = stream.write_all(msg);
            return;
        }
    };

    let status = if body.is_empty() && path == "/favicon.ico" {
        "204 No Content"
    } else {
        "200 OK"
    };

    let resp = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: {cache}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(resp.as_bytes());
    let _ = stream.write_all(body);
}
