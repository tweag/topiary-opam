// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2Mul, ModPowerOf2MulAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_power_of_2_mul<T: PrimitiveUnsigned>(x: T, y: T, pow: u64) -> T {
    assert!(pow <= T::WIDTH);
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
    assert!(
        y.significant_bits() <= pow,
        "y must be reduced mod 2^pow, but {y} >= 2^{pow}"
    );
    x.wrapping_mul(y).mod_power_of_2(pow)
}

#[inline]
fn mod_power_of_2_mul_assign<T: PrimitiveUnsigned>(x: &mut T, y: T, pow: u64) {
    assert!(pow <= T::WIDTH);
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
    assert!(
        y.significant_bits() <= pow,
        "y must be reduced mod 2^pow, but {y} >= 2^{pow}"
    );
    x.wrapping_mul_assign(y);
    x.mod_power_of_2_assign(pow);
}

macro_rules! impl_mod_power_of_2_mul {
    ($t:ident) => {
        impl ModPowerOf2Mul<$t> for $t {
            type Output = $t;

            /// Multiplies two numbers modulo a third number $2^k$. The inputs must be already
            /// reduced modulo $2^k$.
            ///
            /// $f(x, y, k) = z$, where $x, y, z < 2^k$ and $xy \equiv z \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH` or if `self` or `other` are greater
            /// than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_mul#mod_power_of_2_mul).
            #[inline]
            fn mod_power_of_2_mul(self, other: $t, pow: u64) -> $t {
                mod_power_of_2_mul(self, other, pow)
            }
        }

        impl ModPowerOf2MulAssign<$t> for $t {
            /// Multiplies two numbers modulo a third number $2^k$, in place. The inputs must be
            /// already reduced modulo $2^k$.
            ///
            /// $x \gets z$, where $x, y, z < 2^k$ and $xy \equiv z \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH` or if `self` or `other` are greater
            /// than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_mul#mod_power_of_2_mul_assign).
            #[inline]
            fn mod_power_of_2_mul_assign(&mut self, other: $t, pow: u64) {
                mod_power_of_2_mul_assign(self, other, pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_mul);
