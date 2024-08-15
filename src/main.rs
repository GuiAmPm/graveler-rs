use rand::{distributions::Uniform, prelude::*};
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    println!();
    println!();
    println!("Dear Austin,");
    println!("Hey it's me, Gui!");
    println!();
    println!("Starting the dice rolling...");

    // Start the clock
    let start_instant = Instant::now();

    const ROLL_COUNT: u32 = 1_000_000_000;

    // Create the distribution of values beforehand
    let between = Uniform::from(0..4);

    let results: (u32, u8) = (0..ROLL_COUNT) // iterate through the number of samples
        .into_par_iter()
        .map(|roll_number| {
            let mut rng = ThreadRng::default();

            let successes = (0..231) // Number of required movements
                .filter(|_| between.sample(&mut rng) == 0) // Filter the results that are equal to 0 (1/4)
                .count() as u8; // Count the number of results

            (roll_number, successes)
        })
        .reduce_with(|a, b| if a.1 > 177 || a.1 >= b.1 { a } else { b })
        .unwrap();

    // Note: Since it's running in parallel, I can't simply break whenever something is above 177.
    // It's possible to make the code faster, if a 177 is hit if the filter below was applied before the collect above. That would save a lot of memory.
    // But as seen in the video, this is pretty unlikely, so most of the time, the code will need to run through all the samples.
    // and I lose the ability to check the highest result below 177.

    // Stop the clock
    let end_instant = Instant::now();

    let duration = end_instant - start_instant;

    // Statistics
    // These do some lookup in the saved results, may slow down the code, but we are not counting anymore

    if results.1 < 177 {
        // No 177? :(
        println!(
            "In {} tries, no result was above 177. The best result I got was {}",
            ROLL_COUNT, results.1
        );
    } else {
        // We got at least one 177! Which was the first?
        println!(
            "The first above 177 (it was {} successes) happened at {}!",
            results.1, results.0
        );
    }

    // Outputs how long it took and convert to hours, minutes and seconds
    let duration_secs = duration.as_secs();
    println!(
        "It took me {}h {}m {}s to run it {} times.",
        duration_secs / 3600,
        duration_secs / 60 % 60,
        duration_secs % 60,
        ROLL_COUNT
    );

    println!("Sincerely,");
    println!("Gui");
    println!();
    println!();
}
