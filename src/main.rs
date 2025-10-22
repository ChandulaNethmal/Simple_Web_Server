// fs; bring the standard library’s filesystem module into scope
//BufReader, prelude:; stream IO operaions
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use web_server::ThreadPool;

    //Here , bind outs a Result<T,E> 
    //unwrap() will panic if there will be any runtime issues

//Web server implemntation for test
///here we create a thread pool (limited number of threads) and allocate
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
    //take method is defined in the Iterator trait and limits the iteration to the first two items    
    // for stream in listener.incoming().take(2) {    
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Web Server Shutting down.");
}

//The BufReader adds buffering by managing calls to the std::io::Read trait methods

//Vec<_> type annotation to say that we want to collect these items in buffer to a vector
//lines: splitting the stream of data whenever it sees a newline byte
//format! to add the file’s contents as the body of the success response
fn handle_connection_old(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("src/hello.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();

    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("src/404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}

//More concise version  removing repetitions
//implements handling a request to /sleep with a simulated slow response
//A thread pool is a group of spawned threads that are waiting and ready to handle a task
//
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/Hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            println!("Received a sleep req");
            ("HTTP/1.1 200 OK", "src/Hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "src/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
