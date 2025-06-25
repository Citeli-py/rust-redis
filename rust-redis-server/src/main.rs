mod key_value_db;
mod server;

use server::Server;


fn main() {
    let mut redis_server = Server::new(7878);
    redis_server.listen();
}
