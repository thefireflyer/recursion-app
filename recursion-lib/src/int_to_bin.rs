////////////////////////////////////////////////////////////////////////////////

use tailcall::tailcall;

////////////////////////////////////////////////////////////////////////////////

/// Recursively writes `x` in binary
///
/// Inputs:
/// - `x: i64`
///
/// Outputs: `[bool; 64]`
/// The integer `x` expressed in binary, with 64 bits (or bools).
/// The last bit determines whether a number is positive or negative.
///
/// Time complexity: O(1)*
///
/// * It should be noted that while this function takes constant time, it is
/// expensive internally due to repeated math operations and may be out
/// performed by more bit-efficient algorithms.
///
pub fn recursive(x: i64) -> [bool; 64] {
    #[tailcall]
    fn inner(x: i64, r: &mut [bool; 64], i: usize) {
        let y = x - 2i64.pow(i.try_into().unwrap());
        if y >= 0 {
            r[i] = true;
            if i > 0 {
                inner(x - 2i64.pow(i.try_into().unwrap()), r, i - 1)
            }
        } else if i > 0 {
            inner(x, r, i - 1)
        }
    }

    if x == 0 {
        [false; 64]
    } else {
        let mut res = [false; 64];
        res[63] = x.signum() < 0;
        inner(x.abs(), &mut res, 62);
        res
    }
}

//---------------------------------------------------------------------------//

/// Iteratively writes `x` in binary
///
/// Inputs:
/// - `x: i64`
///
/// Outputs: `[bool; 64]`
/// The integer `x` expressed in binary, with 64 bits (or bools).
/// The last bit determines whether a number is positive or negative.
///
/// Time complexity: O(1)*
///
/// * It should be noted that while this function takes constant time, it is
/// expensive internally due to repeated math operations and may be out
/// performed by more bit-efficient algorithms.
///
pub fn iterative(mut x: i64) -> [bool; 64] {
    let mut res = [false; 64];

    //  0 -> 000...0000 --- 0 --- 0 --- 0 ---|
    //  1 -> 000...0001 --- 1 --- 0 --- 0 ---|
    //  2 -> 000...0010 --- 0 --- 1 --- 0 ---|
    //  3 -> 000...0011 --- 1 --- 1 --- 0 ---|
    //  4 -> 000...0100 --- 0 --- 0 --- 1 ---|
    //  5 -> 000...0101 --- 1 --- 0 --- 1 ---|
    //  6 -> 000...0110 --- 0 --- 1 --- 1 ---|
    //  7 -> 000...0111 --- 1 --- 1 --- 1 ---|
    //  8 -> 000...1000 --- 0 --- 0 --- 0 ---|
    //  9 -> 000...1001 --- 1 --- 0 --- 0 ---|
    // 10 -> 000...1010 --- 0 --- 1 --- 0 ---|
    // .. .. ...   .... ...   ...   ...   ...|
    // .. .. ...   .... ... 1 ... 2 ... 4 ...|

    if x.signum() < 0 {
        res[63] = true;
        x = x.abs();
    }

    for i in (0..63usize).rev() {
        let y = x - 2i64.pow(i.try_into().unwrap());
        if y >= 0 {
            res[i] = true;
            x = y;
        }
    }

    res
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    #[test]
    fn iterative() {
        common(super::iterative);
    }

    #[test]
    fn recursive() {
        common(super::recursive);
    }

    fn common(func: fn(i64) -> [bool; 64]) {
        let mut ans = [false; 64];
        assert_eq!(func(0), ans);

        ans[0] = true;
        assert_eq!(func(1), ans);

        ans[0] = false;
        ans[1] = true;
        assert_eq!(func(2), ans);

        ans[0] = true;
        ans[1] = true;
        assert_eq!(func(3), ans);

        ans[0] = false;
        ans[1] = false;
        ans[2] = true;
        assert_eq!(func(4), ans);

        ans[0] = true;
        ans[1] = false;
        ans[2] = true;
        assert_eq!(func(5), ans);

        ans[0] = false;
        ans[1] = true;
        ans[2] = true;
        assert_eq!(func(6), ans);

        ans[0] = true;
        ans[1] = true;
        ans[2] = true;
        assert_eq!(func(7), ans);

        ans[0] = false;
        ans[1] = false;
        ans[2] = false;
        ans[3] = true;
        assert_eq!(func(8), ans);

        ans[0] = true;
        ans[1] = false;
        ans[2] = false;
        ans[3] = true;
        assert_eq!(func(9), ans);

        ans[0] = false;
        ans[1] = true;
        ans[2] = false;
        ans[3] = true;
        assert_eq!(func(10), ans);

        ans[0] = true;
        ans[1] = true;
        ans[2] = false;
        ans[3] = true;
        assert_eq!(func(11), ans);

        ans[0] = true;
        ans[1] = true;
        ans[2] = false;
        ans[3] = true;
        ans[63] = true;
        assert_eq!(func(-11), ans);

        // assert_eq!(func(i64::MIN), [true; 64]);
    }
}

////////////////////////////////////////////////////////////////////////////////
