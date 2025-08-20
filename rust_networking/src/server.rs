/*
Single threaded
*/

// use std::io::{Read, Write};
// use std::net::TcpListener;

// fn main() -> std::io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:7878")?;
//     println!("Server listening on 127.0.0.1:7878");

//     for stream in listener.incoming() {
//         let mut stream = stream?;
//         println!("New connection: {}", stream.peer_addr()?);

//         let mut buffer = [0u8; 512];
//         let n = stream.read(&mut buffer)?;
//         if n == 0 {
//             println!("Client closed connection");
//             continue;
//         }

//         println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

//         // Echo back only the bytes we actually read
//         stream.write_all(&buffer[..n])?;
//         stream.flush()?;
//     }

//     Ok(())
// }

use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878...");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        // Spawn a new thread for each connection
        thread::spawn(move || {
            println!("New connection from {:?}", stream.peer_addr().unwrap());

            let mut buffer = [0; 512];
            loop {
                let n = match stream.read(&mut buffer) {
                    Ok(0) => {
                        println!("Client disconnected.");
                        break;
                    }
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Error reading: {}", e);
                        break;
                    }
                };

                println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

                // Echo back to the client
                if let Err(e) = stream.write_all(&buffer[..n]) {
                    eprintln!("Error writing: {}", e);
                    break;
                }
            }
        });
    }
}
