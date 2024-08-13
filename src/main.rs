use rand::prelude::*;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    println!("Starting the dice rolling...");

    let start_instant = Instant::now();

    const ROLL_COUNT: u32 = 1_000_000_000;
    let results = (0..ROLL_COUNT)
        .into_par_iter()
        .map(|roll_number| {
            let mut successes: u8 = 0;
            let mut rng = ThreadRng::default();

            for _ in 0..231 {
                let roll = rng.gen_range(0..4);

                if roll == 0 {
                    successes += 1;
                }
            }

            (roll_number, successes)
        })
        .collect::<Vec<_>>();
    let end_instant = Instant::now();

    let duration = end_instant - start_instant;

    let above_177 = results
        .iter()
        .filter(|(_, successes)| successes > &177)
        .take(1)
        .collect::<Vec<_>>();

    let max = results.iter().max_by(|a, b| a.1.cmp(&b.1));

    if above_177.is_empty() {
        println!(
            "In {} tries, no result was above 177. The best result I got was {}",
            ROLL_COUNT,
            max.unwrap().1
        );
    } else {
        println!(
            "The first above 177 (it was a {} successes) happened at {}!",
            above_177[0].1, above_177[0].0
        );
    }

    let duration_secs = duration.as_secs();
    println!(
        "It took me {}h {}m {}s to run it {} times.",
        duration_secs / 3600,
        duration_secs / 60 % 60,
        duration_secs % 60,
        ROLL_COUNT
    );
}
