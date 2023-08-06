use crate::server::Server;

mod server;
mod router;
mod handler;

fn main() {
    Server::new("localhost:3000").run()
}
