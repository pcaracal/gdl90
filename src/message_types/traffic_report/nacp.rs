use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, EnumGet)]
#[deku(
    ctx = "_: deku::ctx::Endian, _: deku::ctx::Order",
    id_type = "u8",
    bits = 4
)]
#[repr(u8)]
#[allow(non_camel_case_types)]
/// # 3.5.1.6 Navigation Accuracy Category for Position (`NACp`)
///
/// | Value (a) | NACp (HFOM)             |
/// | --------- | ----------------------- |
/// | 0         | Unknown                 |
/// | 1         | < 10.0 NM               |
/// | 2         | < 4.0 NM                |
/// | 3         | < 2.0 NM                |
/// | 4         | < 1.0 NM                |
/// | 5         | < 0.5 NM                |
/// | 6         | < 0.3 NM                |
/// | 7         | < 0.1 NM                |
/// | 8         | < 0.05 NM               |
/// | 9         | HFOM < 30m & VFOM < 45m |
/// | 10        | HFOM < 10m & VFOM < 15m |
/// | 11        | HFOM < 3m & VFOM < 4m   |
/// | 12-15     | Unused                  |
pub enum NACp {
    #[default]
    NACp0_Unknown = 0,
    NACp1_10NM = 1,
    NACp2_4NM = 2,
    NACp3_2NM = 3,
    NACp4_1NM = 4,
    NACp5_0_5NM = 5,
    NACp6_0_3NM = 6,
    NACp7_0_1NM = 7,
    NACp8_0_05NM = 8,
    NACp9_HFOM_30M_VFOM_45M = 9,
    NACp10_HFOM_10M_VFOM_15M = 10,
    NACp11_HFOM_3M_VFOM_4M = 11,
    // 12..=15 => unused
}
