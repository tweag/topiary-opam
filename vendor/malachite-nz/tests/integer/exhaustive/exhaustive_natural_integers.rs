// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::exhaustive::exhaustive_natural_integers;

#[test]
fn test_exhaustive_natural_integers() {
    assert_eq!(
        exhaustive_natural_integers()
            .take(20)
            .collect_vec()
            .to_debug_string(),
        "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]"
    );
}
