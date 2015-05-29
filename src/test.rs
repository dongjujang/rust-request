//! Carrier
#![doc(html_root_url="https://cosmos-io.github.io/carrier/doc")]

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::str;

fn handle_client(s: TcpStream) {
    let mut stream = match s.try_clone() {
        Ok(stream) => stream,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    
    const BUFFER_SIZE: usize = 1024;
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut raw = String::new();
    loop {
        let len = match stream.read(&mut buffer) {
            Ok(len) => len,
            Err(_) => {
                return;
            }
        };
        match str::from_utf8(&buffer[0 .. len]) {
            Ok(buf) => raw.push_str(buf),
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
        if len < BUFFER_SIZE { break; }
    }
    
    println!("{}", raw);
    
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nConnection: close\r\n\r\n";
    let _ = stream.write(response.as_bytes());
}

#[cfg(test)]
pub fn new() {
    let listener = TcpListener::bind("0.0.0.0:8888").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => { println!("{}", e); }
        }
    }
    drop(listener);
}
