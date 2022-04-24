use combinations::Combinations;
use crate::arrays;
/// # Multiples of 3 or 5
/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
/// Find the sum of all the multiples of 3 or 5 below 1000.
///
/// Find the sum of all the multiples of the given factors below the given number.
pub(crate) fn find_the_sum_of_multiples_of_the_factors() {
    let mut factors = [3, 5];

    let total_sum = _find_the_sum_of_multiples_of_the_factors(&mut factors, 999);
    println!("Total Sum of the multiplies for the factors, {:?} = {}", factors, total_sum);
}

/// Go over each factor and find how many times it is counted the given number and substract from the second combination of it with other factors
/// A(2)  =  2  4  6  8  10  12  14  16  18  20  22  24  26  28  30
/// A(3)  =   3    6   9     12    15    18    21    24    27    30
/// A(5)  =      5      10         15        20          25      30
/// A(2, 3) =      6         12          18          24          30
/// A(2, 5) =            10                  20                  30
/// A(3, 5) =                      15                            30
/// A(2, 3, 5) =                                                 30
/// A(2) + A(3) + A(5) - A(2, 3) - A(2, 5) - A(3, 5) +  A(2, 3, 5)
///
fn _find_the_sum_of_multiples_of_the_factors(factors: &mut [u64], max_value: u64) -> u64 {
    // Sorting factors in ascending way.
    arrays::quick_sort(factors);
    // removing all sort of the numbers from the factors if they are multiplication of the other factors.
    let mut absolute_factors: Vec<u64> = Vec::new();
    absolute_factors.push(factors[0]);
    let mut pos = 1;
    while pos < factors.len() {
        let selected_num = factors[pos];
        if !has_factor_in_array( &factors[..pos], selected_num) {
            absolute_factors.push(selected_num);
        }
        pos += 1;
    }

    // Find the summation for each factors and their combinations
    // Apply the set formula to find out the exact summation.
    // Example:
    // Subsets only have 1 elements -> + A(2) + A(3) + A(5)
    // Subsets only have 2 elements -> - A(2, 3) + A(2, 5) _ A(3, 5)
    // Subsets only have 3 elements -> +  A(2, 3, 5)
    // ...
    let mut total_sum = 0;

    let mut sign: i64 = 1;
    for i in 1..(absolute_factors.len() + 1) {
        let subsets = get_multiplication_of_subsets(& mut absolute_factors, i);
        for s in subsets {
            let sum_of_subsets: i64 = sign * (sum_of_multiplies(s, max_value) as i64);
            total_sum = (total_sum as i64 + sum_of_subsets) as u64;
        }
        sign *= -1;
    }
    total_sum
}

fn sum_of_afs(n: u64, first: u64, last: u64) -> u64 {
    n * (first + last)/2
}

fn sum_of_afs_1_to_n(n: u64)  ->  u64 {
    sum_of_afs(n, 1, n)
}

fn sum_of_multiplies(factor: u64, max_value: u64) -> u64 {
    let amount_of_multiplies =  (max_value/factor) as u64;
    sum_of_afs_1_to_n(amount_of_multiplies) * factor
}

fn multiple(subset: &Vec<u64>) -> u64 {
    let mut mul_of_sub = 1;
    for s in subset {
        mul_of_sub *= s;
    }
    mul_of_sub
}

pub fn get_multiplication_of_subsets(sets: &Vec<u64>, len: usize) -> Vec<u64> {
    if len > sets.len() {
        panic!("Combination length exceding elements amount!");
    } else if len == sets.len() {
        let mut subset = Vec::new();
        subset.push(  multiple(sets));
        return subset;
    }
    return Combinations::new(sets.clone(), len).map(|f| multiple(&f)).collect()
}

fn has_factor_in_array(factors: &[u64], num: u64) -> bool {
    if factors.len() == 0 {
        return false;
    }
    for factor in factors {
        if num == *factor || num % factor == 0 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::multiples::{_find_the_sum_of_multiples_of_the_factors, has_factor_in_array};

    #[test]
    fn _find_the_sum_of_multiples_of_the_factors_test() {
        let total_sum_3_5 = _find_the_sum_of_multiples_of_the_factors(&mut [3, 5], 999);
        assert_eq!(233168, total_sum_3_5);
        let total_sum_3_5 = _find_the_sum_of_multiples_of_the_factors(&mut [3, 5, 5], 999);
        assert_eq!(233168, total_sum_3_5);
    }

    #[test]
    fn has_factor_in_array_test() {
        let sets = [3, 4, 5];
        assert_eq!(has_factor_in_array(&sets, 10), true);
        assert_eq!(has_factor_in_array(&sets, 7), false);
    }
}


