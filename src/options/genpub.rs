use bdk::keys::bip39::{Language, Mnemonic};
use bitcoin::base58;
use bitcoin::bip32::{DerivationPath, Xpriv, Xpub};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::Network;
use std::io::{self, Write};
use std::str::FromStr;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prompt the user to enter a 24-word mnemonic phrase
    print!("Please enter your 24-word mnemonic: ");
    // Flush stdout to ensure the prompt is displayed immediately
    io::stdout().flush()?;

    // Read the user's input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    // Remove any leading or trailing whitespace from the input
    let mnemonic_str = input.trim();

    // Parse the input string into a BIP-39 mnemonic
    // This will return an error if the input does not form a valid mnemonic
    let mnemonic = Mnemonic::parse_in(Language::English, mnemonic_str)?;

    // This seed is used to generate the master private key
    let seed = mnemonic.to_seed("");

    // Create a new Secp256k1 object
    let secp = Secp256k1::new();
    // Generate the xpriv (master private key)
    let master_xprv = Xpriv::new_master(Network::Bitcoin, &seed)?;

    // Define the derivation path for the account according to BIP84: m/84'/0'/0'
    // BIP84 is used for native SegWit (bech32) addresses
    let derivation_path = DerivationPath::from_str("m/84'/0'/0'")?;
    // Derive the account-level private key using the specified derivation path
    let account_xprv = master_xprv.derive_priv(&secp, &derivation_path)?;
    // Convert the derived private key into its corresponding extended public key
    let account_xpub = Xpub::from_priv(&secp, &account_xprv);

    // Serialize the account extended public key (xpub) into bytes
    let mut serialized = account_xpub.encode();

    // Modify the version bytes to match zpub format
    // The version bytes for zpub are: 0x04, 0xb2, 0x47, 0x46
    serialized[0] = 0x04;
    serialized[1] = 0xb2;
    serialized[2] = 0x47;
    serialized[3] = 0x46;
    // Encode the modified serialized data using Base58Check encoding to generate zpub
    let zpub = base58::encode_check(&serialized);

    // Print the zpub to the console
    println!("\nzpub: {}", zpub);

    // Return Ok(()) to indicate that the program executed successfully
    Ok(())
}
