# 🦀 rtranslate

> A simple, dependency-free Rust wrapper for the Google Translate public web API — built using only the Rust standard library.

---

### ✨ Features
- 🔹 No external dependencies — uses `std::process::Command` with `curl`.
- 🔹 Supports **single** and **batch** translations.
- 🔹 Automatically detects the source language (`sl=auto`).
- 🔹 Graceful error handling for empty responses or rate limits.
- 🔹 Ready to use as a library or CLI example.

---

### 📦 Installation

#### Option 1 — Use directly from GitHub
```bash
cargo add --git https://github.com/ntdat104/rtranslate
```

#### Option 2 — Clone manually
```bash
git clone https://github.com/ntdat104/rtranslate.git
cd rtranslate
cargo build
```

---

### 🧰 Usage

#### Single translation
```rust
use rtranslate::translate;

fn main() {
    match translate("Hello world", "auto", "vi") {
        Ok(result) => println!("Translated: {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}
```

Output:
```
Translated: Xin chào thế giới
```

---

#### Batch translation
```rust
use rtranslate::translate_vec;

fn main() {
    let phrases = ["Good morning", "How are you?", "Rust is amazing!"];
    let results = translate_vec(&phrases, "auto", "vi");

    for (i, res) in results.iter().enumerate() {
        match res {
            Ok(t) => println!("{} → {}", phrases[i], t),
            Err(e) => println!("{} → ERROR: {}", phrases[i], e),
        }
    }
}
```

Example output:
```
Good morning → Chào buổi sáng
How are you? → Bạn khỏe không?
Rust is amazing! → Rust thật tuyệt vời!
```

---

### 🧪 Tests

You can run all unit and integration tests via:

```bash
cargo test
```

This will:
- Validate URL encoding and parsing
- Check translation parsing logic
- Optionally perform real translation requests (requires internet)

---

### ▶️ Examples

Run the included usage example:

```bash
cargo run --example basic
```

**`examples/basic.rs`:**
```rust
use rtranslate::{translate, translate_vec};

fn main() {
    println!("--- Single Example ---");
    println!("{:?}", translate("Rust is fast", "auto", "vi"));

    println!("
--- Batch Example ---");
    let texts = ["Good morning", "Good night", "Have fun!"];
    let results = translate_vec(&texts, "auto", "vi");
    for (i, res) in results.iter().enumerate() {
        println!("{} → {:?}", texts[i], res);
    }
}
```

---

### ⚠️ Notes
- This library uses Google’s **unofficial** translate endpoint (`translate.googleapis.com/translate_a/single`).
- There are **no API keys required**, but Google may rate-limit you for frequent requests.
- Recommended for small translation workloads, CLI tools, or quick utilities.

---

### 🪪 License
MIT License © 2025 [Tien Dat (ntdat104)](https://github.com/ntdat104)

---

### 💡 Contributing
Pull requests are welcome!  
Feel free to:
- Add new language tests  
- Improve JSON parsing  
- Add async support (with `hyper` or `reqwest`)  

---

### 🌍 Example Result (EN → VI)
| Input | Output |
|--------|---------|
| Hello world | Xin chào thế giới |
| How are you? | Bạn khỏe không? |
| Rust is great! | Rust thật tuyệt vời! |

---

**rtranslate** — lightweight, offline-friendly, and open to everyone. 🦀
