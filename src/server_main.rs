use std::io::{prelude::*, BufReader, LineWriter};
use std::net::{Shutdown, TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    let mut writer = LineWriter::new(stream.try_clone().unwrap());
    let mut reader = BufReader::new(stream);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.pop(); // removes the \n
    println!("client says: {line}");

    let message = String::from("server says: election is in 5 days. May God help us!");
    writer.write_all(message.as_bytes()).unwrap();
    writer.write_all(&[b'\n']).unwrap(); // This will also signal a `writer.flush()` for us!

    // Alternative
    //let mut buffer = [0; 512];
    // while match stream.read(&mut buffer) {
    //     Ok(size) => {
    //         stream.write_all(&buffer[0..size]).unwrap();
    //         stream.write_all(&[b'\n']).unwrap(); // This will also signal a `writer.flush()` for us!
    //         true
    //     }
    //     Err(_) => {
    //         println!(
    //             "An error occurred, terminating connection with {}",
    //             stream.peer_addr().unwrap()
    //         );
    //         stream.shutdown(Shutdown::Both).unwrap();
    //         false
    //     }
    // } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                std::thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
    }
}
