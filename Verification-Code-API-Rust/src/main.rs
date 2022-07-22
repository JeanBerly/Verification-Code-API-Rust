use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use rand::Rng;
use rand::thread_rng;
use code::VerificationCode;
pub mod code;

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
    let _a = match handle_request(&mut buffer) {
            Ok(_response) => String::from("ok"),
            Err(_response) => String::from("funcao que retorna o erro pro cliente"),
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
    match get_parameters(&first_line, &vec!["name".to_string(), "email".to_string()]){
        Ok(param) => (send_get_response(&Ok(param))),
        Err(err) => println!("ERRO: {}", err)
    };
    return Ok("safe".to_string());
}
fn get_parameters<'a>(url: &'a str, arr: &'a Vec<String>) -> Result<HashMap<String, String>, String> {
    let mut parameters = HashMap::new();
    let split: Vec<&str> = url.split("?").collect();
    if split.len() != 2 {
        return Err("Não foram passados parâmetros".to_string());
    }
    let split: Vec<&str> = split[1].split(" ").collect();
    let split: Vec<&str> = split[0].split("&").collect();
    let mut parameters_left = arr.len();
    for parameter in split {
        let parameter: Vec<&str> = parameter.split("=").collect();
        if arr.contains(&parameter[0].to_owned()){
            parameters.insert(parameter[0].to_string(), parameter[1].to_string());
            parameters_left -= 1;
        }
        if parameters_left == 0{
            break;
        }
    }
    if parameters.len() != 2 {
        return Err("Foi passado o número errado de parâmetros.".to_string());
    }
    return Ok(parameters);
}

fn send_get_response(result: &Result<HashMap<String, String>, String>){
    let response = match result {
        Ok(result) => generate_code(&result),   
        Err(err) => Err(err.to_owned())
    };
    match response {
        Ok(b) => println!("{:?}", b),
        Err(err) => println!("Erro: {}", err)
    }
}

fn generate_code(hashmap: &HashMap<String, String>) -> Result<VerificationCode, String>{
    let code: String = random_code_generator();
    if let Some(name) = hashmap.get("name"){
        if let Some(email) = hashmap.get("email"){
            return Ok(VerificationCode{
                code: code,
                name: name.to_owned(),
                email: email.to_owned()  
            });
        }
    }
    return Err(String::from("Erro"));
}
fn random_code_generator() -> String{
    let mut code = String::from("");
    for _i in 0..5{
        let mut t = thread_rng().gen::<u8>();
        if t > 9 {
            t = t%9;
        }
        code.push((t+48) as char);
    } 
    return code;
}