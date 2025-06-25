use std::net::{Shutdown, TcpStream};
use std::io::{self, Write, Read, Result};

struct Client {
    connection: TcpStream
}

impl Client {

    fn new(server_ip: &'static str, port: u16) -> Client{
        let addr = format!("{}:{}", server_ip, port);
        let connection = TcpStream::connect(addr).expect("Não foi possível conectar ao servidor");

        Client { connection }
    }

    fn send_message(&mut self, message: String) -> String {

        if self.connection.write_all(message.as_bytes()).is_err(){
            return String::from("Erro ao enviar dados");
        }

        let mut buffer = [0u8; 1024];
        let read_result = self.connection.read(&mut buffer);

        if read_result.is_err() {
            return String::from("Erro ao ler os dados do Servidor");
        }

        let n = read_result.unwrap();
        let resposta =  String::from_utf8_lossy(&buffer[..n]).to_string();
        return  resposta;
    }

    fn disconnect(&mut self) -> bool {
        let res =  match self.connection.shutdown(Shutdown::Both){
            Ok(_) => true,
            Err(_) => false
        };

        return  res;
    }
}

fn main() {

    println!("Conectado ao servidor. Digite algo para enviar:");

    loop {
        print!("> ");
        let _ = io::stdout().flush();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro ao ler do stdin");

        if input.trim().is_empty() {
            break;
        }

        let mut cliente = Client::new("127.0.0.1", 7878);
        let resposta = cliente.send_message(String::from(input.trim_end()));
        println!("Rudis> {}", resposta);

        cliente.disconnect();
    }
}
