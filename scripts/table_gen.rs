fn is_prime(n: usize) -> bool {
    match n {
        0 | 1 => false,
        2 | 3 | 5 | 7 
        | 11 | 13 | 17 | 19 
        | 23 | 29 
        | 31 | 37 
        | 41 | 43 | 47 
        | 53 | 59 
        | 61 | 67 
        | 71 | 73 | 79 
        | 83 | 89 
        | 97 => true,
        _ => {
            // NOTE: 不存在大于 2 的偶数为质数。
            //       https://en.wikipedia.org/wiki/Primality_test
            if n % 2 == 0 {
                return false;
            }

            for i in 3..(n / 2) {
                if n % i == 0 {
                    return false;
                }
            }

            return true;
        }
    }
}

fn query(table: &[u64], n: u16) -> bool {
    if n % 2 == 0 {
        // NOTE: 偶数当中，除了 2 之外都不是质数。
        //       同时，我们的静态表里面存储的都是奇数序列，
        //       所以，这里先清除掉偶数的案例。
        return n == 2;
    }

    let pos = (n as usize - 1) / 2;
    let i = pos / 64;
    let r = pos % 64;

    let mask = table[i];

    (mask << r >> 63) == 1
}

fn table_gen() {
    let mut data: Vec<u64> = Vec::new();

    // NOTE: 只测试奇数是否为质数，偶数当中的 `2` 需要自行特别处理。
    let mut n  = 1usize;

    const MAX: usize = u16::MAX as usize;
    loop {
        if n > MAX {
            break;
        }

        let mut mask = 0u64;
        for bi in 0usize..64 {
            if is_prime(n) {
                mask |= 1 << (64 - bi - 1);
            }

            n += 2;
        }

        data.push(mask);
    }
    
    println!("// NOTE: 已确定的 从 1 至 65535（含）奇数序列范围内的素数表。");
    println!("//       512 * 8 / 1024 = 4K 大小");
    println!("static SMALL_PRIME_TABLE: [u64; {}] = [", data.len());
    for line in data.chunks(8) {
        print!("    ");
        for x in line.iter() {
            print!("0x{:016x}, ", x);
        }
        println!();
    }
    println!("];");
    
    println!();
    println!("N={:?} TABLE-LEN={:?}", n, data.len());
    
    // Verify
    print!("Table Verify: ");
    for n in 0..MAX {
        if n % 2 != 0 {
            // NOTE: 确保查询的是奇数
            assert_eq!(query(&data, n as u16), is_prime(n));
        }
    }
    print!(" [OK]\n");
}

fn main() {
    table_gen()
}
