use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;
use tokio_native_tls::TlsAcceptor;
mod configuration;



#[tokio::main]
async fn main() -> io::Result<()> {
    let mut tcp_listener = TcpListener::bind("127.0.0.0")
    .await
    .map_err(|e| {
        e
    })?;

    Ok(())
}
