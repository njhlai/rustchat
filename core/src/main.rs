mod modules;

use modules::server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new(8080);
    server.run().await;
}
