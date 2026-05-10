pub mod address_type;
pub mod emergency_priority_code;
pub mod emitter_category;
pub mod miscellaneous_indicators;
pub mod nacp;
pub mod nic;
pub mod traffic_alert_status;

mod r#impl;

pub use self::{
    address_type::*, emergency_priority_code::*, emitter_category::*, miscellaneous_indicators::*,
    nacp::NACp, nic::NIC, traffic_alert_status::*,
};

use crate::message_types::traffic_report::r#impl::{
    altitude_read, altitude_write, callsign_read, callsign_write, coord_read, coord_write,
    heading_read, heading_write, hv_read, hv_write, vv_read, vv_write,
};

use crate::prelude::*;

/// # 3.4. Ownship Report Message
///
/// The GDL 90 will always output an Ownship Report message once per second. The message
/// uses the same format as the Traffic Report, with the Message ID set to the value 10.
#[derive(Debug, Default, Clone, PartialEq, DekuRead, DekuWrite)]
pub struct OwnshipMessage(pub TrafficReport);

impl From<TrafficReport> for OwnshipMessage {
    fn from(tr: TrafficReport) -> Self {
        Self(tr)
    }
}

/// # 3.5. Traffic Report Message
#[derive(Debug, Default, Clone, PartialEq, DekuRead, DekuWrite)]
pub struct TrafficMessage(pub TrafficReport);

impl From<TrafficReport> for TrafficMessage {
    fn from(tr: TrafficReport) -> Self {
        Self(tr)
    }
}

impl From<OwnshipMessage> for TrafficReport {
    fn from(os: OwnshipMessage) -> Self {
        os.0
    }
}

impl From<TrafficMessage> for TrafficReport {
    fn from(tm: TrafficMessage) -> Self {
        tm.0
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default, Clone, PartialEq, DekuRead, DekuWrite, Builder)]
#[deku(bit_order = "msb", endian = "big")]
/// # 3.5.1 Traffic and Ownship Report Data Format
pub struct TrafficReport {
    /// `s`
    /// Traffic Alert Status.
    pub traffic_alert_status: TrafficAlertStatus,

    /// `t` & `aaaaaa`
    /// Address Type & Participant Address
    pub target_identity: TargetIdentity,

    /// `ll ll ll`
    /// Latitude
    #[deku(
        reader = "coord_read(deku::reader)",
        writer = "coord_write(deku::writer, self.latitude)"
    )]
    pub latitude: Angle,

    /// `nn nn nn`
    /// Longitude
    #[deku(
        reader = "coord_read(deku::reader)",
        writer = "coord_write(deku::writer, self.longitude)"
    )]
    pub longitude: Angle,

    /// `ddd`
    /// Altitude
    #[deku(
        reader = "altitude_read(deku::reader)",
        writer = "altitude_write(deku::writer, self.altitude)"
    )]
    pub altitude: Length,

    /// `m`
    /// Miscellaneous indicators
    pub miscellaneous_indicators: MiscellaneousIndicators,

    /// `i`
    /// Navigation Integrity Category (NIC)
    pub nic: NIC,

    /// `a`
    /// Navigation Accuracy Category for Position (`NACp`)
    pub nacp: NACp,

    /// `hhh`
    /// Horizontal velocity.
    ///
    /// None = unavailable
    #[deku(
        reader = "hv_read(deku::reader)",
        writer = "hv_write(deku::writer, self.horizontal_velocity)"
    )]
    pub horizontal_velocity: Option<Velocity>,

    /// `vvv`
    /// Vertical Velocity.
    ///
    /// None = unavailable
    #[deku(
        reader = "vv_read(deku::reader)",
        writer = "vv_write(deku::writer, self.vertical_velocity)"
    )]
    pub vertical_velocity: Option<Velocity>,

    /// `tt`
    /// Track/Heading.
    /// See Miscellaneous field for Track/Heading indication.
    #[deku(
        reader = "heading_read(deku::reader)",
        writer = "heading_write(deku::writer, self.track_heading)"
    )]
    pub track_heading: Angle,

    /// `ee`
    /// Emitter Category
    pub emitter_category: EmitterCategory,

    /// `cc cc cc cc`
    /// Call Sign: 8 ASCII characters, '0' through '9' and 'A' through 'Z'.
    ///
    /// Padded with spaces (0x20) if fewer than 8 characters.
    /// Truncated to 8 characters if more than 8 characters.
    #[deku(
        reader = "callsign_read(deku::reader)",
        writer = "callsign_write(deku::writer, &self.callsign)"
    )]
    pub callsign: String,

    /// `p`
    /// Emergency/Priority Code
    #[deku(pad_bits_after = "4")] // spare
    pub emergency_priority_code: EmergencyPriorityCode,
}

impl TrafficReport {
    #[must_use]
    pub fn ownship(self) -> OwnshipMessage {
        self.into()
    }

    #[must_use]
    pub fn traffic(self) -> TrafficMessage {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BYTES: [u8; 27] = [
        0x00, 0xAB, 0x45, 0x49, 0x1F, 0xEF, 0x15, 0xA8, 0x89, 0x78, 0x0F, 0x09, 0xA9, 0x07, 0xB0,
        0x01, 0x20, 0x01, 0x4E, 0x38, 0x32, 0x35, 0x56, 0x20, 0x20, 0x20, 0x00,
    ];

    const COORD_EPSILON: f64 = 180.0 / (1 << 23) as f64;

    #[test]
    fn decode() {
        let (_, tr) = TrafficReport::from_bytes((&BYTES, 0)).unwrap();

        assert_eq!(tr.traffic_alert_status, TrafficAlertStatus::NoAlert);
        assert_eq!(tr.target_identity.address_type, AddressType::AdsbIcao);
        assert_eq!(tr.target_identity.participant_address, 11_224_393); // 52642511 octal

        assert_eq_f!(tr.latitude, 44.90708.degrees(), COORD_EPSILON);
        assert_eq_f!(tr.longitude, -122.99488.degrees(), COORD_EPSILON);

        assert_eq!(tr.altitude, 5000.feet());
        assert_eq!(tr.miscellaneous_indicators.report_type, ReportType::Updated);
        assert_eq!(
            tr.miscellaneous_indicators.air_ground_state,
            AirGroundState::Airborne
        );
        assert_eq!(
            tr.miscellaneous_indicators.track_heading_type,
            TrackHeadingType::TrueTrackAngle
        );

        assert_eq!(tr.nic, NIC::NIC10_HPL_25M_VPL_37_5M);
        assert_eq!(tr.nacp, NACp::NACp9_HFOM_30M_VFOM_45M);
        assert_eq!(tr.horizontal_velocity, Some(123.knots()));
        assert_eq_f!(tr.track_heading, 45.degrees(), 360.0 / 256.0);
        assert_eq!(tr.vertical_velocity, Some(64.feet_per_minute()));
        assert_eq!(
            tr.emergency_priority_code,
            EmergencyPriorityCode::NoEmergency
        );
        assert_eq!(tr.emitter_category, EmitterCategory::Light);
        assert_eq!(tr.callsign, "N825V");
    }

    #[test]
    fn encode() {
        let tr = TrafficReport::new(
            TrafficAlertStatus::NoAlert,
            TargetIdentity::new(AddressType::AdsbIcao, 11_224_393),
            44.90708.degrees(),
            -122.99488.degrees(),
            5000.feet(),
            MiscellaneousIndicators::new(
                AirGroundState::Airborne,
                ReportType::Updated,
                TrackHeadingType::TrueTrackAngle,
            ),
            NIC::NIC10_HPL_25M_VPL_37_5M,
            NACp::NACp9_HFOM_30M_VFOM_45M,
            Some(123.knots()),
            Some(64.feet_per_minute()),
            45.degrees(),
            EmitterCategory::Light,
            "N825V",
            EmergencyPriorityCode::NoEmergency,
        );

        let encoded = tr.to_bytes().unwrap();

        assert_eq!(
            encoded,
            BYTES,
            "left:\n{}\nright:\n{}",
            encoded.hex_fmt(),
            BYTES.hex_fmt()
        );
    }

    #[test]
    fn coordinates() {
        const RES: f64 = 180.0 / 0x7F_FF_FF as f64;
        let lats = (-900..=900).map(|v| f64::from(v) / 10.0).degrees();
        let lons = (-1800..=1800).map(|v| f64::from(v) / 20.0).degrees();

        for (lat, lon) in lats.into_iter().zip(lons) {
            let tr = TrafficReport::default()
                .with_latitude(lat)
                .with_longitude(lon);
            assert_eq_f!(tr.latitude, lat, RES);
            assert_eq_f!(tr.longitude, lon, RES);
            let bytes = tr.to_bytes().unwrap();
            let tr_dec = TrafficReport::from_bytes((&bytes, 0)).unwrap().1;
            assert_eq_f!(tr_dec.latitude, lat, RES);
            assert_eq_f!(tr_dec.longitude, lon, RES);
        }
    }

    #[test]
    fn altitude() {
        let altitudes = (0..0xFFE).map(|v| (v * 25) - 1000).feet();

        for alt in altitudes {
            let tr = TrafficReport::default().with_altitude(alt);
            assert_eq!(tr.altitude, alt);
            let bytes = tr.to_bytes().unwrap();
            let tr_dec = TrafficReport::from_bytes((&bytes, 0)).unwrap().1;
            assert_eq!(tr_dec.altitude, alt);
        }
    }

    #[test]
    fn horizontal_velocity() {
        let hvs = (0..4094).knots().into_iter().map(Some).chain([None]);

        for hv in hvs {
            let mut tr = TrafficReport::default();
            tr.set_horizontal_velocity(hv);
            assert_eq!(tr.horizontal_velocity, hv);
            let bytes = tr.to_bytes().unwrap();
            let tr_dec = TrafficReport::from_bytes((&bytes, 0)).unwrap().1;
            assert_eq!(tr_dec.horizontal_velocity, hv);
        }
    }

    #[test]
    fn vertical_velocity() {
        let vs = (-510..510)
            .map(|v| v * 64)
            .feet_per_minute()
            .into_iter()
            .map(Some)
            .chain([None]);

        for vv in vs {
            let mut tr = TrafficReport::default();
            tr.set_vertical_velocity(vv);
            assert_eq!(tr.vertical_velocity, vv);
            let bytes = tr.to_bytes().unwrap();
            let tr_dec = TrafficReport::from_bytes((&bytes, 0)).unwrap().1;
            assert_eq!(tr_dec.vertical_velocity, vv,);
        }
    }

    #[test]
    fn track_heading() {
        const RES: f64 = 360.0 / 256.0;
        for hdg in (0..360).degrees() {
            let tr = TrafficReport::default().with_track_heading(hdg);
            assert_eq_f!(tr.track_heading, hdg, RES);
            let bytes = tr.to_bytes().unwrap();
            let tr_dec = TrafficReport::from_bytes((&bytes, 0)).unwrap().1;
            assert_eq_f!(tr_dec.track_heading, hdg, RES);
        }
    }
}
