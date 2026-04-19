use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, EnumGet)]
#[deku(
    ctx = "_: deku::ctx::Endian, _: deku::ctx::Order",
    id_type = "u8",
    bits = 4
)]
#[repr(u8)]
/// # 3.5.1.12 Emergency/Priority Code
///
/// The Emergency Priority Code is a 4-bit value `p` that provides status information about the traffic.
pub enum EmergencyPriorityCode {
    #[default]
    NoEmergency = 0,
    GeneralEmergency = 1,
    MedicalEmergency = 2,
    MinimumFuel = 3,
    NoCommunication = 4,
    UnlawfulInterference = 5,
    DownedAircraft = 6,
    // 7..=15 => reserved
}
