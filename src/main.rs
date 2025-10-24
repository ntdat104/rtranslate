use rtranslate::{translate, translate_vec};

fn main() {
    // Single
    match translate("Hello world", "auto", "vi") {
        Ok(result) => println!("Single: {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }

    // Multiple
    let phrases = ["Good morning", "How are you?", "Rust is great!"];
    let results = translate_vec(&phrases, "auto", "vi");

    for (i, res) in results.iter().enumerate() {
        match res {
            Ok(t) => println!("{} → {}", phrases[i], t),
            Err(e) => println!("{} → ERROR: {}", phrases[i], e),
        }
    }
}
