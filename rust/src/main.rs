use std::env;
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr)?;
    println!("listening on {}", addr);

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf)?;
        let req = String::from_utf8_lossy(&buf[..n]);
        let path = req.split_whitespace().nth(1).unwrap_or("/");

        let body = if path == "/healthz" {
            "ok\n".to_string()
        } else {
            "Hello from kemeter Rust demo\n".to_string()
        };

        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes())?;
    }

    Ok(())
}
