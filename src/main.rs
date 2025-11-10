use rtranslate::{translate, translate_vec};
use std::time::Instant;

fn main() {
    // Single translation with timing
    let start_single = Instant::now();
    match translate("Hello world", "auto", "vi") {
        Ok(result) => {
            let duration = start_single.elapsed();
            println!("Single: {}", result);
            println!("Time taken: {:?}", duration);
        }
        Err(err) => eprintln!("Error: {}", err),
    }

    // Multiple translation with timing
    let phrases = [
        "Good morning",
        "How are you?",
        "Rust is great!",
        "Faith and Gratitude",
    ];

    let start_multi = Instant::now();
    let results = translate_vec(&phrases, "auto", "vi");
    let duration_multi = start_multi.elapsed();

    for (i, res) in results.iter().enumerate() {
        match res {
            Ok(t) => println!("{} → {}", phrases[i], t),
            Err(e) => println!("{} → ERROR: {}", phrases[i], e),
        }
    }

    println!("Time taken for multiple translations: {:?}", duration_multi);
}
