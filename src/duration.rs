//! Duration types/units

use crate::{
    fixed_point::{self, FixedPoint},
    rate,
    time_int::TimeInt,
    ConversionError, Fraction,
};
use core::{convert::TryFrom, mem::size_of, prelude::v1::*};
pub use fixed_point::FixedPoint as _;
use num::{CheckedDiv, CheckedMul};
pub use units::*;

/// An unsigned, fixed-point duration type
///
/// Each implementation defines an _integer_ type and a [`Fraction`] _scaling factor_.
///
/// # Constructing a duration
///
/// ```rust
/// # use embedded_time::{ duration::*};
/// #
/// assert_eq!(23_u32.milliseconds(), Milliseconds(23_u32));
/// ```
///
/// ## From a [`Generic`] `Duration`
///
/// ### Examples
///
/// ```rust
/// # use embedded_time::{Fraction, duration::*, duration::Generic};
/// # use core::convert::{TryFrom, TryInto};
/// #
/// assert_eq!(
///     Seconds::<u64>::try_from(Generic::new(2_000_u32, Fraction::new(1, 1_000))),
///     Ok(Seconds(2_u64))
/// );
///
/// // TryInto also works
/// assert_eq!(
///     Generic::new(2_000_u64, Fraction::new(1,1_000)).try_into(),
///     Ok(Seconds(2_u64))
/// );
/// ```
///
/// ### Errors
///
/// Failure will only occur if the provided value does not fit in the selected destination type.
///
/// ---
///
/// [`ConversionError::Overflow`] : The conversion of the _scaling factor_ causes an overflow.
///
/// ```rust
/// # use embedded_time::{Fraction, duration::*, duration::Generic, ConversionError};
/// # use core::convert::TryFrom;
/// #
/// assert_eq!(
///     Seconds::<u32>::try_from(Generic::new(u32::MAX, Fraction::new(10,1))),
///     Err(ConversionError::Overflow)
/// );
/// ```
///
/// ---
///
/// [`ConversionError::ConversionFailure`] : The _integer_ conversion to that of the
/// destination type fails.
///
/// ```rust
/// # use embedded_time::{Fraction, duration::*, duration::Generic, ConversionError};
/// # use core::convert::TryFrom;
/// #
/// assert_eq!(
///     Seconds::<u32>::try_from(Generic::new(u32::MAX as u64 + 1, Fraction::new(1,1))),
///     Err(ConversionError::ConversionFailure)
/// );
/// ```
///
/// # Get the integer part
///
/// ```rust
/// # use embedded_time::{ duration::*};
/// #
/// assert_eq!(Milliseconds(23_u32).integer(), &23_u32);
/// ```
///
/// # Formatting
///
/// Just forwards the underlying integer to [`core::fmt::Display::fmt()`]
///
/// ```rust
/// # use embedded_time::{ duration::*};
/// #
/// assert_eq!(format!("{}", Seconds(123_u32)), "123");
/// ```
///
/// # Getting H:M:S.MS... Components
///
/// ```rust
/// # use embedded_time::{ duration::*};
/// #
/// let duration = 38_238_479_u32.microseconds();
/// let hours = Hours::<u32>::try_convert_from(duration).unwrap();
/// let minutes = Minutes::<u32>::try_convert_from(duration).unwrap() % Hours(1_u32);
/// let seconds = Seconds::<u32>::try_convert_from(duration).unwrap() % Minutes(1_u32);
/// let milliseconds = Milliseconds::<u32>::try_convert_from(duration).unwrap() % Seconds(1_u32);
/// // ...
/// ```
///
/// # Converting to `core` types
///
/// [`core::time::Duration`]
///
/// ## Examples
///
/// ```rust
/// # use embedded_time::duration::*;
/// # use core::convert::TryFrom;
/// #
/// let core_duration = core::time::Duration::try_from(2_569_u32.milliseconds()).unwrap();
/// assert_eq!(core_duration.as_secs(), 2);
/// assert_eq!(core_duration.subsec_nanos(), 569_000_000);
/// ```
///
/// ```rust
/// # use embedded_time::duration::*;
/// # use core::convert::TryInto;
/// #
/// let core_duration: core::time::Duration = 2_569_u32.milliseconds().try_into().unwrap();
/// assert_eq!(core_duration.as_secs(), 2);
/// assert_eq!(core_duration.subsec_nanos(), 569_000_000);
/// ```
///
/// # Converting from `core` types
///
/// [`core::time::Duration`]
///
/// ## Examples
///
/// ```rust
/// # use embedded_time::{ duration::*};
/// # use core::convert::TryFrom;
/// #
/// let core_duration = core::time::Duration::new(5, 730023852);
/// assert_eq!(Milliseconds::<u32>::try_from(core_duration), Ok(5_730.milliseconds()));
/// ```
///
/// ```rust
/// # use embedded_time::{ duration::*};
/// # use core::convert::TryInto;
/// #
/// let duration: Result<Milliseconds<u32>, _> = core::time::Duration::new(5, 730023852).try_into();
/// assert_eq!(duration, Ok(5_730.milliseconds()));
/// ```
///
/// ## Errors
///
/// [`ConversionError::ConversionFailure`] : The duration doesn't fit in the type specified
///
/// ```rust
/// # use embedded_time::{ duration::*, ConversionError};
/// # use core::convert::{TryFrom, TryInto};
/// #
/// assert_eq!(
///     Milliseconds::<u32>::try_from(
///         core::time::Duration::from_millis((u32::MAX as u64) + 1)
///     ),
///     Err(ConversionError::ConversionFailure)
/// );
///
/// let duration: Result<Milliseconds<u32>, _> =
///     core::time::Duration::from_millis((u32::MAX as u64) + 1).try_into();
/// assert_eq!(duration, Err(ConversionError::ConversionFailure));
/// ```
///
/// # Add/Sub
///
/// The result of the operation is the LHS type
///
/// ## Examples
///
/// ```rust
/// # use embedded_time::{ duration::*};
/// #
/// assert_eq!((Milliseconds(2_001_u32) - Seconds(1_u32)),
///     Milliseconds(1_001_u32));
///
/// assert_eq!((Milliseconds(1_u32) + Seconds(1_u32)),
///     Milliseconds(1_001_u32));
/// ```
///
/// ## Panics
///
/// The same reason the integer operation would panic. Namely, if the result overflows the type.
///
/// ```rust,should_panic
/// # use embedded_time::{ duration::*};
/// #
/// let _ = Seconds(u32::MAX) + Seconds(1_u32);
/// ```
///
/// # Comparisons
///
/// ```rust
/// # use embedded_time::{ duration::*};
/// #
/// assert_eq!(Seconds(2_u32), Milliseconds(2_000_u32));
/// assert_ne!(Seconds(2_u32), Milliseconds(2_001_u32));
///
/// assert!(Seconds(2_u32) < Milliseconds(2_001_u32));
/// assert!(Seconds(2_u32) > Milliseconds(1_999_u32));
/// ```
///
/// # Remainder
///
/// ```rust
/// # use embedded_time::{ duration::*};
/// #
/// assert_eq!(Minutes(62_u32) % Hours(1_u32), Minutes(2_u32));
/// ```
pub trait Duration: Sized + Copy {
    /// Construct a `Generic` `Duration` from an _named_ `Duration` (eg.
    /// [`Milliseconds`](units::Milliseconds))
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, duration::*, duration::{Generic, Duration}};
    /// # use core::convert::{TryFrom, TryInto};
    /// #
    /// assert_eq!(Seconds(2_u64).to_generic(Fraction::new(1, 2_000)),
    ///     Ok(Generic::new(4_000_u32, Fraction::new(1, 2_000))));
    /// ```
    ///
    /// # Errors
    ///
    /// Failure will only occur if the provided value does not fit in the selected destination type.
    ///
    /// ---
    ///
    /// [`ConversionError::Overflow`] : The conversion of the _scaling factor_ causes an overflow.
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, duration::*, duration::{Duration, Generic}, ConversionError};
    /// # use core::convert::TryFrom;
    /// #
    /// assert_eq!(Seconds(u32::MAX).to_generic::<u32>(Fraction::new(1, 2)),
    ///     Err(ConversionError::Overflow));
    /// ```
    ///
    /// ---
    ///
    /// [`ConversionError::ConversionFailure`] : The integer conversion to that of the destination
    /// type fails.
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, duration::*, duration::{Duration, Generic}, ConversionError};
    /// # use core::convert::TryFrom;
    /// #
    /// assert_eq!(Seconds(u32::MAX as u64 + 1).to_generic::<u32>(Fraction::new(1, 1)),
    ///     Err(ConversionError::ConversionFailure));
    /// ```
    fn to_generic<DestInt: TimeInt>(
        self,
        scaling_factor: Fraction,
    ) -> Result<Generic<DestInt>, ConversionError>
    where
        Self: FixedPoint,
        DestInt: TryFrom<Self::T>,
    {
        Ok(Generic::<DestInt>::new(
            self.into_ticks(scaling_factor)?,
            scaling_factor,
        ))
    }

    /// Convert to _named_ [`Rate`](rate::Rate)
    ///
    /// (the duration is equal to the reciprocal of the rate)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{duration::{Duration, units::*}, rate::*};
    /// #
    /// assert_eq!(
    ///     Microseconds(500_u32).to_rate(),
    ///     Ok(Kilohertz(2_u32))
    /// );
    /// ```
    ///
    /// # Errors
    ///
    /// Failure will only occur if the provided value does not fit in the selected destination type.
    ///
    /// ---
    ///
    /// [`ConversionError::Overflow`] : The conversion of the _scaling factor_ causes an overflow.
    ///
    /// ```rust
    /// # use embedded_time::{duration::{Duration, units::*}, rate::*, ConversionError};
    /// #
    /// assert_eq!(
    ///     Hours(u32::MAX).to_rate::<Megahertz<u32>>(),
    ///     Err(ConversionError::Overflow)
    /// );
    /// ```
    ///
    /// ---
    ///
    /// [`ConversionError::DivByZero`] : The rate is `0`, therefore the reciprocal is undefined.
    ///
    /// ```rust
    /// # use embedded_time::{duration::{Duration, units::*}, rate::*, ConversionError};
    /// #
    /// assert_eq!(
    ///     Seconds(0_u32).to_rate::<Hertz<u32>>(),
    ///     Err(ConversionError::DivByZero)
    /// );
    /// ```
    fn to_rate<Rate: rate::Rate>(&self) -> Result<Rate, ConversionError>
    where
        Rate: FixedPoint,
        Self: FixedPoint,
        Rate::T: TryFrom<Self::T>,
    {
        let conversion_factor = Self::SCALING_FACTOR
            .checked_mul(&Rate::SCALING_FACTOR)?
            .recip();

        if size_of::<Self::T>() >= size_of::<Rate::T>() {
            fixed_point::from_ticks(
                Self::T::from(*conversion_factor.numerator())
                    .checked_div(
                        &self
                            .integer()
                            .checked_mul(&Self::T::from(*conversion_factor.denominator()))
                            .ok_or(ConversionError::Overflow)?,
                    )
                    .ok_or(ConversionError::DivByZero)?,
                Rate::SCALING_FACTOR,
            )
        } else {
            fixed_point::from_ticks(
                Rate::T::from(*conversion_factor.numerator())
                    .checked_div(
                        &Rate::T::try_from(*self.integer())
                            .ok()
                            .unwrap()
                            .checked_mul(&Rate::T::from(*conversion_factor.denominator()))
                            .ok_or(ConversionError::Overflow)?,
                    )
                    .ok_or(ConversionError::DivByZero)?,
                Rate::SCALING_FACTOR,
            )
        }
    }
}

/// The `Generic` `Duration` type allows arbitrary _scaling factor_s to be used without having to
/// impl FixedPoint.
///
/// The purpose of this type is to allow a simple `Duration` that can be defined at run-time. It
/// does this by replacing the `const` _scaling factor_ with a struct field.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Generic<T> {
    integer: T,
    scaling_factor: Fraction,
}

impl<T> Generic<T> {
    /// Constructs a new (ram) fixed-point `Generic` `Duration` value
    pub const fn new(integer: T, scaling_factor: Fraction) -> Self {
        Self {
            integer,
            scaling_factor,
        }
    }

    /// Returns the _integer_ value
    pub const fn integer(&self) -> &T {
        &self.integer
    }

    /// Returns the _scaling factor_ [`Fraction`] value
    pub const fn scaling_factor(&self) -> &Fraction {
        &self.scaling_factor
    }
}

impl<T: TimeInt> Duration for Generic<T> {}

/// Duration units
pub mod units {
    use super::*;
    use crate::{
        fixed_point::{self, FixedPoint},
        fraction::Fraction,
        time_int::TimeInt,
        ConversionError,
    };
    use core::{
        cmp,
        convert::{TryFrom, TryInto},
        fmt::{self, Formatter},
        ops,
    };
    pub use Extensions as _;

    macro_rules! impl_duration {
        ( $name:ident, ($numer:expr, $denom:expr) ) => {
            /// A duration unit type
            #[derive(Copy, Clone, Eq, Ord, Hash, Debug, Default)]
            pub struct $name<T: TimeInt = u32>(pub T);

            impl<T: TimeInt> $name<T> {
                /// See [Constructing a duration](../trait.Duration.html#constructing_a_duration)
                pub fn new(value: T) -> Self {
                    Self(value)
                }
            }

            impl<T: TimeInt> Duration for $name<T> {}

            impl<T: TimeInt> FixedPoint for $name<T> {
                type T = T;
                const SCALING_FACTOR: Fraction = Fraction::new($numer, $denom);

                fn new(value: Self::T) -> Self {
                    Self(value)
                }

                fn integer(&self) -> &Self::T {
                    &self.0
                }
            }

            impl<T: TimeInt> fmt::Display for $name<T> {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    fmt::Display::fmt(&self.0, f)
                }
            }

            impl<T: TimeInt, Rhs: Duration> ops::Add<Rhs> for $name<T>
            where
                Rhs: FixedPoint,
                T: TryFrom<Rhs::T>,
            {
                type Output = Self;

                fn add(self, rhs: Rhs) -> Self::Output {
                    <Self as FixedPoint>::add(self, rhs)
                }
            }

            impl<T: TimeInt, Rhs: Duration> ops::Sub<Rhs> for $name<T>
            where
                T: TryFrom<Rhs::T>,
                Rhs: FixedPoint,
            {
                type Output = Self;

                fn sub(self, rhs: Rhs) -> Self::Output {
                    <Self as FixedPoint>::sub(self, rhs)
                }
            }

            impl<T: TimeInt, Rhs: Duration> ops::Rem<Rhs> for $name<T>
            where
                T: TryFrom<Rhs::T>,
                Rhs: FixedPoint,
            {
                type Output = Self;

                fn rem(self, rhs: Rhs) -> Self::Output {
                    <Self as FixedPoint>::rem(self, rhs)
                }
            }

            impl<T: TimeInt, Rhs: Duration> cmp::PartialEq<Rhs> for $name<T>
            where
                T: TryFrom<Rhs::T>,
                Rhs: FixedPoint,
                Rhs::T: TryFrom<T>,
            {
                fn eq(&self, rhs: &Rhs) -> bool {
                    <Self as FixedPoint>::eq(self, rhs)
                }
            }

            impl<T: TimeInt, Rhs: Duration> PartialOrd<Rhs> for $name<T>
            where
                T: TryFrom<Rhs::T>,
                Rhs: FixedPoint,
                Rhs::T: TryFrom<T>,
            {
                fn partial_cmp(&self, rhs: &Rhs) -> Option<core::cmp::Ordering> {
                    <Self as FixedPoint>::partial_cmp(self, rhs)
                }
            }

            impl<SourceInt: TimeInt, DestInt: TimeInt> TryFrom<Generic<SourceInt>>
                for $name<DestInt>
            where
                DestInt: TryFrom<SourceInt>,
            {
                type Error = ConversionError;

                /// See [Constructing a duration > From a Generic
                /// Duration](../trait.Duration.html#from-a-generic-duration)
                fn try_from(generic_duration: Generic<SourceInt>) -> Result<Self, Self::Error> {
                    fixed_point::from_ticks(
                        generic_duration.integer,
                        generic_duration.scaling_factor,
                    )
                }
            }

            impl<T: TimeInt> From<$name<T>> for Generic<T> {
                // TODO: documentation
                fn from(duration: $name<T>) -> Self {
                    Self::new(*duration.integer(), $name::<T>::SCALING_FACTOR)
                }
            }
        };

        ( $name:ident, ($numer:expr, $denom:expr), ge_secs ) => {
            impl_duration![$name, ($numer, $denom)];

            impl<T: TimeInt> TryFrom<$name<T>> for core::time::Duration {
                type Error = ConversionError;

                fn try_from(duration: $name<T>) -> Result<Self, Self::Error> {
                    let seconds = Seconds::<u64>::try_convert_from(duration)?;
                    Ok(Self::from_secs(*seconds.integer()))
                }
            }

            impl<T: TimeInt> TryFrom<core::time::Duration> for $name<T> {
                type Error = ConversionError;

                fn try_from(core_duration: core::time::Duration) -> Result<Self, Self::Error> {
                    let seconds = Seconds(core_duration.as_secs());
                    Self::try_convert_from(seconds)
                }
            }
        };
        ( $name:ident, ($numer:expr, $denom:expr), $from_core_dur:ident, $as_core_dur:ident ) => {
            impl_duration![$name, ($numer, $denom)];

            impl<T: TimeInt> TryFrom<$name<T>> for core::time::Duration {
                type Error = ConversionError;

                fn try_from(duration: $name<T>) -> Result<Self, Self::Error> {
                    Ok(Self::$from_core_dur((*duration.integer()).into()))
                }
            }

            impl<T: TimeInt> TryFrom<core::time::Duration> for $name<T> {
                type Error = ConversionError;

                fn try_from(core_duration: core::time::Duration) -> Result<Self, Self::Error> {
                    Ok(Self(
                        core_duration
                            .$as_core_dur()
                            .try_into()
                            .map_err(|_| ConversionError::ConversionFailure)?,
                    ))
                }
            }
        };
    }
    impl_duration![Hours, (3600, 1), ge_secs];
    impl_duration![Minutes, (60, 1), ge_secs];
    impl_duration![Seconds, (1, 1), ge_secs];
    impl_duration![Milliseconds, (1, 1_000), from_millis, as_millis];
    impl_duration![Microseconds, (1, 1_000_000), from_micros, as_micros];
    impl_duration![Nanoseconds, (1, 1_000_000_000), from_nanos, as_nanos];

    /// Create time-based extensions from primitive numeric types.
    ///
    /// Basic construction of time-based values.
    ///
    /// ```rust
    /// # use embedded_time::duration::*;
    /// #
    /// assert_eq!(5_u32.nanoseconds(), Nanoseconds(5_u32));
    /// assert_eq!(5_u32.microseconds(), Microseconds(5_u32));
    /// assert_eq!(5_u32.milliseconds(), Milliseconds(5_u32));
    /// assert_eq!(5_u32.seconds(), Seconds(5_u32));
    /// assert_eq!(5_u32.minutes(), Minutes(5_u32));
    /// assert_eq!(5_u32.hours(), Hours(5_u32));
    /// ```
    pub trait Extensions: TimeInt {
        /// nanoseconds
        fn nanoseconds(self) -> Nanoseconds<Self> {
            Nanoseconds::new(self)
        }
        /// microseconds
        fn microseconds(self) -> Microseconds<Self> {
            Microseconds::new(self)
        }
        /// milliseconds
        fn milliseconds(self) -> Milliseconds<Self> {
            Milliseconds::new(self)
        }
        /// seconds
        fn seconds(self) -> Seconds<Self> {
            Seconds::new(self)
        }
        /// minutes
        fn minutes(self) -> Minutes<Self> {
            Minutes::new(self)
        }
        /// hours
        fn hours(self) -> Hours<Self> {
            Hours::new(self)
        }
    }

    impl Extensions for u32 {}
    impl Extensions for u64 {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{duration, duration::*, rate::*};
    use core::convert::TryInto;

    #[test]
    fn try_from_generic() {
        assert_eq!(
            Seconds::try_from(duration::Generic::new(246_u32, Fraction::new(1, 2))),
            Ok(Seconds(123_u32))
        );
    }

    #[test]
    fn to_generic() {
        assert_eq!(
            Seconds(123_u32).to_generic(Fraction::new(1, 2)),
            Ok(duration::Generic::new(246_u32, Fraction::new(1, 2)))
        );
    }

    #[test]
    fn try_into_generic_err() {
        assert_eq!(
            Seconds(u32::MAX).to_generic::<u32>(Fraction::new(1, 2)),
            Err(ConversionError::Overflow)
        );
    }

    #[test]
    fn get_generic_integer() {
        let generic = duration::Generic::new(246_u32, Fraction::new(1, 2));
        assert_eq!(generic.integer(), &246_u32);
    }

    #[test]
    fn check_for_overflows() {
        let mut time = 1_u64;
        time *= 60;
        assert_eq!(Minutes(time), Hours(1_u32));
        time *= 60;
        assert_eq!(Seconds(time), Hours(1_u32));
        time *= 1000;
        assert_eq!(Milliseconds(time), Hours(1_u32));
        time *= 1000;
        assert_eq!(Microseconds(time), Hours(1_u32));
        time *= 1000;
        assert_eq!(Nanoseconds(time), Hours(1_u32));
    }

    #[test]
    fn remainder() {
        assert_eq!(Minutes(62_u32) % Hours(1_u32), Minutes(2_u32));
        assert_eq!(Minutes(62_u32) % Milliseconds(1_u32), Minutes(0_u32));
        assert_eq!(Minutes(62_u32) % Minutes(60_u32), Minutes(2_u32));
    }

    #[test]
    fn convert_to_rate() {
        assert_eq!(Milliseconds(500_u32).to_rate(), Ok(Hertz(2_u32)));

        assert_eq!(Microseconds(500_u32).to_rate(), Ok(Kilohertz(2_u32)));
    }

    #[test]
    fn convert_from_core_duration() {
        let core_duration = core::time::Duration::from_nanos(5_025_678_901_234);
        assert_eq!(
            core_duration.try_into(),
            Ok(Nanoseconds::<u64>(5_025_678_901_234))
        );
        assert_eq!(
            core_duration.try_into(),
            Ok(Microseconds::<u64>(5_025_678_901))
        );
        assert_eq!(core_duration.try_into(), Ok(Milliseconds::<u32>(5_025_678)));
        assert_eq!(core_duration.try_into(), Ok(Seconds::<u32>(5_025)));
        assert_eq!(core_duration.try_into(), Ok(Minutes::<u32>(83)));
        assert_eq!(core_duration.try_into(), Ok(Hours::<u32>(1)));
    }

    #[test]
    fn convert_to_core_duration() {
        assert_eq!(
            Nanoseconds(123_u32).try_into(),
            Ok(core::time::Duration::from_nanos(123))
        );
        assert_eq!(
            Microseconds(123_u32).try_into(),
            Ok(core::time::Duration::from_micros(123))
        );
        assert_eq!(
            Milliseconds(123_u32).try_into(),
            Ok(core::time::Duration::from_millis(123))
        );
        assert_eq!(
            Seconds(123_u32).try_into(),
            Ok(core::time::Duration::from_secs(123))
        );
        assert_eq!(
            Minutes(123_u32).try_into(),
            Ok(core::time::Duration::from_secs(123 * 60))
        );
        assert_eq!(
            Hours(123_u32).try_into(),
            Ok(core::time::Duration::from_secs(123 * 3600))
        );
    }

    #[test]
    fn duration_scaling() {
        assert_eq!(1_u32.nanoseconds(), 1_u32.nanoseconds());
        assert_eq!(1_u32.microseconds(), 1_000_u32.nanoseconds());
        assert_eq!(1_u32.milliseconds(), 1_000_000_u32.nanoseconds());
        assert_eq!(1_u32.seconds(), 1_000_000_000_u32.nanoseconds());
        assert_eq!(1_u32.minutes(), 60_000_000_000_u64.nanoseconds());
        assert_eq!(1_u32.hours(), 3_600_000_000_000_u64.nanoseconds());
    }
}
