use bdk::keys::bip39::{Language, Mnemonic};
use rand::RngCore;
use std::io::{self, Write};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prompt the user for the mnemonic word count (12 or 24)
    print!("Enter mnemonic word count (12 or 24): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    // Determine the required entropy size:
    // 12 words require 16 bytes (128 bits), 24 words require 32 bytes (256 bits)
    let entropy_size = match input {
        "12" => 16,
        "24" => 32,
        _ => {
            println!("Invalid input. Please enter '12' or '24'.");
            return Ok(());
        }
    };

    let mut entropy = vec![0u8; entropy_size];

    rand::rng().fill_bytes(&mut entropy);

    // Generate a BIP-39 mnemonic phrase from the random entropy
    let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy)?;

    // Print the generated mnemonic phrase to the console
    println!("Generated {}-word mnemonic:\n{}", input, mnemonic);

    // Return Ok(()) to indicate that the program executed successfully
    Ok(())
}
