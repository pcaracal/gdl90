use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, EnumGet)]
#[deku(
    ctx = "_: deku::ctx::Endian, _: deku::ctx::Order",
    id_type = "u8",
    bits = 4
)]
#[repr(u8)]
/// # 3.5.1.1 Traffic Alert Status.
pub enum TrafficAlertStatus {
    #[default]
    NoAlert = 0,
    TrafficAlert = 1,
    // 2..=15 => reserved
}
