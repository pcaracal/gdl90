use crate::prelude::*;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    DekuRead,
    DekuWrite,
    EnumGet,
    num_enum::FromPrimitive,
    num_enum::IntoPrimitive,
)]
#[deku(
    ctx = "_: deku::ctx::Endian, _: deku::ctx::Order",
    id_type = "u8",
    bits = 4
)]
#[repr(u8)]
#[allow(non_camel_case_types)]
/// # 3.5.1.6 Navigation Integrity Category (NIC)
///
/// | Value (i) | NIC (HPL)               |
/// | --------- | ----------------------- |
/// | 0         | Unknown                 |
/// | 1         | < 20.0 NM               |
/// | 2         | < 8.0 NM                |
/// | 3         | < 4.0 NM                |
/// | 4         | < 2.0 NM                |
/// | 5         | < 1.0 NM                |
/// | 6         | < 0.6 NM                |
/// | 7         | < 0.2 NM                |
/// | 8         | < 0.1 NM                |
/// | 9         | HPL < 75m & VPL < 112m  |
/// | 10        | HPL < 25m & VPL < 37.5m |
/// | 11        | HPL < 7.5m & VPL < 11m  |
/// | 12-15     | Unused                  |
pub enum NIC {
    #[default]
    NIC0_Unknown = 0,
    NIC1_20NM = 1,
    NIC2_8NM = 2,
    NIC3_4NM = 3,
    NIC4_2NM = 4,
    NIC5_1NM = 5,
    NIC6_0_6NM = 6,
    NIC7_0_2NM = 7,
    NIC8_0_1NM = 8,
    NIC9_HPL_75M_VPL_112M = 9,
    NIC10_HPL_25M_VPL_37_5M = 10,
    NIC11_HPL_7_5M_VPL_11M = 11,
    // 12..=15 => unused
}
