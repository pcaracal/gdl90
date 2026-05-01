use crate::prelude::*;

mod crc;
mod r#impl;

pub trait GDL90Encode {
    /// Encode into a GDL90 byte vector, ready to be sent.
    ///
    /// Data encoded, CRC added, escaped and flag bytes set.
    ///
    /// GDL90 allows multiple messages to be sent in a single packet.
    /// The resulting byte vector can be concatenated with others
    /// and sent together as one packet, without any additional processing.
    ///
    /// # Errors
    ///
    /// If something went wrong during serialization of the message data.
    fn into_gdl90_bytes(self) -> GDL90Result<Vec<u8>>;
}

impl Message {
    /// Consume a GDL90 packet and parse all messages in the given bytes.
    ///
    /// # Errors
    ///
    /// - Message too short (less than 3 bytes)
    /// - CRC mismatch
    /// - Invalid escape sequence
    /// - Message data parsing errors
    #[must_use]
    pub fn from_gdl90_bytes(bytes: impl AsRef<[u8]>) -> Vec<GDL90Result<Message>> {
        Self::from_gdl90_bytes_impl(bytes)
    }
}

/// GDL90 Messages + ForeFlight extended spec
///
/// # Usage
///
/// `Message::into_gdl90_bytes()` to encode a message into a GDL90 byte vector.
/// `Message::from_gdl90_bytes()` to parse an encoded GDL90 packet into messages.
///
/// `GDL90Encode::into_gdl90_bytes()` is implemented for `T: Into<Message>`, which is implemented for every variant's inner type.
/// It's not necessary to wrap message data in `Message`, as `into_gdl90_bytes()` can be used on the inner type directly.
///
/// <https://www.faa.gov/sites/faa.gov/files/air_traffic/technology/adsb/archival/GDL90_Public_ICD_RevA.PDF>
#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite, EnumGet)]
#[deku(id_type = "u8")]
pub enum Message {
    #[deku(id = 0)]
    Heartbeat(Heartbeat),

    #[deku(id = 2)]
    Initialization(Initialization),

    #[deku(id = 7)]
    UplinkData(Box<UplinkData>),

    #[deku(id = 9)]
    HeightAboveTerrain(HeightAboveTerrain),

    #[deku(id = 10)]
    Ownship(OwnshipMessage),

    #[deku(id = 11)]
    OwnshipGeometricAltitude(OwnshipGeometricAltitude),

    #[deku(id = 20)]
    Traffic(TrafficMessage),

    #[deku(id = 30)]
    BasicReport,

    #[deku(id = 31)]
    LongReport,

    #[deku(id = 0x65)]
    ForeFlight(ForeFlightMessage),

    #[deku(id = 0xC9)]
    Custom(CustomMessage),
}

/// ForeFlight Messages (extended spec)
///
/// `ForeFlightMessage::*` inner types implement `Into<ForeFlightMessage>` and `Into<Message>`,
/// so `ForeFlightID` and `ForeFlightAHRS` can be encoded into GDL90 directly.
///
/// These messages have a ForeFlight Message sub-ID, so a separate enum makes encoding and decoding much simpler.
///
/// <https://www.foreflight.com/connect/spec>
#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite, EnumGet)]
#[deku(id_type = "u8")]
pub enum ForeFlightMessage {
    #[deku(id = 0)]
    ID(ForeFlightID),

    #[deku(id = 1)]
    AHRS(ForeFlightAHRS),
}

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite, EnumGet)]
#[deku(id_type = "u8")]
pub enum CustomMessage {
    #[deku(id = 0)]
    PreciseOwnship(CustomPreciseOwnship),
}

#[cfg(test)]
mod tests;
