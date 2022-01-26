use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs;

fn parse_request(mut stream: &TcpStream) -> HashMap<String, String> {
    let mut parsed = HashMap::new();
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    for line in String::from_utf8_lossy(&buffer[..]).lines() {
        match line.split_once(" ") {
            Some(splat_line) => {
                match splat_line {
                    (name, value) => {
                        parsed.insert(name.to_string(), value.to_string());
                    }
                }
            },
            None => {}

        }
    }

    parsed
}

fn find_filename(get_request: &Option<&String>) -> String {
    
    match get_request {
        Some(request) => {

            let splat_str = request.split_once(" ").unwrap();

            splat_str.0.replace("/", "").to_string()
        },
        None => {
            "Invalid method!".to_string()
        }
    }
}

fn get_response(mut stream: &TcpStream) -> String {
    let parsed_request = parse_request(&mut stream);

    let mut filename = find_filename(&parsed_request.get("GET"));
   
    if filename.is_empty() {
       filename = "index".to_string(); 
    }

    let contents = fs::read_to_string(format!("public/{}.html", filename));

    match contents {
        Ok(file_content) => {
            format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}",
                file_content.len(),
                file_content
            )
        },
        Err(_) => {
            let err_file = fs::read_to_string("public/not_found.html").unwrap();
            format!(
                "HTTP/1.1 404 NOT_FOUND\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}",
                err_file.len(),
                err_file 
            )
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let response = get_response(&mut stream);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

