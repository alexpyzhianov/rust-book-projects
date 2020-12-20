use rand::Rng;
use std::cmp::Ordering;
use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let can_cheat = args.last().unwrap() == "--cheat";

    let min_number = 1;
    let max_number = rand::thread_rng().gen_range(100..1000);
    let target = rand::thread_rng().gen_range(min_number..max_number);

    let mut attempts = ((max_number as f32).log2().ceil() - 1.0) as i32;
    let mut left_clamp = min_number;
    let mut right_clamp = max_number;

    println!(
        "Guess the number! Min is {}, max is {}. You have {} attempts",
        min_number, max_number, attempts
    );

    loop {
        println!("===================================");
        println!("Enter your guess: ");

        if can_cheat {
            println!(
                "CHEAT: YOUR BEST BET IS {}!",
                (right_clamp - left_clamp) / 2 + left_clamp
            );
        }

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Please enter a number no longer than two digits");

        let guess: i32 = guess.trim().parse().expect("Not a valid number");

        match guess.cmp(&target) {
            Ordering::Less => {
                left_clamp = guess;
                println!("Too low!")
            }
            Ordering::Greater => {
                right_clamp = guess;
                println!("Too high!")
            }
            Ordering::Equal => {
                println!("Correct!");

                if can_cheat {
                    println!("CHEAT: TOLD YA")
                }
                break;
            }
        }

        attempts -= 1;

        if attempts > 0 {
            if can_cheat {
                println!(
                    "CHEAT: SO IT'S IN BETWEEN {} AND {}. INTERESTING...",
                    left_clamp, right_clamp
                );
            }

            let chance = 1.0 / ((right_clamp as f32) - (left_clamp as f32));
            println!(
                "Your chance to win in the next round is around {:.2} percent",
                chance * 100.0
            );
            println!("{} attempts left", attempts);
        } else {
            if can_cheat {
                println!("CHEAT: SORRY, FAILED YA")
            }

            println!("The target was {}", target);
            println!("No attempts left. Better luck next time");
            break;
        }
    }
}
