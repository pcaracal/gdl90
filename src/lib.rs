use log::debug;
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
    /// The buffer must contain the CRC
    ///
    /// # Errors
    ///
    /// If the buffer is empty or crc is invalid
    pub fn from_bytes(crc_table: &[u16], buf: &[u8]) -> anyhow::Result<Self> {
        if buf.is_empty() {
            return Err(anyhow::anyhow!("Empty buffer"));
        }

        let buf = if buf[0] == FLAG { &buf[1..] } else { buf };
        let buf = if buf[buf.len() - 1] == FLAG {
            &buf[..buf.len() - 1]
        } else {
            buf
        };

        let buf = unescape(buf);

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

    #[must_use]
    pub fn to_bytes(self, crc_table: &[u16]) -> Vec<u8> {
        match self {
            Self::OwnshipReport(tr) => {
                let mut bytes = vec![10];
                bytes.extend_from_slice(&tr.to_bytes());
                let crc = crc_compute(crc_table, &bytes);
                bytes.extend_from_slice(&crc.to_be_bytes());
                let escaped = escape(&bytes);

                let mut bytes = vec![FLAG];
                bytes.extend_from_slice(&escaped);
                bytes.push(FLAG);
                bytes
            }
            Self::TrafficReport(tr) => {
                let mut bytes = vec![20];
                bytes.extend_from_slice(&tr.to_bytes());
                let crc = crc_compute(crc_table, &bytes);
                bytes.extend_from_slice(&crc.to_be_bytes());
                let escaped = escape(&bytes);

                let mut bytes = vec![FLAG];
                bytes.extend_from_slice(&escaped);
                bytes.push(FLAG);
                bytes
            }
            _ => vec![FLAG, FLAG],
        }
    }
}

#[must_use]
pub fn escape(buf: &[u8]) -> Vec<u8> {
    let mut escaped = Vec::new();

    for byte in buf {
        match byte {
            0x7E => {
                escaped.push(0x7D);
                escaped.push(0x5E);
            }
            0x7D => {
                escaped.push(0x7D);
                escaped.push(0x5D);
            }
            _ => escaped.push(*byte),
        }
    }

    escaped
}

#[must_use]
pub fn unescape(buf: &[u8]) -> Vec<u8> {
    let mut unescaped = Vec::new();
    let mut escape = false;

    for byte in buf {
        if escape {
            match byte {
                0x5E => unescaped.push(0x7E),
                0x5D => unescaped.push(0x7D),
                _ => debug!("Invalid escape sequence"),
            }
            escape = false;
        } else if *byte == 0x7D {
            escape = true;
        } else {
            unescaped.push(*byte);
        }
    }

    unescaped
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
