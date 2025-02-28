use bitcoin::address::{Address, KnownHrp};
use bitcoin::bip32::{ChildNumber, Xpub};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::CompressedPublicKey;
use bs58;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    io::{self, Write},
    str::FromStr,
};

// SLIP-132 version codes
fn get_slip132_prefixes() -> HashMap<&'static str, [u8; 4]> {
    let mut prefixes = HashMap::new();
    prefixes.insert("xpub", [4, 136, 178, 30]);
    prefixes.insert("zpub", [4, 178, 71, 70]);
    prefixes
}

// convert zpub to xpub
fn convert_zpub_to_xpub(zpub: &str) -> Result<String, Box<dyn std::error::Error>> {
    let prefixes = get_slip132_prefixes();

    // Control first 4 bytes of zpub
    let decoded = bs58::decode(zpub).into_vec()?;

    if decoded.len() != 82 {
        return Err(format!(
            "Invalid zpub length: expected 82 bytes, got {}",
            decoded.len()
        )
        .into());
    }

    let current_prefix = &decoded[..4];

    // If current prefix is zpub, change it to xpub
    if current_prefix != prefixes.get("zpub").unwrap() {
        return Err("Input is not a valid zpub".into());
    }

    // Convert first 4 bytes to xpub format
    let prefix_xpub = prefixes.get("xpub").unwrap();
    let mut new_data = decoded.clone();
    new_data[..4].copy_from_slice(prefix_xpub);

    // Calc SHA-256 hash and add new checksum
    let checksum = Sha256::digest(&Sha256::digest(&new_data[..78])); // SHA-256 for first 78 bytes
    let checksum = &checksum[..4]; // Checksum of first 4 bytes

    // Add new checksum
    let mut final_data = new_data[..78].to_vec();
    final_data.extend_from_slice(checksum);

    // Encode in Base58 to generate a new xpub
    let xpub = bs58::encode(final_data).into_string();
    Ok(xpub)
}

fn generate_addresses(
    extended_key: &str,
    num_addresses: u32,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let secp = Secp256k1::new();
    let prefixes = get_slip132_prefixes();

    // Decode the provided extended key (xpub or zpub) to determine its type
    let decoded = bs58::decode(extended_key).into_vec()?;
    if decoded.len() != 82 {
        return Err("Invalid extended key length. Expected 82 bytes when decoded.".into());
    }
    let current_prefix = &decoded[..4];

    // Convert input to xpub if necessary
    let xpub_str = if current_prefix == prefixes.get("zpub").unwrap() {
        // Input is zpub, convert it to xpub
        convert_zpub_to_xpub(extended_key)?
    } else if current_prefix == prefixes.get("xpub").unwrap() {
        // Input is already an xpub, use it directly
        extended_key.to_string()
    } else {
        return Err("Input is not a valid xpub or zpub.".into());
    };

    // Convert the xpub string into an Xpub object
    let xpub = Xpub::from_str(&xpub_str)?;

    let mut addresses = Vec::new();

    for i in 0..num_addresses {
        // Use 0 for external chain, use i for index
        let path = vec![
            ChildNumber::Normal { index: 0 },
            ChildNumber::Normal { index: i },
        ];

        // Generate child xpub
        let child_xpub = xpub.derive_pub(&secp, &path)?;

        // Make pubkey compressed
        let compressed_pubkey = &CompressedPublicKey(child_xpub.public_key);

        // Generate Bech32 address
        let address = Address::p2wpkh(compressed_pubkey, KnownHrp::Mainnet);

        addresses.push(address.to_string());
    }

    Ok(addresses)
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter your xpub or zpub key: ");
    io::stdout().flush()?;

    let mut pubkey = String::new();
    io::stdin().read_line(&mut pubkey)?;
    let pubkey = pubkey.trim();

    let addresses = generate_addresses(pubkey, 5)?;

    println!("\nGenerated Bitcoin Bech32 Addresses:");
    for address in addresses {
        println!("{}", address);
    }

    // Return Ok(()) to indicate that the program executed successfully
    Ok(())
}
