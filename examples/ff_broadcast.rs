#![allow(clippy::unreadable_literal)]

#[macro_use]
extern crate utilities_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

mod common;

use std::{
    net::SocketAddr,
    sync::{Arc, atomic::AtomicUsize},
    time::Instant,
};

use common::prelude::*;
use dashmap::DashMap;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::net::UdpSocket;

const INTERVAL: Duration = Duration::from_millis(100);

#[tokio::main]
async fn main() -> Result<()> {
    init_logger();

    let count = Arc::new(AtomicUsize::new(0));
    let apps: Arc<DashMap<SocketAddr, (Instant, ForeFlightBroadcast)>> = Arc::default();
    let socket = UdpSocket::bind("0.0.0.0:63093").await?;
    socket.set_broadcast(true)?;

    let pb = ProgressBar::new_spinner().with_style(ProgressStyle::with_template(
        "{spinner} {elapsed_precise} {msg}",
    )?);
    pb.enable_steady_tick(INTERVAL);

    let pbc = pb.clone();
    let countc = count.clone();
    let appsc = apps.clone();
    tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        loop {
            let Ok((len, addr)) = socket.recv_from(&mut buf).await else {
                pbc.println("Failed to receive data");
                continue;
            };

            match ForeFlightBroadcast::from_json(&buf[..len]) {
                Ok(b) => {
                    countc.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    appsc.insert(addr, (Instant::now(), b));
                }
                Err(why) => pbc.println(format!("Failed to parse broadcast from {addr}: {why}")),
            }
        }
    });

    let mut i = tokio::time::interval(INTERVAL);
    loop {
        i.tick().await;

        let mut lines = apps
            .iter()
            .map(|entry| {
                let dur = entry.0.elapsed().as_secs_f32();
                let time = format!("{dur:.1}s");
                let time = if dur < 6. {
                    time.green().to_string()
                } else if dur < 15. {
                    time.yellow().to_string()
                } else {
                    time.red().to_string()
                };

                (
                    entry.0,
                    format!(
                        "App {}: Port {} ({time} ago from {})",
                        entry.1.app.blue(),
                        entry.1.gdl90.port.blue(),
                        entry.key().blue()
                    ),
                )
            })
            .collect::<Vec<_>>();
        lines.sort_by_key(|(i, _)| *i);
        pb.set_message(format!(
            "{} Broadcasts\n\n{}",
            count.load(std::sync::atomic::Ordering::SeqCst).blue(),
            lines
                .into_iter()
                .map(|(_, s)| s)
                .collect::<Vec<_>>()
                .join("\n"),
        ));
    }
}
