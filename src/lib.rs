#![cfg_attr(
    not(test),
    deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)
)]
#![allow(clippy::doc_markdown)]

#[macro_use]
extern crate utilities_derive;

pub mod error;
mod ffi;
pub mod message;
pub mod message_types;
pub mod util;

pub mod prelude {
    pub use crate::error::*;
    pub use crate::message::*;
    pub use crate::message_types::*;
    pub use crate::util::*;

    #[cfg(test)]
    pub(crate) use crate::assert_eq_f;
    pub(crate) use deku::{
        ctx::{BitSize, ByteSize, Endian},
        prelude::*,
    };
    pub(crate) use uom::si::{
        angle::{degree, minute, radian, revolution, second},
        f64::{Angle, Length, Velocity},
        length::{foot, kilometer, meter, nautical_mile},
        velocity::{
            foot_per_minute, foot_per_second, kilometer_per_hour, knot, meter_per_second,
            mile_per_hour,
        },
    };
}
