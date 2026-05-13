#![allow(clippy::wildcard_imports, clippy::unnecessary_cast, clippy::ptr_as_ptr)]

use crate::ffi::opaque::*;
use crate::prelude as rs;

impl ForeFlightAHRS {
    fn swift_heading_type(&self) -> ffi::AHRSHeadingType {
        match self.heading_type {
            rs::AHRSHeadingType::True => ffi::AHRSHeadingType::True,
            rs::AHRSHeadingType::Magnetic => ffi::AHRSHeadingType::Magnetic,
        }
    }
}

impl TrafficReport {
    fn swift_heading_type(&self) -> ffi::HeadingType {
        match self.miscellaneous_indicators.track_heading_type {
            rs::TrackHeadingType::NotValid => ffi::HeadingType::Invalid,
            rs::TrackHeadingType::TrueTrackAngle => ffi::HeadingType::TrueTrack,
            rs::TrackHeadingType::HeadingMagnetic => ffi::HeadingType::MagneticHeading,
            rs::TrackHeadingType::HeadingTrue => ffi::HeadingType::TrueHeading,
        }
    }
}

#[swift_bridge::bridge]
mod ffi {
    enum MessageType {
        Heartbeat,
        Initialization,
        UplinkData,
        HeightAboveTerrain,
        Ownship,
        OwnshipGeometricAltitude,
        Traffic,
        BasicReport,
        LongReport,
        ForeFlightID,
        ForeFlightAHRS,
        CustomPreciseOwnship,
    }

    enum AHRSHeadingType {
        True = 0,
        Magnetic = 1,
    }

    enum HeadingType {
        Invalid,
        TrueTrack,
        MagneticHeading,
        TrueHeading,
    }

    extern "Rust" {
        type Angle;
        fn radians(self: &Angle) -> f64;
        fn degrees(self: &Angle) -> f64;
        fn seconds(self: &Angle) -> f64;
        fn minutes(self: &Angle) -> f64;
        fn revolutions(self: &Angle) -> f64;

        type Length;
        fn feet(self: &Length) -> f64;
        fn meters(self: &Length) -> f64;
        fn kilometers(self: &Length) -> f64;
        fn nautical_miles(self: &Length) -> f64;

        type Velocity;
        fn feet_per_second(self: &Velocity) -> f64;
        fn feet_per_minute(self: &Velocity) -> f64;
        fn meters_per_second(self: &Velocity) -> f64;
        fn kilometers_per_hour(self: &Velocity) -> f64;
        fn miles_per_hour(self: &Velocity) -> f64;
        fn knots(self: &Velocity) -> f64;
    }

    extern "Rust" {
        type MessageResult;
        fn is_ok(self: &MessageResult) -> bool;
        fn is_err(self: &MessageResult) -> bool;
        fn ok(self: &MessageResult) -> Option<Message>;
        fn err(self: &MessageResult) -> String;
    }

    extern "Rust" {
        type Message;
        #[swift_bridge(associated_to = Message)]
        fn from_gdl90_bytes(bytes: Vec<u8>) -> Vec<MessageResult>;
        fn to_string(self: &Message) -> String;

        type Heartbeat;
        fn heartbeat(self: &Message) -> Option<Heartbeat>;
        fn is_heartbeat(self: &Message) -> bool;

        fn timestamp(self: &Heartbeat) -> u32;

        type Initialization;
        fn initialization(self: &Message) -> Option<Initialization>;
        fn is_initialization(self: &Message) -> bool;

        type UplinkData;
        fn uplink_data(self: &Message) -> Option<UplinkData>;
        fn is_uplink_data(self: &Message) -> bool;

        type HeightAboveTerrain;
        fn height_above_terrain(self: &Message) -> Option<HeightAboveTerrain>;
        fn is_height_above_terrain(self: &Message) -> bool;

        #[swift_bridge(swift_name = "height_above_terrain")]
        fn opt_height_above_terrain(self: &HeightAboveTerrain) -> Option<Length>;

        type TrafficReport;
        fn ownship(self: &Message) -> Option<TrafficReport>;
        fn is_ownship(self: &Message) -> bool;

        type OwnshipGeometricAltitude;
        fn ownship_geometric_altitude(self: &Message) -> Option<OwnshipGeometricAltitude>;
        fn is_ownship_geometric_altitude(self: &Message) -> bool;

        fn traffic(self: &Message) -> Option<TrafficReport>;
        fn is_traffic(self: &Message) -> bool;

        #[swift_bridge(swift_name = "latitude")]
        fn get_latitude(self: &TrafficReport) -> Angle;
        #[swift_bridge(swift_name = "longitude")]
        fn get_longitude(self: &TrafficReport) -> Angle;
        #[swift_bridge(swift_name = "altitude")]
        fn get_altitude(self: &TrafficReport) -> Length;
        #[swift_bridge(swift_name = "ground_speed")]
        fn opt_horizontal_velocity(self: &TrafficReport) -> Option<Velocity>;
        #[swift_bridge(swift_name = "vertical_velocity")]
        fn opt_vertical_velocity(self: &TrafficReport) -> Option<Velocity>;
        #[swift_bridge(swift_name = "track_heading")]
        fn get_track_heading(self: &TrafficReport) -> Angle;
        #[swift_bridge(swift_name = "heading_type")]
        fn swift_heading_type(self: &TrafficReport) -> HeadingType;

        type ForeFlightID;
        fn fore_flight_id(self: &Message) -> Option<ForeFlightID>;
        fn is_fore_flight_id(self: &Message) -> bool;

        type ForeFlightAHRS;
        fn fore_flight_ahrs(self: &Message) -> Option<ForeFlightAHRS>;
        fn is_fore_flight_ahrs(self: &Message) -> bool;

        #[swift_bridge(swift_name = "roll")]
        fn opt_roll(self: &ForeFlightAHRS) -> Option<Angle>;
        #[swift_bridge(swift_name = "pitch")]
        fn opt_pitch(self: &ForeFlightAHRS) -> Option<Angle>;
        #[swift_bridge(swift_name = "heading_type")]
        fn swift_heading_type(self: &ForeFlightAHRS) -> AHRSHeadingType;
        #[swift_bridge(swift_name = "heading")]
        fn opt_heading(self: &ForeFlightAHRS) -> Option<Angle>;
        #[swift_bridge(swift_name = "indicated_airspeed")]
        fn opt_indicated_airspeed(self: &ForeFlightAHRS) -> Option<Velocity>;
        #[swift_bridge(swift_name = "true_airspeed")]
        fn opt_true_airspeed(self: &ForeFlightAHRS) -> Option<Velocity>;

        type CustomPreciseOwnship;
        fn custom_precise_ownship(self: &Message) -> Option<CustomPreciseOwnship>;
        fn is_custom_precise_ownship(self: &Message) -> bool;

        #[swift_bridge(swift_name = "latitude")]
        fn get_latitude(self: &CustomPreciseOwnship) -> Angle;
        #[swift_bridge(swift_name = "longitude")]
        fn get_longitude(self: &CustomPreciseOwnship) -> Angle;
        #[swift_bridge(swift_name = "altitude")]
        fn get_altitude(self: &CustomPreciseOwnship) -> Length;
        #[swift_bridge(swift_name = "track_heading")]
        fn get_ground_speed(self: &CustomPreciseOwnship) -> Velocity;
    }

    extern "Rust" {
        type ForeFlightBroadcast;
        #[swift_bridge(associated_to = ForeFlightBroadcast)]
        fn from_json(json: String) -> Result<ForeFlightBroadcast, String>;
        fn to_json(self: &ForeFlightBroadcast) -> Result<String, String>;
    }
}
