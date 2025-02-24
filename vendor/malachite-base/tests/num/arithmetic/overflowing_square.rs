// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};

#[test]
fn test_overflowing_square() {
    fn test<T: PrimitiveInt>(x: T, out: T, overflow: bool) {
        assert_eq!(x.overflowing_square(), (out, overflow));

        let mut x = x;
        assert_eq!(x.overflowing_square_assign(), overflow);
        assert_eq!(x, out);
    }
    test::<u8>(0, 0, false);
    test::<i16>(1, 1, false);
    test::<u32>(2, 4, false);
    test::<i64>(3, 9, false);
    test::<u128>(10, 100, false);
    test::<isize>(123, 15129, false);
    test::<u32>(1000, 1000000, false);

    test::<i16>(-1, 1, false);
    test::<i32>(-2, 4, false);
    test::<i64>(-3, 9, false);
    test::<i128>(-10, 100, false);
    test::<isize>(-123, 15129, false);
    test::<i32>(-1000, 1000000, false);

    test::<u16>(1000, 16960, true);
    test::<i16>(-1000, 16960, true);
}

fn overflowing_square_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        let mut square = x;
        let overflow = square.overflowing_square_assign();
        assert_eq!((square, overflow), x.overflowing_square());
        assert_eq!((square, overflow), x.overflowing_pow(2));
        assert_eq!(x.wrapping_square(), square);
        assert_eq!(x.checked_square().is_none(), overflow);
        if !overflow {
            assert_eq!(square, x.square());
        }
    });
}

fn overflowing_square_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|x| {
        let mut square = x;
        let overflow = square.overflowing_square_assign();
        assert_eq!((square, overflow), x.overflowing_square());
        assert_eq!((square, overflow), x.overflowing_pow(2));
        assert_eq!(x.wrapping_square(), square);
        assert_eq!(x.checked_square().is_none(), overflow);
        if !overflow {
            assert_eq!(square, x.square());
        }
    });
}

#[test]
fn overflowing_square_properties() {
    apply_fn_to_unsigneds!(overflowing_square_properties_helper_unsigned);
    apply_fn_to_signeds!(overflowing_square_properties_helper_signed);
}
