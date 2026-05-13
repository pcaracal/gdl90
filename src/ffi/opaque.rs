#![allow(clippy::unnecessary_box_returns)]

use crate::prelude as rs;
use rs::uom_utils::FromUom;
use std::ops::Deref;

macro_rules! impl_bridge_wrapper {
    ($($Wrapper:ident($Inner:ty)),* $(,)?) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $Wrapper(pub $Inner);
            impl From<$Inner> for $Wrapper { fn from(inner: $Inner) -> Self { Self(inner) } }
            impl<T> From<T> for Box<$Wrapper> where $Inner: From<T> { fn from(inner: T) -> Self { Box::new($Wrapper(inner.into())) } }
            impl Deref for $Wrapper {
                type Target = $Inner;
                fn deref(&self) -> &Self::Target { &self.0 }
             }
        )*
    };
}

impl_bridge_wrapper! {
    Angle(rs::Angle),
    AngleOpt(Option<rs::Angle>),
    Length(rs::Length),
    LengthOpt(Option<rs::Length>),
    VelocityOpt(Option<rs::Velocity>),
    Velocity(rs::Velocity),

    MessageResult(Result<Message, String>),
    Message(rs::Message),
    Heartbeat(rs::Heartbeat),
    Initialization(rs::Initialization),
    UplinkData(rs::UplinkData),
    HeightAboveTerrain(rs::HeightAboveTerrain),
    TrafficReport(rs::TrafficReport),
    OwnshipGeometricAltitude(rs::OwnshipGeometricAltitude),
    ForeFlightID(rs::ForeFlightID),
    ForeFlightAHRS(rs::ForeFlightAHRS),
    CustomPreciseOwnship(rs::CustomPreciseOwnship),
}

macro_rules! bridge {
    [ $($fn:ident -> Box<$ret:ty>),* $(,)? ] => { $( pub fn $fn(&self) -> Box<$ret> { Box::new(self.0.$fn().clone().into()) })* };
    [ $($fn:ident -> $ret:ty),* $(,)? ] => { $( pub fn $fn(&self) -> $ret { self.0.$fn() })* };
    [ map: $map:expr, $($fn:ident -> $ret:ty),* $(,)? ] => { $( pub fn $fn(&self) -> $ret { pastey::paste! { self.0.$fn().$map().into() } })* };
    [ map_self: $map:expr, $($fn:ident -> $ret:ty),* $(,)? ] => { $( pub fn $fn(&self) -> $ret { pastey::paste! { self.0.$map().$fn() } })* };
}

macro_rules! bridge_fn {
    [ $($fn:ident: $args:tt -> $ret:ty $block:block),* $(,)? ] => { $( pub fn $fn $args -> $ret { $block })* };
}

impl MessageResult {
    bridge![
        is_ok -> bool,
        is_err -> bool,
    ];

    bridge_fn![
        ok: (&self) -> Option<Box<Message>> { self.0.as_ref().ok().cloned().map(Box::new) },
        err: (&self) -> String { self.0.as_ref().err().cloned().unwrap_or_default() },
        unwrap: (&self) -> Result<Box<Message>, String> { self.0.as_ref().cloned().map(Box::new).map_err(ToString::to_string) }
    ];
}

impl Angle {
    bridge![
        radians -> f64,
        degrees -> f64,
        seconds -> f64,
        minutes -> f64,
        revolutions -> f64,
    ];
}
impl AngleOpt {
    bridge![
        map_self: unwrap_or_default,
        radians -> f64,
        degrees -> f64,
        seconds -> f64,
        minutes -> f64,
        revolutions -> f64,
    ];
}

impl Length {
    bridge![
        feet -> f64,
        meters -> f64,
        kilometers -> f64,
        nautical_miles -> f64,
    ];
}
impl LengthOpt {
    bridge![
        map_self: unwrap_or_default,
        feet -> f64,
        meters -> f64,
        kilometers -> f64,
        nautical_miles -> f64,
    ];
}

impl Velocity {
    bridge![
        feet_per_second -> f64,
        feet_per_minute -> f64,
        meters_per_second -> f64,
        kilometers_per_hour -> f64,
        miles_per_hour -> f64,
        knots -> f64,
    ];
}
impl VelocityOpt {
    bridge![
        map_self: unwrap_or_default,
        feet_per_second -> f64,
        feet_per_minute -> f64,
        meters_per_second -> f64,
        kilometers_per_hour -> f64,
        miles_per_hour -> f64,
        knots -> f64,
    ];

    bridge_fn![
        is_some: (&self) -> bool { self.0.is_some() },
        is_none: (&self) -> bool { self.0.is_none() },
    ];
}

macro_rules! impl_ffi_message_getter {
    ($($ty:ident($fn:ident)),* $(,)?) => {
            $(
            pub fn $fn(&self) -> Result<Box<$ty>, String> {
                if let Some(msg) = self.0.$fn() {
                    let msg: crate::message_types::$ty = msg.clone().into();
                    Ok(msg.into())
                } else {
                    Err(format!("Message is not {}!", stringify!($ty)))
                }
            }

            pastey::paste!{
                pub fn [<is_ $fn>](&self) -> bool {
                    self.0.[<is_$fn>]()
                }
            }
        )*
    };
}

impl Message {
    impl_ffi_message_getter! {
        Heartbeat(heartbeat),
        Initialization(initialization),
        UplinkData(uplink_data),
        HeightAboveTerrain(height_above_terrain),
        TrafficReport(ownship),
        OwnshipGeometricAltitude(ownship_geometric_altitude),
        TrafficReport(traffic),
        ForeFlightID(fore_flight_id),
        ForeFlightAHRS(fore_flight_ahrs),
        CustomPreciseOwnship(custom_precise_ownship),
    }

    bridge_fn![
        from_gdl90_bytes: (bytes: &[u8]) -> Vec<MessageResult> { rs::Message::from_gdl90_bytes(bytes).into_iter().map(|r| r.map(Message).map_err(|e| e.to_string())).map(MessageResult).collect() },
        to_string: (&self) -> String { format!("{self:#?}") },
    ];
}

impl Heartbeat {
    bridge![
        timestamp -> u32,
    ];
}

impl HeightAboveTerrain {
    bridge_fn![
        height_above_terrain: (&self) -> Box<LengthOpt> { self.0.height_above_terrain.into() },
    ];
}

impl TrafficReport {
    bridge![
         latitude -> Box<Angle>,
         longitude -> Box<Angle>,
         altitude -> Box<Length>,
         track_heading -> Box<Angle>,
    ];

    bridge![
         map: cloned,
         horizontal_velocity -> Box<VelocityOpt>,
         vertical_velocity -> Box<VelocityOpt>,
    ];
}

impl ForeFlightAHRS {
    bridge![
         map: cloned,
         roll -> Box<AngleOpt>,
         pitch -> Box<AngleOpt>,
         heading -> Box<AngleOpt>,
         indicated_airspeed -> Box<VelocityOpt>,
         true_airspeed -> Box<VelocityOpt>,
    ];
}

impl CustomPreciseOwnship {
    bridge![
         latitude -> Box<Angle>,
         longitude -> Box<Angle>,
         altitude -> Box<Length>,
         ground_speed -> Box<Velocity>,
    ];
}
