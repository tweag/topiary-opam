// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_float::exhaustive::exhaustive_non_positive_finite_floats;
use malachite_float::test_util::exhaustive::exhaustive_floats_helper_helper_with_limit;

#[test]
fn test_exhaustive_non_positive_finite_floats() {
    exhaustive_floats_helper_helper_with_limit(
        100,
        exhaustive_non_positive_finite_floats(),
        &[
            "-0.0", "-1.0", "-2.0", "-1.0", "-0.5", "-1.5", "-2.0", "-1.0", "-4.0", "-1.2", "-3.0",
            "-1.5", "-0.5", "-1.8", "-2.0", "-1.0", "-0.2", "-1.1", "-2.5", "-1.2", "-0.8", "-1.4",
            "-3.0", "-1.5", "-4.0", "-1.6", "-3.5", "-1.8", "-0.5", "-1.9", "-2.0", "-1.0", "-8.0",
            "-1.06", "-2.2", "-1.12", "-0.6", "-1.19", "-2.5", "-1.25", "-6.0", "-1.3", "-2.8",
            "-1.38", "-0.8", "-1.44", "-3.0", "-1.5", "-0.2", "-1.56", "-3.2", "-1.62", "-0.9",
            "-1.7", "-3.5", "-1.75", "-4.0", "-1.81", "-3.8", "-1.88", "-0.5", "-1.94", "-2.0",
            "-1.0", "-0.1", "-1.03", "-2.1", "-1.06", "-0.56", "-1.09", "-2.2", "-1.12", "-5.0",
            "-1.16", "-2.4", "-1.19", "-0.62", "-1.22", "-2.5", "-1.25", "-0.4", "-1.28", "-2.6",
            "-1.31", "-0.7", "-1.34", "-2.8", "-1.38", "-6.0", "-1.41", "-2.9", "-1.44", "-0.75",
            "-1.47", "-3.0", "-1.5", "-8.0", "-1.53", "-3.1", "-1.56",
        ],
        &[
            "-0x0.0",
            "-0x1.0#1",
            "-0x2.0#1",
            "-0x1.0#2",
            "-0x0.8#1",
            "-0x1.8#2",
            "-0x2.0#2",
            "-0x1.0#3",
            "-0x4.0#1",
            "-0x1.4#3",
            "-0x3.0#2",
            "-0x1.8#3",
            "-0x0.8#2",
            "-0x1.c#3",
            "-0x2.0#3",
            "-0x1.0#4",
            "-0x0.4#1",
            "-0x1.2#4",
            "-0x2.8#3",
            "-0x1.4#4",
            "-0x0.c#2",
            "-0x1.6#4",
            "-0x3.0#3",
            "-0x1.8#4",
            "-0x4.0#2",
            "-0x1.a#4",
            "-0x3.8#3",
            "-0x1.c#4",
            "-0x0.8#3",
            "-0x1.e#4",
            "-0x2.0#4",
            "-0x1.0#5",
            "-0x8.0#1",
            "-0x1.1#5",
            "-0x2.4#4",
            "-0x1.2#5",
            "-0x0.a#3",
            "-0x1.3#5",
            "-0x2.8#4",
            "-0x1.4#5",
            "-0x6.0#2",
            "-0x1.5#5",
            "-0x2.c#4",
            "-0x1.6#5",
            "-0x0.c#3",
            "-0x1.7#5",
            "-0x3.0#4",
            "-0x1.8#5",
            "-0x0.4#2",
            "-0x1.9#5",
            "-0x3.4#4",
            "-0x1.a#5",
            "-0x0.e#3",
            "-0x1.b#5",
            "-0x3.8#4",
            "-0x1.c#5",
            "-0x4.0#3",
            "-0x1.d#5",
            "-0x3.c#4",
            "-0x1.e#5",
            "-0x0.8#4",
            "-0x1.f#5",
            "-0x2.0#5",
            "-0x1.00#6",
            "-0x0.2#1",
            "-0x1.08#6",
            "-0x2.2#5",
            "-0x1.10#6",
            "-0x0.9#4",
            "-0x1.18#6",
            "-0x2.4#5",
            "-0x1.20#6",
            "-0x5.0#3",
            "-0x1.28#6",
            "-0x2.6#5",
            "-0x1.30#6",
            "-0x0.a#4",
            "-0x1.38#6",
            "-0x2.8#5",
            "-0x1.40#6",
            "-0x0.6#2",
            "-0x1.48#6",
            "-0x2.a#5",
            "-0x1.50#6",
            "-0x0.b#4",
            "-0x1.58#6",
            "-0x2.c#5",
            "-0x1.60#6",
            "-0x6.0#3",
            "-0x1.68#6",
            "-0x2.e#5",
            "-0x1.70#6",
            "-0x0.c#4",
            "-0x1.78#6",
            "-0x3.0#5",
            "-0x1.80#6",
            "-0x8.0#2",
            "-0x1.88#6",
            "-0x3.2#5",
            "-0x1.90#6",
        ],
    );
}
