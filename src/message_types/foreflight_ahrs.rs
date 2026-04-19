use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, DekuRead, DekuWrite, Builder)]
#[deku(bit_order = "msb", endian = "big")]
/// # ForeFlight AHRS Message
pub struct ForeFlightAHRS {
    #[deku(
        reader = "ForeFlightAHRS::roll_pitch_read(deku::reader)",
        writer = "ForeFlightAHRS::roll_pitch_write(deku::writer, self.roll)"
    )]
    pub roll: Option<Angle>,

    #[deku(
        reader = "ForeFlightAHRS::roll_pitch_read(deku::reader)",
        writer = "ForeFlightAHRS::roll_pitch_write(deku::writer, self.pitch)"
    )]
    pub pitch: Option<Angle>,

    pub heading_type: AHRSHeadingType,

    #[deku(
        reader = "ForeFlightAHRS::hdg_read(deku::reader)",
        writer = "ForeFlightAHRS::hdg_write(deku::writer, self.heading)"
    )]
    pub heading: Option<Angle>,

    #[deku(
        reader = "ForeFlightAHRS::ias_tas_read(deku::reader)",
        writer = "ForeFlightAHRS::ias_tas_write(deku::writer, self.indicated_airspeed)"
    )]
    pub indicated_airspeed: Option<Velocity>,

    #[deku(
        reader = "ForeFlightAHRS::ias_tas_read(deku::reader)",
        writer = "ForeFlightAHRS::ias_tas_write(deku::writer, self.true_airspeed)"
    )]
    pub true_airspeed: Option<Velocity>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, EnumGet)]
#[deku(
    id_type = "u8",
    bits = 1,
    ctx = "_: deku::ctx::Endian, _: deku::ctx::Order"
)]
#[repr(u8)]
pub enum AHRSHeadingType {
    #[default]
    True = 0,
    Magnetic = 1,
}

impl ForeFlightAHRS {
    const CTX: (Endian, ByteSize) = (Endian::Big, ByteSize(2));
    const HDG_CTX: (Endian, BitSize) = (Endian::Big, BitSize(15));
    const ROLL_PITCH_INVALID: i16 = 0x7FFF;
    const HDG_INVALID: i16 = 0xFFFFu16.cast_signed();
    const IAS_TAS_INVALID: u16 = 0xFFFF;

    fn roll_pitch_read<R: std::io::Read + std::io::Seek>(
        reader: &mut deku::reader::Reader<R>,
    ) -> Result<Option<Angle>, DekuError> {
        let deg10 = i16::from_reader_with_ctx(reader, Self::CTX)?;
        Ok(if deg10 == Self::ROLL_PITCH_INVALID {
            None
        } else {
            Some(deg10.degrees() / 10.0)
        })
    }
    fn roll_pitch_write<W: std::io::Write + std::io::Seek>(
        writer: &mut Writer<W>,
        deg: Option<Angle>,
    ) -> Result<(), DekuError> {
        let deg10: i16 = if let Some(deg) = deg {
            (deg.degrees() * 10.0).clamp_into()
        } else {
            Self::ROLL_PITCH_INVALID
        };
        deg10.to_writer(writer, Self::CTX)
    }

    fn hdg_read<R: std::io::Read + std::io::Seek>(
        reader: &mut deku::reader::Reader<R>,
    ) -> Result<Option<Angle>, DekuError> {
        let deg10 = i16::from_reader_with_ctx(reader, Self::HDG_CTX)?;
        Ok(if deg10 == Self::HDG_INVALID {
            None
        } else {
            Some(deg10.degrees() / 10.0)
        })
    }
    fn hdg_write<W: std::io::Write + std::io::Seek>(
        writer: &mut Writer<W>,
        deg: Option<Angle>,
    ) -> Result<(), DekuError> {
        let deg10: i16 = if let Some(deg) = deg {
            let deg = deg.get::<degree>();
            (deg * 10.0).clamp_into()
        } else {
            Self::HDG_INVALID
        };
        deg10.to_writer(writer, Self::HDG_CTX)
    }

    fn ias_tas_read<R: std::io::Read + std::io::Seek>(
        reader: &mut deku::reader::Reader<R>,
    ) -> Result<Option<Velocity>, DekuError> {
        let speed = u16::from_reader_with_ctx(reader, Self::CTX)?;
        Ok(if speed == Self::IAS_TAS_INVALID {
            None
        } else {
            Some(speed.knots())
        })
    }
    fn ias_tas_write<W: std::io::Write + std::io::Seek>(
        writer: &mut Writer<W>,
        speed: Option<Velocity>,
    ) -> Result<(), DekuError> {
        let speed: u16 = if let Some(speed) = speed {
            speed.knots().clamp_into()
        } else {
            Self::IAS_TAS_INVALID
        };
        speed.to_writer(writer, Self::CTX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BYTES0: [u8; 10] = [0x07, 0x08, 0x07, 0x08, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF];
    const BYTES1: [u8; 10] = [0x01, 0x41, 0x01, 0x41, 0x00, 0x00, 0x00, 0x01, 0x00, 0x02];
    const BYTES2: [u8; 10] = [0x00, 0x00, 0x00, 0x00, 0x07, 0x08, 0x00, 0x12, 0x00, 0x12];
    const BYTES3: [u8; 10] = [0xFF, 0x85, 0xFF, 0x85, 0xFF, 0x85, 0x00, 0xF3, 0x00, 0xF4];
    const BYTES4: [u8; 10] = [0xF8, 0xF8, 0xF8, 0xF8, 0xF8, 0xF8, 0x0A, 0x04, 0x0E, 0x08];
    const BYTES5: [u8; 10] = [0x7F, 0xFF, 0x7F, 0xFF, 0x01, 0x41, 0xFF, 0xFF, 0xF0, 0x0F];

    #[test]
    fn decode() {
        let a0 = ForeFlightAHRS::from_bytes((&BYTES0, 0)).unwrap().1;
        let a1 = ForeFlightAHRS::from_bytes((&BYTES1, 0)).unwrap().1;
        let a2 = ForeFlightAHRS::from_bytes((&BYTES2, 0)).unwrap().1;
        let a3 = ForeFlightAHRS::from_bytes((&BYTES3, 0)).unwrap().1;
        let a4 = ForeFlightAHRS::from_bytes((&BYTES4, 0)).unwrap().1;
        let a5 = ForeFlightAHRS::from_bytes((&BYTES5, 0)).unwrap().1;

        assert_eq!(a0.roll, Some(180.degrees()));
        assert_eq!(a1.roll, Some(32.1.degrees()));
        assert_eq!(a2.roll, Some(0.degrees()));
        assert_eq!(a3.roll, Some(-12.3.degrees()));
        assert_eq!(a4.roll, Some(-180.degrees()));
        assert_eq!(a5.roll, None);

        assert_eq!(a0.pitch, Some(180.degrees()));
        assert_eq!(a1.pitch, Some(32.1.degrees()));
        assert_eq!(a2.pitch, Some(0.degrees()));
        assert_eq!(a3.pitch, Some(-12.3.degrees()));
        assert_eq!(a4.pitch, Some(-180.degrees()));
        assert_eq!(a5.pitch, None);

        assert_eq!(a0.heading_type, AHRSHeadingType::Magnetic);
        assert_eq!(a1.heading_type, AHRSHeadingType::True);
        assert_eq!(a2.heading_type, AHRSHeadingType::True);
        assert_eq!(a3.heading_type, AHRSHeadingType::Magnetic);
        assert_eq!(a4.heading_type, AHRSHeadingType::Magnetic);
        assert_eq!(a5.heading_type, AHRSHeadingType::True);

        assert_eq!(a0.heading, None);
        assert_eq!(a1.heading, Some(0.degrees()));
        assert_eq!(a2.heading, Some(180.degrees()));
        assert_eq!(a3.heading, Some(-12.3.degrees()));
        assert_eq!(a4.heading, Some(-180.degrees()));
        assert_eq!(a5.heading, Some(32.1.degrees()));

        assert_eq!(a0.indicated_airspeed, Some(0.knots()));
        assert_eq!(a1.indicated_airspeed, Some(1.knots()));
        assert_eq!(a2.indicated_airspeed, Some(18.knots()));
        assert_eq!(a3.indicated_airspeed, Some(243.knots()));
        assert_eq!(a4.indicated_airspeed, Some(2564.knots()));
        assert_eq!(a5.indicated_airspeed, None);

        assert_eq!(a0.true_airspeed, None);
        assert_eq!(a1.true_airspeed, Some(2.knots()));
        assert_eq!(a2.true_airspeed, Some(18.knots()));
        assert_eq!(a3.true_airspeed, Some(244.knots()));
        assert_eq!(a4.true_airspeed, Some(3592.knots()));
        assert_eq!(a5.true_airspeed, Some(61455.knots()));
    }

    #[test]
    fn encode() {
        let a0 = ForeFlightAHRS {
            roll: Some(180.degrees()),
            pitch: Some(180.degrees()),
            heading_type: AHRSHeadingType::Magnetic,
            heading: None,
            indicated_airspeed: Some(0.knots()),
            true_airspeed: None,
        };
        let a1 = ForeFlightAHRS {
            roll: Some(32.1.degrees()),
            pitch: Some(32.1.degrees()),
            heading_type: AHRSHeadingType::True,
            heading: Some(0.degrees()),
            indicated_airspeed: Some(1.knots()),
            true_airspeed: Some(2.knots()),
        };
        let a2 = ForeFlightAHRS {
            roll: Some(0.degrees()),
            pitch: Some(0.degrees()),
            heading_type: AHRSHeadingType::True,
            heading: Some(180.degrees()),
            indicated_airspeed: Some(18.knots()),
            true_airspeed: Some(18.knots()),
        };
        let a3 = ForeFlightAHRS {
            roll: Some(-12.3.degrees()),
            pitch: Some(-12.3.degrees()),
            heading_type: AHRSHeadingType::Magnetic,
            heading: Some(-12.3.degrees()),
            indicated_airspeed: Some(243.knots()),
            true_airspeed: Some(244.knots()),
        };
        let a4 = ForeFlightAHRS {
            roll: Some(-180.degrees()),
            pitch: Some(-180.degrees()),
            heading_type: AHRSHeadingType::Magnetic,
            heading: Some(-180.degrees()),
            indicated_airspeed: Some(2564.knots()),
            true_airspeed: Some(3592.knots()),
        };
        let a5 = ForeFlightAHRS {
            roll: None,
            pitch: None,
            heading_type: AHRSHeadingType::True,
            heading: Some(32.1.degrees()),
            indicated_airspeed: None,
            true_airspeed: Some(61455.knots()),
        };

        for (i, (a, b)) in [
            (a0, BYTES0),
            (a1, BYTES1),
            (a2, BYTES2),
            (a3, BYTES3),
            (a4, BYTES4),
            (a5, BYTES5),
        ]
        .into_iter()
        .enumerate()
        {
            let e = a.to_bytes().unwrap();
            assert_eq!(e, b, "a{i} - b{i}");
            let d = ForeFlightAHRS::from_bytes((&e, 0)).unwrap().1;
            assert_eq!(a, d, "a{i} - b{i}");
        }
    }
}
