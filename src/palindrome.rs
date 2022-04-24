/// Largest palindrome product
/// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
/// Find the largest palindrome made from the product of two 3-digit numbers.
pub(crate) fn find_the_largest_palindrome_product() {
    let product_digit: u32 = 3;
    let largest_palindrome_product = _find_the_largest_palindrome_product(product_digit);
    match largest_palindrome_product {
        Some(num) => {
            let ((a, b), c) = num;
            println!("The largest palindrome for {} digit products, {}x{} = {} ", 3, a, b, c);
        },
        None => {
            println!("There is no largest palindrome for {} digit products is", 3);
        }
    }
}

fn _find_the_largest_palindrome_product(product_digit: u32) -> Option<((u64,u64), u64)> {
    if product_digit <= 0 {
        return None
    }
    let d : u64= 10;
    let max_product_value =  d.pow(product_digit as u32);
    let min_product_value =  d.pow(product_digit as u32 - 1);
    let mut max_value =  max_product_value * max_product_value - 1;
    while max_value > min_product_value {
        if _is_palindrome(max_value) {
            let  mut p1 = max_product_value - 1;
            while p1 > min_product_value {
                if max_value % p1 == 0 {
                    let p2 = max_value/p1;
                    if p2 < max_product_value && p2 > min_product_value {
                        return Some(((p1, p2), max_value))
                    }
                }
                p1 -= 1;
            }
        }
        max_value -= 1;
    }
    return None
}

fn _is_palindrome(num: u64) -> bool {
    let s = num.to_string();
    let bytes = s.as_bytes();
    for start in 0..bytes.len() {
        let end = bytes.len()-1-start;
        if start >= end {
            return true
        }
        if bytes[start] != bytes[end] {
            return false
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::palindrome;

    #[test]
    fn find_the_largest_palindrome_product() {
        let product_digit: u32 = 0;
        let largest_palindrome_product = palindrome::_find_the_largest_palindrome_product(product_digit);
        assert_eq!(largest_palindrome_product.is_none(), true);

        let product_digit: u32 = 1;
        let largest_palindrome_product = palindrome::_find_the_largest_palindrome_product(product_digit);
        assert_eq!(largest_palindrome_product.is_some(), true);
        let ((_, _), c) = largest_palindrome_product.unwrap();
        assert_eq!(9, c);

        let product_digit: u32 = 3;
        let largest_palindrome_product = palindrome::_find_the_largest_palindrome_product(product_digit);
        assert_eq!(largest_palindrome_product.is_some(), true);
        let ((_, _), c) = largest_palindrome_product.unwrap();
        assert_eq!(906609, c);
    }

    #[test]
    fn _is_palindrome() {
        assert_eq!(palindrome::_is_palindrome(1), true);
        assert_eq!(palindrome::_is_palindrome(9), true);
        assert_eq!(palindrome::_is_palindrome(10), false);
        assert_eq!(palindrome::_is_palindrome(11), true);
        assert_eq!(palindrome::_is_palindrome(12), false);
        assert_eq!(palindrome::_is_palindrome(99), true);
        assert_eq!(palindrome::_is_palindrome(100), false);
        assert_eq!(palindrome::_is_palindrome(101), true);
        assert_eq!(palindrome::_is_palindrome(111), true);
        assert_eq!(palindrome::_is_palindrome(199), false);
        assert_eq!(palindrome::_is_palindrome(123321), true);
        assert_eq!(palindrome::_is_palindrome(1234321), true);
        assert_eq!(palindrome::_is_palindrome(123321123321), true);
        assert_eq!(palindrome::_is_palindrome(12343211234321), true);
    }
}

