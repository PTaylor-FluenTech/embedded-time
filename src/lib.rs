#![cfg_attr(not(test), no_std)]
#![feature(const_fn)]
#![feature(const_trait_impl)]
#![feature(const_generics)]
#![feature(associated_type_bounds)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

pub mod duration;
// mod instant;
mod numerical_traits;
// mod ratio;

pub use duration::Duration;
pub use duration::{IntTrait, Integer};
// pub use instant::Clock;
// pub use instant::Instant;
pub use num::rational::Ratio;

/// A collection of imports that are widely useful.
///
/// Unlike the standard library, this must be explicitly imported:
///
/// ```rust,no_run
/// use embedded_time::prelude::*;
/// ```
/// The prelude may grow in minor releases. Any removals will only occur in
/// major releases.
pub mod prelude {
    // Rename traits to `_` to avoid any potential name conflicts.
    pub use crate::duration::IntTrait as _IntTrait;
    pub use crate::duration::Time as _Time;
    pub use crate::numerical_traits::NumericalDuration as _NumericalDuration;
    pub use num::Integer as _Integer;
}

// #[cfg(test)]
// mod tests {
//     use super::{prelude::*, *};
//
//     #[derive(Copy, Clone)]
//     struct MockClock;
//
//     impl Clock for MockClock {
//         type Rep = i64;
//         const PERIOD: Ratio<Self::Rep> = Ratio::<Self::Rep>::new_raw(1, 1_000_000_000);
//
//         fn now() -> Instant<Self>
//         where
//             Self: Sized,
//         {
//             Instant(Duration::<Self::Rep>::new(5_025_678_910_111, Self::PERIOD))
//         }
//     }
//
//     #[test]
//     fn it_works() {
//         let now = Instant::<MockClock>::now();
//         assert_eq!(
//             now.duration_since_epoch(),
//             5_025_678_910_111_i64.nanoseconds()
//         );
//         assert_eq!(now.duration_since_epoch().as_micros(), 5_025_678_910);
//         assert_eq!(now.duration_since_epoch().as_millis(), 5_025_678);
//         assert_eq!(format!("{}", now), "01:23:45.678");
//         assert_eq!(format!("{}", now.duration_since_epoch()), "01:23:45.678");
//     }
// }
