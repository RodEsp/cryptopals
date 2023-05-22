mod base64;
mod hex;
mod xor;

fn main() {
    println!("Challenge 1: {}", base64::encode(hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")));

    println!(
        "Challenge 2: {}",
        hex::encode(xor::fixed(hex::decode("1c0111001f010100061a024b53535009181c")).into())
    );
}
