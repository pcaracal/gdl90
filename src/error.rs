pub type GDL90Result<T> = std::result::Result<T, GDL90Error>;

#[derive(Debug, thiserror::Error)]
pub enum GDL90Error {
    #[error("message too short, minimum length is 3 bytes, got {0} bytes")]
    MessageTooShort(usize),

    #[error("invalid escape sequence: [0x7D, {0:#02X}]")]
    InvalidEscapeSequence(u8),

    #[error("crc mismatch: expected {expected}, got {got}")]
    CrcMismatch { expected: u16, got: u16 },

    #[error("serialization/deserialization error: {0}")]
    DekuError(#[from] deku::DekuError),
}

#[macro_export]
macro_rules! bail {
    ($err:expr) => {
        return Err($err);
    };
}
