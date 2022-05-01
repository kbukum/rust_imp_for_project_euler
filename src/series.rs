/// # Sum square difference
/// The sum of the squares of the first ten natural numbers is,
/// 1^2  + 2^2 + ... + 10^2 = 385
/// The square of the sum of the first ten natural numbers is,
/// (1 + 2 + ... + 10) ^2 = 55^2 = 3025
/// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is,
/// 3025 - 385 = 2640
/// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
/// Sum of Square of sequences, 1..n = 1^2 + 2^2 + ... + n^2
/// Square of Sum of sequences, (1..n = 1 + 2 + ... + n)^2 = 1^2 + 1 * (((n * n + 1)/2) - 1) + 1^2 + 2 * (((n * (n + 1))/2) - 2)
pub(crate) fn sum_square_difference() {
    let n = 100;
    let sum_square_diff = _sum_square_difference(n);
    println!("Sum square difference for {} natural numbers = {}", n, sum_square_diff);
}

fn _sum_square_difference(n: u64) -> u64 {
    let mut total_sum = 0;
    let sum_of_sequences = (n * (n + 1)) / 2;
    for i in 1..(n + 1) {
        total_sum += i * (sum_of_sequences - i);
    }
    total_sum
}

/// # Largest product in a series
/// The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.
///
/*
"73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450";
 */
///
///  Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
pub(crate) fn find_the_greatest_product_of_adjacent_digits() {
    let number_series =
        "73167176531330624919225119674426574742355349194934
         96983520312774506326239578318016984801869478851843
         85861560789112949495459501737958331952853208805511
         12540698747158523863050715693290963295227443043557
         66896648950445244523161731856403098711121722383113
         62229893423380308135336276614282806444486645238749
         30358907296290491560440772390713810515859307960866
         70172427121883998797908792274921901699720888093776
         65727333001053367881220235421809751254540594752243
         52584907711670556013604839586446706324415722155397
         53697817977846174064955149290862569321978468622482
         83972241375657056057490261407972968652414535100474
         82166370484403199890008895243450658541227588666881
         16427171479924442928230863465674813919123162824586
         17866458359124566529476545682848912883142607690042
         24219022671055626321111109370544217506941658960408
         07198403850962455444362981230987879927244284909188
         84580156166097919133875499200524063689912560717606
         05886116467109405077541002256983155200055935729725
         71636269561882670428252483600823257530420752963450";
    let adjacent_digits: usize = 13;
    let greatest_product = _find_the_greatest_product_of_adjacent_digits(&number_series, adjacent_digits);
    match greatest_product {
        Ok(n) => println!("Greatest product of the adjacent {} digits = {}", adjacent_digits, n),
        Err(err) => println!("{}", err)
    }
}

fn _find_the_greatest_product_of_adjacent_digits(number_series: &'static str, adjacent_digits: usize) -> Result<u64, String> {
    if adjacent_digits > number_series.len() {
        return Err("Adjacent digits amount is higher than the length of number series".to_string());
    }

    // let number_series: &[u8] = number_series.as_bytes();
    let mut greatest_product: u64 = 1;
    let max_counter = number_series.len() - adjacent_digits + 1;
    // for p in 0..adjacent_digits {
    //     map.insert(p, 1);
    // }

    for i in 0..max_counter {
        let mut pos = 0;
        let mut p = 1;
        let mut non_number_counter = 0;
        while pos < (adjacent_digits + non_number_counter) && (i + pos + non_number_counter) < number_series.len() {
            // get the character by index
            let character = number_series.get((i + pos)..(i + pos + 1));
            if character.is_none() { // if the given value is not a character
                non_number_counter += 1;
                continue;
            }
            // get the number from the character
            let number: Result<u64, _> = character.unwrap().parse();
            if number.is_err() { // if the given value is not a number
                non_number_counter += 1;
                continue;
            }
            let value = number.unwrap();
            p *= value;
            pos += 1;
            // check if we have all digits multiplication
            if pos % (adjacent_digits + non_number_counter) == 0 {
                if greatest_product < p {
                    greatest_product = p;
                }
                continue;
            }
        }
    }
    println!("\n");
    return Ok(greatest_product);
}

/// # Special Pythagorean triplet
/// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
/// a^2 + b^2 = c^2
/// For example, 3^2 + 4^2 = 9 + 16 = 25 = 52.
///
/// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
/// Find the product abc.

pub fn find_pythagorean_triplet_set() {
    let s = 1000; // sum of triplets
    match  _find_pythagorean_triplet_set(s) {
        Some((a, b, c)) => {
            println!("The Pythagorean Triplet Set found, (a, b, c) as ({}, {}, {})", a, b, c);
            println!("The Multiplication of Pythagorean Triplet Set, (a, b, c) is a*b*c = {}*{}*{} = {}", a, b, c, a*b*c);
        },
        None => println!("There is not Pythagorean Triplet Set where a, b and c are natural numbers and a^2 + b^2 = c^2 and a + b + c = {}", s)
    }
}

pub fn _find_pythagorean_triplet_set(s: u64) -> Option<(u64,u64,u64)> {
    // a, b, c natural numbers -> can be positive values (note: sometimes 0 included.)
    // a^2 + b^2 = c^2
    let min_a: u64 = 1;
    let max_a: u64 = s/3; // Range for a is [1, (s/3) - 1]
    let max_b: u64 = s/2; // Range for b is [2, (s/2-1)]

    for a in min_a..max_a {
        for b in (a + 1)..max_b {
            // a + b + c = s
            // a^2 + b^2 = c^2
            let c = s - a - b;
            if c <= b { continue }
            if c.pow(2) == (a.pow(2) + b.pow(2)) {
                return Some((a, b, c as u64))
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::series;
    #[test]
    fn _find_pythagorean_triplet_set() {
        let s = 12; // sum of triplets
        match  series::_find_pythagorean_triplet_set(s) {
            Some((a, b, c)) => {
                assert_eq!(3, a);
                assert_eq!(4, b);
                assert_eq!(5, c);
                assert_eq!(60, a*b*c);
            },
            None => assert!(true)
        }
    }

    #[test]
    fn _find_the_greatest_product_of_adjacent_digits() {
        let number_series =
            "73167176531330624919225119674426574742355349194934
                 96983520312774506326239578318016984801869478851843
                 85861560789112949495459501737958331952853208805511
                 12540698747158523863050715693290963295227443043557
                 66896648950445244523161731856403098711121722383113
                 62229893423380308135336276614282806444486645238749
                 30358907296290491560440772390713810515859307960866
                 70172427121883998797908792274921901699720888093776
                 65727333001053367881220235421809751254540594752243
                 52584907711670556013604839586446706324415722155397
                 53697817977846174064955149290862569321978468622482
                 83972241375657056057490261407972968652414535100474
                 82166370484403199890008895243450658541227588666881
                 16427171479924442928230863465674813919123162824586
                 17866458359124566529476545682848912883142607690042
                 24219022671055626321111109370544217506941658960408
                 07198403850962455444362981230987879927244284909188
                 84580156166097919133875499200524063689912560717606
                 05886116467109405077541002256983155200055935729725
                 71636269561882670428252483600823257530420752963450";
        let adjacent_digits: usize = 4;
        let greatest_product = series::_find_the_greatest_product_of_adjacent_digits(&number_series, adjacent_digits);
        match greatest_product {
            Ok(num) => assert_eq!(5832, num),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn _sum_square_difference() {
        let n: u64 = 10;
        let sum_square_diff = series::_sum_square_difference(n);
        assert_eq!(2640, sum_square_diff);
    }
}
