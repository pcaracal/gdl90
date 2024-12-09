use anyhow::anyhow;
use log::trace;

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
        let report_type = match misc & 0b0000_0100 >> 2 {
            1 => ReportType::Extrapolated,
            _ => ReportType::Updated,
        };
        let ground_state = match misc & 0b0000_1000 >> 3 {
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
    HPL_75M_VPL_112M,
    HPL_25M_VPL_37_5M,
    HPL_7_5M_VPL_11M,
    Unused,
}

impl From<u8> for NIC {
    fn from(nic: u8) -> Self {
        match nic {
            1 => Self::NM20,
            2 => Self::NM8,
            3 => Self::NM4,
            4 => Self::NM2,
            5 => Self::NM1,
            6 => Self::NM0_6,
            7 => Self::NM0_2,
            8 => Self::NM0_1,
            9 => Self::HPL_75M_VPL_112M,
            10 => Self::HPL_25M_VPL_37_5M,
            11 => Self::HPL_7_5M_VPL_11M,
            _ => Self::Unknown,
        }
    }
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
    HFOM_30M_VFOM_45M,
    HFOM_10M_VFOM_15M,
    HFOM_3M_VFOM_4M,
    Unused,
}

impl From<u8> for NACp {
    fn from(nacp: u8) -> Self {
        match nacp {
            1 => Self::NM10,
            2 => Self::NM4,
            3 => Self::NM2,
            4 => Self::NM1,
            5 => Self::NM0_5,
            6 => Self::NM0_3,
            7 => Self::NM0_1,
            8 => Self::NM0_05,
            9 => Self::HFOM_30M_VFOM_45M,
            10 => Self::HFOM_10M_VFOM_15M,
            11 => Self::HFOM_3M_VFOM_4M,
            _ => Self::Unknown,
        }
    }
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

#[derive(Debug, Default, PartialEq)]
#[allow(dead_code)]
pub struct TrafficReport {
    traffic_alert_status: TrafficAlertStatus,
    address_type: AddressType,
    participant_address: u32,
    latitude_deg: f64,
    longitude_deg: f64,
    pressure_altitude_ft: i32,
    misc_indicators: MiscIndicators,
    nic: NIC,
    nacp: NACp,
    horizontal_velocity_kt: u16,
    vertical_velocity_fps: f64,
    track_heading: f64,
    emitter_category: EmitterCategory,
    call_sign: String,
    emergy_priority_code: EmergencyPriorityCode,
}

impl TrafficReport {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `TrafficReport` from a byte buffer
    /// buffer is expected to begin with `st` and end with `px` as per
    /// <https://www.faa.gov/sites/faa.gov/files/air_traffic/technology/adsb/archival/GDL90_Public_ICD_RevA.PDF> 3.5.1
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer is not 27 bytes long
    pub fn from_bytes(buf: &[u8]) -> anyhow::Result<Self> {
        if buf.len() < 27 {
            return Err(anyhow!("Invalid buffer length"));
        }

        let mut tr = Self::new();

        tr.traffic_alert_status = TrafficAlertStatus::from(buf[0] >> 4);
        trace!("Traffic Alert Status: {:?}", tr.traffic_alert_status);
        tr.address_type = AddressType::from(buf[0] & 0x0F);
        trace!("Address Type: {:?}", tr.address_type);

        tr.participant_address = u32::from_be_bytes(buf[1..5].try_into()?) >> 8;
        trace!("Participant Address: {}", tr.participant_address);

        let lat = i32::from_be_bytes(buf[4..8].try_into()?) >> 8;
        tr.latitude_deg = f64::from(lat) / f64::from(0x7F_FFFF) * 180.0;
        trace!("Latitude: {}", tr.latitude_deg);

        let lon = i32::from_be_bytes(buf[7..11].try_into()?) / 256;
        tr.longitude_deg = f64::from(lon) / f64::from(0x7F_FFFF) * 180.0;
        trace!("Longitude: {}", tr.longitude_deg);

        tr.pressure_altitude_ft =
            i32::from(i16::from_be_bytes(buf[10..12].try_into()?) >> 4) * 25 - 1000;
        trace!("Pressure Altitude: {}", tr.pressure_altitude_ft);
        tr.misc_indicators = MiscIndicators::from(buf[11]);
        trace!("Misc Indicators: {:?}", tr.misc_indicators);

        tr.nic = NIC::from(buf[12] >> 4);
        trace!("NIC: {:?}", tr.nic);
        tr.nacp = NACp::from(buf[12] & 0b0000_1111);
        trace!("NACp: {:?}", tr.nacp);

        tr.horizontal_velocity_kt = u16::from_be_bytes(buf[13..15].try_into()?) >> 4;
        trace!("Horizontal Velocity: {}", tr.horizontal_velocity_kt);

        let mut vv1 = buf[14] & 0x0F;
        let vv2 = buf[15];
        if vv1 & 0b0000_1000 != 0 {
            vv1 |= 0xF0;
        }

        tr.vertical_velocity_fps = f64::from(i16::from_be_bytes([vv1, vv2])) / 64.0 * 60.0;

        trace!("Vertical Velocity: {}", tr.vertical_velocity_fps);

        tr.track_heading = f64::from(buf[16]) / 256.0 * 360.0;
        trace!("Track Heading: {}", tr.track_heading);

        tr.emitter_category = EmitterCategory::from(buf[17]);
        trace!("Emitter Category: {:?}", tr.emitter_category);

        tr.call_sign = buf[18..26]
            .iter()
            .map(|&c| c as char)
            .collect::<String>()
            .trim()
            .to_string();
        trace!("Call Sign: {}", tr.call_sign);

        tr.emergy_priority_code = EmergencyPriorityCode::from(buf[26] >> 4);
        trace!("Emergency Priority Code: {:?}", tr.emergy_priority_code);

        Ok(tr)
    }
}
