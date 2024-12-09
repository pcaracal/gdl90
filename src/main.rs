#[macro_use]
extern crate log;
use gdl90::messages::traffic_report::TrafficReport;
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

        let tr = TrafficReport::from_bytes(&buf[2..]);
        info!("{:#?}", tr);
    }

    // 3.5.2 Traffic Report Example
    // let data = &[
    //     0x14, 0x00, 0xAB, 0x45, 0x49, 0x1F, 0xEF, 0x15, 0xA8, 0x89, 0x78, 0x0F, 0x09, 0xA9,
    //     0x07, 0xB0, 0x01, 0x20, 0x01, 0x4E, 0x38, 0x32, 0x35, 0x56, 0x20, 0x20, 0x20, 0x00,
    // ];
}
