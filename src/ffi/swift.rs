use crate::prelude as rs;
use rs::Heartbeat;

struct Angle {
    v: rs::Angle,
}
struct Length {
    v: rs::Length,
}
struct Velocity {
    v: rs::Velocity,
}

#[swift_bridge::bridge]
mod ffi {
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
    }

    extern "Rust" {
        type Heartbeat;
        #[swift_bridge(get(gps_pos_valid))]
        fn gps_pos_valid(self: &Heartbeat) -> bool;
        #[swift_bridge(get(maint_reqd))]
        fn maint_reqd(self: &Heartbeat) -> bool;
    }
}
