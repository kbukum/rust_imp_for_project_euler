/// The largest prime factor
/// The prime factors of 13195 are 5, 7, 13 and 29.
/// What is the largest prime factor of the number 600851475143 ?
pub(crate) fn find_the_largest_prime_factor() {
    let n: u64 = 600851475143;
    let largest_prime_factor = _find_the_largest_prime_factor(n);
    println!("Largest prime factor for {} = {}", n, largest_prime_factor);
}

fn _find_the_largest_prime_factor(n: u64) -> u64 {
    let mut factor: u64 = 1;
    let mut largest_prime_factor: u64 = 1;
    let mut remain_n = n;
    while largest_prime_factor < remain_n {
        factor += 1;
        while remain_n % factor == 0 {
            remain_n = remain_n/factor;
            largest_prime_factor = factor;
        }
    }
    largest_prime_factor
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::prime_factor::_find_the_largest_prime_factor;

    #[test]
    fn _find_the_largest_prime_factor_test() {
        let mut numbers_map: HashMap<u64, u64> =HashMap::new();
        numbers_map.insert(2, 2);
        numbers_map.insert(3, 3);
        numbers_map.insert(4, 2);
        numbers_map.insert(12, 3);
        numbers_map.insert(20, 5);
        numbers_map.insert(23, 23);
        numbers_map.insert(35, 7);
        numbers_map.insert(37, 37);
        numbers_map.insert(41, 41);
        numbers_map.insert(43, 43);
        numbers_map.insert(600851475143, 6857);
        for (n, expected_larges_prime_factor) in numbers_map {
            let largest_prime_factor = _find_the_largest_prime_factor(n);
            assert_eq!(expected_larges_prime_factor, largest_prime_factor);
        }
    }
}
