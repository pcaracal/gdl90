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
    /// The buffer is expected to not contain FLAG bytes
    /// The buffer must contain the CRC
    ///
    /// # Errors
    ///
    /// If the buffer is empty or crc is invalid
    pub fn from_bytes(crc_table: &[u16], buf: &[u8]) -> anyhow::Result<Self> {
        if buf.is_empty() {
            return Err(anyhow::anyhow!("Empty buffer"));
        }

        if buf.len() < 3 {
            return Err(anyhow::anyhow!("Buffer too short"));
        }

        let crc = u16::from_be_bytes([buf[buf.len() - 2], buf[buf.len() - 1]]);
        let comp = crc_compute(crc_table, &buf[..buf.len() - 2]);
        if crc != comp {
            return Err(anyhow::anyhow!(
                "Invalid CRC - Expected {crc:04X} - Got {comp:04X}"
            ));
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

#[must_use]
pub fn crc_init() -> Vec<u16> {
    let mut crc_table = vec![0; 256];
    let mut crc: u16;

    for i in 0..256 {
        crc = i << 8;
        for _ in 0..8 {
            crc = (crc << 1) ^ (if (crc & 0x8000) > 0 { 0x1021 } else { 0 });
        }
        crc_table[i as usize] = crc;
    }

    crc_table
}

#[must_use]
pub fn crc_compute(crc_table: &[u16], buf: &[u8]) -> u16 {
    let mut crc: u16 = 0;
    for byte in buf {
        crc = crc_table[(crc >> 8) as usize] ^ (crc << 8) ^ u16::from(*byte);
    }

    crc.swap_bytes()
}
