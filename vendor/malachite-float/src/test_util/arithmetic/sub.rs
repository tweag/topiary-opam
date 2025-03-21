// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::common::rug_float_significant_bits;
use crate::Float;
use crate::InnerFloat::{Infinity, NaN, Zero};
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;
use std::cmp::max;
use std::cmp::Ordering::{self, *};

pub fn rug_sub_prec_round(
    x: &rug::Float,
    y: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut sum = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = sum.assign_round(x - y, rm);
    (sum, o)
}

#[inline]
pub fn rug_sub_round(x: &rug::Float, y: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_sub_prec_round(
        x,
        y,
        max(rug_float_significant_bits(x), rug_float_significant_bits(y)),
        rm,
    )
}

#[inline]
pub fn rug_sub_prec(x: &rug::Float, y: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_sub_prec_round(x, y, prec, Round::Nearest)
}

pub fn rug_sub(x: &rug::Float, y: &rug::Float) -> rug::Float {
    rug_sub_prec_round(
        x,
        y,
        max(rug_float_significant_bits(x), rug_float_significant_bits(y)),
        Round::Nearest,
    )
    .0
}

pub fn rug_sub_rational_prec_round(
    x: &rug::Float,
    y: &rug::Rational,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut sum = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = sum.assign_round(x - y, rm);
    (sum, o)
}

pub fn rug_sub_rational_round(
    x: &rug::Float,
    y: &rug::Rational,
    rm: Round,
) -> (rug::Float, Ordering) {
    rug_sub_rational_prec_round(x, y, rug_float_significant_bits(x), rm)
}

pub fn rug_sub_rational_prec(
    x: &rug::Float,
    y: &rug::Rational,
    prec: u64,
) -> (rug::Float, Ordering) {
    rug_sub_rational_prec_round(x, y, prec, Round::Nearest)
}

pub fn rug_sub_rational(x: &rug::Float, y: &rug::Rational) -> rug::Float {
    rug_sub_rational_prec_round(x, y, rug_float_significant_bits(x), Round::Nearest).0
}

pub fn sub_rational_prec_round_naive(
    x: Float,
    y: Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match (x, y) {
        (x @ Float(NaN | Infinity { .. }), _) => (x, Equal),
        (float_negative_zero!(), y) => {
            if y == 0u32 {
                (float_negative_zero!(), Equal)
            } else {
                Float::from_rational_prec_round(-y, prec, rm)
            }
        }
        (float_zero!(), y) => Float::from_rational_prec_round(-y, prec, rm),
        (x, y) => {
            let (mut sum, o) =
                Float::from_rational_prec_round(Rational::exact_from(x) - y, prec, rm);
            if rm == Floor && sum == 0u32 {
                sum.neg_assign();
            }
            (sum, o)
        }
    }
}
