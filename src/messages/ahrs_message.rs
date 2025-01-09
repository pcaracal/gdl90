use anyhow::anyhow;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AHRSMessage {
    pub roll: f64,
    pub pitch: f64,
    pub true_or_magnetic_heading: bool, // True = 0, Magnetic = 1
    pub heading: f64,
    pub indicated_airspeed: f64,
    pub true_airspeed: f64,
}

impl AHRSMessage {
    /// Creates an `AHRSMessage` from a byte buffer
    /// Buffer must begin with AHRS Sub-Message D and end with True Airspeed
    /// Must not contain CRC, ID or Flag bytes
    /// Must already be escaped
    #[allow(clippy::missing_errors_doc, clippy::cast_possible_wrap)]
    pub fn from_bytes(buf: &[u8]) -> anyhow::Result<Self> {
        if buf.len() != 11 {
            return Err(anyhow!("Invalid buffer length"));
        }

        // Roll in units of 1/10 degree
        // 0x7fff for invalid.
        // Positive values indicate right wing down, negative values indicate right wing up.
        // The message will be rejected if roll is outside of the range [-1800, 1800]
        let roll = u16::from(buf[1]) << 8 | u16::from(buf[2]);
        let roll = roll as i16;
        if roll == 0x7FFF {
            return Err(anyhow!("Invalid roll"));
        }
        let roll = f64::from(roll) / 10.0;

        let mut ar = Self {
            roll,
            ..Default::default()
        };

        // Pitch in units of 1/10 degree
        // 0x7fff for invalid.
        // Positive values indicate nose up, negative values indicate nose down.
        // The message will be rejected if pitch is outside of the range [-1800, 1800]
        let pitch = u16::from(buf[3]) << 8 | u16::from(buf[4]);
        let pitch = pitch as i16;
        if pitch == 0x7FFF {
            return Err(anyhow!("Invalid pitch"));
        }
        ar.pitch = f64::from(pitch) / 10.0;

        // Most significant bit (bit 15)
        // 0: True Heading
        // 1: Magnetic Heading
        // Bits 14-0: Heading in units of 1/10 degree
        // Track should NOT be used here.
        // 0xffff for invalid.
        // The message will be rejected if heading is outside of the range [-3600,3600]
        ar.true_or_magnetic_heading = buf[5] & 0x80 == 0;
        let heading = u16::from(buf[5] & 0x7F) << 8 | u16::from(buf[6]);
        if heading == 0xFFFF {
            return Err(anyhow!("Invalid heading"));
        }
        let heading = heading << 1;
        let heading = heading as i16;
        let heading = heading >> 1;
        ar.heading = f64::from(heading) / 10.0;

        // Value in Knots
        // 0xffff for invalid.
        let indicated_airspeed = u16::from(buf[7]) << 8 | u16::from(buf[8]);
        if indicated_airspeed == 0xFFFF {
            return Err(anyhow!("Invalid indicated airspeed"));
        }
        ar.indicated_airspeed = f64::from(indicated_airspeed);

        // Value in Knots
        // 0xffff for invalid.
        let true_airspeed = u16::from(buf[9]) << 8 | u16::from(buf[10]);
        if true_airspeed == 0xFFFF {
            return Err(anyhow!("Invalid true airspeed"));
        }
        ar.true_airspeed = f64::from(true_airspeed);

        Ok(ar)
    }

    /// Converts the `TrafficReport` to a byte buffer
    /// This buffer does not contain: FLAG, Message ID, CRC
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn to_bytes(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(11);

        buf.push(0x01);

        let roll = (self.roll * 10.0) as i16;
        buf.extend_from_slice(&roll.to_be_bytes());

        let pitch = (self.pitch * 10.0) as i16;
        buf.extend_from_slice(&pitch.to_be_bytes());

        let heading = (self.heading * 10.0) as i16;
        buf.extend_from_slice(&heading.to_be_bytes());
        buf[5] |= if self.true_or_magnetic_heading {
            0x80
        } else {
            0x00
        };

        let indicated_airspeed = self.indicated_airspeed as u16;
        buf.extend_from_slice(&indicated_airspeed.to_be_bytes());

        let true_airspeed = self.true_airspeed as u16;
        buf.extend_from_slice(&true_airspeed.to_be_bytes());

        buf
    }
}
