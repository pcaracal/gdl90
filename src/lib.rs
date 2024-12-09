use messages::traffic_report::TrafficReport;

pub mod messages;

pub const FLAG: u8 = 0x7E;

#[derive(Debug, Default, PartialEq)]
pub enum GDL90Message {
    Heartbeat,
    Initialization,
    UplinkData,
    HeightAboveTerrain,
    OwnshipReport(TrafficReport),
    OwnshipGeometricAltitude,
    TrafficReport(TrafficReport),
    BasicReport,
    LongReport,
    #[default]
    Unknown,
}

impl GDL90Message {
    /// Creates a `GDL90Message` from a byte buffer
    /// buffer is expected to not contain FLAG bytes
    ///
    /// # Errors
    ///
    /// If the buffer is empty
    pub fn from_bytes(buf: &[u8]) -> anyhow::Result<Self> {
        if buf.is_empty() {
            return Err(anyhow::anyhow!("Empty buffer"));
        }

        let id = buf[0];
        match id {
            0 => Ok(Self::Heartbeat),
            2 => Ok(Self::Initialization),
            7 => Ok(Self::UplinkData),
            9 => Ok(Self::HeightAboveTerrain),
            10 => Ok(Self::OwnshipReport(TrafficReport::from_bytes(&buf[1..])?)),
            11 => Ok(Self::OwnshipGeometricAltitude),
            20 => Ok(Self::TrafficReport(TrafficReport::from_bytes(&buf[1..])?)),
            30 => Ok(Self::BasicReport),
            31 => Ok(Self::LongReport),
            _ => Ok(Self::Unknown),
        }
    }
}
