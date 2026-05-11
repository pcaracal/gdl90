#![allow(clippy::unnecessary_box_returns)]

use crate::prelude::*;
use cxx::CxxString;
use cxx::CxxVector;
use deku::DekuEnumExt;
use ffi::MessageType;
use std::ops::Deref;

include!("ffi/bridge.rs");

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
        #[rust_name = "bridge_from_gdl90_bytes"]
        fn from_gdl90_bytes(bytes: &CxxVector<u8>) -> Vec<MessageResult>;

        #[rust_name = "bridge_get_type"]
        fn get_type(self: &Message) -> MessageType;

        #[rust_name = "bridge_to_string"]
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
        type Heartbeat;
        #[rust_name = "bridge_heartbeat"]
        fn heartbeat(self: &Message) -> Result<Box<Heartbeat>>;
        fn is_heartbeat(self: &Message) -> bool;

        #[rust_name = "bridge_timestamp"]
        fn timestamp(self: &Heartbeat) -> u32;

        type Initialization;
        #[rust_name = "bridge_initialization"]
        fn initialization(self: &Message) -> Result<Box<Initialization>>;
        fn is_initialization(self: &Message) -> bool;

        type UplinkData;
        #[rust_name = "bridge_uplink_data"]
        fn uplink_data(self: &Message) -> Result<Box<UplinkData>>;
        fn is_uplink_data(self: &Message) -> bool;

        type HeightAboveTerrain;
        #[rust_name = "bridge_height_above_terrain"]
        fn height_above_terrain(self: &Message) -> Result<Box<HeightAboveTerrain>>;
        fn is_height_above_terrain(self: &Message) -> bool;

        #[rust_name = "bridge_height_above_terrain"]
        fn height_above_terrain(self: &HeightAboveTerrain) -> Box<Length>;

        type TrafficReport;
        #[rust_name = "bridge_ownship"]
        fn ownship(self: &Message) -> Result<Box<TrafficReport>>;
        fn is_ownship(self: &Message) -> bool;

        type OwnshipGeometricAltitude;
        #[rust_name = "bridge_ownship_geometric_altitude"]
        fn ownship_geometric_altitude(self: &Message) -> Result<Box<OwnshipGeometricAltitude>>;
        fn is_ownship_geometric_altitude(self: &Message) -> bool;

        #[rust_name = "bridge_traffic"]
        fn traffic(self: &Message) -> Result<Box<TrafficReport>>;
        fn is_traffic(self: &Message) -> bool;

        #[rust_name = "bridge_latitude"]
        fn latitude(self: &TrafficReport) -> Box<Angle>;
        #[rust_name = "bridge_longitude"]
        fn longitude(self: &TrafficReport) -> Box<Angle>;
        #[rust_name = "bridge_altitude"]
        fn altitude(self: &TrafficReport) -> Box<Length>;
        #[rust_name = "bridge_horizontal_velocity"]
        fn horizontal_velocity(self: &TrafficReport) -> Box<Velocity>;
        #[rust_name = "bridge_vertical_velocity"]
        fn vertical_velocity(self: &TrafficReport) -> Box<Velocity>;
        #[rust_name = "bridge_track_heading"]
        fn track_heading(self: &TrafficReport) -> Box<Angle>;

        type ForeFlightID;
        #[rust_name = "bridge_fore_flight_id"]
        fn fore_flight_id(self: &Message) -> Result<Box<ForeFlightID>>;
        fn is_fore_flight_id(self: &Message) -> bool;

        type ForeFlightAHRS;
        #[rust_name = "bridge_fore_flight_ahrs"]
        fn fore_flight_ahrs(self: &Message) -> Result<Box<ForeFlightAHRS>>;
        fn is_fore_flight_ahrs(self: &Message) -> bool;

        #[rust_name = "bridge_roll"]
        fn roll(self: &ForeFlightAHRS) -> Box<Angle>;
        #[rust_name = "bridge_pitch"]
        fn pitch(self: &ForeFlightAHRS) -> Box<Angle>;
        #[rust_name = "bridge_heading_type"]
        fn heading_type(self: &ForeFlightAHRS) -> AHRSHeadingType;
        #[rust_name = "bridge_heading"]
        fn heading(self: &ForeFlightAHRS) -> Box<Angle>;
        #[rust_name = "bridge_indicated_airspeed"]
        fn indicated_airspeed(self: &ForeFlightAHRS) -> Box<Velocity>;
        #[rust_name = "bridge_true_airspeed"]
        fn true_airspeed(self: &ForeFlightAHRS) -> Box<Velocity>;

        type CustomPreciseOwnship;
        #[rust_name = "bridge_custom_precise_ownship"]
        fn custom_precise_ownship(self: &Message) -> Result<Box<CustomPreciseOwnship>>;
        fn is_custom_precise_ownship(self: &Message) -> bool;

        #[rust_name = "bridge_latitude"]
        fn latitude(self: &CustomPreciseOwnship) -> Box<Angle>;
        #[rust_name = "bridge_longitude"]
        fn longitude(self: &CustomPreciseOwnship) -> Box<Angle>;
        #[rust_name = "bridge_altitude"]
        fn altitude(self: &CustomPreciseOwnship) -> Box<Length>;
        #[rust_name = "bridge_ground_speed"]
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
