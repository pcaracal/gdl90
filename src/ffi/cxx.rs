#![allow(clippy::unnecessary_box_returns, clippy::wildcard_imports)]

use crate::ffi::opaque::*;
use crate::message_types::foreflight_broadcast::Port;
use crate::prelude as rs;
use cxx::CxxString;
use deku::DekuEnumExt;

impl ffi::ForeFlightBroadcast {
    fn bridge_to_json(&self) -> String {
        crate::message_types::foreflight_broadcast::ForeFlightBroadcast::new(
            &self.app,
            Port::new(self.port),
        )
        .to_json()
        .unwrap_or_default()
    }

    fn bridge_from_json(json: &CxxString) -> Result<ffi::ForeFlightBroadcast, String> {
        crate::message_types::foreflight_broadcast::ForeFlightBroadcast::from_json(json.as_bytes())
            .map(|r| ffi::ForeFlightBroadcast {
                app: r.app,
                port: r.gdl90.port,
            })
            .map_err(|e| format!("Failed to parse ForeFlightBroadcast: {e}"))
    }
}

impl ForeFlightAHRS {
    fn heading_type(&self) -> ffi::AHRSHeadingType {
        match self.heading_type {
            rs::AHRSHeadingType::True => ffi::AHRSHeadingType::True,
            rs::AHRSHeadingType::Magnetic => ffi::AHRSHeadingType::Magnetic,
        }
    }
}

impl Message {
    fn get_type(&self) -> ffi::MessageType {
        if let Some(ff) = self.fore_flight() {
            if ff.is_id() {
                return ffi::MessageType::ForeFlightID;
            } else if ff.is_ahrs() {
                return ffi::MessageType::ForeFlightAHRS;
            }
        }

        if self.custom().is_some() {
            return ffi::MessageType::CustomPreciseOwnship;
        }

        #[allow(clippy::expect_used)]
        match self.deku_id().expect("Message::deku_id failed") {
            0 => ffi::MessageType::Heartbeat,
            2 => ffi::MessageType::Initialization,
            7 => ffi::MessageType::UplinkData,
            9 => ffi::MessageType::HeightAboveTerrain,
            10 => ffi::MessageType::Ownship,
            11 => ffi::MessageType::OwnshipGeometricAltitude,
            20 => ffi::MessageType::Traffic,
            30 => ffi::MessageType::BasicReport,
            31 => ffi::MessageType::LongReport,
            _ => unreachable!(),
        }
    }
}

fn bridge_message_type_to_string(message_type: ffi::MessageType) -> String {
    format!("{message_type:?}")
}

#[allow(clippy::module_inception)]
#[cxx::bridge(namespace = "gdl90")]
mod ffi {
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

    #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
    enum AHRSHeadingType {
        True = 0,
        Magnetic = 1,
    }

    extern "Rust" {
        type MessageResult;
        fn is_ok(self: &MessageResult) -> bool;
        fn is_err(self: &MessageResult) -> bool;
        fn err(self: &MessageResult) -> String;
        fn unwrap(self: &MessageResult) -> Result<Box<Message>>;
    }

    extern "Rust" {
        type Message;

        #[Self = Message]
        fn from_gdl90_bytes(bytes: &[u8]) -> Vec<MessageResult>;
        fn get_type(self: &Message) -> MessageType;
        fn to_string(self: &Message) -> String;
        #[namespace = ""]
        #[rust_name = "bridge_message_type_to_string"]
        fn to_string(message_type: MessageType) -> String;
    }

    extern "Rust" {
        type Angle;
        fn radians(self: &Angle) -> f64;
        fn degrees(self: &Angle) -> f64;
        fn seconds(self: &Angle) -> f64;
        fn minutes(self: &Angle) -> f64;
        fn revolutions(self: &Angle) -> f64;
        type AngleOpt;
        fn radians(self: &AngleOpt) -> f64;
        fn degrees(self: &AngleOpt) -> f64;
        fn seconds(self: &AngleOpt) -> f64;
        fn minutes(self: &AngleOpt) -> f64;
        fn revolutions(self: &AngleOpt) -> f64;

        type Length;
        fn feet(self: &Length) -> f64;
        fn meters(self: &Length) -> f64;
        fn kilometers(self: &Length) -> f64;
        fn nautical_miles(self: &Length) -> f64;
        type LengthOpt;
        fn feet(self: &LengthOpt) -> f64;
        fn meters(self: &LengthOpt) -> f64;
        fn kilometers(self: &LengthOpt) -> f64;
        fn nautical_miles(self: &LengthOpt) -> f64;

        type Velocity;
        fn feet_per_second(self: &Velocity) -> f64;
        fn feet_per_minute(self: &Velocity) -> f64;
        fn meters_per_second(self: &Velocity) -> f64;
        fn kilometers_per_hour(self: &Velocity) -> f64;
        fn miles_per_hour(self: &Velocity) -> f64;
        fn knots(self: &Velocity) -> f64;
        type VelocityOpt;
        fn feet_per_second(self: &VelocityOpt) -> f64;
        fn feet_per_minute(self: &VelocityOpt) -> f64;
        fn meters_per_second(self: &VelocityOpt) -> f64;
        fn kilometers_per_hour(self: &VelocityOpt) -> f64;
        fn miles_per_hour(self: &VelocityOpt) -> f64;
        fn knots(self: &VelocityOpt) -> f64;
    }

    extern "Rust" {
        type Heartbeat;
        fn heartbeat(self: &Message) -> Result<Box<Heartbeat>>;
        fn is_heartbeat(self: &Message) -> bool;

        fn timestamp(self: &Heartbeat) -> u32;

        type Initialization;
        fn initialization(self: &Message) -> Result<Box<Initialization>>;
        fn is_initialization(self: &Message) -> bool;

        type UplinkData;
        fn uplink_data(self: &Message) -> Result<Box<UplinkData>>;
        fn is_uplink_data(self: &Message) -> bool;

        type HeightAboveTerrain;
        fn height_above_terrain(self: &Message) -> Result<Box<HeightAboveTerrain>>;
        fn is_height_above_terrain(self: &Message) -> bool;

        fn height_above_terrain(self: &HeightAboveTerrain) -> Box<LengthOpt>;

        type TrafficReport;
        fn ownship(self: &Message) -> Result<Box<TrafficReport>>;
        fn is_ownship(self: &Message) -> bool;

        type OwnshipGeometricAltitude;
        fn ownship_geometric_altitude(self: &Message) -> Result<Box<OwnshipGeometricAltitude>>;
        fn is_ownship_geometric_altitude(self: &Message) -> bool;

        fn traffic(self: &Message) -> Result<Box<TrafficReport>>;
        fn is_traffic(self: &Message) -> bool;

        fn latitude(self: &TrafficReport) -> Box<Angle>;
        fn longitude(self: &TrafficReport) -> Box<Angle>;
        fn altitude(self: &TrafficReport) -> Box<Length>;
        fn horizontal_velocity(self: &TrafficReport) -> Box<VelocityOpt>;
        fn vertical_velocity(self: &TrafficReport) -> Box<VelocityOpt>;
        fn track_heading(self: &TrafficReport) -> Box<Angle>;

        type ForeFlightID;
        fn fore_flight_id(self: &Message) -> Result<Box<ForeFlightID>>;
        fn is_fore_flight_id(self: &Message) -> bool;

        type ForeFlightAHRS;
        fn fore_flight_ahrs(self: &Message) -> Result<Box<ForeFlightAHRS>>;
        fn is_fore_flight_ahrs(self: &Message) -> bool;

        fn roll(self: &ForeFlightAHRS) -> Box<AngleOpt>;
        fn pitch(self: &ForeFlightAHRS) -> Box<AngleOpt>;
        fn heading_type(self: &ForeFlightAHRS) -> AHRSHeadingType;
        fn heading(self: &ForeFlightAHRS) -> Box<AngleOpt>;
        fn indicated_airspeed(self: &ForeFlightAHRS) -> Box<VelocityOpt>;
        fn true_airspeed(self: &ForeFlightAHRS) -> Box<VelocityOpt>;

        type CustomPreciseOwnship;
        fn custom_precise_ownship(self: &Message) -> Result<Box<CustomPreciseOwnship>>;
        fn is_custom_precise_ownship(self: &Message) -> bool;

        fn latitude(self: &CustomPreciseOwnship) -> Box<Angle>;
        fn longitude(self: &CustomPreciseOwnship) -> Box<Angle>;
        fn altitude(self: &CustomPreciseOwnship) -> Box<Length>;
        fn ground_speed(self: &CustomPreciseOwnship) -> Box<Velocity>;
    }

    #[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct ForeFlightBroadcast {
        app: String,
        port: u16,
    }

    extern "Rust" {
        #[Self = ForeFlightBroadcast]
        #[rust_name = "bridge_from_json"]
        fn from_json(json: &CxxString) -> Result<ForeFlightBroadcast>;
        #[rust_name = "bridge_to_json"]
        fn to_json(self: &ForeFlightBroadcast) -> String;
    }
}
