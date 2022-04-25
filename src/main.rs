mod multiples;
mod arrays;
mod fibonacci;
mod prime_factor;
mod palindrome;
mod series;

/// This global variable refers to the title for the application.
static  APPLICATION_NAME: &str = "Implementation of the Project Euler Problems in Rust!";

pub fn print_header(header: &str, fn_list: Vec<(&str, fn())>) {
    println!("### {}", header);
    for (sub_header, run) in fn_list {
        println!("#### {}", sub_header);
        run();
        println!("------------------\n");
    }
}

fn main() {
    println!("{}", APPLICATION_NAME);

    print_header("Multiples", vec![
        ("Find the sum of the multiples of the factors",   multiples::find_the_sum_of_multiples_of_the_factors),
    ]);

    print_header("Fibonacci Series", vec![
        ("Find the sum of the even fibonacci sequences",   fibonacci::find_sum_of_even_fib_numbers),
    ]);

    print_header("Prime Factor", vec![
        ("Find the largest prime factor",  prime_factor::find_the_largest_prime_factor),
        ("Find the nth prime factor",  prime_factor::find_the_nth_prime),
    ]);

    print_header("Largest palindrome product", vec![
        ("Find the largest palindrome of the products",  palindrome::find_the_largest_palindrome_product),
    ]);

    print_header("Smallest multiple", vec![
        ("Find the smallest multiple",  multiples::find_the_smallest_of_multiples_evenly_divided),
    ]);

    print_header("Series", vec![
        ("Sum square difference",  series::sum_square_difference),
    ]);
}
