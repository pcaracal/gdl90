use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, DekuRead, DekuWrite, Builder)]
#[deku(bit_order = "msb", endian = "big")]
/// # 3.7. Height Above Terrain
pub struct HeightAboveTerrain {
    /// Height above terrain. Resolution = 1 foot
    ///
    /// None = invalid (default)
    #[deku(
        reader = "HeightAboveTerrain::hat_read(deku::reader)",
        writer = "HeightAboveTerrain::hat_write(deku::writer, self.height_above_terrain)"
    )]
    pub height_above_terrain: Option<Length>,
}

impl HeightAboveTerrain {
    // 16-bit signed integer. Resolution = 1 foot
    const HAT_CTX: (Endian, ByteSize) = (Endian::Big, ByteSize(2));
    const HAT_INVALID: i16 = 0x8000u16.cast_signed();
    fn hat_read<R: std::io::Read + std::io::Seek>(
        reader: &mut deku::reader::Reader<R>,
    ) -> Result<Option<Length>, DekuError> {
        let ft = i16::from_reader_with_ctx(reader, Self::HAT_CTX)?;

        Ok(if ft == Self::HAT_INVALID {
            None
        } else {
            Some(ft.feet())
        })
    }
    fn hat_write<W: std::io::Write + std::io::Seek>(
        writer: &mut Writer<W>,
        hat: Option<Length>,
    ) -> Result<(), DekuError> {
        let encoded = if let Some(hat) = hat {
            hat.feet().clamp_into()
        } else {
            Self::HAT_INVALID
        };
        encoded.to_writer(writer, Self::HAT_CTX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BYTES: [u8; 2] = [0x12, 0x34];

    #[test]
    fn decode() {
        let hat = HeightAboveTerrain::from_bytes((&BYTES, 0)).unwrap().1;
        assert_eq!(hat.height_above_terrain, Some(0x1234.feet()));
    }

    #[test]
    fn encode() {
        let hat = HeightAboveTerrain::new(Some(0x1234.feet()));
        let bytes = hat.to_bytes().unwrap();
        assert_eq!(bytes, BYTES);
    }
}
