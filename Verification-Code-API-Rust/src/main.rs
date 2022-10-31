use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use http_request::{Request, Methods};
use rand::Rng;
use rand::thread_rng;
use code::VerificationCode;
use std::thread;
pub mod code;
pub mod http_request;

fn main() {
    
    let codes: Arc<Mutex<HashMap<String, VerificationCode>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut codes_clone1 = Arc::clone(&codes);
    let mut codes_clone2 = Arc::clone(&codes);


    let thread1 = thread::spawn(move || {
    let listener_generate = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener_generate.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request = Request::new(buffer);
        // if let Err(ref e) = res {
        //     println!("{}", e);
        // }   
        // if let Ok(ok) = res {
        //     codes
        // }
    }});

    let thread2 = thread::spawn(move || {
        let listener_check_code = TcpListener::bind("127.0.0.1:7879").unwrap();
        for stream_check_code in listener_check_code.incoming(){
        let stream_check_code = stream_check_code.unwrap();
        let _res_check_code = handle_verification(stream_check_code, &mut codes_clone2);
    }});

    thread1.join().unwrap();
    thread2.join().unwrap();

}

// fn handle_connection(mut stream: TcpStream, codes: &mut Arc<Mutex<HashMap<String, VerificationCode>>>) -> Result<VerificationCode, String> {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();
//     let buffer = String::from_utf8_lossy(&buffer).as_ref();
//     let res = handle_request(&mut buffer);
//     if let Err(ref e) = res {
//         let status_line = "HTTP/1.1 404 NOT FOUND";
//         let contents = format!("Erro: {}", e);
//         let response = format!(
//             "{}\r\nContent-Length: {}\r\n\r\n{}",
//             status_line,
//             contents.len(),
//             contents
//         );
//         stream.write(response.as_bytes()).unwrap();
//         return Err(e.to_string());
//     }
//     if let Ok(ok) = res {
//         if exists_valid_existent_code(codes, &ok.email) {
//             let status_line = "HTTP/1.1 404 NOT FOUND";
//             let contents = format!("Erro: Já existe um código válido",);
//             let response = format!(
//                 "{}\r\nContent-Length: {}\r\n\r\n{}",
//                 status_line,
//                 contents.len(),
//                 contents
//             );
//             stream.write(response.as_bytes()).unwrap();
//             return Err(contents.to_string());
//         }
//         let status_line = "HTTP/1.1 200 OK";
//         let contents = format!("code: {}, email: {}, name: {}", ok.code, ok.email, ok.name);
//         let response = format!(
//             "{}\r\nContent-Length: {}\r\n\r\n{}",
//             status_line,
//             contents.len(),
//             contents
//         );
//         stream.write(response.as_bytes()).unwrap();
//         codes.lock().unwrap().insert(ok.email.to_string(), VerificationCode{code: ok.code.to_owned(), name: ok.name.to_owned(), email: ok.email.to_owned(), emission_time: ok.emission_time});
//         return Ok(ok);
//     }
//     return Err("fodase".to_string());
// }
// fn exists_valid_existent_code(codes: &Mutex<HashMap<String, VerificationCode>>, email: &str) -> bool{
//     if let Some(code) = codes.lock().unwrap().get(&email.to_owned()){
//         if code.emission_time.elapsed().unwrap() > Duration::from_secs(10) {
//             return false;
//         }
//         return true;
//     }
//     return false;
// }
// fn handle_request(buffer: &str) -> Result<VerificationCode, String>{
//     let get = "GET";
//     if buffer.starts_with(get) {
//         handle_port(buffer);
//         return handle_get(&buffer);
//     }
//     let resposta = String::from("Only get request");
//     return Err(resposta);
// }

// fn handle_port(buffer: &str){
//     let host1 = b"Host: localhost:7878";
//     let host2 = b"Host: localhost:7879";
//     if buffer.contains(host1){
//         return handle_request(buffer);
//     }
//     if buffer.contais(host2){
//         return handle_verification(buffer);
//     }
// }

// fn handle_get(buffer: &str) -> Result<VerificationCode, String>{
//     let mut first_line = String::from("");
//     for i in buffer {
//         if *i == b'\n' {
//             break;
//         }
//         first_line.push(*i as char);
//     }
//     match get_parameters(&first_line, &vec!["name".to_string(), "email".to_string()]){
//         Ok(param) => return generate_get_response(&Ok(param)),
//         Err(err) => return Err(err)
//     };
// }
// fn get_parameters<'a>(url: &'a str, arr: &'a Vec<String>) -> Result<HashMap<String, String>, String> {
//     let mut parameters = HashMap::new();
//     let split: Vec<&str> = url.split("?").collect();
//     if split.len() != 2 {
//         return Err("Não foram passados parâmetros".to_string());
//     }
//     let split: Vec<&str> = split[1].split(" ").collect();
//     let split: Vec<&str> = split[0].split("&").collect();
//     let mut parameters_left = arr.len();
//     for parameter in split {
//         let parameter: Vec<&str> = parameter.split("=").collect();
//         if arr.contains(&parameter[0].to_owned()){
//             parameters.insert(parameter[0].to_string(), parameter[1].to_string());
//             parameters_left -= 1;
//         }
//         if parameters_left == 0{
//             break;
//         }
//     }
//     if parameters.len() != 2 {
//         return Err("Foi passado o número errado de parâmetros.".to_string());
//     }
//     return Ok(parameters);
// }
//
// fn generate_get_response(result: &Result<HashMap<String, String>, String>) -> Result<VerificationCode, String>{
//     match result {
//         Ok(param) => return generate_code(param),
//         Err(err) => return Err(err.to_owned())
//     };
// }

fn generate_code(hashmap: &HashMap<String, String>) -> Result<VerificationCode, String>{
    let code: String = random_code_generator();
    if let Some(name) = hashmap.get("name"){
        if let Some(email) = hashmap.get("email"){
            return Ok(VerificationCode{
                code: code,
                name: name.to_owned(),
                email: email.to_owned(),
                emission_time: SystemTime::now()
            });
        }
    }
    return Err(String::from("Erro ao gerar código"));
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

fn handle_verification(mut stream: TcpStream, codes: &mut Arc<Mutex<HashMap<String, VerificationCode>>>){
        

}