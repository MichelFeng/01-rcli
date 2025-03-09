use rand::seq::IndexedRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    no_uppercase: bool,
    no_lowercase: bool,
    no_number: bool,
    no_symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut password = String::with_capacity(length as usize);
    let mut chars = Vec::new();
    if !no_uppercase {
        chars.extend_from_slice(UPPER);
    }
    if !no_lowercase {
        chars.extend_from_slice(LOWER);
    }
    if !no_number {
        chars.extend_from_slice(NUMBER);
    }
    if !no_symbol {
        chars.extend_from_slice(SYMBOL);
    }
    for _ in 0..length {
        let c = chars.choose(&mut rng).unwrap();
        password.push(*c as char);

        // let idx = rng.random_range(0..chars.len());
        // password.push(chars[idx] as char);
    }

    println!("Generated password: {}", password);

    // output the password strength
    let result = zxcvbn(&password, &[]);
    eprintln!("Password strength: {}", result.score());
    Ok(())
}
