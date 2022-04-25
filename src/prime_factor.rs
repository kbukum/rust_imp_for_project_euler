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

pub(crate) fn find_the_nth_prime() {
    let n: u64 = 10001;
    let nth_prime_number = _find_the_nth_prime(n);
    println!("The {} prime number is {}", _ordinal_suffix_of(n), nth_prime_number)
}

fn _find_the_nth_prime(n: u64) -> u64 {
    let mut nth_prime : u64 = 2;
    let mut counter = 0;
    if n == 1 { // first prime is 2
        return nth_prime;
    }
    counter += 1; // first prime counted
    nth_prime += 1;

    if n == 2 { // second prime is 3
        return nth_prime;
    }
    counter += 1; // second prime counted

    let mut previous_odd_primes = Vec::new();
    {
        previous_odd_primes.push(nth_prime);
    }

    loop {
        nth_prime += 2;
        let mut is_prime = true;
        for prime_factor in &previous_odd_primes {
            if nth_prime % prime_factor == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            counter += 1;
            if counter == n {
                return nth_prime;
            }
            previous_odd_primes.push(nth_prime);
        }
    }
}

fn _ordinal_suffix_of(i: u64) -> String {
    let j = i % 10;
    let k = i % 100;
    if j == 1 && k != 11 {
        return format!("{}st", i);
    }
    if j == 2 && k != 12 {
        return format!("{}nd", i);
    }
    if j == 3 && k != 13 {
        return format!("{}rd", i);
    }
    return format!("{}th", i);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::prime_factor;

    #[test]
    fn find_the_nth_prime() {
        let n: u64 = 6;
        let nth_prime_number = prime_factor::_find_the_nth_prime(n);
        assert_eq!(13, nth_prime_number)
    }


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
            let largest_prime_factor = prime_factor::_find_the_largest_prime_factor(n);
            assert_eq!(expected_larges_prime_factor, largest_prime_factor);
        }
    }
}
