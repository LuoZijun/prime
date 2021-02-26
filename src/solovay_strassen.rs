// Solovay–Strassen primality test
// https://en.wikipedia.org/wiki/Solovay%E2%80%93Strassen_primality_test
use crate::Primality;
use crate::miller_rabin::modpow;

use rand::Rng;
use num_integer::Integer;
use num_bigint::{BigUint, BigInt, RandBigInt};


// a | n or (a/n)
// 
// Legendre symbol
// https://en.wikipedia.org/wiki/Legendre_symbol
fn legendre_symbol_biguint(
    a: &BigUint, 
    n: &BigUint, 
    zero: &BigUint, 
    one: &BigUint, 
    three: &BigUint, 
    five: &BigUint
) -> i8 {
    let mut a = a.clone();
    let mut n = n.clone();

    let mut res = 1i8;

    while &a != zero {
        while &(&a % 2u8) == zero {
            a /= 2u8;
            let r = &n % 8u8;
            if &r == three || &r == five {
                res = -res;
            }
        }

        core::mem::swap(&mut a, &mut n);

        let r1 = &a % 4u8;
        let r2 = &n % 4u8;
        if &r1 == three && &r2 == three {
            res = -res;
        }
        a = &a % &n;
    }

    if &n == one {
        res
    } else {
        0i8
    } 
}


// Algorithm and running time
// https://en.wikipedia.org/wiki/Solovay%E2%80%93Strassen_primality_test#Algorithm_and_running_time
pub fn solovay_strassen_primality_test_biguint(n: &BigUint, k: usize) -> Primality {
    // Input #1: n > 3, an odd integer to be tested for primality
    // Input #2: k, the number of rounds of testing to perform
    debug_assert!(k > 0);

    let zero  = BigUint::from(0u8);
    let one   = BigUint::from(1u8);
    let two   = BigUint::from(2u8);
    let three = BigUint::from(3u8);
    let five  = BigUint::from(5u8);
    
    if cfg!(debug_assertions) {
        debug_assert!(n > &three && n % 2u8 != zero);
    }
    

    let n_minus_one = n - 1u8;
    // (n - 1) / 2
    let exp = &n_minus_one / 2u8;
    // NOTE: 此处，因为 num 库没用提供内部方法，所以开销较大。
    // let n1 = BigInt::from(n.clone());
    
    let mut rng = rand::thread_rng();

    // repeat k times
    for _ in 0..k {
        // choose a randomly in the range [2, n − 1]
        let a: BigUint = rng.gen_biguint_range(&two, &n_minus_one);

        let x: i8 = legendre_symbol_biguint(&a, n, &zero, &one, &three, &five);

        match x {
            -1 => {
                // NOTE: 当 n > 1 时，-1 ModFloor n = n - 1
                // debug_assert!(n > 1);

                // x (mod n)
                let r1 = &n_minus_one;
                // a ^ ((n - 1) / 2) % n
                let r2 = a.modpow(&exp, n);
                if r1 != &r2 {
                    // composite
                    return Primality::Composite;
                }
            },
            0 => {
                return Primality::Composite;
            },
            1 => {
                // NOTE: 当 n > 1 时，+1 ModFloor n = 1
                // debug_assert!(n > 1);

                // x (mod n)
                let r1 = &one;
                // a ^ ((n - 1) / 2) % n
                let r2 = a.modpow(&exp, n);
                if r1 != &r2 {
                    // composite
                    return Primality::Composite;
                }
            },
            _ => unreachable!(),
        }

        // // if x = 0 or a ^ ((n - 1) / 2) != x (mod n) then return composite
        // if x == 0 {
        //     // composite
        //     return Primality::Composite;
        // }

        // // a ^ ((n - 1) / 2) % n
        // let r2 = BigInt::from(a.modpow(&exp, n));

        // // x (mod n)
        // let x = BigInt::from(x);
        // let r1 = x.mod_floor(&n1);

        // if r1 != r2 {
        //     // composite
        //     return Primality::Composite;
        // }
    }

    // probably prime
    return Primality::ProbablyPrime;
}


// a | n or (a/n)
// 
// Legendre symbol
// https://en.wikipedia.org/wiki/Legendre_symbol
fn legendre_symbol_u64(a: u64, n: u64) -> i8 {
    // a, in the range [2, n − 1]
    // Output: -1, 0, +1
    // 
    // The Legendre symbol is defined for prime p as
    // 
    //      ( a \ p ) = s
    // 
    // if s ==  0, p divides a
    // if s == -1, if a is a quadratic residue modulo p
    // if s == +1, if a is a quadratic non-residue modulo p
    // 
    // if ( a \ p ) = 1, then the equation
    // 
    //      x² = a (mod p)
    // 
    debug_assert!(n > 1 && a >= 2 && a < n - 1);

    let mut a = a as u128;
    let mut n = n as u128;
    
    let mut ret = 1i8;
    
    while a != 0 {
        while a % 2 == 0 {
            a /= 2;
            let r = n % 8;
            if r == 3 || r == 5 {
                ret = -ret;
            }
        }

        let a_copy = a;
        a = n;
        n = a_copy;

        if a % 4 == 3 && n % 4 == 3 {
            ret = -ret;
        }

        a = a % n;
    }

    if n == 1 {
        ret
    } else {
        0i8
    }
}



pub fn solovay_strassen_primality_test_u64(n: u64, k: usize) -> Primality {
    // inputs: n, a value to test for primality
    //         k, a parameter that determines the accuracy of the test
    // output: composite if n is composite, otherwise probably prime
    // 
    // NOTE: 因为 随机数 a 的取值范围为 [2, n - 1], 并且勒让德符号中的分母只限奇质数，
    //       所以这里送入判断的数字 需 > 3, 以及 为奇数。
    debug_assert!(n > 3 && n % 2 != 0);
    debug_assert!(k > 0);
    match n {
        0 | 1 => Primality::ZeroOrOne,
        2 | 3 | 5 => Primality::ProbablyPrime,
        // 2 | 3 => Primality::Prime,
        _ => {
            let mut rng = rand::thread_rng();

            let n_minus_one = n - 1;
            let exp = n_minus_one / 2;

            for _ in 0..k {
                // choose a randomly in the range [2, n − 1]
                let a: u64 = rng.gen_range(2, n_minus_one);

                // x ← ( a \ n ), Legendre symbol
                let x: i8 = legendre_symbol_u64(a, n);

                // if x = 0 or a ^ ((n - 1) / 2) != x (mod n) then return composite
                match x {
                    -1 => {
                        // NOTE: 当 n > 1 时，-1 ModFloor n = n - 1
                        debug_assert!(n > 1);

                        // x (mod n)
                        let r1 = n_minus_one;
                        // a ^ ((n - 1) / 2) % n
                        let r2 = modpow(a, exp, n);
                        if r1 != r2 {
                            // composite
                            return Primality::Composite;
                        }
                    },
                    0 => {
                        return Primality::Composite;
                    },
                    1 => {
                        // NOTE: 当 n > 1 时，+1 ModFloor n = 1
                        debug_assert!(n > 1);

                        // x (mod n)
                        let r1 = 1;
                        // a ^ ((n - 1) / 2) % n
                        let r2 = modpow(a, exp, n);
                        if r1 != r2 {
                            // composite
                            return Primality::Composite;
                        }
                    },
                    _ => unreachable!(),
                }
            }

            // probably prime
            return Primality::ProbablyPrime;
        }
    }
}


#[bench]
fn bench_legendre_symbol_u64(b: &mut test::Bencher) {
    b.iter(|| {
        let a = test::black_box(u64::MAX - 2);
        let n = test::black_box(u64::MAX);
        legendre_symbol_u64(a, n)
    })
}

#[bench]
fn bench_solovay_strassen_primality_test_u64(b: &mut test::Bencher) {
    b.iter(|| {
        let n = test::black_box(18446744073709551615u64);
        solovay_strassen_primality_test_u64(n, 3)
    })
}

#[test]
fn test_solovay_strassen_primality_test_biguint() {
    let sets = [
        // 2u64, 3, 
        5u64, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 
        43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97
    ];

    for n in sets.iter() {
        let n = BigUint::from(*n);
        let ret = solovay_strassen_primality_test_biguint(&n, 3);
        assert!(ret == Primality::ProbablyPrime || ret == Primality::Prime);
    }

    use crate::table_query_u16;

    for n in 5..u16::MAX {
        if n % 2 != 0 {
            let b: bool = table_query_u16(n).into();
            let n = BigUint::from(n);
            let a: bool = solovay_strassen_primality_test_biguint(&n, 4).into();
            assert_eq!(a, b, "N={} a={} b={}", n, a, b);
        }
    }
    
    assert!(solovay_strassen_primality_test_biguint(&BigUint::from(7u8), 3) == Primality::ProbablyPrime);
    assert!(solovay_strassen_primality_test_biguint(&BigUint::from(9u8), 3) == Primality::Composite);
    assert!(solovay_strassen_primality_test_biguint(&BigUint::from(11u8), 3) == Primality::ProbablyPrime);
    assert!(solovay_strassen_primality_test_biguint(&BigUint::from(15u8), 3) == Primality::Composite);
}

#[test]
fn test_solovay_strassen_primality_test_u64() {
    let sets = [
        // 2u64, 3, 
        5u64, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 
        43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97
    ];

    for n in sets.iter() {
        let ret = solovay_strassen_primality_test_u64(*n, 8);
        assert!(ret == Primality::ProbablyPrime || ret == Primality::Prime, "N={:?} RET={:?}", n, ret);
    }

    use crate::table_query_u16;
    for n in 5..u16::MAX {
        if n % 2 != 0 {
            let b: bool = table_query_u16(n).into();
            let a: bool = solovay_strassen_primality_test_u64(n as u64, 5).into();
            assert_eq!(a, b, "N={} a={} b={}", n, a, b);
        }
    }
    
    assert!(solovay_strassen_primality_test_u64(7, 3) == Primality::ProbablyPrime);
    assert!(solovay_strassen_primality_test_u64(9, 3) == Primality::Composite);
    assert!(solovay_strassen_primality_test_u64(11, 3) == Primality::ProbablyPrime);
    assert!(solovay_strassen_primality_test_u64(15, 3) == Primality::Composite);
}
