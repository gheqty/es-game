//! Minimal f64 transcendentals for `no_std`.
//!
//! `core` provides `f64` arithmetic but not `sin`/`cos`/`sqrt`/`floor`.
//! The game only needs them for visual bobbing, water shimmer and
//! distance comparisons, so good-enough approximations are fine.

#[inline]
pub fn abs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

#[inline]
pub fn floor(x: f64) -> f64 {
    let i = x as i64;
    let f = i as f64;
    if x < f { f - 1.0 } else { f }
}

/// Newton-Raphson square root. Clamps negatives to 0.
pub fn sqrt(x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    let mut z = x;
    let mut i = 0;
    while i < 16 {
        z = 0.5 * (z + x / z);
        i += 1;
    }
    z
}

/// Reduce angle to `[-pi, pi]`.
fn reduce(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    let two_pi = 2.0 * pi;
    let mut y = x - two_pi * floor(x / two_pi);
    if y > pi {
        y -= two_pi;
    }
    y
}

/// Taylor-series sine, accurate enough for visual animation.
pub fn sin(x: f64) -> f64 {
    let x = reduce(x);
    let x2 = x * x;
    let x3 = x2 * x;
    let x5 = x3 * x2;
    let x7 = x5 * x2;
    x - x3 / 6.0 + x5 / 120.0 - x7 / 5040.0
}

pub fn cos(x: f64) -> f64 {
    sin(x + core::f64::consts::FRAC_PI_2)
}
