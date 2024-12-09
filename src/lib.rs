pub mod messages;

pub const FLAG: u8 = 0x7E;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum MessageID {
    Heartbeat,
    Initialization,
    UplinkData,
    HeightAboveTerrain,
    OwnshipReport,
    OwnshipGeometricAltitude,
    TrafficReport,
    BasicReport,
    LongReport,
    #[default]
    Unknown,
}

impl From<u8> for MessageID {
    fn from(id: u8) -> Self {
        match id {
            0 => Self::Heartbeat,
            2 => Self::Initialization,
            7 => Self::UplinkData,
            9 => Self::HeightAboveTerrain,
            10 => Self::OwnshipReport,
            11 => Self::OwnshipGeometricAltitude,
            20 => Self::TrafficReport,
            30 => Self::BasicReport,
            31 => Self::LongReport,
            _ => Self::Unknown,
        }
    }
}
