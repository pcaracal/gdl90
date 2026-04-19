pub mod uom_utils;

pub use self::uom_utils::*;

macro_rules! impl_clamp_into {
    [$(($from:ty, $to:ty)),*] => {
        /// Clamp `f64` into `T`'s range' then cast into `T`.
        /// Fixes precision issues so 29.999999999999999 is 30, not 29
        pub(crate) trait ClampInto<T> {
            fn clamp_into(self) -> T;
        }

        $(
            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            impl ClampInto<$to> for $from {
                fn clamp_into(self) -> $to {
                    let solution_for_ieee754 = if (self - self.round()).abs() < 1e-6 {
                        self.round()
                    } else {
                        self
                    };
                    solution_for_ieee754.clamp(<$to>::MIN as $from, <$to>::MAX as $from) as $to
                }
            }
        )*
    };
}
impl_clamp_into![(f64, u32), (f64, i32), (f64, u16), (f64, i16), (f64, u8)];

#[cfg(test)]
#[macro_export]
macro_rules! assert_eq_f {
    ($left:expr, $right:expr, $e:expr) => {
        if ($left.value - $right.value).abs() >= $e {
            panic!(
                "assertion `left ≈ right` failed
     left: {:?}
    right: {:?}",
                $left, $right
            )
        }
    };
}

#[allow(dead_code)]
#[cfg(test)]
pub(crate) trait HexFmt<H: std::fmt::UpperHex = u8>
where
    Self: AsRef<[H]>,
{
    fn hex_fmt(&self) -> String {
        format!(
            "[\n  {}\n]",
            self.as_ref()
                .chunks(8)
                .map(|c| {
                    c.iter()
                        .map(|x| format!("{x:#04X}"))
                        .collect::<Vec<String>>()
                        .join(", ")
                })
                .collect::<Vec<String>>()
                .join(",\n  ")
        )
    }

    fn hex_print(&self) {
        println!("{}", self.hex_fmt());
    }
}
#[cfg(test)]
impl<T: AsRef<[u8]>> HexFmt for T {}
