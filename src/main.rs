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
