use rtranslate::{translate, translate_vec};

fn main() {
    println!("--- Single Example ---");
    match translate("Rust is fast", "auto", "vi") {
        Ok(res) => println!("Translated: {}", res),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n--- Batch Example ---");
    let inputs = ["Good morning", "Good night", "Have fun!"];
    let outputs = translate_vec(&inputs, "auto", "vi");

    for (i, out) in outputs.iter().enumerate() {
        match out {
            Ok(t) => println!("{} → {}", inputs[i], t),
            Err(e) => println!("{} → ERROR: {}", inputs[i], e),
        }
    }
}
