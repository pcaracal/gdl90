use indicatif::{HumanBytes, HumanCount};
use termwiz::lineedit::{LineEditor, NopLineEditorHost, line_editor_terminal};

use std::{collections::VecDeque, net::Ipv4Addr};

pub mod prelude {
    pub use crate::common::*;

    pub use anyhow::Result;
    pub use gdl90::prelude::*;
    pub use owo_colors::OwoColorize;
    pub use std::time::Duration;
}
use prelude::*;

pub fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    pretty_env_logger::init();
}

pub const STATE_FILE: &str = "examples-state.json";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Builder)]
#[builder(skip(ctor))]
pub struct TargetState {
    pub target_ip: Ipv4Addr,
    pub target_port: u16,
}

impl TargetState {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.target_ip, self.target_port)
    }

    pub async fn load_or_input() -> Result<Self> {
        let state = match TargetState::load_from_file().await {
            Ok(state) => state,
            Err(why) => {
                warn!("Failed to load state from file: {why}");

                loop {
                    let mut terminal = line_editor_terminal()?;
                    let mut editor = LineEditor::new(&mut terminal);
                    let mut host = NopLineEditorHost::default();

                    println!("\nPlease enter the target IP:port ('q' or 'exit' to quit)");

                    let Some(line) = editor.read_line(&mut host)? else {
                        continue;
                    };

                    if line.trim().eq_ignore_ascii_case("q")
                        || line.trim().eq_ignore_ascii_case("exit")
                    {
                        std::process::exit(0);
                    }

                    if let Ok(state) = Self::parse_from_str(&line) {
                        if let Err(why) = state.save_to_file() {
                            error!("Failed to save state to file: {why}");
                        }
                        break state;
                    }
                }
            }
        };

        Ok(state)
    }

    pub async fn load_from_file() -> Result<Self> {
        let state_str = tokio::fs::read_to_string(STATE_FILE).await?;
        let state = serde_json::from_str(&state_str)?;
        Ok(state)
    }

    fn parse_from_str(line: &str) -> Result<Self> {
        let mut parts = line.split(':');
        let ip_str = parts.next().ok_or(anyhow!("Missing IP address"))?;
        let port_str = parts.next().ok_or(anyhow!("Missing port"))?;
        let target_ip: Ipv4Addr = ip_str.parse()?;
        let target_port: u16 = port_str.parse()?;
        Ok(TargetState {
            target_ip,
            target_port,
        })
    }

    fn save_to_file(&self) -> Result<()> {
        let state_str = serde_json::to_string_pretty(self)?;
        std::fs::write(STATE_FILE, state_str)?;
        Ok(())
    }
}

#[derive(Debug, Builder)]
#[builder(async_ctor, return_type = anyhow::Result<Self>)]
pub struct Socket {
    pub target: TargetState,

    #[builder(skip(set, ctor), default = tokio::net::UdpSocket::bind("0.0.0.0:4000").await?)]
    pub s: tokio::net::UdpSocket,

    #[builder(skip(ctor))]
    pub messages_sent: u64,

    #[builder(skip(ctor))]
    pub bytes_sent_total: u64,

    #[builder(skip(ctor))]
    pub last_few_bytes_sent: VecDeque<u64>,
}

impl Socket {
    pub async fn send(&mut self, data: impl AsRef<[u8]>) -> anyhow::Result<()> {
        let s = self.s.send_to(data.as_ref(), self.target.addr()).await?;
        self.messages_sent += 1;
        self.bytes_sent_total += s as u64;
        self.last_few_bytes_sent.push_front(s as u64);
        self.last_few_bytes_sent.truncate(10);
        Ok(())
    }

    pub fn stats(&self) -> String {
        format!(
            "Total: {} Messages ({}) | {}",
            HumanCount(self.messages_sent).blue(),
            HumanBytes(self.bytes_sent_total).blue(),
            self.last_few_bytes_sent
                .iter()
                .map(|b| HumanBytes(*b).blue().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
