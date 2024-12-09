#[derive(Debug, Default, PartialEq, Eq)]
pub enum TrafficAlertStatus {
    #[default]
    NoAlert,
    TrafficAlert,
    Reserved,
}

impl From<u8> for TrafficAlertStatus {
    fn from(alert: u8) -> Self {
        match alert {
            0 => Self::NoAlert,
            1 => Self::TrafficAlert,
            _ => Self::Reserved,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum AddressType {
    #[default]
    ADSBIcao,
    ADSBSelfAssigned,
    TISBIcao,
    TISBTrackFileId,
    SurfaceVehicle,
    GroundStationBeacon,
    Reserved,
}

impl From<u8> for AddressType {
    fn from(address_type: u8) -> Self {
        match address_type {
            0 => Self::ADSBIcao,
            1 => Self::ADSBSelfAssigned,
            2 => Self::TISBIcao,
            3 => Self::TISBTrackFileId,
            4 => Self::SurfaceVehicle,
            5 => Self::GroundStationBeacon,
            _ => Self::Reserved,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum TrackHeading {
    #[default]
    TrueTrackAngle,
    HeadingMagnetic,
    HeadingTrue,
    Invalid,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ReportType {
    #[default]
    Updated,
    Extrapolated,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum AirGround {
    #[default]
    OnGround,
    Airborne,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct MiscIndicators {
    pub tt: TrackHeading,
    pub report_type: ReportType,
    pub ground_state: AirGround,
}

impl From<u8> for MiscIndicators {
    fn from(misc: u8) -> Self {
        let tt = match misc & 0b0000_0011 {
            1 => TrackHeading::TrueTrackAngle,
            2 => TrackHeading::HeadingMagnetic,
            3 => TrackHeading::HeadingTrue,
            _ => TrackHeading::Invalid,
        };
        let report_type = match misc & 0b0000_0100 {
            1 => ReportType::Extrapolated,
            _ => ReportType::Updated,
        };
        let ground_state = match misc & 0b0000_1000 {
            1 => AirGround::Airborne,
            _ => AirGround::OnGround,
        };
        Self {
            tt,
            report_type,
            ground_state,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NIC {
    #[default]
    Unknown,
    NM20,
    NM8,
    NM4,
    NM2,
    NM1,
    NM0_6,
    NM0_2,
    NM0_1,
    HPL75_VPL112,
    HPL25_VPL37_5,
    HPL7_5_VPL11,
    Unused,
}

#[derive(Debug, Default, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NACp {
    #[default]
    Unknown,
    NM10,
    NM4,
    NM2,
    NM1,
    NM0_5,
    NM0_3,
    NM0_1,
    NM0_05,
    HFOM30_VFOM45,
    HFOM10_VFOM15,
    HFOM3_VFOM4,
    Unused,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum EmitterCategory {
    #[default]
    NoInformation,
    Light,
    Small,
    Large,
    HighVortexLarge,
    HeavyIcao,
    HighlyManeuverable,
    Rotorcraft,
    Glider,
    LighterThanAir,
    Parachutist,
    Ultralight,
    UnmannedAerialVehicle,
    SpaceOrTransatmosphericVehicle,
    SurfaceEmergencyVehicle,
    SurfaceServiceVehicle,
    PointObstacle,
}

impl From<u8> for EmitterCategory {
    fn from(emitter: u8) -> Self {
        match emitter {
            1 => Self::Light,
            2 => Self::Small,
            3 => Self::Large,
            4 => Self::HighVortexLarge,
            5 => Self::HeavyIcao,
            6 => Self::HighlyManeuverable,
            7 => Self::Rotorcraft,
            9 => Self::Glider,
            10 => Self::LighterThanAir,
            11 => Self::Parachutist,
            12 => Self::Ultralight,
            14 => Self::UnmannedAerialVehicle,
            15 => Self::SpaceOrTransatmosphericVehicle,
            17 => Self::SurfaceEmergencyVehicle,
            18 => Self::SurfaceServiceVehicle,
            19 => Self::PointObstacle,
            _ => Self::NoInformation,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum EmergencyPriorityCode {
    #[default]
    NoEmergency,
    GeneralEmergency,
    MedicalEmergency,
    MinimumFuel,
    NoCommunication,
    UnlawfulInterference,
    DownedAircraft,
    Reserved,
}

impl From<u8> for EmergencyPriorityCode {
    fn from(code: u8) -> Self {
        match code {
            0 => Self::NoEmergency,
            1 => Self::GeneralEmergency,
            2 => Self::MedicalEmergency,
            3 => Self::MinimumFuel,
            4 => Self::NoCommunication,
            5 => Self::UnlawfulInterference,
            6 => Self::DownedAircraft,
            _ => Self::Reserved,
        }
    }
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct TrafficReport {
    traffic_alert_status: TrafficAlertStatus,
    address_type: AddressType,
    participant_address: u32,
    latitude_deg: f32,
    longitude_deg: f32,
    pressure_altitude_ft: i16,
    misc_indicators: MiscIndicators,
    nic: NIC,
    nacp: NACp,
    horizontal_velocity_kt: u16,
    vertical_velocity_fpm: f32,
    track_heading: f32,
    emitter_category: EmitterCategory,
    call_sign: String,
    emergy_priority_code: EmergencyPriorityCode,
}
