use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

use crate::key_value_db::KeyValueDB;

pub struct Server {
    listener: TcpListener,
    port: u16,
    database: KeyValueDB,
}

impl Server {
    pub fn new(port: u16) -> Server {
        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(addr)
            .expect("Não foi possível iniciar o servidor");

        Server {
            listener,
            port,
            database: KeyValueDB::new(),
        }
    }

    pub fn listen(&mut self) {
        let listener = self.listener.try_clone().expect("Erro ao clonar listener");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_connection(stream),
                Err(e) => eprintln!("Erro: {}", e),
            }
        }
    }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        println!("Conectado com {}", stream.local_addr().unwrap());

        let mut buffer = [0u8; 1024];

        match stream.read(&mut buffer) {
            Ok(0) => println!("Conexão encerrada pelo cliente"),
            Ok(n) => {
                let requisicao = String::from_utf8_lossy(&buffer[..n]).to_string();
                println!("Recebido: {}", requisicao);

                let (comando, chave, valor) = self.database.parse_input(&requisicao);

                let resposta = match comando.as_str() {
                    "GET" => self.database.get(chave).unwrap_or("Chave não encontrada".to_string()),
                    "SET" => {
                        self.database.set(chave, valor);
                        self.database.save();
                        "Valor setado".to_string()
                    },
                    _ => "Erro".to_string(),
                };

                if stream.write_all(resposta.as_bytes()).is_err() {
                    println!("Erro ao enviar resposta");
                }
            }
            Err(e) => eprintln!("Erro ao ler do stream: {}", e),
        }
    }
}
