use crate::prelude::*;

type Ctx = (Endian, BitSize);

// 24-bit signed binary fraction.
// Resolution = 180 / 2^23 degrees.
const COORD_CTX: Ctx = (Endian::Big, BitSize(24));
pub(super) fn coord_read<R: std::io::Read + std::io::Seek>(
    reader: &mut deku::reader::Reader<R>,
) -> Result<Angle, DekuError> {
    let enc = i32::from_reader_with_ctx(reader, COORD_CTX)?;
    let deg = (f64::from(enc) * 180.0) / f64::from(1 << 23);
    Ok(deg.degrees())
}
pub(super) fn coord_write<W: std::io::Write + std::io::Seek>(
    writer: &mut Writer<W>,
    coord: Angle,
) -> Result<(), DekuError> {
    let enc: i32 = ((coord.degrees() * f64::from(1 << 23)) / 180.0).clamp_into();
    enc.to_writer(writer, COORD_CTX)
}

// Resolution = 25 feet.
// Altitude (ft) = ("ddd" * 25) - 1,000
const ALTITUDE_RESOLUTION: f64 = 25.0;
const ALTITUDE_CTX: Ctx = (Endian::Big, BitSize(12));
pub(super) fn altitude_read<R: std::io::Read + std::io::Seek>(
    reader: &mut deku::reader::Reader<R>,
) -> Result<Option<Length>, DekuError> {
    let encoded = u16::from_reader_with_ctx(reader, ALTITUDE_CTX)?;
    if encoded == 0xFFF {
        return Ok(None);
    }
    let ft = (f64::from(encoded) * ALTITUDE_RESOLUTION) - 1000.0;
    Ok(Some(ft.feet()))
}
pub(super) fn altitude_write<W: std::io::Write + std::io::Seek>(
    writer: &mut Writer<W>,
    altitude: Option<Length>,
) -> Result<(), DekuError> {
    if let Some(altitude) = altitude {
        let encoded: u16 = ((altitude.feet() + 1000.0) / ALTITUDE_RESOLUTION).clamp_into();
        let encoded = encoded & 0xFFF;
        encoded.to_writer(writer, ALTITUDE_CTX)
    } else {
        0xFFFu16.to_writer(writer, ALTITUDE_CTX)
    }
}

// 12-bit unsigned integer. Resolution = 1 kt.
const HV_CTX: Ctx = (Endian::Big, BitSize(12));
const HV_UNAVAILABLE: u16 = 0xFFF;
pub(super) fn hv_read<R: std::io::Read + std::io::Seek>(
    reader: &mut deku::reader::Reader<R>,
) -> Result<Option<Velocity>, DekuError> {
    let kt = u16::from_reader_with_ctx(reader, HV_CTX)?;

    Ok(if kt == HV_UNAVAILABLE {
        None
    } else {
        Some(kt.knots())
    })
}
pub(super) fn hv_write<W: std::io::Write + std::io::Seek>(
    writer: &mut Writer<W>,
    hv: Option<Velocity>,
) -> Result<(), DekuError> {
    let encoded = if let Some(hv) = hv {
        let hv: u16 = hv.knots().clamp_into();
        hv & 0xFFF
    } else {
        HV_UNAVAILABLE
    };
    encoded.to_writer(writer, HV_CTX)
}

// 12-bit signed integer. Resolution = 64 fpm.
const VV_CTX: Ctx = (Endian::Big, BitSize(12));
const VV_UNAVAILABLE: u16 = 0x800;
pub(super) fn vv_read<R: std::io::Read + std::io::Seek>(
    reader: &mut deku::reader::Reader<R>,
) -> Result<Option<Velocity>, DekuError> {
    let fpm = u16::from_reader_with_ctx(reader, VV_CTX)?;

    Ok(if fpm == VV_UNAVAILABLE {
        None
    } else {
        Some(((fpm << 4).cast_signed() << 2).feet_per_minute())
    })
}
pub(super) fn vv_write<W: std::io::Write + std::io::Seek>(
    writer: &mut Writer<W>,
    vv: Option<Velocity>,
) -> Result<(), DekuError> {
    let encoded = if let Some(vv) = vv {
        let fpm: i16 = (vv.feet_per_minute() / 64.0).clamp_into();
        fpm.cast_unsigned() & 0xFFF
    } else {
        VV_UNAVAILABLE
    };
    encoded.to_writer(writer, VV_CTX)
}

// Resolution = 360/256 degrees.
const HEADING_RESOLUTION: f64 = 360.0 / 256.0;
const HEADING_CTX: Ctx = (Endian::Big, BitSize(8));
pub(super) fn heading_read<R: std::io::Read + std::io::Seek>(
    reader: &mut deku::reader::Reader<R>,
) -> Result<Angle, DekuError> {
    let encoded = u8::from_reader_with_ctx(reader, HEADING_CTX)?;
    let deg = f64::from(encoded) * HEADING_RESOLUTION;
    Ok(deg.degrees())
}
pub(super) fn heading_write<W: std::io::Write + std::io::Seek>(
    writer: &mut Writer<W>,
    heading: Angle,
) -> Result<(), DekuError> {
    let deg = heading.degrees();
    let encoded: u8 = (deg / HEADING_RESOLUTION).clamp_into();
    encoded.to_writer(writer, HEADING_CTX)
}

// 8 ASCII characters, '0' through '9' and 'A' through 'Z'.
pub(super) fn callsign_read<R: std::io::Read + std::io::Seek>(
    reader: &mut deku::reader::Reader<R>,
) -> Result<String, DekuError> {
    let value = <[u8; 8]>::from_reader_with_ctx(reader, ())?;
    String::from_utf8(value.to_vec())
        .map_err(|e| DekuError::Parse(e.to_string().into()))
        .map(|s| s.trim().to_string())
}

pub(super) fn callsign_write<W: std::io::Write + std::io::Seek>(
    writer: &mut Writer<W>,
    callsign: &str,
) -> Result<(), DekuError> {
    let bytes = callsign.as_bytes();
    let to_copy = bytes.len().min(8);
    let mut out = [0x20; 8];
    out[..to_copy].copy_from_slice(&bytes[..to_copy]);
    out.to_writer(writer, ())
}
