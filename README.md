# Rudis 🦀 — Um Mini Redis em Rust

Projeto educacional que implementa um **servidor de chave-valor estilo Redis** com um cliente TCP simples, desenvolvido em **Rust**. O servidor suporta comandos `GET` e `SET`, persistência automática em disco via JSON e é totalmente modularizado.

## 📦 Estrutura do Projeto

```yaml
├── rust-redis-client/ # Cliente TCP que envia comandos para o servidor
│ └── src/main.rs
├── rust-redis-server/ # Servidor estilo Redis com armazenamento persistente
│ ├── src/
│ │ ├── key_value_db.rs # Banco de dados em memória + persistência
│ │ ├── server.rs # Implementação do servidor TCP
│ │ └── main.rs # Ponto de entrada do servidor
└── .gitignore # Ignora pastas target e dados persistidos
```

---

## 🚀 Como executar

### 1. Clone o repositório:

```bash
git clone https://github.com/seu-usuario/rust-redis-clone.git
cd rust-redis-clone
```

### 2. Inicie o servidor
```bash
cd rust-redis-server
cargo run
```

O servidor ficará escutando na porta `7878`.

### 3. Use o cliente
```bash
cd rust-redis-client
cargo run
```

Digite comandos no terminal:

```bash
> SET chave valor da chave
Rudis> Valor setado
> GET chave
Rudis> valor da chave
```

## 💾 Persistência
Todos os pares chave-valor são armazenados no arquivo `rust-redis-server/data.json`, garantindo que os dados não sejam perdidos entre execuções do servidor.

## 📚 Comandos Suportados
- `SET chave valor`: Armazena o valor associado à chave.
- `GET chave`: Retorna o valor da chave, ou "Chave não encontrada".

## 🛠️ Tecnologias Utilizadas
- Rust
- TcpListener/TcpStream para rede
- serde e serde_json para serialização
- regex para parsing de comandos

## 🎯 Objetivos do Projeto
✅ Entender como funciona a comunicação TCP<br>
✅ Criar um banco de dados em memória simples<br>
✅ Persistir dados localmente<br>
✅ Modularizar o código com múltiplos arquivos<br>
✅ Praticar boas práticas com Rust<br>

## 🧠 Aprendizados
- Gerenciamento de conexões TCP
- Manipulação de strings com regex
- Serialização com JSON
- Modularização com múltiplos arquivos .rs
- Controle de erros com Result e match