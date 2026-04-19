use chrono::Timelike;

use crate::prelude::*;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Builder)]
#[deku(bit_order = "msb", endian = "big")]
/// # 3.1. Heartbeat Message
///
/// The GDL 90 outputs a Heartbeat message at the beginning of each UTC second.
pub struct Heartbeat {
    /// position is available for ADS-B Tx
    #[deku(bits = 1)]
    pub gps_pos_valid: bool,

    /// GDL 90 Maintenance Req'd
    #[deku(bits = 1)]
    pub maint_reqd: bool,

    /// IDENT talkback
    #[deku(bits = 1)]
    pub ident: bool,

    /// Address Type talkback
    #[deku(bits = 1)]
    pub addr_type: bool,

    /// GPS Battery low voltage
    #[deku(bits = 1)]
    pub gps_batt_low: bool,

    /// ATC Services talkback
    #[deku(bits = 1, pad_bits_after = "1")]
    pub ratcs: bool,

    /// GDL 90 is initialized
    #[deku(bits = 1)]
    pub uat_initialized: bool,

    /// Seconds since 0000Z (UTC midnight), bit 16 (MSB)
    #[deku(bits = 1)]
    #[builder(skip)]
    timestamp_msb: bool,

    /// CSA has been requested
    #[deku(bits = 1)]
    pub csa_requested: bool,

    /// CSA is not available at this time
    #[deku(bits = 1, pad_bits_after = "4")]
    pub csa_not_available: bool,

    /// UTC timing is valid
    #[deku(bits = 1)]
    pub utc_ok: bool,

    /// Seconds since 0000Z (UTC midnight), bits 0-15
    #[deku(bits = 16, endian = "little")]
    #[builder(skip)]
    timestamp: u16,

    /// # 3.1.4. Received Message Counts
    #[deku(bits = 16)]
    pub message_counts: u16,
}

impl Heartbeat {
    #[must_use]
    /// Seconds since 0000Z (UTC midnight)
    pub fn timestamp(&self) -> u32 {
        (u32::from(self.timestamp_msb) << 16) | u32::from(self.timestamp)
    }

    /// Seconds since 0000Z (UTC midnight)
    pub fn set_timestamp(&mut self, timestamp: u32) {
        self.timestamp_msb = (timestamp & (1 << 16)) != 0;
        self.timestamp = (timestamp & 0xFFFF) as u16;
    }

    #[must_use]
    /// Seconds since 0000Z (UTC midnight)
    pub fn with_timestamp(mut self, timestamp: u32) -> Self {
        self.set_timestamp(timestamp);
        self
    }

    /// Set the timestamp to now (seconds since 0000Z (UTC midnight))
    pub fn set_timestamp_now(&mut self) {
        self.set_timestamp(Self::now());
    }

    #[must_use]
    /// Set the timestamp to now (seconds since 0000Z (UTC midnight))
    pub fn with_timestamp_now(mut self) -> Self {
        self.set_timestamp_now();
        self
    }

    fn now() -> u32 {
        chrono::Utc::now().num_seconds_from_midnight()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deku::{DekuContainerRead, DekuContainerWrite};

    const BYTES: [u8; 6] = [0b1000_0001, 0b0100_0001, 0xDB, 0xD0, 0x08, 0x02];

    #[test]
    fn decode() {
        let hb = Heartbeat::from_bytes((&BYTES, 0)).unwrap().1;

        assert!(hb.gps_pos_valid);
        assert!(!hb.maint_reqd);
        assert!(!hb.ident);
        assert!(!hb.addr_type);
        assert!(!hb.gps_batt_low);
        assert!(!hb.ratcs);
        assert!(hb.uat_initialized);

        assert!(!hb.timestamp_msb);
        assert!(hb.csa_requested);
        assert!(!hb.csa_not_available);
        assert!(hb.utc_ok);

        assert_eq!(hb.timestamp(), 0x0_D0_DB);

        assert_eq!(hb.message_counts, 0x0802);
    }

    #[test]
    fn encode() {
        let mut hb = Heartbeat::default()
            .with_gps_pos_valid()
            .with_uat_initialized()
            .with_csa_requested()
            .with_utc_ok()
            .with_timestamp(0x0_D0_DB)
            .with_message_counts(0x0802);

        assert_eq!(hb.timestamp(), 0x0_D0_DB);

        let bytes = hb.to_bytes().unwrap();
        assert_eq!(bytes, BYTES);

        let (_, hb2) = Heartbeat::from_bytes((&bytes, 0)).unwrap();
        assert_eq!(hb, hb2);
        assert_eq!(hb.timestamp(), hb2.timestamp());

        hb.timestamp_msb = true;
        assert_eq!(hb.timestamp(), 0x1_D0_DB);
    }
}
