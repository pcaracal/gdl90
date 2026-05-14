use crate::prelude as rs;
use rs::{FromUom, IntoUom, Message};

use cxx::CxxVector;

#[cxx::bridge(namespace = "gdl90")]
mod ffi {
    #[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
    pub struct Angle {
        valid: bool,
        value: f64,
    }
    #[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
    pub struct Length {
        valid: bool,
        value: f64,
    }
    #[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
    pub struct Velocity {
        valid: bool,
        value: f64,
    }
    extern "Rust" {
        fn radians(self: &Angle) -> f64;
        fn degrees(self: &Angle) -> f64;
        fn seconds(self: &Angle) -> f64;
        fn minutes(self: &Angle) -> f64;
        fn revolutions(self: &Angle) -> f64;

        fn meters(self: &Length) -> f64;
        fn kilometers(self: &Length) -> f64;
        fn feet(self: &Length) -> f64;
        fn nautical_miles(self: &Length) -> f64;

        fn meters_per_second(self: &Velocity) -> f64;
        fn kilometers_per_hour(self: &Velocity) -> f64;
        fn feet_per_second(self: &Velocity) -> f64;
        fn feet_per_minute(self: &Velocity) -> f64;
        fn miles_per_hour(self: &Velocity) -> f64;
        fn knots(self: &Velocity) -> f64;
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Heartbeat {
        /// position is available for ADS-B Tx
        pub gps_pos_valid: bool,
        /// GDL 90 Maintenance Req'd
        pub maint_reqd: bool,
        /// IDENT talkback
        pub ident: bool,
        /// Address Type talkback
        pub addr_type: bool,
        /// GPS Battery low voltage
        pub gps_batt_low: bool,
        /// ATC Services talkback
        pub ratcs: bool,
        /// GDL 90 is initialized
        pub uat_initialized: bool,
        /// CSA has been requested
        pub csa_requested: bool,
        /// CSA is not available at this time
        pub csa_not_available: bool,
        /// UTC timing is valid
        pub utc_ok: bool,
        /// # 3.1.4. Received Message Counts
        pub message_counts: u16,
        /// Seconds since 0000Z (UTC midnight)
        pub timestamp: u32,
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Initialization {
        /// Initiate audio test
        pub audio_test: bool,
        /// Suppress GDL 90 audio output
        pub audio_inhibit: bool,
        /// CDTI capability is operating
        pub cdti_ok: bool,
        /// Disable GDL 90 audible traffic alerts
        pub csa_audio_disable: bool,
        /// Disable CSA traffic alerting
        pub csa_disable: bool,
    }

    #[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
    pub struct HeightAboveTerrain {
        /// Height above terrain. Resolution = 1 foot
        pub height_above_terrain: Length,
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitAnd, BitOr, BitXor)]
    pub enum TrafficAlertStatus {
        #[default]
        NoAlert = 0,
        TrafficAlert = 1,
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TargetIdentity {
        /// Address Type: Describes the type of address conveyed in the Participant Address
        pub address_type: AddressType,
        /// Participant Address (24 bits).
        pub participant_address: u32,
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitAnd, BitOr, BitXor)]
    pub enum AddressType {
        #[default]
        AdsbIcao = 0,
        AdsbSelfAssigned = 1,
        TisbIcao = 2,
        TisbTrackFileId = 3,
        SurfaceVehicle = 4,
        GroundStationBeacon = 5,
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MiscellaneousIndicators {
        pub air_ground_state: AirGroundState,
        pub report_type: ReportType,
        pub track_heading_type: TrackHeadingType,
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitAnd, BitOr, BitXor)]
    pub enum AirGroundState {
        #[default]
        OnGround = 0,
        Airborne = 1,
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitAnd, BitOr, BitXor)]
    pub enum ReportType {
        #[default]
        Updated = 0,
        Extrapolated = 1,
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitAnd, BitOr, BitXor)]
    pub enum TrackHeadingType {
        #[default]
        NotValid = 0,
        TrueTrackAngle = 1,
        HeadingMagnetic = 2,
        HeadingTrue = 3,
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitAnd, BitOr, BitXor)]
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
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitAnd, BitOr, BitXor)]
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
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitAnd, BitOr, BitXor)]
    pub enum EmitterCategory {
        #[default]
        /// No aircraft type information
        NoInformation = 0,
        /// Light (ICAO) < 15 500 lbs
        Light = 1,
        /// Small - 15 500 to 75 000 lbs
        Small = 2,
        /// Large - 75 000 to 300 000 lbs
        Large = 3,
        /// High Vortex Large (e.g., aircraft such as B757)
        HighVortexLarge = 4,
        /// Heavy (ICAO) - > 300 000 lbs
        Heavy = 5,
        /// Highly Maneuverable > 5G acceleration and high speed
        HighlyManeuverable = 6,
        /// Rotorcraft
        Rotorcraft = 7,
        /// Glider/sailplane
        GliderSailplane = 9,
        /// Lighter than air
        LighterThanAir = 10,
        /// Parachutist/sky diver
        ParachutistSkyDiver = 11,
        /// Ultra light/hang glider/paraglider
        UltraLightHangGliderParaglider = 12,
        /// Unmanned aerial vehicle
        UnmannedAerialVehicle = 14,
        /// Space/transatmospheric vehicle
        SpaceTransatmosphericVehicle = 15,
        /// Surface vehicle — emergency vehicle
        SurfaceVehicleEmergencyVehicle = 17,
        /// Surface vehicle — service vehicle
        SurfaceVehicleServiceVehicle = 18,
        /// Point Obstacle (includes tethered balloons)
        PointObstacleIncludesTetheredBalloons = 19,
        /// Cluster Obstacle
        ClusterObstacle = 20,
        /// Line Obstacle
        LineObstacle = 21,
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitAnd, BitOr, BitXor)]
    pub enum EmergencyPriorityCode {
        #[default]
        NoEmergency = 0,
        GeneralEmergency = 1,
        MedicalEmergency = 2,
        MinimumFuel = 3,
        NoCommunication = 4,
        UnlawfulInterference = 5,
        DownedAircraft = 6,
    }

    #[derive(Default, Clone, PartialEq, PartialOrd)]
    pub struct TrafficReport {
        /// Traffic Alert Status.
        pub traffic_alert_status: TrafficAlertStatus,
        /// Address Type & Participant Address
        pub target_identity: TargetIdentity,
        /// Latitude
        pub latitude: Angle,
        /// Longitude
        pub longitude: Angle,
        /// Altitude
        pub altitude: Length,
        /// Miscellaneous indicators
        pub miscellaneous_indicators: MiscellaneousIndicators,
        /// Navigation Integrity Category (NIC)
        pub nic: NIC,
        /// Navigation Accuracy Category for Position (`NACp`)
        pub nacp: NACp,
        /// Horizontal velocity.
        pub horizontal_velocity: Velocity,
        /// Vertical Velocity.
        pub vertical_velocity: Velocity,
        /// Track/Heading.
        /// See Miscellaneous field for Track/Heading indication.
        pub track_heading: Angle,
        /// Emitter Category
        pub emitter_category: EmitterCategory,
        /// Call Sign: 8 ASCII characters, '0' through '9' and 'A' through 'Z'.
        pub callsign: String,
        /// Emergency/Priority Code
        pub emergency_priority_code: EmergencyPriorityCode,
    }

    #[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
    pub struct VerticalMetrics {
        pub vertical_warning_indicator: bool,
        /// Vertical Figure of Merit (`VFOM`). Resolution = 1 meter
        pub vfom: Length,
    }

    #[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
    pub struct OwnshipGeometricAltitude {
        /// Ownship Geometric Altitude. Resolution = 5 feet
        pub ownship_geo_altitude: Length,
        pub vertical_metrics: VerticalMetrics,
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitAnd, BitOr, BitXor)]
    pub enum FFInternetPolicy {
        #[default]
        Unrestricted = 0,
        Expensive = 1,
        Disallowed = 2,
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitAnd, BitOr, BitXor)]
    pub enum FFGeometricAltitudeDatum {
        #[default]
        WGS84 = 0,
        MSL = 1,
    }

    #[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ForeFlightID {
        /// Must be 1
        pub version: u8,
        pub device_serial_number: u64,
        pub device_name: String,
        pub device_long_name: String,
        pub foreflight_internet_policy: FFInternetPolicy,
        pub geometric_altitude_datum: FFGeometricAltitudeDatum,
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitAnd, BitOr, BitXor)]
    pub enum AHRSHeadingType {
        #[default]
        True = 0,
        Magnetic = 1,
    }

    #[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
    pub struct ForeFlightAHRS {
        pub roll: Angle,
        pub pitch: Angle,
        pub heading_type: AHRSHeadingType,
        pub heading: Angle,
        pub indicated_airspeed: Velocity,
        pub true_airspeed: Velocity,
    }

    #[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
    pub struct PreciseOwnship {
        pub latitude: Angle,
        pub longitude: Angle,
        pub altitude: Length,
        pub ground_speed: Velocity,
    }

    extern "Rust" {
        type MessageResult;

        #[rust_name = "parse_gdl90_cxx_bytes"]
        fn parse_gdl90_bytes(bytes: &CxxVector<u8>) -> Vec<MessageResult>;
        fn parse_gdl90_bytes(bytes: &[u8]) -> Vec<MessageResult>;
        fn is_ok(self: &MessageResult) -> bool;
        fn is_err(self: &MessageResult) -> bool;
        fn err(self: &MessageResult) -> String;
        fn unwrap(self: &MessageResult) -> Result<Box<Message>>;

        type Message;

        fn is_heartbeat(self: &Message) -> bool;
        fn is_initialization(self: &Message) -> bool;
        fn is_height_above_terrain(self: &Message) -> bool;
        fn is_ownship(self: &Message) -> bool;
        fn is_ownship_geometric_altitude(self: &Message) -> bool;
        fn is_traffic(self: &Message) -> bool;
        fn is_fore_flight_id(self: &Message) -> bool;
        fn is_fore_flight_ahrs(self: &Message) -> bool;
        fn is_precise_ownship(self: &Message) -> bool;

        fn get_heartbeat(self: &Message) -> Result<Heartbeat>;
        fn get_initialization(self: &Message) -> Result<Initialization>;
        fn get_height_above_terrain(self: &Message) -> Result<HeightAboveTerrain>;
        fn get_ownship(self: &Message) -> Result<TrafficReport>;
        fn get_ownship_geometric_altitude(self: &Message) -> Result<OwnshipGeometricAltitude>;
        fn get_traffic(self: &Message) -> Result<TrafficReport>;
        fn get_fore_flight_id(self: &Message) -> Result<ForeFlightID>;
        fn get_fore_flight_ahrs(self: &Message) -> Result<ForeFlightAHRS>;
        fn get_precise_ownship(self: &Message) -> Result<PreciseOwnship>;
    }
}

struct MessageResult(rs::GDL90Result<Message>);

fn parse_gdl90_bytes(bytes: &[u8]) -> Vec<MessageResult> {
    Message::from_gdl90_bytes(bytes)
        .into_iter()
        .map(MessageResult)
        .collect()
}

fn parse_gdl90_cxx_bytes(bytes: &CxxVector<u8>) -> Vec<MessageResult> {
    Message::from_gdl90_bytes(bytes.as_slice())
        .into_iter()
        .map(MessageResult)
        .collect()
}

impl MessageResult {
    fn is_ok(&self) -> bool {
        self.0.is_ok()
    }

    fn is_err(&self) -> bool {
        self.0.is_err()
    }

    fn err(&self) -> String {
        self.0
            .as_ref()
            .err()
            .map(ToString::to_string)
            .unwrap_or_default()
    }

    fn unwrap(&self) -> Result<Box<Message>, String> {
        match self.0.as_ref() {
            Ok(msg) => Ok(msg.clone().into()),
            Err(why) => Err(format!("called `unwrap()` but it was an error: {why}")),
        }
    }
}

macro_rules! impl_getter {
    [$(($fn:ident, $ty:ty)),* $(,)?] => {
        pastey::paste! {$(
            fn [<get_ $fn>](&self) -> Result<ffi::$ty, String> {
                match self.$fn() {
                    Some(v) => Ok(v.into()),
                    None => Err(format!("not a {} message: {self:?}", stringify!($ty))),
                }
            }
        )*}
    };
}

impl Message {
    impl_getter![
        (heartbeat, Heartbeat),
        (initialization, Initialization),
        (height_above_terrain, HeightAboveTerrain),
        (ownship, TrafficReport),
        (ownship_geometric_altitude, OwnshipGeometricAltitude),
        (traffic, TrafficReport),
        (fore_flight_id, ForeFlightID),
        (fore_flight_ahrs, ForeFlightAHRS),
        (precise_ownship, PreciseOwnship),
    ];
}

macro_rules! bridge_uom {
    [$(($ty:ty, $base:ident, [$($fn:ident),* $(,)?])),* $(,)?] => {
        pastey::paste! {$(
            impl<T: Into<Option<rs::$ty>>> From<T> for ffi::$ty {
                fn from(v: T) -> Self {
                    v.into().map(|v| ffi::$ty { valid: true, value: v.$base() }).unwrap_or_default()
                }
            }
            impl ffi::$ty {
                $( fn $fn(&self) -> f64 { self.value.$base().$fn() })*
            }
        )*}
    };
}

bridge_uom![
    (
        Angle,
        radians,
        [radians, degrees, seconds, minutes, revolutions]
    ),
    (Length, meters, [meters, kilometers, feet, nautical_miles]),
    (
        Velocity,
        meters_per_second,
        [
            meters_per_second,
            kilometers_per_hour,
            feet_per_second,
            feet_per_minute,
            miles_per_hour,
            knots
        ]
    ),
];

impl From<&rs::Heartbeat> for ffi::Heartbeat {
    fn from(v: &rs::Heartbeat) -> Self {
        ffi::Heartbeat {
            gps_pos_valid: v.gps_pos_valid,
            maint_reqd: v.maint_reqd,
            ident: v.ident,
            addr_type: v.addr_type,
            gps_batt_low: v.gps_batt_low,
            ratcs: v.ratcs,
            uat_initialized: v.uat_initialized,
            csa_requested: v.csa_requested,
            csa_not_available: v.csa_not_available,
            utc_ok: v.utc_ok,
            message_counts: v.message_counts,
            timestamp: v.timestamp(),
        }
    }
}

impl From<&rs::Initialization> for ffi::Initialization {
    fn from(v: &rs::Initialization) -> Self {
        ffi::Initialization {
            audio_test: v.audio_test,
            audio_inhibit: v.audio_inhibit,
            cdti_ok: v.cdti_ok,
            csa_audio_disable: v.csa_audio_disable,
            csa_disable: v.csa_disable,
        }
    }
}

impl From<&rs::HeightAboveTerrain> for ffi::HeightAboveTerrain {
    fn from(v: &rs::HeightAboveTerrain) -> Self {
        ffi::HeightAboveTerrain {
            height_above_terrain: v.height_above_terrain.into(),
        }
    }
}

impl<'a, T> From<&'a T> for ffi::TrafficReport
where
    rs::TrafficReport: From<&'a T>,
{
    fn from(v: &'a T) -> Self {
        let v: rs::TrafficReport = v.into();
        ffi::TrafficReport {
            traffic_alert_status: ffi::TrafficAlertStatus {
                repr: v.traffic_alert_status as u8,
            },
            target_identity: ffi::TargetIdentity {
                address_type: ffi::AddressType {
                    repr: v.target_identity.address_type as u8,
                },
                participant_address: v.target_identity.participant_address,
            },
            latitude: v.latitude.into(),
            longitude: v.longitude.into(),
            altitude: v.altitude.into(),
            miscellaneous_indicators: ffi::MiscellaneousIndicators {
                air_ground_state: ffi::AirGroundState {
                    repr: v.miscellaneous_indicators.air_ground_state as u8,
                },
                report_type: ffi::ReportType {
                    repr: v.miscellaneous_indicators.report_type as u8,
                },
                track_heading_type: ffi::TrackHeadingType {
                    repr: v.miscellaneous_indicators.track_heading_type as u8,
                },
            },
            nic: ffi::NIC { repr: v.nic as u8 },
            nacp: ffi::NACp { repr: v.nacp as u8 },
            horizontal_velocity: v.horizontal_velocity.into(),
            vertical_velocity: v.vertical_velocity.into(),
            track_heading: v.track_heading.into(),
            emitter_category: ffi::EmitterCategory {
                repr: v.emitter_category as u8,
            },
            callsign: v.callsign.clone(),
            emergency_priority_code: ffi::EmergencyPriorityCode {
                repr: v.emergency_priority_code as u8,
            },
        }
    }
}

impl From<&rs::OwnshipGeometricAltitude> for ffi::OwnshipGeometricAltitude {
    fn from(v: &rs::OwnshipGeometricAltitude) -> Self {
        ffi::OwnshipGeometricAltitude {
            ownship_geo_altitude: v.ownship_geo_altitude.into(),
            vertical_metrics: ffi::VerticalMetrics {
                vertical_warning_indicator: v.vertical_metrics.vertical_warning_indicator,
                vfom: v.vertical_metrics.vfom.into(),
            },
        }
    }
}

impl From<&rs::ForeFlightID> for ffi::ForeFlightID {
    fn from(v: &rs::ForeFlightID) -> Self {
        ffi::ForeFlightID {
            version: v.version,
            device_serial_number: v.device_serial_number,
            device_name: v.device_name.clone(),
            device_long_name: v.device_long_name.clone(),
            foreflight_internet_policy: ffi::FFInternetPolicy {
                repr: v.capabilities.foreflight_internet_policy as u8,
            },
            geometric_altitude_datum: ffi::FFGeometricAltitudeDatum {
                repr: v.capabilities.geometric_altitude_datum as u8,
            },
        }
    }
}

impl From<&rs::ForeFlightAHRS> for ffi::ForeFlightAHRS {
    fn from(v: &rs::ForeFlightAHRS) -> Self {
        ffi::ForeFlightAHRS {
            roll: v.roll.into(),
            pitch: v.pitch.into(),
            heading_type: ffi::AHRSHeadingType {
                repr: v.heading_type as u8,
            },
            heading: v.heading.into(),
            indicated_airspeed: v.indicated_airspeed.into(),
            true_airspeed: v.true_airspeed.into(),
        }
    }
}

impl From<&rs::PreciseOwnship> for ffi::PreciseOwnship {
    fn from(v: &rs::PreciseOwnship) -> Self {
        ffi::PreciseOwnship {
            latitude: v.latitude.into(),
            longitude: v.longitude.into(),
            altitude: v.altitude.into(),
            ground_speed: v.ground_speed.into(),
        }
    }
}
