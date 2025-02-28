use std::io;

mod options {
    pub mod genaddress;
    pub mod genpub;
    pub mod genwords;
}
fn main() {
    println!("What do you want to do?");
    println!("1 - generate random 24 words");
    println!("2 - generate xpub & zpub from 24 words");
    println!("3 - generate bech32 addresses from xpub or zpub");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error...");

    match input.trim() {
        "1" => {
            let output = options::genwords::main();
            println!("{:?}", output);
        }

        "2" => {
            let output = options::genpub::main();
            println!("{:?}", output);
        }

        "3" => {
            let output = options::genaddress::main();
            println!("{:?}", output);
        }
        _ => {
            println!("Invalid input.");
        }
    }
}
