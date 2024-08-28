use howdy_multi::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Couldn't bind to port"); // could be handled more elgantly
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        // remove the take(2) if u want to keep going lol
        let stream = stream.unwrap();

        // not gonna do that but could
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shitting down.");
}

// sleeps but we use thread pool
// small fixed number of threads to protecc against DoS

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match request_line.split(" ").collect::<Vec<&str>>()[1] {
        "/" => ("HTTP/1.1 200 OK", "hello.html"),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(format!("html/{}", filename)).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

// LESS OLD

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let request_line = buf_reader.lines().next().unwrap().unwrap();

//     let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
//         ("HTTP/1.1 200 OK", "hello.html")
//     } else {
//         ("HTTP/1.1 404 NOT FOUND", "404.html")
//     };

//     // this is how you would use match statement for route:

//     // let request_route = match request_line.split(" ").collect::<Vec<&str>>()[1] {
//     //     "/" => "home",
//     //     _ => "other",
//     // };
//     // println!("{request_route}");

//     // could ofc nest match statements depending on request type etc

//     let contents = fs::read_to_string(format!("html/{}", filename)).unwrap();
//     let length = contents.len();

//     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

//     stream.write_all(response.as_bytes()).unwrap();
// }

// OLD:
// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);

//     // let http_request: Vec<_> = buf_reader
//     //     .lines()
//     //     .map(|result| result.unwrap())
//     //     .take_while(|line| !line.is_empty())
//     //     .collect();

//     // http calls have this format:
//     // Method Request-URI HTTP-Version CRLF
//     // headers CRLF
//     // message-body

//     // println!("Request: {:#?}", http_request);

//     // http responses have the format:
//     // HTTP-Version Status-Code Reason-Phrase CRLF
//     // headers CRLF
//     // message-body

//     // this is already a valid response
//     // let response = "HTTP/1.1 200 OK\r\n\r\n";

//     let request_line = buf_reader.lines().next().unwrap().unwrap(); // gives us just the first line and unwraps twice

//     if request_line == "GET / HTTP/1.1" {
//         let status_line = "HTTP/1.1 200 OK";
//         let contents = fs::read_to_string("html/hello.html").unwrap();
//         let length = contents.len();

//         let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

//         stream.write_all(response.as_bytes()).unwrap(); //this could fail, would need more elegant handling
//     } else {
//         let status_line = "HTTP/1.1 404 NOT FOUND";
//         let contents = fs::read_to_string("html/404.html").unwrap();
//         let length = contents.len();

//         let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

//         stream.write_all(response.as_bytes()).unwrap();
//     }
// }
