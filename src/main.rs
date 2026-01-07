#![allow(unused_imports, unused_variables, dead_code)]
use std::io::{Read, Write};
use std::net::{TcpListener,TcpStream};
use std::thread::spawn;


fn handle_request(mut stream:TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).expect("Error reading from stream");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request recieved : {}",request);
    let response = "hello world".as_bytes();
    stream.write(response).expect("Error writing in stream");
}

fn main () {
    let listner = TcpListener::bind("127.0.0.1:8080").expect("Failed to listen ");
    println!("Server is listining on {:?}", listner);
    for stream in listner.incoming(){
        match stream {
            Ok(stream) => {
                spawn(move || handle_request(stream));
            }
            Err(e) => {
                eprint!("Error :{}", e);
            }
        }
    }

}