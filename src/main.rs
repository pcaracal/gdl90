#[macro_use]
extern crate log;
use std::{env, sync::Arc, time::Duration};

use gdl90::{crc_init, messages::traffic_report::TrafficReportBuilder, GDL90Message};
use tokio::{
    net::UdpSocket,
    time::{interval, Instant},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    let socket = UdpSocket::bind("0.0.0.0:4000").await?;
    socket.set_broadcast(true)?;
    info!("UDP Listening on: {}", socket.local_addr()?);

    let crc_table = Arc::new(crc_init());

    let mut tr = TrafficReportBuilder::new()
        .with_lat_lon_alt_hdg(
            50.033687266045376,
            8.525938397372625,
            1000,
            179.0,
            gdl90::messages::traffic_report::TrackHeading::HeadingTrue,
        )
        .horizontal_velocity_kt(40)
        .build();

    let mut i = interval(Duration::from_millis(100));
    let mut ts = Instant::now();
    loop {
        let msg = GDL90Message::OwnshipReport(tr.clone());
        let bytes = msg.to_bytes(&crc_table);

        let r = socket
            .send_to(
                &bytes,
                env::var("SEND_TO").expect("SEND_TO env var must be set"),
            )
            .await;
        info!("Sent: {:?}", r);

        let plat = tr.latitude_deg;
        let plon = tr.longitude_deg;
        let pspd = tr.horizontal_velocity_kt;
        let phdg = tr.track_heading;
        debug!(
            "Ownship: lat={}, lon={}, spd={}, hdg={}",
            plat, plon, pspd, phdg
        );

        let (lat, lon) = move_pos(
            (plat, plon),
            pspd.into(),
            phdg,
            Instant::now().duration_since(ts).as_secs_f64(),
        );

        tr.latitude_deg = lat;
        tr.longitude_deg = lon;

        ts = Instant::now();
        i.tick().await;
    }
}

#[must_use]
pub fn move_pos(
    start: (f64, f64),
    speed_knots: f64,
    heading_deg: f64,
    duration_secs: f64,
) -> (f64, f64) {
    const EARTH_RADIUS_NM: f64 = 3443.92;
    let speed_nmps = speed_knots / 3600.0;
    let distance_nm = speed_nmps * duration_secs;

    let lat1 = start.0.to_radians();
    let lon1 = start.1.to_radians();
    let heading = heading_deg.to_radians();
    let central = distance_nm / EARTH_RADIUS_NM;

    let lat2 = (lat1.sin() * central.cos() + lat1.cos() * central.sin() * heading.cos()).asin();

    let lon2 = (heading.sin() * central.sin() * lat1.cos())
        .atan2(central.cos() - lat1.sin() * lat2.sin())
        + lon1;

    (lat2.to_degrees(), lon2.to_degrees())
}
