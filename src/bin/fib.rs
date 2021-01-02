// 0 1 1 2 3
fn fib(n: usize) -> Vec<u64> {
    if n <= 0 {
        panic!("The index should be a positive integer")
    } else if n == 1 {
        return vec![0];
    }

    let mut numbers: Vec<u64> = vec![0, 1];

    while numbers.len() < n {
        let last = numbers[numbers.len() - 1];
        let second_to_last = numbers[numbers.len() - 2];
        numbers.push(last + second_to_last);
    }

    numbers
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n = &args[1];

    let n: usize = n
        .trim()
        .parse()
        .expect("Invalid value input. First argument must be a valid number");

    println!("{:?}", fib(n))
}
