mod least_common_multiple;
mod prime_factors;
mod digits_sum;
mod pangram;


fn main() {
    // println!("{}", least_common_multiple::get_common_multiple(4, 8, 32));
    println!("{:?}", prime_factors::get_prime_factors(20));
    println!("{:?}", digits_sum::get_sum(234));
    println!("{}", pangram::is_pangram(String::from("abcdefghijklmnopqrstuvwxyz")));
}
