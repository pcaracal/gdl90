#[macro_use]
extern crate log;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    let socket = UdpSocket::bind("0.0.0.0:4000").await?;
    socket.set_broadcast(true)?;
    info!("UDP Listening on: {}", socket.local_addr()?);

    Ok(())
}
