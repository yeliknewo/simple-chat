mod client;
mod server;

fn main() {

    let stdin = std::io::stdin();

    let mut buffer = String::new();

    stdin.read_line(&mut buffer).unwrap();

    if buffer.contains("server") {
        server::start_server("127.0.0.1:80").unwrap();
    } else if buffer.contains("client") {
        client::start_client("192.168.1.136:80").unwrap();
    }

}
