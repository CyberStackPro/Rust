use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to server at 127.0.0.1:7878");

    let mut input = String::new();
    println!("Type a message and press Enter:");
    io::stdin().read_line(&mut input)?;

    // Optionally trim trailing newline
    let msg = input.trim_end().as_bytes();

    stream.write_all(msg)?;
    stream.flush()?;

    let mut buffer = [0u8; 512];
    let n = stream.read(&mut buffer)?;
    println!("Reply: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
