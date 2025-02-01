use bdk::keys::bip39::{Language, Mnemonic};
use rand::RngCore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define a 32-byte array to store 256 bits of entropy
    // Using 256 bits (32 bytes) of entropy is standard for generating a 24-word mnemonic phrase
    let mut entropy = [0u8; 32];

    // Fill the 'entropy' array with random bytes
    // This uses the thread-local random number generator to ensure sufficient randomness
    rand::thread_rng().fill_bytes(&mut entropy);

    // Generate a BIP-39 mnemonic phrase from the random entropy
    let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy)?;

    // Print the generated 24-word mnemonic phrase to the console
    println!("Generated 24-word mnemonic:\n{}", mnemonic);

    // Return Ok(()) to indicate that the program executed successfully
    Ok(())
}
