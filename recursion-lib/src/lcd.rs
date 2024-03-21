////////////////////////////////////////////////////////////////////////////////

use tailcall::tailcall;

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
/// A basic expression.
/// It consists of:
/// - A fraction
/// - A co-efficient
pub struct Expr {
    /// The co-efficient
    pub c: i64,
    /// The nominator of the fraction
    pub n: i64,
    /// The denominator of the fraction
    pub d: i64,
}

//---------------------------------------------------------------------------//

impl Expr {
    pub fn new(c: i64, n: i64, d: i64) -> Self {
        Self { c, n, d }
    }
}

//---------------------------------------------------------------------------//

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}({}/{})", self.c, self.n, self.d))
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Recursively computes the Lowest Common Denominator of a fraction. Kind of.
/// Really it does several things.
/// - It converts improper fractions to proper fractions.
///     - (i.e. 11/5 -> 2(1/5) )
/// - It simplifies integer fractions.
///     - (i.e. 11/22 -> 1/2)
/// - It find the smallest integer factor between the denominator and nominator.
///     - (i.e. 21/45 -> 7/15)
///
/// Inputs:
/// - `e: Expr` The expression to simplify.
///
/// Outputs: `Expr`
/// The simplified expression.
///
/// Time complexity: O(denominator)
///
pub fn recursive(mut e: Expr) -> Expr {
    #[tailcall]
    /// Simplifies expression in the (n/m)/(d/m) form.
    /// Will maximize m with the following constraints:
    /// - n/m is an integer
    /// - d/m is an integer
    /// - (n/m)/(d/m) = n/d
    /// - m < d
    fn inner(mut e: Expr, m: i64, gm: i64) -> Expr {
        if m < e.d {
            // currently looking for larger common factor
            if e.n % m == 0 && e.d % m == 0 {
                // found better factor, re-curse with new best factor
                inner(e, m + 1, m)
            } else {
                // not any better, continue looking
                inner(e, m + 1, gm)
            }
        } else if e.n % gm == 0 && e.d % gm == 0 && gm < e.d {
            // finishing looking for a better factor and found one!
            // update our expression with the lowest common denominator
            e.n = e.n / gm;
            e.d = e.d / gm;
            e
        } else {
            // couldn't find any common factors
            e
        }
    }

    if e.d == 0 {
        // basic edge case handling
        e
    } else if e.n == e.d {
        // n/n or d/d, either way its all equal to 1
        Expr::new(1, 1, 1)
    } else {
        // general case

        // handle case where n is a multiple of d
        // (d*m)/d -> m OR (n*m)/d -> m(n/d)
        e.c = 1.max(e.n / e.d);
        e.n = e.n / e.c;

        if e.d % e.n == 0 {
            // handle case where d is a constant multiple of n
            // n/(n*m) -> 1/m
            e.d = e.d / e.n;
            e.n = 1;
            e
        } else {
            // general general case

            // Form: n/d
            // Known constraints:
            // - n != d
            // - n % d != 0
            // - d % n != 0
            // - 1 < n < d

            // find (n/m)/(d/m) cases
            // iterate up to the current denominator
            inner(e, 2, 2)
        }
    }
}

//---------------------------------------------------------------------------//

/// Iteratively computes the Lowest Common Denominator of a fraction. Kind of.
/// Really it does several things.
/// - It converts improper fractions to proper fractions.
///     - (i.e. 11/5 -> 2(1/5) )
/// - It simplifies integer fractions.
///     - (i.e. 11/22 -> 1/2)
/// - It find the smallest integer factor between the denominator and nominator.
///     - (i.e. 21/45 -> 7/15)
///
/// Inputs:
/// - `e: Expr` The expression to simplify.
///
/// Outputs: `Expr`
/// The simplified expression.
///
/// Time complexity: O(denominator)
///
pub fn iterative(mut e: Expr) -> Expr {
    // basic undefined edge case
    if e.d == 0 {
        return e;
    }

    println!("--- {}", e);

    if e.n == e.d {
        println!("n/n or d/d");
        // 1/1 or similar
        Expr::new(1, 1, 1)
    } else {
        // general case

        println!("{}", e);

        // handle case where n is a multiple of d
        // (d*m)/d -> m OR (n*m)/d -> m(n/d)
        e.c = 1.max(e.n / e.d);
        e.n = e.n / e.c;
        println!("{}", e);

        if e.d % e.n == 0 {
            println!("n/(n*m)");
            // handle case where d is a constant multiple of n
            // n/(n*m) -> 1/m
            e.d = e.d / e.n;
            e.n = 1;
            println!("{}", e);
        } else if e.d > 1 {
            // Form: n/d
            // Known constraints:
            // - n != d
            // - n % d != 0
            // - d % n != 0
            // - 1 < n < d

            let mut m = 2;
            let mut gm = 2;

            // find (n/m)/(d/m) cases
            // iterate up to the current denominator
            while m < e.d {
                m += 1;
                // check if we've found a new smallest denominator
                //(in other words, the largest factor)
                if e.n % m == 0 && e.d % m == 0 {
                    // update tracker
                    gm = m;
                }
            }

            println!("gm:{}", gm);

            if e.n % gm == 0 && e.d % gm == 0 && gm < e.d {
                // n and d are both divisible by a common factor less than d
                // this gives us our lowest common denominator
                e.n = e.n / gm;
                e.d = e.d / gm;
            } else {
                // n or d are most likely prime and already in a stable form
            }
        }

        e
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Expr;

    #[test]
    fn iterative() {
        common(super::iterative);
    }

    #[test]
    fn recursive() {
        common(super::recursive);
    }

    fn common(func: fn(Expr) -> Expr) {
        // (m/(d*m)) -> 1/d
        for i in -20..100 {
            if i != 0 {
                for m in 1..100 {
                    let input = Expr::new(1, 1 * m, i * m);

                    let real = func(input);
                    let expected = Expr::new(1, 1, i);
                    println!("{} >< {}", real, expected);
                    assert_eq!(real, expected);
                }
            }
        }

        // (d*m)/d -> m
        for i in -20..100 {
            if i != 0 {
                for m in 1..100 {
                    let input = Expr::new(1, i * m, i);

                    let real = func(input);
                    let expected = Expr::new(m, 1, 1);
                    println!("{} >< {}", real, expected);
                    assert_eq!(real, expected);
                }
            }
        }

        // (n*m)/(d*m) -> n/d
        // inputs must be primes
        // https://byjus.com/maths/prime-numbers/#prime-numbers-1-to-200
        let primes = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173,
            179,
            // 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            // 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            // 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            // 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599,
            // 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
            // 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823,
            // 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941,
            // 947, 953, 967, 971, 977, 983, 991, 997,
        ];

        for i in 0..primes.len() {
            for j in 0..i {
                let n = primes[j];
                let d = primes[i];

                if d != 0 {
                    for m in 1..100 {
                        let input = Expr::new(1, n * m, d * m);

                        let real = func(input);
                        let expected = Expr::new(1, n, d);
                        println!("{} >< {}", real, expected);
                        assert_eq!(real, expected);
                    }
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
