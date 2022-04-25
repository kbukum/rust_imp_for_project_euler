
/// # Sum square difference
/// The sum of the squares of the first ten natural numbers is,
/// 1^2  + 2^2 + ... + 10^2 = 385
/// The square of the sum of the first ten natural numbers is,
/// (1 + 2 + ... + 10) ^2 = 55^2 = 3025
/// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is,
/// 3025 - 385 = 2640
/// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
pub(crate) fn sum_square_difference() {
    /// Sum of Square of sequences, 1..n = 1^2 + 2^2 + ... + n^2
    /// Square of Sum of sequences, (1..n = 1 + 2 + ... + n)^2 = 1^2 + 1 * (((n * n + 1)/2) - 1) + 1^2 + 2 * (((n * (n + 1))/2) - 2)
    let n = 100;
    let sum_square_diff = _sum_square_difference(n);
    println!("Sum square difference for {} natural numbers = {}", n, sum_square_diff);
}

fn _sum_square_difference(n: u64) -> u64 {
    let mut total_sum = 0;
    let sum_of_sequences = (n * (n + 1))/2;
    for i in 1..(n+1) {
        total_sum += i * (sum_of_sequences - i);
    }
    total_sum
}

#[cfg(test)]
mod tests {
    use crate::series;

    #[test]
    fn _sum_square_difference() {
        let n: u64 = 10;
        let sum_square_diff = series::_sum_square_difference(n);
        assert_eq!(2640, sum_square_diff);
    }
}
