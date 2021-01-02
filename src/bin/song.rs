use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut gifts_history: Vec<(usize, usize)> = vec![(0, 0); 13];

    for max_day in 1..13 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            book::ORDINALS[max_day]
        );

        gifts_history[max_day] = (
            rng.gen_range(0..book::ADJECTIVES.len()),
            rng.gen_range(0..book::NOUNS.len()),
        );

        for day in (1..(max_day + 1)).rev() {
            let (adj, noun) = gifts_history[day];
            println!("{}", describe_gift(day, max_day != 1, adj, noun));
        }

        println!("")
    }
}

fn describe_gift(day: usize, append_and: bool, adj: usize, noun: usize) -> String {
    if day == 1 && append_and {
        "And a partridge in a pear tree.".to_string()
    } else if day == 1 {
        "A partridge in a pear tree.".to_string()
    } else {
        format!(
            "{} {} {}",
            book::NUMERALS[day],
            book::ADJECTIVES[adj],
            book::NOUNS[noun]
        )
        .to_string()
    }
}
