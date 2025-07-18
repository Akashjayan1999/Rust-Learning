use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;
fn main() {
   let listner = TcpListener::bind("127.0.0.1:8080").unwrap();

   for steam in listner.incoming() {
      let stream = steam.unwrap();
      println!("New connection from {}", stream.peer_addr().unwrap());
      handle_connection(stream);
     
   }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer =[0;1024];
  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";
  let (status_line, filename) = if buffer.starts_with(get) {
//     //   let message = String::from_utf8_lossy(&buffer[..]);
// //   println!("Message from client: {}", message);
//   let contents =fs::read_to_string("index.html").unwrap();
// //   let response = "HTTP/1.1 200 OK\r\n\r\n";
//    println!("Message from client: {}", contents);
//   let response = format!("\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);
//   stream.write(response.as_bytes()).unwrap();
//   stream.flush().unwrap();
  ("HTTP/1.1 200 OK", "index.html")
  }else{
   ("HTTP/1.1 404 Not Found", "404.html")
  };
  //   let message = String::from_utf8_lossy(&buffer[..]);
//   println!("Message from client: {}", message);
  
//   let response = "HTTP/1.1 200 OK\r\n\r\n";
    // let status_line = "HTTP/1.1 404 Not Found";
    let contents = fs::read_to_string(filename).unwrap();
    let repsonse = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents);
    stream.write(repsonse.as_bytes()).unwrap();
    stream.flush().unwrap();
}


//HTTP-Version Status-Code Reasin-Phrase CRLf
//headers CRLF
//message-body
//
//ex: Http/1.1 200 OK\r\n\r\n
