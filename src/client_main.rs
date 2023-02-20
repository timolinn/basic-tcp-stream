use std::io::{BufRead, BufReader, LineWriter, Write};
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:3333").unwrap();
    let mut writer = LineWriter::new(&stream);
    writer.write_all(b"Hallo, TCP! Dat is lekker!").unwrap();
    writer.write_all(&[b'\n']).unwrap();

    let mut reader = BufReader::new(&stream);
    let mut lines = String::new();
    reader.read_line(&mut lines).unwrap();
    println!("{lines}");
}
