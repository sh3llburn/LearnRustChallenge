use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use base64::{engine::general_purpose, Engine};

// main.rs
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
}

// ==================================================================================
// Decrypting ROT13 -> base64 -> ROT13
// ==================================================================================
// Author: Jonathan Ennis
// Date: 12 October 2024
// License: MIT 
//
// Description:
// This Rust program decrypts a string that was encrypted using a sequence of
// ROT13 -> base64 -> ROT13 to return the original plaintext. 
//
// The ciphertext is stored in a file called "ciphertext" and is read into memory 
// for processing. The decryption process consists of applying ROT13, decoding from 
// base64, and applying ROT13 again.
// ==================================================================================

/// This function handles the ROT13 decryption.
///
/// # Arguments
/// - `input`: A string slice that holds the ciphertext.
///
/// # Returns
/// - A `String` that holds the decrypted result.
///
/// # Example
/// ```
/// let decrypted = rot13("Uryyb, Jbeyq!"); // Output: "Hello, World!"
/// ```

fn rot13(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else {b'A'};
                let rotated = (c as u8 - base + 13) % 26 + base;
                rotated as char
            } else {
                c    // Non-alphabetic character
            }
        })
        .collect()
}

/// Decodes a Base64-encoded string into a UTF-8 string.
///
/// # Arguments
/// - `input`: A string slice that holds the Base64-encoded input.
///
/// # Returns
/// - A `String` containing the Base64-decoded UTF-8 string.
///
/// # Panics
/// 
/// Will panic if the input can't be decoded from Base64 or converted to a 
/// valid UTF-8 string. 
/// 
/// # Example
/// ```
/// /// let encoded = "SGVsbG8gd29ybGQh"; // "Hello world!" in base64
/// let decoded = decode_base64(encoded);
/// assert_eq!(decoded, "Hello world!");
/// ```

fn decode_base64(input: &str) -> String {
    let decoded_bytes = general_purpose::STANDARD.decode(input).expect("Failed to decode base64");
    String::from_utf8(decoded_bytes).expect("Failed to convert bytes to string")
}

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let args = Cli::parse();

    let file = File::open(&args.path).expect("Could not open file.");
    let reader = BufReader::new(file);
    let mut cipher = String::new();

    // Grab the ciphertext, using `?` to propagate any error
    for line in reader.lines() {
        cipher = line?;
    }

    let encoded_string = rot13(&cipher); // Grab the base64 encoded string 

    let decoded_string = decode_base64(&encoded_string);

    let flag = rot13(&decoded_string);

    println!("Flag: {}", flag); // Print the flag

    Ok(()) // Return Ok
} 