const fn crc_init() -> [u16; 256] {
    let mut crc_table = [0u16; 256];
    let mut i = 0;

    while i < 256 {
        let mut crc = i << 8;
        let mut j = 0;
        while j < 8 {
            crc = (crc << 1) ^ (if (crc & 0x8000) > 0 { 0x1021 } else { 0 });
            j += 1;
        }
        crc_table[i as usize] = crc;
        i += 1;
    }
    crc_table
}

pub const CRC_TABLE: [u16; 256] = crc_init();

#[must_use]
pub fn crc_calc(buf: impl AsRef<[u8]>) -> u16 {
    let mut crc: u16 = 0;
    for byte in buf.as_ref() {
        crc = CRC_TABLE[(crc >> 8) as usize] ^ (crc << 8) ^ u16::from(*byte);
    }
    crc
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_crc_init() {
        assert_eq!(CRC_TABLE[0], 0x0000);
        assert_eq!(CRC_TABLE[1], 0x1021);
        assert_eq!(CRC_TABLE[2], 0x2042);
        assert_eq!(CRC_TABLE[254], 0x0ED1);
        assert_eq!(CRC_TABLE[255], 0x1EF0);
    }

    #[test]
    fn test_crc_compute() {
        let crc = crc_calc(b"a few bytes to check yes");
        assert_eq!(crc, 0x2A72);

        let crc = crc_calc([0x00, 0x81, 0x41, 0xDB, 0xD0, 0x08, 0x02]);
        assert_eq!(crc, 0x8BB3);
    }
}
