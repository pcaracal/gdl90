use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Builder)]
#[deku(bit_order = "msb", endian = "big")]
/// # ForeFlight ID Message
pub struct ForeFlightID {
    #[deku(bytes = 1)]
    #[builder(skip(ctor), default = 1)]
    /// Must be 1
    pub version: u8,

    #[deku(bytes = 8)]
    pub device_serial_number: u64,

    #[deku(
        reader = "ForeFlightID::name_read(deku::reader)",
        writer = "ForeFlightID::name_write(deku::writer, &self.device_name)"
    )]
    pub device_name: String,

    #[deku(
        reader = "ForeFlightID::long_name_read(deku::reader)",
        writer = "ForeFlightID::long_name_write(deku::writer, &self.device_long_name)"
    )]
    pub device_long_name: String,

    #[deku(pad_bits_before = "29")]
    pub capabilities: Capabilities,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Builder)]
#[deku(ctx = "_: deku::ctx::Endian, _: deku::ctx::Order")]
/// 4 bytes, 3 bits used (at the end)
pub struct Capabilities {
    pub foreflight_internet_policy: ForeFlightInternetPolicy,
    pub geometric_altitude_datum: GeometricAltitudeDatum, // this one goes last since LSB?
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, EnumGet)]
#[deku(id_type = "u8", bits = 1)]
#[repr(u8)]
pub enum GeometricAltitudeDatum {
    #[default]
    WGS84 = 0,
    MSL = 1,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, EnumGet)]
#[deku(id_type = "u8", bits = 2)]
#[repr(u8)]
pub enum ForeFlightInternetPolicy {
    // not sure if the bit order is correct here
    #[default]
    Unrestricted = 0,
    Expensive = 1,
    Disallowed = 2,
}

impl Default for ForeFlightID {
    fn default() -> Self {
        Self {
            version: 1,
            device_serial_number: Self::INVALID_DEVICE_SERIAL_NUMBER,
            device_name: String::new(),
            device_long_name: String::new(),
            capabilities: Capabilities::default(),
        }
    }
}

impl ForeFlightID {
    pub const INVALID_DEVICE_SERIAL_NUMBER: u64 = u64::MAX;

    #[must_use]
    pub fn with_geometric_altitude_datum(
        mut self,
        geometric_altitude_datum: GeometricAltitudeDatum,
    ) -> Self {
        self.capabilities.geometric_altitude_datum = geometric_altitude_datum;
        self
    }

    #[must_use]
    pub fn with_foreflight_internet_policy(
        mut self,
        foreflight_internet_policy: ForeFlightInternetPolicy,
    ) -> Self {
        self.capabilities.foreflight_internet_policy = foreflight_internet_policy;
        self
    }
}

impl ForeFlightID {
    fn read_str<R: std::io::Read + std::io::Seek, const N: usize>(
        reader: &mut deku::reader::Reader<R>,
    ) -> Result<String, DekuError> {
        let value = <[u8; N]>::from_reader_with_ctx(reader, ByteSize(N))?;
        String::from_utf8(value.to_vec())
            .map_err(|e| DekuError::Parse(e.to_string().into()))
            .map(|s| s.trim().to_string())
    }

    fn write_str<W: std::io::Write + std::io::Seek, const N: usize>(
        writer: &mut Writer<W>,
        name: &str,
    ) -> Result<(), DekuError> {
        let bytes = name.as_bytes();
        let mut out = [0x20; N];
        let to_copy = bytes.len().min(N);
        out[..to_copy].copy_from_slice(&bytes[..to_copy]);
        out.to_writer(writer, ())
    }

    fn name_read<R: std::io::Read + std::io::Seek>(
        reader: &mut deku::reader::Reader<R>,
    ) -> Result<String, DekuError> {
        Self::read_str::<R, 8>(reader)
    }

    fn name_write<W: std::io::Write + std::io::Seek>(
        writer: &mut Writer<W>,
        name: &str,
    ) -> Result<(), DekuError> {
        Self::write_str::<W, 8>(writer, name)
    }

    fn long_name_read<R: std::io::Read + std::io::Seek>(
        reader: &mut deku::reader::Reader<R>,
    ) -> Result<String, DekuError> {
        Self::read_str::<R, 16>(reader)
    }

    fn long_name_write<W: std::io::Write + std::io::Seek>(
        writer: &mut Writer<W>,
        name: &str,
    ) -> Result<(), DekuError> {
        Self::write_str::<W, 16>(writer, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BYTES: [u8; 37] = [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0xD2, 0x4E, 0x61, 0x6D, 0x65, 0x2E, 0x20,
        0x20, 0x20, 0x4C, 0x6F, 0x6E, 0x67, 0x20, 0x4E, 0x61, 0x6D, 0x65, 0x2E, 0x20, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0x03,
    ];

    #[test]
    fn decode() {
        let id = ForeFlightID::from_bytes((&BYTES, 0)).unwrap().1;

        assert_eq!(id.version, 1);
        assert_eq!(id.device_serial_number, 1234);
        assert_eq!(id.device_name, "Name.");
        assert_eq!(id.device_long_name, "Long Name.");
        assert_eq!(
            id.capabilities,
            Capabilities::new(
                ForeFlightInternetPolicy::Expensive,
                GeometricAltitudeDatum::MSL,
            )
        );
    }

    #[test]
    fn encode() {
        let id = ForeFlightID::default()
            .with_device_serial_number(1234)
            .with_device_name("Name.")
            .with_device_long_name("Long Name.")
            .with_geometric_altitude_datum(GeometricAltitudeDatum::MSL)
            .with_foreflight_internet_policy(ForeFlightInternetPolicy::Expensive);

        let bytes = id.to_bytes().unwrap();
        assert_eq!(bytes, BYTES);
    }
}
