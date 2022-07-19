use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let a = match handle_request(&mut buffer) {
            Ok(response) => String::from("ok"),
            Err(response) => String::from("funcao que retorna o erro pro cliente"),
    };
}
fn handle_request(buffer: &mut [u8; 1024]) -> Result<String, String>{
    let get = b"GET";
    if buffer.starts_with(get) {
        return handle_get(&buffer);
    }
    let resposta = String::from("Not enough parameters");
    return Err(resposta);
}
fn handle_get(buffer: &[u8; 1024]) -> Result<String, String>{
    let mut first_line = String::from("");
    for i in buffer {
        if *i == b'\n' {
            break;
        }
        first_line.push(*i as char);
    }
    get_parameters(&first_line);
    return Ok("safe".to_string());
}
fn get_parameters<'a>(url: &'a str) -> HashMap<String, String> {
    let mut parameters = HashMap::new();
    let mut split: Vec<&str> = url.split("?").collect();
    let mut split: Vec<&str> = split[1].split(" ").collect();
    let mut split: Vec<&str> = split[0].split("&").collect();
    let mut first_parameter: Vec<&str> = split[0].split("=").collect();
    let mut second_parameter: Vec<&str> = split[1].split("=").collect();
    parameters.insert(first_parameter[0].to_string(), first_parameter[1].to_string());
    parameters.insert(second_parameter[0].to_string(), second_parameter[1].to_string());
    println!("{:?}", parameters);
    return parameters;
} 