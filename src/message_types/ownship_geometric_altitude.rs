use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, DekuRead, DekuWrite, Builder)]
#[deku(bit_order = "msb", endian = "big")]
/// # 3.8. Ownship Geometric Altitude
pub struct OwnshipGeometricAltitude {
    /// Ownship Geometric Altitude. Resolution = 5 feet
    #[deku(
        reader = "OwnshipGeometricAltitude::oga_read(deku::reader)",
        writer = "OwnshipGeometricAltitude::oga_write(deku::writer, self.ownship_geo_altitude)"
    )]
    pub ownship_geo_altitude: Length,

    pub vertical_metrics: VerticalMetrics,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, DekuRead, DekuWrite, Builder)]
#[deku(ctx = "_: deku::ctx::Endian, _: deku::ctx::Order")]
pub struct VerticalMetrics {
    #[deku(bits = 1)]
    pub vertical_warning_indicator: bool,

    #[deku(
        reader = "VerticalMetrics::vfom_read(deku::reader)",
        writer = "VerticalMetrics::vfom_write(deku::writer, self.vfom)"
    )]
    /// Vertical Figure of Merit (`VFOM`). Resolution = 1 meter
    ///
    /// `None` = unavailable
    /// `VerticalMetrics::VFOM_MAX` = 32766m or more
    pub vfom: Option<Length>,
}

impl OwnshipGeometricAltitude {
    // 16-bit signed integer. Resolution = 5 feet
    const OGA_CTX: (Endian, ByteSize) = (Endian::Big, ByteSize(2));
    fn oga_read<R: std::io::Read + std::io::Seek>(
        reader: &mut deku::reader::Reader<R>,
    ) -> Result<Length, DekuError> {
        let ft = i16::from_reader_with_ctx(reader, Self::OGA_CTX)?;
        Ok((ft * 5).feet())
    }
    fn oga_write<W: std::io::Write + std::io::Seek>(
        writer: &mut Writer<W>,
        oga: Length,
    ) -> Result<(), DekuError> {
        let ft: i16 = (oga.feet() / 5.0).clamp_into();
        ft.to_writer(writer, Self::OGA_CTX)
    }
}

impl VerticalMetrics {
    // 16-bit usigned integer. Resolution = 1 meter
    const VFOM_CTX: (Endian, BitSize) = (Endian::Big, BitSize(15));
    const VFOM_UNAVAILABLE: u16 = 0x7FFF;
    pub const VFOM_MAX: u16 = 0x7FFE;
    fn vfom_read<R: std::io::Read + std::io::Seek>(
        reader: &mut deku::reader::Reader<R>,
    ) -> Result<Option<Length>, DekuError> {
        let m = u16::from_reader_with_ctx(reader, Self::VFOM_CTX)?;
        Ok(if m == Self::VFOM_UNAVAILABLE {
            None
        } else {
            Some(m.min(Self::VFOM_MAX).meters())
        })
    }
    fn vfom_write<W: std::io::Write + std::io::Seek>(
        writer: &mut Writer<W>,
        vfom: Option<Length>,
    ) -> Result<(), DekuError> {
        let encoded = if let Some(vfom) = vfom {
            let m: u16 = vfom.meters().clamp_into();
            m.min(Self::VFOM_MAX)
        } else {
            Self::VFOM_UNAVAILABLE
        };
        encoded.to_writer(writer, Self::VFOM_CTX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BYTES1: [u8; 4] = [0xFF, 0x38, 0xFF, 0xFF];
    const BYTES2: [u8; 4] = [0x00, 0x00, 0x7F, 0xFE];
    const BYTES3: [u8; 4] = [0x00, 0xC8, 0x00, 0x0A];
    const BYTES4: [u8; 4] = [0x12, 0x34, 0x80, 0x32];

    #[test]
    fn decode() {
        let (_, oga1) = OwnshipGeometricAltitude::from_bytes((&BYTES1, 0)).unwrap();
        let (_, oga2) = OwnshipGeometricAltitude::from_bytes((&BYTES2, 0)).unwrap();
        let (_, oga3) = OwnshipGeometricAltitude::from_bytes((&BYTES3, 0)).unwrap();
        let (_, oga4) = OwnshipGeometricAltitude::from_bytes((&BYTES4, 0)).unwrap();

        assert_eq!(oga1.ownship_geo_altitude, -1000.feet());
        assert_eq!(oga2.ownship_geo_altitude, 0.feet());
        assert_eq!(oga3.ownship_geo_altitude, 1000.feet());
        assert_eq!(oga4.ownship_geo_altitude, 23300.feet());
    }

    #[test]
    fn encode() {
        let oga1 = OwnshipGeometricAltitude {
            ownship_geo_altitude: -1000.feet(),
            vertical_metrics: VerticalMetrics {
                vertical_warning_indicator: true,
                vfom: None,
            },
        };
        let oga2 = OwnshipGeometricAltitude {
            ownship_geo_altitude: 0.feet(),
            vertical_metrics: VerticalMetrics {
                vertical_warning_indicator: false,
                vfom: Some(40000.meters()),
            },
        };
        let oga3 = OwnshipGeometricAltitude {
            ownship_geo_altitude: 1000.feet(),
            vertical_metrics: VerticalMetrics {
                vertical_warning_indicator: false,
                vfom: Some(10.meters()),
            },
        };
        let oga4 = OwnshipGeometricAltitude {
            ownship_geo_altitude: 23300.feet(),
            vertical_metrics: VerticalMetrics {
                vertical_warning_indicator: true,
                vfom: Some(50.meters()),
            },
        };

        let bytes1 = oga1.to_bytes().unwrap();
        let bytes2 = oga2.to_bytes().unwrap();
        let bytes3 = oga3.to_bytes().unwrap();
        let bytes4 = oga4.to_bytes().unwrap();

        assert_eq!(bytes1, BYTES1);
        assert_eq!(bytes2, BYTES2);
        assert_eq!(bytes3, BYTES3);
        assert_eq!(bytes4, BYTES4);
    }
}
