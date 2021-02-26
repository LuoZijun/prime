// Trial division
// https://en.wikipedia.org/wiki/Trial_division
// https://en.wikipedia.org/wiki/Primality_test#Simple_methods
use crate::Primality;

use num_bigint::BigUint;


pub fn trial_division_u64(n: u64) -> Primality {
    // Primality test using 6k ±1 optimization
    // https://en.wikipedia.org/wiki/Primality_test#Python_code
    match n {
        0 | 1 => Primality::ZeroOrOne,
        // Lists of Primes at the Prime Pages
        // https://primes.utm.edu/lists/
        // 
        // [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
        2 | 3 | 5 | 7 
        | 11 | 13 | 17 | 19 
        | 23 | 29 
        | 31 | 37 
        | 41 | 43 | 47 
        | 53 | 59 
        | 61 | 67 
        | 71 | 73 | 79 
        | 83 | 89 
        | 97 => Primality::Prime,
        _ => {
            if n % 2 == 0 || n % 3 == 0 {
                return Primality::Composite;
            }

            let mut i = 5u64;
            loop {
                if i.pow(2) > n {
                    break;
                }

                if n % i == 0 || n % ( i + 2 ) == 0 {
                    return Primality::Composite;
                }

                i += 6;
            }

            return Primality::Prime;
        }
    }
}

pub fn trial_division_u128(n: u128) -> Primality {
    if n <= u64::MAX as u128 {
        trial_division_u64(n as u64)
    } else {
        if n % 2 == 0 || n % 3 == 0 {
            return Primality::Composite;
        }

        let mut i = 5u128;
        loop {
            if i.pow(2) > n {
                break;
            }

            if n % i == 0 || n % ( i + 2 ) == 0 {
                return Primality::Composite;
            }

            i += 6;
        }

        return Primality::Prime;
    }
}

pub fn trial_division_biguint(n: &BigUint) -> Primality {
    use core::convert::TryFrom;
    
    if let Ok(small_uint) = u128::try_from(n) {
        return trial_division_u128(small_uint);
    }

    let zero  = BigUint::from(0u8);
    
    if n % 2u8 == zero || n % 3u8 == zero {
        return Primality::Composite;
    }

    let mut i = BigUint::from(5u8);
    loop {
        let i_square = i.pow(2u32);
        if &i_square > n {
            break;
        }
        
        let i_plus_two = &i + 2u8;
        if n % &i == zero || n % i_plus_two == zero {
            return Primality::Composite;
        }

        i += 6u8;
    }

    return Primality::Prime;
}


#[bench]
fn bench_trial_division_u64(b: &mut test::Bencher) {
    b.iter(|| {
        let n = test::black_box(u64::MAX);
        trial_division_u64(n)
    })
}

#[bench]
fn bench_trial_division_u128(b: &mut test::Bencher) {
    b.iter(|| {
        let n = test::black_box(u128::MAX);
        trial_division_u128(n)
    })
}

#[bench]
fn bench_trial_division_biguint(b: &mut test::Bencher) {
    let n = "115792089237316195423570985008687907853269984665640564039457584007913129639935".parse::<BigUint>().unwrap();
    // let n = test::black_box(BigUint::from(n));
    b.iter(|| {
        trial_division_biguint(&n)
    })
}
