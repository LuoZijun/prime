// Miller–Rabin primality test
// https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
use crate::Primality;

use num_bigint::BigUint;
use num_bigint::RandBigInt;


// a * b % m
fn modmul_u64(a: u64, b: u64, m: u64) -> u64 {
    match a.checked_mul(b) {
        Some(r) => r % m,
        None => {
            let ret = (a as u128) * (b as u128) % (m as u128);
            assert!(ret <= u64::MAX as u128);
            ret as u64
        },
    }
}

// Modular exponentiation
// https://en.wikipedia.org/wiki/Modular_exponentiation
// 
// base ^ exponent % modulus
pub(crate) fn modpow(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut base = base;
    let mut exponent = exponent;
    let modulus = modulus;

    if modulus == 1 {
        return 0;
    }

    let mut result: u64 = 1;
    base = base % modulus;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = modmul_u64(result, base, modulus);
        }

        exponent = exponent >> 1;
        base = modmul_u64(base, base, modulus);
    }

    return result;
}

pub fn miller_rabin_primality_test_u64(n: u64) -> Primality {
    // Deterministic Miller primality testing
    // https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test#Deterministic_variants
    // 
    // Input: n > 1, an odd integer to be tested for primality
    // Output: “composite” if n is composite, “prime” otherwise
    debug_assert!(n > 1 && n % 2 != 0);

    let n_minus_one = n - 1;

    // write n as 2r·d + 1 with d odd (by factoring out powers of 2 from n − 1)
    let mut d = n_minus_one;
    let mut r = 0u64;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    // Testing against small sets of bases
    // https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test#Testing_against_small_sets_of_bases
    // 
    // Deterministic variants of the Miller-Rabin primality test:
    // https://miller-rabin.appspot.com/
    let witnesses: &[u64] = match n {
                          0..=                        2 => unreachable!(),
                          3..=                     2046 => &[2],
                       2047..=                1_373_652 => &[2, 3],
                  1_373_653..=                9_080_190 => &[31, 73],
                  9_080_191..=               25_326_000 => &[2, 3, 5],
                 25_326_001..=            3_215_031_750 => &[2, 3, 5, 7],
              3_215_031_751..=            4_759_123_140 => &[2, 7, 61],
              4_759_123_141..=        1_122_004_669_632 => &[2, 13, 23, 1662803],
          1_122_004_669_633..=        2_152_302_898_746 => &[2, 3, 5, 7, 11],
          2_152_302_898_747..=        3_474_749_660_382 => &[2, 3, 5, 7, 11, 13],
          3_474_749_660_383..=      341_550_071_728_320 => &[2, 3, 5, 7, 11, 13, 17],
        341_550_071_728_321..=3_825_123_056_546_413_050 => &[2, 3, 5, 7, 11, 13, 17, 19, 23],
        // n < 18_446_744_073_709_551_616 = 22 <= 2 ^ 64 - 1
        _ => &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37],
    };

    let k = witnesses.len();
    'WitnessLoop: for i in 0..k {
        let a = witnesses[i];
        let mut x = modpow(a, d, n);
        if x == 1 || x == n_minus_one {
            continue 'WitnessLoop;
        }

        // repeat r − 1 times:
        for _ in 0..r {
            // x ← x2 mod n
            x = modpow(x, 2, n);
            if x == n_minus_one {
                continue 'WitnessLoop;
            }
        }

        // composite
        return Primality::Composite;
    }

    // prime
    return Primality::Prime;
}

pub fn miller_rabin_primality_test_biguint(n: &BigUint, k: usize) -> Primality {
    // Miller–Rabin test
    // https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test#Miller%E2%80%93Rabin_test
    // 
    // Input #1: n > 3, an odd integer to be tested for primality
    // Input #2: k, the number of rounds of testing to perform
    // Output: “composite” if n is found to be composite, “probably prime” otherwise
    debug_assert!(k > 0);
    
    let zero = BigUint::from(0u8);
    let one  = BigUint::from(1u8);
    let two  = BigUint::from(2u8);

    if cfg!(debug_assertions) {
        let four = BigUint::from(4u8);
        debug_assert!(n > &four && n % 2u8 != zero);
    }

    let n_minus_one: BigUint = n - 1u8;
    let n_minus_two: BigUint = n - 2u8;

    // write n as 2r·d + 1 with d odd (by factoring out powers of 2 from n − 1)
    let mut d = n_minus_one.clone();
    let mut r = 0usize;
    while &d % 2u8 == zero {
        d /= 2u8;
        r += 1;
    }

    let mut rng = rand::thread_rng();

    'WitnessLoop: for _ in 0..k {
        // pick a random integer a in the range [2, n − 2]
        let a = rng.gen_biguint_range(&two, &n_minus_two);
        let mut x = a.modpow(&d, &n);
        
        if &x == &one || &x == &n_minus_one {
            continue 'WitnessLoop;
        }

        // repeat r − 1 times:
        for _ in 0..r {
            // x ← x ^ 2 mod n
            x = x.modpow(&two, &n);
            if &x == &n_minus_one {
                continue 'WitnessLoop;
            }
        }

        // composite
        return Primality::Composite;
    }
    
    // probably prime
    return Primality::ProbablyPrime;
}


#[bench]
fn bench_miller_rabin_primality_test_biguint(b: &mut test::Bencher) {
    let n = test::black_box(BigUint::from(18446744073709551615u64));

    b.iter(|| {
        miller_rabin_primality_test_biguint(&n, 4)
    })
}

#[bench]
fn bench_miller_rabin_primality_test_u64(b: &mut test::Bencher) {
    b.iter(|| {
        let n = test::black_box(18446744073709551615u64);
        miller_rabin_primality_test_u64(n)
    })
}

#[bench]
fn bench_modpow_u64(b: &mut test::Bencher) {
    let n = u64::MAX;
    let mut d = n - 1;
    while d % 2 == 0 {
        d /= 2;
    }

    // 2 ... n - 2
    let a = n - 2;

    b.iter(|| {
        modpow(a, d, n)
    })
}



