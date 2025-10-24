//! # rtranslate 🦀
//!
//! A minimal, dependency-free Rust wrapper for Google Translate web API.
//!
//! ```
//! use rtranslate::translate;
//!
//! fn main() {
//!     let result = translate("Hello", "auto", "vi").unwrap();
//!     println!("Translated: {}", result);
//! }
//! ```
//!
//! Also supports batch translation:
//!
//! ```
//! use rtranslate::translate_vec;
//!
//! fn main() {
//!     let phrases = ["Good morning", "Rust is great"];
//!     let results = translate_vec(&phrases, "auto", "vi");
//!     for r in results {
//!         println!("{:?}", r);
//!     }
//! }
//! ```

use std::process::Command;
use std::fmt;

/// Error type for rtranslate
#[derive(Debug)]
pub enum TranslateError {
    CommandFailed(String),
    Utf8Error(String),
    ParseError(String),
    EmptyResponse,
    RateLimited,
}

impl fmt::Display for TranslateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranslateError::CommandFailed(e) => write!(f, "Command failed: {}", e),
            TranslateError::Utf8Error(e) => write!(f, "UTF-8 decode failed: {}", e),
            TranslateError::ParseError(e) => write!(f, "Parse error: {}", e),
            TranslateError::EmptyResponse => write!(f, "Empty response from server"),
            TranslateError::RateLimited => write!(f, "Rate limited by Google Translate"),
        }
    }
}

impl std::error::Error for TranslateError {}

/// Translate a single string.
///
/// # Example
/// ```
/// let translated = rtranslate::translate("Hello world", "auto", "vi").unwrap();
/// println!("Translated: {}", translated);
/// ```
pub fn translate(text: &str, from: &str, to: &str) -> Result<String, TranslateError> {
    let q = url_encode(text);
    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
        from, to, q
    );

    let output = Command::new("curl")
        .arg("-s")
        .arg(&url)
        .output()
        .map_err(|e| TranslateError::CommandFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(TranslateError::CommandFailed(format!(
            "curl exited with: {:?}",
            output.status.code()
        )));
    }

    let body = String::from_utf8(output.stdout)
        .map_err(|e| TranslateError::Utf8Error(e.to_string()))?;

    if body.trim().is_empty() {
        return Err(TranslateError::EmptyResponse);
    }

    // Detect rate limit or block
    if body.contains("<html>") || body.contains("503") || body == "[]" {
        return Err(TranslateError::RateLimited);
    }

    parse_translation(&body)
}

/// Translate multiple strings (sync)
///
/// # Example
/// ```
/// let phrases = ["Rust", "is", "awesome"];
/// let results = rtranslate::translate_vec(&phrases, "auto", "vi");
/// for (i, res) in results.iter().enumerate() {
///     match res {
///         Ok(t) => println!("{} → {}", phrases[i], t),
///         Err(e) => println!("{} → ERROR: {}", phrases[i], e),
///     }
/// }
/// ```
pub fn translate_vec(texts: &[&str], from: &str, to: &str) -> Vec<Result<String, TranslateError>> {
    let mut results = Vec::with_capacity(texts.len());
    for text in texts {
        results.push(translate(text, from, to));
    }
    results
}

fn parse_translation(body: &str) -> Result<String, TranslateError> {
    if let Some(start) = body.find("[[[\"") {
        let after = &body[start + 4..];
        if let Some(end) = after.find('"') {
            let translated = &after[..end];
            if translated.trim().is_empty() {
                return Err(TranslateError::EmptyResponse);
            }
            return Ok(translated.to_string());
        }
    }
    Err(TranslateError::ParseError(format!(
        "Unexpected response format: {}",
        &body[..body.len().min(120)]
    )))
}

fn url_encode(input: &str) -> String {
    input
        .bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
            _ => format!("%{:02X}", b),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode_basic() {
        assert_eq!(url_encode("Hello world!"), "Hello%20world%21");
    }

    #[test]
    fn test_parse_translation_valid() {
        let json = r#"[[["Xin chào","Hello",null,null,3,null,null,[[]]]],null,"en"]"#;
        let result = parse_translation(json).unwrap();
        assert_eq!(result, "Xin chào");
    }

    #[test]
    fn test_parse_translation_invalid() {
        let json = "INVALID";
        assert!(parse_translation(json).is_err());
    }

    #[test]
    fn test_empty_body_error() {
        let err = translate("", "auto", "vi").unwrap_err();
        assert!(matches!(err, TranslateError::EmptyResponse | TranslateError::RateLimited | TranslateError::ParseError(_)));
    }
}
