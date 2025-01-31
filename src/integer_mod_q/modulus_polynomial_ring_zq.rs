// Copyright © 2023 Marvin Beckmann
//
// This file is part of qFALL-math.
//
// qFALL-math is free software: you can redistribute it and/or modify it under
// the terms of the Mozilla Public License Version 2.0 as published by the
// Mozilla Foundation. See <https://mozilla.org/en-US/MPL/2.0/>.

//! [`ModulusPolynomialRingZq`] is the context object for
//! [`PolynomialRingZq`](super::PolynomialRingZq).
//! This implementation uses the [FLINT](https://flintlib.org/) library.

use flint_sys::fq::fq_ctx_struct;
use std::rc::Rc;

mod cmp;
mod from;
mod get;
mod ownership;
mod serialize;
mod to_string;

/// [`ModulusPolynomialRingZq`] represents the modulus object for
/// [`PolynomialRingZq`](crate::integer_mod_q::PolynomialRingZq)
///
/// Attributes
/// - `modulus`: holds the specific content, i.e. the prime `q` and f(X); it
/// holds [FLINT](https://flintlib.org/)'s [struct](fq_ctx_struct)
///
/// # Example
/// ```
/// use qfall_math::integer_mod_q::ModulusPolynomialRingZq;
/// use qfall_math::integer_mod_q::PolyOverZq;
/// use std::str::FromStr;
///
/// // initialize X^2 + 1 mod 17, i.e. a polynomial with prime modulus
/// let poly_mod = PolyOverZq::from_str("3  1 0 1 mod 17").unwrap();
/// let modulus = ModulusPolynomialRingZq::try_from(&poly_mod);
/// ```
#[derive(Debug)]
pub struct ModulusPolynomialRingZq {
    modulus: Rc<fq_ctx_struct>,
}
