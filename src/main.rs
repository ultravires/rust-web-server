use std::{
    io::{BufRead, BufReader, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line: Vec<_> = http_request[0].split(' ').collect();
    let request_method = request_line[0].to_string();
    let request_pathname = request_line[1].to_string();

    let docroot = "www";

    // let welcome_page = ["index.html"];
    let error_page_404 = "/error/404.html";

    let status_line = "HTTP/1.1 200 OK";
    let mut content_type = "text/plain";
    if request_pathname.ends_with(".html") {
        content_type = "text/html";
    } else if request_pathname.ends_with(".js") {
        content_type = "text/javascript";
    }
    let is_static_exist = std::path::Path::new(&format!("{docroot}{request_pathname}")).exists();
    let mut contents = String::new();
    if is_static_exist == true {
        println!("{} {}", request_method, request_pathname);
        contents = std::fs::read_to_string(format!("{docroot}{request_pathname}")).unwrap();
    } else {
        println!("{} {} Not Found 404", request_method, request_pathname);
        content_type = "text/html";
        contents = std::fs::read_to_string(format!("{docroot}{error_page_404}")).unwrap();
    }
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 8080)),
        SocketAddr::from(([127, 0, 0, 1], 8081)),
    ];
    let listener = TcpListener::bind(&addrs[..]).unwrap();
    println!(
        "Server listening on port {}",
        listener.local_addr().unwrap().port()
    );
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("[Error] 连接错误！{}", e.to_string());
            }
        }
    }
}
