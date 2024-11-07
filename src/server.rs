use std::io::Error;

use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(listener: TcpListener) -> Self {
        Self { listener }
    }

    pub async fn run(&mut self) -> Result<(), ()> {
        loop {
            let mut sock = match self.accept_conn().await {
                Ok(stream) => stream,
                Err(err) => {
                    println!("{}", err);
                    panic!("Error connection");
                }
            };

            tokio::spawn(async move {
                if let Err(err) = &mut sock.write_all("Hello!".as_bytes()).await {
                    println!("{}", err);
                    panic!("Error writing response");
                }
            });
        }
    }

    async fn accept_conn(&mut self) -> Result<TcpStream, Error> {
        loop {
            return match self.listener.accept().await {
                Ok((sock, _)) => Ok(sock),
                Err(e) => Err(e),
            };
        }
    }
}
