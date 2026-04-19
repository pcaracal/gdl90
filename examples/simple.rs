#![allow(clippy::unreadable_literal)]

mod common;

use common::prelude::*;

use indicatif::ProgressBar;

#[macro_use]
extern crate utilities_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

const LAT: f64 = 47.464722;
const LON: f64 = 8.549167;
const ALT: i32 = 31000;
const HDG: u16 = 214;
const INTERVAL: Duration = Duration::from_millis(100);

#[tokio::main]
async fn main() -> Result<()> {
    init_logger();

    let target = TargetState::load_or_input().await?;
    info!(
        "Sends heartbeat + ownship + ahrs every {}ms to {}",
        INTERVAL.as_millis().blue(),
        target.addr().blue()
    );

    info!("Latitude: {}", LAT.yellow());
    info!("Longitude: {}", LON.yellow());
    info!("Altitude: {} ft", ALT.to_string().yellow());
    info!("Heading: {}°\n", HDG.to_string().yellow());

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(INTERVAL);

    let mut socket = Socket::new(target).await?;
    let mut interval = tokio::time::interval(INTERVAL);
    loop {
        interval.tick().await;

        let heartbeat = Heartbeat::default()
            .with_gps_pos_valid()
            .with_uat_initialized()
            .with_utc_ok()
            .with_timestamp_now();

        let ownship = TrafficReport::default()
            .with_latitude(LAT.degrees())
            .with_longitude(LON.degrees())
            .with_altitude(ALT.feet())
            .with_track_heading(HDG.degrees())
            .with_miscellaneous_indicators(MiscellaneousIndicators::new(
                AirGroundState::Airborne,
                ReportType::Updated,
                TrackHeadingType::HeadingTrue,
            ))
            .with_horizontal_velocity(0.knots())
            .ownship();

        let ahrs = ForeFlightAHRS::default()
            .with_heading(HDG.degrees())
            .with_heading_type(AHRSHeadingType::True);

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&heartbeat.into_gdl90_bytes()?);
        bytes.extend_from_slice(&ownship.into_gdl90_bytes()?);
        bytes.extend_from_slice(&ahrs.into_gdl90_bytes()?);

        socket.send(bytes).await?;
        pb.set_message(socket.stats());
    }
}
