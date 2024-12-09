#[macro_use]
extern crate log;
use gdl90::messages::traffic_report::TrafficAlertStatus;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    let socket = UdpSocket::bind("127.0.0.1:4000").await?;
    socket.set_broadcast(true)?;
    info!("UDP Listening on: {}", socket.local_addr()?);

    loop {
        let mut buf = vec![0; 1024];
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let buf = &buf[..len];
        info!("Received {} bytes from {}", buf.len(), addr);
        debug!("Data: {:?}", buf);
    }

    Ok(())
}
