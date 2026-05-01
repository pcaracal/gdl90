use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, DekuRead, DekuWrite, Builder)]
#[deku(bit_order = "msb", endian = "big")]
/// Custom message I made up for some testing purposes, this is not real
/// Lat, Lon, Alt, GS each sent with 64bit precision
pub struct CustomPreciseOwnship {
    #[deku(
        reader = "CustomPreciseOwnship::angle_read(deku::reader)",
        writer = "CustomPreciseOwnship::angle_write(deku::writer, self.latitude)"
    )]
    pub latitude: Angle,

    #[deku(
        reader = "CustomPreciseOwnship::angle_read(deku::reader)",
        writer = "CustomPreciseOwnship::angle_write(deku::writer, self.longitude)"
    )]
    pub longitude: Angle,

    /// Altitude in feet
    #[deku(
        reader = "CustomPreciseOwnship::length_read(deku::reader)",
        writer = "CustomPreciseOwnship::length_write(deku::writer, self.altitude)"
    )]
    pub altitude: Length,

    /// Ground Speed in knots
    #[deku(
        reader = "CustomPreciseOwnship::velocity_read(deku::reader)",
        writer = "CustomPreciseOwnship::velocity_write(deku::writer, self.ground_speed)"
    )]
    pub ground_speed: Velocity,
}

impl CustomPreciseOwnship {
    const CTX: (Endian, ByteSize) = (Endian::Big, ByteSize(8));

    fn angle_read<R: std::io::Read + std::io::Seek>(
        reader: &mut deku::reader::Reader<R>,
    ) -> Result<Angle, DekuError> {
        Ok(f64::from_reader_with_ctx(reader, Self::CTX)?.degrees())
    }
    fn angle_write<W: std::io::Write + std::io::Seek>(
        writer: &mut Writer<W>,
        deg: Angle,
    ) -> Result<(), DekuError> {
        deg.degrees().to_writer(writer, Self::CTX)
    }

    fn length_read<R: std::io::Read + std::io::Seek>(
        reader: &mut deku::reader::Reader<R>,
    ) -> Result<Length, DekuError> {
        Ok(f64::from_reader_with_ctx(reader, Self::CTX)?.feet())
    }
    fn length_write<W: std::io::Write + std::io::Seek>(
        writer: &mut Writer<W>,
        len: Length,
    ) -> Result<(), DekuError> {
        len.feet().to_writer(writer, Self::CTX)
    }

    fn velocity_read<R: std::io::Read + std::io::Seek>(
        reader: &mut deku::reader::Reader<R>,
    ) -> Result<Velocity, DekuError> {
        Ok(f64::from_reader_with_ctx(reader, Self::CTX)?.knots())
    }
    fn velocity_write<W: std::io::Write + std::io::Seek>(
        writer: &mut Writer<W>,
        vel: Velocity,
    ) -> Result<(), DekuError> {
        vel.knots().to_writer(writer, Self::CTX)
    }
}
