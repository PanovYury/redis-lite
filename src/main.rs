
use redis_lite::server::Server;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let listener = match TcpListener::bind("127.0.0.1:6379").await {
        Ok(tcp_listener) => tcp_listener,
        Err(err) => panic!(
            "Could not bind the TCP listener to 127.0.0.1:6379. Err: {}",
            err
        ),
    };

    let mut server = Server::new(listener);

    server.run().await?;

    Ok(())
}
