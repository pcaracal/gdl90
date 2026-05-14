use super::*;

const HEARTBEAT: [u8; 11] = [
    0x7E, 0x00, 0x81, 0x41, 0xDB, 0xD0, 0x08, 0x02, 0xB3, 0x8B, 0x7E,
];

const OWNSHIP: [u8; 32] = [
    0x7E, 0x0A, 0x00, 0xAB, 0x45, 0x49, 0x1F, 0xEF, 0x15, 0xA8, 0x89, 0x78, 0x0F, 0x09, 0xA9, 0x07,
    0xB0, 0x01, 0x20, 0x01, 0x4E, 0x38, 0x32, 0x35, 0x56, 0x20, 0x20, 0x20, 0x00, 0x85, 0x5B, 0x7E,
];

const FF_AHRS: [u8; 16] = [
    0x7E, 0x65, 0x01, 0xF8, 0xF8, 0xF8, 0xF8, 0xF8, 0xF8, 0x0A, 0x04, 0x0E, 0x08, 0x99, 0xDD, 0x7E,
];

fn new_heartbeat() -> Heartbeat {
    Heartbeat::default()
        .with_uat_initialized()
        .with_gps_pos_valid()
        .with_csa_requested()
        .with_utc_ok()
        .with_timestamp(53467)
        .with_message_counts(2050)
}

fn new_ownship() -> OwnshipMessage {
    TrafficReport::new(
        TrafficAlertStatus::NoAlert,
        TargetIdentity::new(AddressType::AdsbIcao, 11_224_393),
        44.90708.degrees(),
        -122.99488.degrees(),
        Some(5000.feet()),
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
    )
    .into()
}

fn new_ahrs() -> ForeFlightAHRS {
    ForeFlightAHRS::default()
        .with_roll(-180.degrees())
        .with_pitch(-180.degrees())
        .with_heading_type(AHRSHeadingType::Magnetic)
        .with_heading(-180.degrees())
        .with_indicated_airspeed(2564.knots())
        .with_true_airspeed(3592.knots())
}

#[test]
fn heartbeat_encode_decode() {
    let hb = new_heartbeat();

    let bytes = hb.into_gdl90_bytes().unwrap();
    assert_eq!(bytes, HEARTBEAT);

    let hb_r = Message::from_gdl90_bytes(bytes);
    assert_eq!(hb_r.len(), 1);
    let hb_r = hb_r[0].as_ref().unwrap().heartbeat().unwrap();
    assert_eq!(hb_r, &hb);

    let hb_dec = Message::from_gdl90_bytes(HEARTBEAT);
    assert_eq!(hb_dec.len(), 1);
    let hb_dec = hb_dec[0].as_ref().unwrap().heartbeat().unwrap();
    assert_eq!(hb_dec, &hb);
}

#[test]
fn ownship_encode_decode() {
    let os = new_ownship();

    let bytes = os.clone().into_gdl90_bytes().unwrap();
    assert_eq!(bytes, OWNSHIP);

    let os_dec = Message::from_gdl90_bytes(OWNSHIP);
    assert_eq!(os_dec.len(), 1);
    let os_dec = os_dec[0].as_ref().unwrap().ownship().unwrap().clone();
    let os_dec = os_dec.clone().into_gdl90_bytes().unwrap();
    assert_eq!(os_dec, OWNSHIP);

    assert_eq!(bytes, os_dec);
}

#[test]
fn foreflight_ahrs_encode_decode() {
    let ahrs = new_ahrs();

    let bytes = ahrs.into_gdl90_bytes().unwrap();
    assert_eq!(bytes, FF_AHRS);

    let ahrs_dec = Message::from_gdl90_bytes(FF_AHRS);
    assert_eq!(ahrs_dec.len(), 1);
    let ahrs_dec = *ahrs_dec[0]
        .as_ref()
        .unwrap()
        .fore_flight()
        .unwrap()
        .ahrs()
        .unwrap();
    assert_eq!(ahrs_dec, ahrs);
    let ahrs_dec = ahrs_dec.into_gdl90_bytes().unwrap();
    assert_eq!(ahrs_dec, FF_AHRS);
}

#[test]
fn multiple_messages_encode_decode() {
    let mut bytes = Vec::new();

    {
        bytes.extend_from_slice(&new_heartbeat().into_gdl90_bytes().unwrap());
        bytes.extend_from_slice(&new_ownship().into_gdl90_bytes().unwrap());
        bytes.extend_from_slice(&new_ahrs().into_gdl90_bytes().unwrap());
        bytes.extend_from_slice(&new_ahrs().into_gdl90_bytes().unwrap());

        let messages = Message::from_gdl90_bytes(bytes.clone());
        assert_eq!(messages.len(), 4);

        let m0 = messages[0].as_ref().unwrap();
        let m1 = messages[1].as_ref().unwrap();
        let m2 = messages[2].as_ref().unwrap();
        let m3 = messages[3].as_ref().unwrap();

        assert!(m0.is_heartbeat());
        assert!(!m0.is_fore_flight());
        assert_eq!(m0.heartbeat().unwrap(), &new_heartbeat());

        assert!(m1.is_ownship());
        assert!(!m1.is_fore_flight());

        assert!(m2.is_fore_flight());
        assert!(m2.fore_flight().unwrap().is_ahrs());
        assert_eq!(m2.fore_flight().unwrap().ahrs().unwrap(), &new_ahrs());

        assert!(m3.is_fore_flight());
        assert!(m3.fore_flight().unwrap().is_ahrs());
        assert_eq!(m3.fore_flight().unwrap().ahrs().unwrap(), &new_ahrs());
    }

    let mut bytes2 = vec![];
    bytes2.extend_from_slice(&HEARTBEAT);
    bytes2.extend_from_slice(&OWNSHIP);
    bytes2.extend_from_slice(&FF_AHRS);
    bytes2.extend_from_slice(&FF_AHRS);

    assert_eq!(bytes, bytes2);
}
