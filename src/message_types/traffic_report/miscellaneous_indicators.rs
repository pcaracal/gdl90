use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Builder)]
#[deku(ctx = "_: deku::ctx::Endian, _: deku::ctx::Order")]
/// # 3.5.1.5 Miscellaneous Indicators
pub struct MiscellaneousIndicators {
    pub air_ground_state: AirGroundState,
    pub report_type: ReportType,
    pub track_heading_type: TrackHeadingType,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, EnumGet)]
#[deku(id_type = "u8", bits = 1)]
#[repr(u8)]
pub enum AirGroundState {
    #[default]
    OnGround = 0,
    Airborne = 1,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, EnumGet)]
#[deku(id_type = "u8", bits = 1)]
#[repr(u8)]
pub enum ReportType {
    #[default]
    Updated = 0,
    Extrapolated = 1,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, EnumGet)]
#[deku(id_type = "u8", bits = 2)]
#[repr(u8)]
pub enum TrackHeadingType {
    #[default]
    NotValid = 0,
    TrueTrackAngle = 1,
    HeadingMagnetic = 2,
    HeadingTrue = 3,
}
