use create2::calc_addr;
use std::env::args;
use std::process::exit;

fn main() {
    if args().len() != 4 {
        println!("usage: create2 [addr] [salt] [code]");
        exit(1);
    }

    let addr = args().nth(1).expect("no address given");
    let salt = args().nth(2).expect("no salt given");
    let code = args().nth(3).expect("no code given");

    let addr = hex::decode(addr.replace("0x", "")).expect("address must be in hex");
    let salt = hex::decode(salt.replace("0x", "")).expect("salt must be in hex");
    let code = hex::decode(code.replace("0x", "")).expect("code must be in hex");

    if addr.len() != 20 {
        println!("address must be 20 bytes");
        exit(1);
    }

    if salt.len() != 32 {
        println!("salt must be 32 bytes");
        exit(1);
    }

    let mut fixed_addr = [0; 20];
    let mut fixed_salt = [0; 32];

    fixed_addr.copy_from_slice(&addr[0..20]);
    fixed_salt.copy_from_slice(&salt[0..32]);

    let addr = calc_addr(&fixed_addr, &fixed_salt, &code);

    println!("{}", hex::encode(addr));
}
