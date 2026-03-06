use std::env;
use std::fmt::{Display, Formatter};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 2 {
        panic!("wrong number of args")
    }
    let limit: i32;
    let Some(val) = args.get(1) else {
        panic!("wrong number of inputs")
    };
    match val.parse::<i32>() {
        Ok(number) => limit = number,
        Err(_) => panic!("failed to parse input"),
    }

    println!(
        "getting the prime numbers using siev of eratothenes up to {}",
        limit
    );
    let data = sieve_of_eratosthenes(limit);
    for datum in data {
        if !datum.marked {
            print!(" {};", datum)
        }
    }
}

#[derive(Debug)]
struct Number {
    val: i32,
    marked: bool,
}
impl Number {
    fn new(val: i32) -> Number {
        Number { val, marked: false }
    }
}
impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
fn prepare_data(limit:i32)->Vec<Number>{
    //first i want to generate the full list of all numbers up until the limit
    //i used the range syntax and added 1 to make it exclusive, and started at 2 since we know 1 is not a prime
    let mut data: Vec<Number> = Vec::new();
    for x in 2..limit + 1 {
        data.push(Number::new(x));
    }
    data
}
fn sieve_of_eratosthenes(limit: i32) -> Vec<Number> {

    let mut current_p = 2;
    let mut data =prepare_data(limit);
    loop {
        let mut i=0;
        while i<data.len(){
            if data[i].val != current_p  && data[i].val % current_p == 0 {
                data.remove(i);
            }
            i+=1;
        }
        let p = get_p(&data, current_p);
        match p {
            None => {
                break;
            }
            Some(val) => current_p = val,
        }

        
    }

    data
}

fn get_p(data: &Vec<Number>, current_p: i32) -> Option<i32> {
    for datum in data {
        return if !datum.marked && datum.val > current_p {
            Some(datum.val)
        } else {
            continue;
        };
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn load_known_primes(path: &str) -> Vec<i32> {
        let content = fs::read_to_string(path).expect("failed to read primes file");

        content
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("failed to parse prime from file"))
            .collect()
    }

    fn assert_first_n_primes_match(actual: Vec<i32>, path: &str, count: usize) {
        let expected: Vec<i32> = load_known_primes(path).into_iter().take(count).collect();

        assert_eq!(
            actual.len(),
            count,
            "algorithm returned {} primes, expected {}",
            actual.len(),
            count
        );

        assert_eq!(actual, expected, "generated primes do not match file");
    }

    #[test]
    fn test_first_100_primes() {
        // Example: if you know your sieve limit is enough to produce at least 100 primes
        let actual: Vec<i32> = sieve_of_eratosthenes(541)
            .into_iter()
            .map(|n| n.val)
            .take(100)
            .collect();

        assert_first_n_primes_match(
            actual,
            "test/data/primes1.txt",
            100
        );
    }
    #[test]
    fn test_up_to10000_primes() {
        // Example: if you know your sieve limit is enough to produce at least 100 primes
        let actual: Vec<i32> = sieve_of_eratosthenes(10000)
            .into_iter()
            .map(|n| n.val)
            .take(1229)
            .collect();

        assert_first_n_primes_match(
            actual,
            "test/data/primes1.txt",
            1229
        );
    }
}