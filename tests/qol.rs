//! This file is included for use in tests only, hence it's in the tests folder. It gives some
//! useful macros that make writing tests easier

/// Fuzzy equality macro
///
/// If you need to check the returned result and expected result but are also expecting some
/// fuzziness on the values, such as when you use trig somewhere, use this
/// ```
/// mod qol;
///
/// fuzzy_eq!(
///     Matrix::rotation2(Angle::Degrees(180.),
///     Matrix::new[[-1., 0.], [0., -1.]]
/// )
/// ```
#[macro_export]
macro_rules! fuzzy_eq {
    ($l:expr, $r:expr) => { assert!($l.fuzzy_eq($r), "Assertion failed: left != right\nLeft:  {:?}\nRight: {:?}", $l, $r) };
}
