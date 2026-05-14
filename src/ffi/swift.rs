#![allow(
    clippy::unnecessary_cast,
    clippy::ptr_as_ptr,
    clippy::enum_variant_names
)]

use crate::prelude as rs;
use rs::{
    ForeFlightAHRS, HeightAboveTerrain, Message, OwnshipGeometricAltitude, PreciseOwnship,
    TrafficReport, VerticalMetrics,
};

#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(swift_repr = "struct")]
    struct Heartbeat {
        /// position is available for ADS-B Tx
        gps_pos_valid: bool,
        /// GDL 90 Maintenance Req'd
        maint_reqd: bool,
        /// IDENT talkback
        ident: bool,
        /// Address Type talkback
        addr_type: bool,
        /// GPS Battery low voltage
        gps_batt_low: bool,
        /// ATC Services talkback
        ratcs: bool,
        /// GDL 90 is initialized
        uat_initialized: bool,
        /// CSA has been requested
        csa_requested: bool,
        /// CSA is not available at this time
        csa_not_available: bool,
        /// UTC timing is valid
        utc_ok: bool,
        /// Seconds since 0000Z (UTC midnight)
        timestamp: u32,
        /// # 3.1.4. Received Message Counts
        message_counts: u16,
    }

    #[swift_bridge(swift_repr = "struct")]
    struct Initialization {
        /// Initiate audio test
        audio_test: bool,
        /// Suppress GDL 90 audio output
        audio_inhibit: bool,
        /// CDTI capability is operating
        cdti_ok: bool,
        /// Disable GDL 90 audible traffic alerts
        csa_audio_disable: bool,
        /// Disable CSA traffic alerting
        csa_disable: bool,
    }

    enum TrafficAlertStatus {
        #[default]
        NoAlert = 0,
        TrafficAlert = 1,
    }

    enum AddressType {
        #[default]
        AdsbIcao = 0,
        AdsbSelfAssigned = 1,
        TisbIcao = 2,
        TisbTrackFileId = 3,
        SurfaceVehicle = 4,
        GroundStationBeacon = 5,
    }

    #[swift_bridge(swift_repr = "struct")]
    struct TargetIdentity {
        /// Address Type: Describes the type of address conveyed in the Participant Address
        address_type: AddressType,
        /// Participant Address (24 bits).
        participant_address: u32,
    }

    enum AirGroundState {
        #[default]
        OnGround = 0,
        Airborne = 1,
    }

    enum ReportType {
        #[default]
        Updated = 0,
        Extrapolated = 1,
    }

    enum TrackHeadingType {
        #[default]
        NotValid = 0,
        TrueTrackAngle = 1,
        HeadingMagnetic = 2,
        HeadingTrue = 3,
    }

    #[swift_bridge(swift_repr = "struct")]
    struct MiscellaneousIndicators {
        air_ground_state: AirGroundState,
        report_type: ReportType,
        track_heading_type: TrackHeadingType,
    }

    enum NIC {
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

    enum NACp {
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

    enum EmitterCategory {
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

    enum EmergencyPriorityCode {
        #[default]
        NoEmergency = 0,
        GeneralEmergency = 1,
        MedicalEmergency = 2,
        MinimumFuel = 3,
        NoCommunication = 4,
        UnlawfulInterference = 5,
        DownedAircraft = 6,
    }

    enum ForeFlightInternetPolicy {
        #[default]
        Unrestricted = 0,
        Expensive = 1,
        Disallowed = 2,
    }

    enum GeometricAltitudeDatum {
        #[default]
        WGS84 = 0,
        MSL = 1,
    }

    #[swift_bridge(swift_repr = "struct")]
    struct ForeFlightID {
        /// Must be 1
        version: u8,
        device_serial_number: u64,
        device_name: String,
        device_long_name: String,
        foreflight_internet_policy: ForeFlightInternetPolicy,
        geometric_altitude_datum: GeometricAltitudeDatum,
    }

    enum AHRSHeadingType {
        #[default]
        True = 0,
        Magnetic = 1,
    }

    extern "Rust" {
        type Angle;
        #[swift_bridge(get_with(v = rs::FromUom::radians))]
        fn radians(self: &Angle) -> f64;
        #[swift_bridge(get_with(v = rs::FromUom::degrees))]
        fn degrees(self: &Angle) -> f64;
        #[swift_bridge(get_with(v = rs::FromUom::seconds))]
        fn seconds(self: &Angle) -> f64;
        #[swift_bridge(get_with(v = rs::FromUom::minutes))]
        fn minutes(self: &Angle) -> f64;
        #[swift_bridge(get_with(v = rs::FromUom::revolutions))]
        fn revolutions(self: &Angle) -> f64;

        type Length;
        #[swift_bridge(get_with(v = rs::FromUom::meters))]
        fn meters(self: &Length) -> f64;
        #[swift_bridge(get_with(v = rs::FromUom::kilometers))]
        fn kilometers(self: &Length) -> f64;
        #[swift_bridge(get_with(v = rs::FromUom::feet))]
        fn feet(self: &Length) -> f64;
        #[swift_bridge(get_with(v = rs::FromUom::nautical_miles))]
        fn nautical_miles(self: &Length) -> f64;

        type Velocity;
        #[swift_bridge(get_with(v = rs::FromUom::meters_per_second))]
        fn meters_per_second(self: &Velocity) -> f64;
        #[swift_bridge(get_with(v = rs::FromUom::kilometers_per_hour))]
        fn kilometers_per_hour(self: &Velocity) -> f64;
        #[swift_bridge(get_with(v = rs::FromUom::feet_per_second))]
        fn feet_per_second(self: &Velocity) -> f64;
        #[swift_bridge(get_with(v = rs::FromUom::feet_per_minute))]
        fn feet_per_minute(self: &Velocity) -> f64;
        #[swift_bridge(get_with(v = rs::FromUom::miles_per_hour))]
        fn miles_per_hour(self: &Velocity) -> f64;
        #[swift_bridge(get_with(v = rs::FromUom::knots))]
        fn knots(self: &Velocity) -> f64;

        type MessageResult;
        fn parse_gdl90_bytes(bytes: &[u8]) -> Vec<MessageResult>;
        fn is_ok(self: &MessageResult) -> bool;
        fn is_err(self: &MessageResult) -> bool;
        fn ok(self: &MessageResult) -> Option<Message>;
        fn err(self: &MessageResult) -> String;
        fn unwrap(self: &MessageResult) -> Result<Message, String>;

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

        #[swift_bridge(return_with = OptionExt::map_into)]
        fn heartbeat(self: &Message) -> Option<Heartbeat>;

        #[swift_bridge(return_with = OptionExt::map_into)]
        fn initialization(self: &Message) -> Option<Initialization>;

        type HeightAboveTerrain;
        fn height_above_terrain(self: &Message) -> Option<&HeightAboveTerrain>;

        type TrafficReport;
        #[swift_bridge(return_with = OptionExt::map_into)]
        fn ownship(self: &Message) -> Option<TrafficReport>;
        #[swift_bridge(return_with = OptionExt::map_into)]
        fn traffic(self: &Message) -> Option<TrafficReport>;
        #[swift_bridge(return_into)]
        fn traffic_alert_status(self: &TrafficReport) -> TrafficAlertStatus;
        #[swift_bridge(return_into)]
        fn target_identity(self: &TrafficReport) -> TargetIdentity;
        #[swift_bridge(return_into)]
        fn latitude(self: &TrafficReport) -> Angle;
        #[swift_bridge(return_into)]
        fn longitude(self: &TrafficReport) -> Angle;
        #[swift_bridge(return_with = OptionExt::map_into)]
        fn altitude(self: &TrafficReport) -> Option<Length>;
        #[swift_bridge(return_into)]
        fn miscellaneous_indicators(self: &TrafficReport) -> MiscellaneousIndicators;
        #[swift_bridge(return_into)]
        fn nic(self: &TrafficReport) -> NIC;
        #[swift_bridge(return_into)]
        fn nacp(self: &TrafficReport) -> NACp;
        #[swift_bridge(return_with = OptionExt::map_into)]
        fn horizontal_velocity(self: &TrafficReport) -> Option<Velocity>;
        #[swift_bridge(return_with = OptionExt::map_into)]
        fn vertical_velocity(self: &TrafficReport) -> Option<Velocity>;
        #[swift_bridge(return_into)]
        fn track_heading(self: &TrafficReport) -> Angle;
        #[swift_bridge(return_into)]
        fn emitter_category(self: &TrafficReport) -> EmitterCategory;
        #[swift_bridge(return_into)]
        fn callsign(self: &TrafficReport) -> String;
        #[swift_bridge(return_into)]
        fn emergency_priority_code(self: &TrafficReport) -> EmergencyPriorityCode;

        type OwnshipGeometricAltitude;
        fn ownship_geometric_altitude(self: &Message) -> Option<&OwnshipGeometricAltitude>;
        #[swift_bridge(return_into)]
        fn ownship_geo_altitude(self: &OwnshipGeometricAltitude) -> Length;
        type VerticalMetrics;
        fn vertical_metrics(self: &OwnshipGeometricAltitude) -> &VerticalMetrics;
        #[swift_bridge(get(vertical_warning_indicator))]
        fn vertical_warning_indicator(self: &VerticalMetrics) -> bool;
        #[swift_bridge(return_with = OptionExt::map_into)]
        fn vfom(self: &VerticalMetrics) -> Option<Length>;

        #[swift_bridge(return_with = OptionExt::map_into)]
        fn fore_flight_id(self: &Message) -> Option<ForeFlightID>;

        type ForeFlightAHRS;
        fn fore_flight_ahrs(self: &Message) -> Option<&ForeFlightAHRS>;
        #[swift_bridge(return_with = OptionExt::map_into)]
        fn roll(self: &ForeFlightAHRS) -> Option<Angle>;
        #[swift_bridge(return_with = OptionExt::map_into)]
        fn pitch(self: &ForeFlightAHRS) -> Option<Angle>;
        #[swift_bridge(return_into)]
        fn heading_type(self: &ForeFlightAHRS) -> AHRSHeadingType;
        #[swift_bridge(return_with = OptionExt::map_into)]
        fn heading(self: &ForeFlightAHRS) -> Option<Angle>;
        #[swift_bridge(return_with = OptionExt::map_into)]
        fn indicated_airspeed(self: &ForeFlightAHRS) -> Option<Velocity>;
        #[swift_bridge(return_with = OptionExt::map_into)]
        fn true_airspeed(self: &ForeFlightAHRS) -> Option<Velocity>;

        type PreciseOwnship;
        fn precise_ownship(self: &Message) -> Option<&PreciseOwnship>;
        #[swift_bridge(return_into)]
        fn latitude(self: &PreciseOwnship) -> Angle;
        #[swift_bridge(return_into)]
        fn longitude(self: &PreciseOwnship) -> Angle;
        #[swift_bridge(return_into)]
        fn altitude(self: &PreciseOwnship) -> Length;
        #[swift_bridge(return_into)]
        fn ground_speed(self: &PreciseOwnship) -> Velocity;

        #[swift_bridge(return_into)]
        fn to_nacp(value: u8) -> NACp;
        #[swift_bridge(return_into)]
        fn to_nic(value: u8) -> NIC;
        #[swift_bridge(args_into = (nacp))]
        fn from_nacp(nacp: NACp) -> u8;
        #[swift_bridge(args_into = (nic))]
        fn from_nic(nic: NIC) -> u8;
        #[swift_bridge(args_into = (nacp))]
        fn nacp_horizontal_accuracy(nacp: NACp) -> Option<Length>;
        #[swift_bridge(args_into = (nacp))]
        fn nacp_vertical_accuracy(nacp: NACp) -> Option<Length>;
    }
}

macro_rules! bridge_uom {
    [$($ty:ident),*] => {
        $(
            struct $ty {
                v: rs::$ty,
            }
            impl From<rs::$ty> for $ty {
                fn from(v: rs::$ty) -> Self {
                    Self { v }
                }
            }
            impl From<&rs::$ty> for $ty {
                fn from(v: &rs::$ty) -> Self {
                    Self::from(*v)
                }
            }
        )*
    };
}

bridge_uom![Angle, Length, Velocity];

struct MessageResult(rs::GDL90Result<rs::Message>);

fn parse_gdl90_bytes(bytes: &[u8]) -> Vec<MessageResult> {
    rs::Message::from_gdl90_bytes(bytes)
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

    fn ok(&self) -> Option<rs::Message> {
        self.0.as_ref().ok().cloned()
    }

    fn err(&self) -> String {
        self.0
            .as_ref()
            .err()
            .map(ToString::to_string)
            .unwrap_or_default()
    }

    fn unwrap(&self) -> Result<rs::Message, String> {
        match self.0.as_ref() {
            Ok(msg) => Ok(msg.clone()),
            Err(why) => Err(format!("called `unwrap()` but it was an error: {why}")),
        }
    }
}

fn to_nacp(value: u8) -> rs::NACp {
    rs::NACp::from(value)
}

fn to_nic(value: u8) -> rs::NIC {
    rs::NIC::from(value)
}

fn from_nacp(nacp: rs::NACp) -> u8 {
    nacp.into()
}

fn from_nic(nic: rs::NIC) -> u8 {
    nic.into()
}

fn nacp_horizontal_accuracy(nacp: rs::NACp) -> Option<Length> {
    nacp.horizontal_accuracy().map_into()
}

fn nacp_vertical_accuracy(nacp: rs::NACp) -> Option<Length> {
    nacp.vertical_accuracy().map_into()
}

trait OptionExt<R> {
    fn map_into(self) -> Option<R>;
}

impl<T, R> OptionExt<R> for Option<T>
where
    R: From<T>,
{
    fn map_into(self) -> Option<R> {
        self.map(R::from)
    }
}

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

impl From<&rs::TargetIdentity> for ffi::TargetIdentity {
    fn from(v: &rs::TargetIdentity) -> Self {
        ffi::TargetIdentity {
            address_type: v.address_type.into(),
            participant_address: v.participant_address,
        }
    }
}

impl From<&rs::MiscellaneousIndicators> for ffi::MiscellaneousIndicators {
    fn from(v: &rs::MiscellaneousIndicators) -> Self {
        ffi::MiscellaneousIndicators {
            air_ground_state: v.air_ground_state.into(),
            report_type: v.report_type.into(),
            track_heading_type: v.track_heading_type.into(),
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
            foreflight_internet_policy: v.capabilities.foreflight_internet_policy.into(),
            geometric_altitude_datum: v.capabilities.geometric_altitude_datum.into(),
        }
    }
}

macro_rules! bridge_enums {
    [$(($ty:ident, [$($v:ident),* $(,)?])),* $(,)?] => {
        $(
            impl From<&rs::$ty> for ffi::$ty { fn from(v: &rs::$ty) -> Self { match v { $(rs::$ty::$v => ffi::$ty::$v),* } } }
            impl From<rs::$ty> for ffi::$ty { fn from(v: rs::$ty) -> Self { Self::from(&v) } }
            impl From<&ffi::$ty> for rs::$ty { fn from(v: &ffi::$ty) -> Self { match v { $(ffi::$ty::$v => rs::$ty::$v),* } } }
            impl From<ffi::$ty> for rs::$ty { fn from(v: ffi::$ty) -> Self { Self::from(&v) } }
        )*
    };
}

bridge_enums![
    (TrafficAlertStatus, [NoAlert, TrafficAlert]),
    (
        AddressType,
        [
            AdsbIcao,
            AdsbSelfAssigned,
            TisbIcao,
            TisbTrackFileId,
            SurfaceVehicle,
            GroundStationBeacon
        ]
    ),
    (AirGroundState, [OnGround, Airborne]),
    (ReportType, [Updated, Extrapolated]),
    (
        TrackHeadingType,
        [NotValid, TrueTrackAngle, HeadingMagnetic, HeadingTrue]
    ),
    (
        NIC,
        [
            NIC0_Unknown,
            NIC1_20NM,
            NIC2_8NM,
            NIC3_4NM,
            NIC4_2NM,
            NIC5_1NM,
            NIC6_0_6NM,
            NIC7_0_2NM,
            NIC8_0_1NM,
            NIC9_HPL_75M_VPL_112M,
            NIC10_HPL_25M_VPL_37_5M,
            NIC11_HPL_7_5M_VPL_11M
        ]
    ),
    (
        NACp,
        [
            NACp0_Unknown,
            NACp1_10NM,
            NACp2_4NM,
            NACp3_2NM,
            NACp4_1NM,
            NACp5_0_5NM,
            NACp6_0_3NM,
            NACp7_0_1NM,
            NACp8_0_05NM,
            NACp9_HFOM_30M_VFOM_45M,
            NACp10_HFOM_10M_VFOM_15M,
            NACp11_HFOM_3M_VFOM_4M
        ]
    ),
    (
        EmitterCategory,
        [
            NoInformation,
            Light,
            Small,
            Large,
            HighVortexLarge,
            Heavy,
            HighlyManeuverable,
            Rotorcraft,
            GliderSailplane,
            LighterThanAir,
            ParachutistSkyDiver,
            UltraLightHangGliderParaglider,
            UnmannedAerialVehicle,
            SpaceTransatmosphericVehicle,
            SurfaceVehicleEmergencyVehicle,
            SurfaceVehicleServiceVehicle,
            PointObstacleIncludesTetheredBalloons,
            ClusterObstacle,
            LineObstacle
        ]
    ),
    (
        EmergencyPriorityCode,
        [
            NoEmergency,
            GeneralEmergency,
            MedicalEmergency,
            MinimumFuel,
            NoCommunication,
            UnlawfulInterference,
            DownedAircraft
        ]
    ),
    (
        ForeFlightInternetPolicy,
        [Unrestricted, Expensive, Disallowed]
    ),
    (GeometricAltitudeDatum, [WGS84, MSL]),
    (AHRSHeadingType, [True, Magnetic])
];
