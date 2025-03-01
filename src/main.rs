use std::io;

mod options {
    pub mod genaddress;
    pub mod genpub;
    pub mod genwords;
}
fn main() {
    println!("What do you want to do?");
    println!("1 - generate mnemonic phrase");
    println!("2 - generate xpub & zpub from mnemonic phrase");
    println!("3 - generate bech32 addresses from xpub or zpub");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error...");
        match input.trim() {
            "1" => {
                let output = options::genwords::main();
                println!("{:?}", output);
                break;
            }

            "2" => {
                let output = options::genpub::main();
                println!("{:?}", output);
                break;
            }

            "3" => {
                let output = options::genaddress::main();
                println!("{:?}", output);
                break;
            }
            _ => {
                println!("Invalid input.");
                continue;
            }
        }
    }
}
