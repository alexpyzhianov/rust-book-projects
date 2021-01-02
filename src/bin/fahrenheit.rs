fn main() {
    let args: Vec<String> = std::env::args().collect();
    let units = &args[1];
    let value = &args[2];

    let value: f64 = value
        .trim()
        .parse()
        .expect("Invalid value input. Second argument must be a valid float number");

    let result = match units.as_str() {
        "C" => value * 9.0 / 5.0 + 32.0,
        "F" => (value - 32.0) * 5.0 / 9.0,
        _ => panic!("Invalid units input. First argument must be C or F"),
    };

    println!("{:?}", result)
}
