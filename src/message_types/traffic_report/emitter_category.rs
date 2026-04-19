use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, EnumGet)]
#[deku(
    ctx = "_: deku::ctx::Endian, _: deku::ctx::Order",
    id_type = "u8",
    bits = 8
)]
#[repr(u8)]
/// # 3.5.1.10 Emitter Category
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
    // 8 => Unassigned,
    // 13 => Unassigned,
    // 16 => Unassigned,
    // 22..=39 => Reserved,
}
