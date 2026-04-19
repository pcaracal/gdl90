use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Builder)]
#[deku(ctx = "_: deku::ctx::Endian, _: deku::ctx::Order")]
/// # 3.5.1.2 Target Identity
///
/// The identity of a target is formed by the combination of the Address Type `t` along with the
/// Participant Address `aaaaaa`. Together these form a 28-bit field that uniquely identifies a given
/// ADS-B or TIS-B participant.
pub struct TargetIdentity {
    /// `t`
    /// Address Type: Describes the type of address conveyed in the Participant Address
    pub address_type: AddressType,

    /// `aa aa aa`
    /// Participant Address (24 bits).
    #[deku(bytes = 3, endian = "big")]
    pub participant_address: u32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, EnumGet)]
#[deku(id_type = "u8", bits = 4)]
#[repr(u8)]
pub enum AddressType {
    #[default]
    AdsbIcao = 0,
    AdsbSelfAssigned = 1,
    TisbIcao = 2,
    TisbTrackFileId = 3,
    SurfaceVehicle = 4,
    GroundStationBeacon = 5,
    // 6..=15 => reserved
}
