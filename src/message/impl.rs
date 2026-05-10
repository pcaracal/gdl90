use crate::{
    bail,
    message::{GDL90Encode, Message, crc::crc_calc},
    prelude::*,
};

pub(crate) const FLAG: u8 = 0x7E;
pub(crate) const ESCAPE: u8 = 0x7D;
pub(crate) const ESCAPE_XOR: u8 = 0x20;

// Traits

impl<T: Into<Message>> GDL90Encode for T {
    fn into_gdl90_bytes(self) -> GDL90Result<Vec<u8>> {
        self.into().into_gdl90_bytes_impl()
    }
}

macro_rules! impl_message_from {
    ($($variant:ident($ty:ty)),* $(,)?) => {
        $(
            impl From<$ty> for Message {
                fn from(value: $ty) -> Self {
                    Self::$variant(value.into())
                }
            }
        )*
    };
}

impl_message_from! {
    Heartbeat(Heartbeat),
    Initialization(Initialization),
    UplinkData(UplinkData),
    HeightAboveTerrain(HeightAboveTerrain),
    Ownship(OwnshipMessage),
    OwnshipGeometricAltitude(OwnshipGeometricAltitude),
    Traffic(TrafficMessage),
    ForeFlight(ForeFlightMessage),
    Custom(CustomMessage),
}

impl From<ForeFlightID> for Message {
    fn from(value: ForeFlightID) -> Self {
        Self::ForeFlight(ForeFlightMessage::ID(value))
    }
}

impl From<ForeFlightAHRS> for Message {
    fn from(value: ForeFlightAHRS) -> Self {
        Self::ForeFlight(ForeFlightMessage::AHRS(value))
    }
}

impl From<CustomPreciseOwnship> for Message {
    fn from(value: CustomPreciseOwnship) -> Self {
        Self::Custom(CustomMessage::PreciseOwnship(value))
    }
}

impl Message {
    #[must_use]
    pub fn is_fore_flight_id(&self) -> bool {
        matches!(self, Self::ForeFlight(ForeFlightMessage::ID(_)))
    }
    #[must_use]
    pub fn is_fore_flight_ahrs(&self) -> bool {
        matches!(self, Self::ForeFlight(ForeFlightMessage::AHRS(_)))
    }
    #[must_use]
    pub fn is_custom_precise_ownship(&self) -> bool {
        matches!(self, Self::Custom(CustomMessage::PreciseOwnship(_)))
    }
    #[must_use]
    pub fn fore_flight_id(&self) -> Option<&ForeFlightID> {
        if let Self::ForeFlight(ForeFlightMessage::ID(id)) = self {
            Some(id)
        } else {
            None
        }
    }
    #[must_use]
    pub fn fore_flight_ahrs(&self) -> Option<&ForeFlightAHRS> {
        if let Self::ForeFlight(ForeFlightMessage::AHRS(ahrs)) = self {
            Some(ahrs)
        } else {
            None
        }
    }
    #[must_use]
    pub fn custom_precise_ownship(&self) -> Option<&CustomPreciseOwnship> {
        if let Self::Custom(CustomMessage::PreciseOwnship(ownship)) = self {
            Some(ownship)
        } else {
            None
        }
    }
}

// Encoding & Decoding

impl Message {
    /// Message -> GDL90 packet
    fn into_gdl90_bytes_impl(self) -> GDL90Result<Vec<u8>> {
        let escaped = MessageWrapper::new(self)?.escape()?;

        let mut result = Vec::with_capacity(escaped.escaped_bytes.len() + 2);
        result.push(FLAG);
        result.extend(escaped.escaped_bytes);
        result.push(FLAG);

        Ok(result)
    }

    /// GDL90 packet -> Messages
    pub(super) fn from_gdl90_bytes_impl(bytes: impl AsRef<[u8]>) -> Vec<GDL90Result<Self>> {
        Self::split_bytes_to_escaped(bytes)
            .into_iter()
            .map(EscapedMessage::unescape)
            .collect::<Vec<GDL90Result<Self>>>()
    }

    fn split_bytes_to_escaped(bytes: impl AsRef<[u8]>) -> Vec<EscapedMessage> {
        let mut escaped_messages: Vec<EscapedMessage> = vec![];
        let mut start: Option<usize> = None;
        for (i, &byte) in bytes.as_ref().iter().enumerate() {
            if byte == FLAG {
                if let Some(s) = start
                    && i > s + 1
                {
                    escaped_messages.push(EscapedMessage::new(bytes.as_ref()[s + 1..i].to_vec()));
                }
                start = Some(i);
            }
        }
        escaped_messages
    }
}

#[derive(Debug, Default, Clone, Builder)]
struct EscapedMessage {
    escaped_bytes: Vec<u8>,
}

/// No flag bytes
impl EscapedMessage {
    /// Remove all escape sequences, validate length & crc, and parse the actual message data
    fn unescape(self) -> GDL90Result<Message> {
        let mut acc: Vec<u8> = Vec::with_capacity(self.escaped_bytes.len());

        for byte in self.escaped_bytes {
            if let Some(last) = acc.last()
                && *last == ESCAPE
            {
                acc.pop();
                acc.push(Self::unescape_byte(byte)?);
            } else {
                acc.push(byte);
            }
        }

        MessageWrapper::from_unescaped_bytes(acc).map(|w| w.message)
    }

    fn unescape_byte(byte: u8) -> GDL90Result<u8> {
        let unescaped = byte ^ ESCAPE_XOR;
        if unescaped != FLAG && unescaped != ESCAPE {
            bail!(GDL90Error::InvalidEscapeSequence(byte));
        }
        Ok(unescaped)
    }
}

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite, Builder)]
#[builder(skip(ctor))]
/// Convenience wrapper to encode & decode GDL90 messages with crc to/from bytes
struct MessageWrapper {
    message: Message,
    #[deku(endian = "little")]
    crc: u16,
}

impl MessageWrapper {
    /// For encoding. Sets the crc
    fn new(message: Message) -> GDL90Result<Self> {
        let message_bytes = message.to_bytes()?;
        let crc = crc_calc(&message_bytes);
        Ok(Self { message, crc })
    }

    /// For decoding. Validates length & crc and parses the message data
    fn from_unescaped_bytes(bytes: impl AsRef<[u8]>) -> GDL90Result<Self> {
        let bytes = bytes.as_ref();
        let len = bytes.len();
        if len < 3 {
            bail!(GDL90Error::MessageTooShort(len));
        }

        let wrapper = Self::try_from(bytes)?;
        let crc_actual = crc_calc(&bytes[..len - 2]);

        if crc_actual != wrapper.crc {
            bail!(GDL90Error::CrcMismatch {
                expected: wrapper.crc,
                got: crc_actual
            });
        }

        Ok(wrapper)
    }

    /// Assumes crc is already set. wrapper -> escaped
    fn escape(self) -> GDL90Result<EscapedMessage> {
        let bytes = self
            .to_bytes()?
            .into_iter()
            .flat_map(Self::escape_byte)
            .collect::<Vec<u8>>();

        Ok(EscapedMessage::new(bytes))
    }

    fn escape_byte(byte: u8) -> Vec<u8> {
        if byte == FLAG || byte == ESCAPE {
            vec![ESCAPE, byte ^ ESCAPE_XOR]
        } else {
            vec![byte]
        }
    }
}
