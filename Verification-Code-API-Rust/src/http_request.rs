use std::{collections::HashMap};

#[derive(Debug)]

pub struct Request{

    pub method: String,
    pub host: String,
    pub parameters: HashMap<String, String>,
}

pub trait Methods {
    fn new(buffer: [u8; 1024]) -> Request;
    fn get_methods(buffer: [u8; 1024]) -> String;
    fn get_parameters(buffer: [u8; 1024]) -> HashMap<String, String>;
    fn get_host(buffer: [u8; 1024]) -> String;
}

impl Methods for Request{
    fn new(buffer: [u8; 1024]) -> Request{
        return Request { method: String::from("teste"), host: String::from("teste"), parameters: HashMap::new() };
    }
    fn get_methods(buffer: [u8; 1024]) -> String {

        if buffer.starts_with(b"GET") {
            return String::from("GET");
        }
        return String::from("Invalid");
    }
    fn get_parameters(buffer: [u8; 1024]) -> HashMap<String, String> {
        let mut url = String::from("");
        for i in buffer {
            if i == b'\n' {
                break;
            }
            url.push(i as char);
        }

        let split: Vec<&str> = url.split("?").collect();
        if split.len() != 2 {
            return HashMap::new();
        } else {
            let split: Vec<&str> = split[1].split(" ").collect();
            let split: Vec<&str> = split[0].split("&").collect();
            let mut parameters: HashMap<String, String> = HashMap::new();
            for parameter in split {
                let parameter: Vec<&str> = parameter.split("=").collect();
                parameters.insert(parameter[0].to_string(), parameter[1].to_string());            
            }
            return parameters;
        }
    }

    fn get_host(buffer: [u8; 1024]) -> String {
        let buffer = String::from_utf8_lossy(&buffer).as_ref();
        let host1 = "Host: localhost:7878";
        let host2 = "Host: localhost:7879";
        if buffer.contains(host1){
            return String::from(host1);
        }
        if buffer.contains(host2){
            return String::from(host2);
        }
        return String::from("Invalid");
    }
}

