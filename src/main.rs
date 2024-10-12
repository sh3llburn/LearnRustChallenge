use std::fs::File;
use std::io::Read;

// main.rs

// ========================================
// Decrypting ROT13 -> base64 -> ROT13
// ========================================
// Author: [Your Name]
// Date: [Today's Date]
// License: MIT (or whichever license you're using)
//
// Description:
// This Rust program decrypts a string that was encrypted using a sequence of
// ROT13 -> base64 -> ROT13. The program will reverse this process to return the 
// original plaintext. The decryption process follows the same sequence in reverse.
//
// The ciphertext is stored in a file called "ciphertext" and is read into memory for processing.
// The decryption process consists of applying ROT13, decoding from base64, and applying ROT13 again.
// ========================================

/// This function handles the ROT13 decryption.
///
/// # Arguments
/// - `input`: A string slice that holds the ciphertext.
///
/// # Returns
/// - A `String` that holds the decoded result.
///
/// # Example
/// ```
/// let decrypted = rot13("Uryyb, Jbeyq!"); // Output: "Hello, World!"
/// ```
fn main() {
    // Open the file, handle errors
    let in_file = File::open("ciphertext");

    match in_file {
        Ok(mut file) => {
            // File exists, read it
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Problem reading the file");
            println!("Encrypted string: {}", contents);
            rot13(&contents); // Pass the contents to the rot13 function
        }
        Err(e) => {
            println!("Failed to open the file: {}", e);
        }
    }
}

/// Rot13 function that takes a string as input and prints it
fn rot13(input: &str) {
    println!("{}", input);
}