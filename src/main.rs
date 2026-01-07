// importing necessary modules 
#![allow(unused_imports, unused_variables, dead_code)]
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

fn handle_client(mut stream: TcpStream){
    //this is a buffer to read data from the client 
    let mut buffer = [0; 1024];
    // this reads the data from stream and store it in buffer. excpect gives error with msg when stream.read fails
    stream.read(&mut buffer).expect("Error reading from stream");
    // this line converts the data in the buffer into utf8 incoded string
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Recieved request: {}", request);
    // as_bytes converts the text into bytes 
    let response = "hello world ".as_bytes();
    stream.write(response).expect("failed to write response");
}


fn main() {
    //creating a TCP listner 
    let listner = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to address");
    println!("Server listining on 127.0.0.1:8000");
    // loop all the incomming listner 
    for stream in listner.incoming(){
        match stream{
            Ok(stream) => {
                //spawn helps to create new threat and if the connections are many then it runs concurrently 
                // using move because new threat must own the stream
                std::thread::spawn( move || handle_client(stream));
            }
            Err(e) => {
                //macro to print a error message 
                eprint!("Failed to estb connection {}",e );
                // stderr - standard error system 
                
            }
        }
    }
}
