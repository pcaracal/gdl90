macro_rules! impl_bridge_wrapper {
    ($($Wrapper:ident($Inner:ty)),* $(,)?) => {
        $(
            #[derive(Debug)]
            struct $Wrapper($Inner);
            impl<T> From<T> for Box<$Wrapper> where $Inner: From<T> { fn from(inner: T) -> Self { Box::new($Wrapper(inner.into())) } }
            impl Deref for $Wrapper {
                type Target = $Inner;
                fn deref(&self) -> &Self::Target { &self.0 }
             }
        )*
    };
}

impl_bridge_wrapper! {
    MessageResult(crate::error::GDL90Result<Message>),
    Angle(Option<uom::si::f64::Angle>),
    Length(Option<uom::si::f64::Length>),
    Velocity(Option<uom::si::f64::Velocity>),
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
            .map(std::string::ToString::to_string)
            .unwrap_or_default()
    }

    fn unwrap(&self) -> Result<Box<Message>, String> {
        self.0
            .as_ref()
            .cloned()
            .map(Box::new)
            .map_err(|e| format!("Called unwrap on MessageResult but it was an error: {e}"))
    }
}

impl Angle {
    fn radians(&self) -> f64 {
        self.0.unwrap_or_default().radians()
    }
    fn degrees(&self) -> f64 {
        self.0.unwrap_or_default().degrees()
    }
    fn seconds(&self) -> f64 {
        self.0.unwrap_or_default().seconds()
    }
    fn minutes(&self) -> f64 {
        self.0.unwrap_or_default().minutes()
    }
    fn revolutions(&self) -> f64 {
        self.0.unwrap_or_default().revolutions()
    }
}

impl Length {
    fn feet(&self) -> f64 {
        self.0.unwrap_or_default().feet()
    }
    fn meters(&self) -> f64 {
        self.0.unwrap_or_default().meters()
    }
    fn kilometers(&self) -> f64 {
        self.0.unwrap_or_default().kilometers()
    }
    fn nautical_miles(&self) -> f64 {
        self.0.unwrap_or_default().nautical_miles()
    }
}

impl Velocity {
    fn feet_per_second(&self) -> f64 {
        self.0.unwrap_or_default().feet_per_second()
    }
    fn feet_per_minute(&self) -> f64 {
        self.0.unwrap_or_default().feet_per_minute()
    }
    fn meters_per_second(&self) -> f64 {
        self.0.unwrap_or_default().meters_per_second()
    }
    fn kilometers_per_hour(&self) -> f64 {
        self.0.unwrap_or_default().kilometers_per_hour()
    }
    fn miles_per_hour(&self) -> f64 {
        self.0.unwrap_or_default().miles_per_hour()
    }
    fn knots(&self) -> f64 {
        self.0.unwrap_or_default().knots()
    }
}

macro_rules! impl_ffi_message_getter {
    ($($ty:ident($fn:ident)),* $(,)?) => {
        pastey::paste!{$(
            fn [<bridge_ $fn>](&self) -> Result<Box<$ty>, String> {
                if let Some(msg) = self.$fn() {
                    let msg: crate::message_types::$ty = msg.clone().into();
                    Ok(msg.into())
                } else {
                    Err(format!("Message is not {}! Actual: {:?}", stringify!($ty), self.bridge_get_type()))
                }
            }
        )*}
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

    fn bridge_from_gdl90_bytes(bytes: &CxxVector<u8>) -> Vec<MessageResult> {
        Message::from_gdl90_bytes(bytes.as_slice())
            .into_iter()
            .map(MessageResult)
            .collect()
    }

    fn bridge_get_type(&self) -> ffi::MessageType {
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

    fn bridge_to_string(&self) -> String {
        format!("{self:#?}")
    }
}

fn bridge_message_type_to_string(message_type: MessageType) -> String {
    format!("{message_type:?}")
}

impl Heartbeat {
    fn bridge_timestamp(&self) -> u32 {
        self.timestamp()
    }
}

impl HeightAboveTerrain {
    fn bridge_height_above_terrain(&self) -> Box<Length> {
        self.height_above_terrain.into()
    }
}

impl TrafficReport {
    fn bridge_latitude(&self) -> Box<Angle> {
        self.latitude.into()
    }

    fn bridge_longitude(&self) -> Box<Angle> {
        self.longitude.into()
    }

    fn bridge_altitude(&self) -> Box<Length> {
        self.altitude.into()
    }

    fn bridge_horizontal_velocity(&self) -> Box<Velocity> {
        self.horizontal_velocity.into()
    }

    fn bridge_vertical_velocity(&self) -> Box<Velocity> {
        self.vertical_velocity.into()
    }

    fn bridge_track_heading(&self) -> Box<Angle> {
        self.track_heading.into()
    }
}

impl ForeFlightAHRS {
    fn bridge_roll(&self) -> Box<Angle> {
        self.roll.into()
    }
    fn bridge_pitch(&self) -> Box<Angle> {
        self.pitch.into()
    }
    fn bridge_heading_type(&self) -> ffi::AHRSHeadingType {
        match self.heading_type {
            AHRSHeadingType::True => ffi::AHRSHeadingType::True,
            AHRSHeadingType::Magnetic => ffi::AHRSHeadingType::Magnetic,
        }
    }
    fn bridge_heading(&self) -> Box<Angle> {
        self.heading.into()
    }
    fn bridge_indicated_airspeed(&self) -> Box<Velocity> {
        self.indicated_airspeed.into()
    }
    fn bridge_true_airspeed(&self) -> Box<Velocity> {
        self.true_airspeed.into()
    }
}

impl CustomPreciseOwnship {
    fn bridge_latitude(&self) -> Box<Angle> {
        self.latitude.into()
    }
    fn bridge_longitude(&self) -> Box<Angle> {
        self.longitude.into()
    }
    fn bridge_altitude(&self) -> Box<Length> {
        self.altitude.into()
    }
    fn bridge_ground_speed(&self) -> Box<Velocity> {
        self.ground_speed.into()
    }
}
