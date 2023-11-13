use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        println!("Connection established!")
    }
}

fn handle_connection(mut stream: TcpStream) {
    // ** Basic
    // let buf_reader = BufReader::new(&mut stream);
    // // println!("{:#?}", buf_reader.buffer());
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // // println!("Request: {:#?}", http_request)
    // let response = "HTTP/1.1 200 OK\r\n\r\n";

    // stream.write_all(response.as_bytes()).unwrap();

    // ** Render HTML
    // let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // let status_line = "HTTP/1.1 200 OK";
    // let contents = fs::read_to_string("./index.html").unwrap();
    // let length = contents.len();

    // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    // println!("{response}");

    // stream.write_all(response.as_bytes()).unwrap();

    // ** Validating Request and Response
    let buf_reader = BufReader::new(&mut stream);
    // println!("{:?}", buf_reader.lines().next().unwrap().unwrap());
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("index.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
