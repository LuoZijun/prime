// PRIMES is in P
// https://www.cse.iitk.ac.in/users/manindra/algebra/primality_v6.pdf
// 
// AKS primality test (wikipedia)
// https://en.wikipedia.org/wiki/AKS_primality_test
use crate::Primality;


// AKS test for primes (Example Code)
// https://rosettacode.org/wiki/AKS_test_for_primes#Rust
pub fn aks_primality_test_usize(n: usize) -> Primality {
    // Input:  integern > 1
    debug_assert!(n > 1);
    
    let n1 = n + 1;
    let n2 = n / 2 + 1;

    let pp: i64 = core::convert::TryFrom::try_from(n).expect("oops ...");

    // NOTE: 在计算大数时，系数的存储将变得几乎不可能实现。
    let mut coefficients = vec![0i64; n1];
    coefficients[0] = 1;
    for i in 1..n1 {
        let mut prev = coefficients[0];
        for j in 1..i {
            let old = coefficients[j];
            // NOTE: 此处的减法会溢出，应该使用 BigInt。
            // coefficients[j] = old.wrapping_sub(prev);
            coefficients[j] = old - prev;
            prev = old;
        }
        
        coefficients[i] = -prev;
    }

    for i in 1..n2 {
        if coefficients[i] % pp != 0 {
            return Primality::Composite;
        }
    }

    return Primality::Prime;
}

