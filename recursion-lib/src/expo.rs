////////////////////////////////////////////////////////////////////////////////

/// Recursively calculates x raised to the power of e
///
/// Inputs:
/// - `x: f64`
/// - `e: i32`
///
/// Outputs: `f64`
/// x raised to the power of e.
///
/// Time complexity: O(e)
pub fn recursive(x: f64, e: i32) -> f64 {
    if e > 0 {
        x * recursive(x, e - 1)
    } else if e < 0 {
        1.0 / recursive(x, e * -1)
    } else {
        1.0
    }
}

//---------------------------------------------------------------------------//

/// Iteratively calculates x raised to the power of e
///
/// Inputs:
/// - `x: f64`
/// - `e: i32`
///
/// Outputs: `f64`
/// x raised to the power of e.
///
/// Time complexity: O(e)
pub fn iterative(x: f64, e: i32) -> f64 {
    let mut res = 1.0;

    for _ in 0..e.abs() {
        res = res * x;
    }

    if e < 0 {
        1.0 / res
    } else {
        res
    }
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

    fn common(func: fn(f64, i32) -> f64) {
        for i in -50..100 {
            for n in -50..50 {
                for m in -10..10 {
                    let x = (n as f64) / (m as f64);

                    let real = func(x, i);
                    let expected = x.powi(i);

                    println!("{} >< {}", real, expected);

                    let real = real.to_le_bytes();
                    let expected = expected.to_le_bytes();

                    println!("{:?} >< {:?}", real, expected);

                    let mut diff = 0;
                    for i in 0..8 {
                        if real[i] != expected[i] {
                            diff += 1;
                        }
                    }

                    let tolerance = 3;

                    assert!(diff < tolerance);
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
