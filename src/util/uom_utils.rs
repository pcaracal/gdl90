use crate::prelude::*;

macro_rules! impl_uom_convert {
    ([$(
        $uom_type:ty, [$(
            ($fn:ident, $unit:ty)
        ),*]
    ),*]) => {
        /// Convenience trait to convert an `uom` type into `f64`
        pub trait FromUom
        {
            $($(
                #[inline]
                fn $fn(self) -> f64
                where
                    Self: Into<$uom_type>,
                {
                    self.into().get::<$unit>()
                }
            )*)*
        }
        $( impl FromUom for $uom_type {} )*

        /// Convenience trait to convert `f64` into an `uom` type
        pub trait IntoUom: Copy
        {
            $($(
                #[inline]
                fn $fn(self) -> $uom_type where Self: Copy + Into<f64>,
                {
                    <$uom_type>::new::<$unit>(self.into())
                }
            )*)*
        }
        impl<T: Into<f64>> IntoUom for T where T: Copy {}

        /// Convenience trait to convert an `uom` iter into an `f64` iter
        /// Probably only useful in unit tests
        pub trait IntoUomIterator
        where
            Self: Sized + IntoIterator,
            Self::Item: Copy + Into<f64>,
        {
            $($(
            #[inline]
            fn $fn(self) -> impl IntoIterator<Item = $uom_type> {
                self.into_iter().map(|v| v.$fn())
            }
            )*)*
        }
        impl<T> IntoUomIterator for T where T: Sized + IntoIterator, T::Item: Copy + Into<f64> {}
    };
}

impl_uom_convert! {
    [
        Angle, [
            (radians, radian),
            (degrees, degree),
            (seconds, second),
            (minutes, minute),
            (revolutions, revolution)
        ],
        Length, [
            (meters, meter),
            (kilometers, kilometer),
            (feet, foot),
            (nautical_miles, nautical_mile)
        ],
        Velocity, [
            (meters_per_second, meter_per_second),
            (kilometers_per_hour, kilometer_per_hour),
            (feet_per_second, foot_per_second),
            (feet_per_minute, foot_per_minute),
            (miles_per_hour, mile_per_hour),
            (knots, knot)
        ]
    ]
}
