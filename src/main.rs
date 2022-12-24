use server::server::new_server;

mod queue;
mod server;


fn main() {
 

    let mut webserver = new_server();
    webserver.start();
}
