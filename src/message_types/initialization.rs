use crate::prelude::*;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Builder)]
#[deku(bit_order = "msb", endian = "big")]
/// # 3.2. Initialization Message
pub struct Initialization {
    /// Initiate audio test
    #[deku(bits = 1, pad_bits_before = "1", pad_bits_after = "4")]
    pub audio_test: bool,

    /// Suppress GDL 90 audio output
    #[deku(bits = 1)]
    pub audio_inhibit: bool,

    /// CDTI capability is operating
    #[deku(bits = 1, pad_bits_after = "6")]
    pub cdti_ok: bool,

    /// Disable GDL 90 audible traffic alerts
    #[deku(bits = 1)]
    pub csa_audio_disable: bool,

    /// Disable CSA traffic alerting
    #[deku(bits = 1)]
    pub csa_disable: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use deku::{DekuContainerRead, DekuContainerWrite};

    const BYTES: [u8; 2] = [0b0100_0001, 0b0000_0010];

    #[test]
    fn decode() {
        let init = Initialization::from_bytes((&BYTES, 0)).unwrap().1;

        assert!(init.audio_test);
        assert!(!init.audio_inhibit);
        assert!(init.cdti_ok);

        assert!(init.csa_audio_disable);
        assert!(!init.csa_disable);
    }

    #[test]
    fn encode() {
        let init = Initialization::default()
            .with_audio_test()
            .with_cdti_ok()
            .with_csa_audio_disable();

        let bytes = init.to_bytes().unwrap();
        assert_eq!(bytes, BYTES);
    }
}
