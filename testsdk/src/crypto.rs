use ed25519_dalek::{Keypair, Signer, Verifier};
use rand::rngs::OsRng;
use std::string::String;

#[derive(Debug)]
struct CryptoKeypair {
    private_key: String,
    public_key: String,
}

fn generate_keypair() -> CryptoKeypair {
    let mut csprng = OsRng {};
    let keypair = Keypair::generate(&mut csprng);

    let private_key = hex::encode(&keypair.secret.to_bytes());
    let public_key = hex::encode(&keypair.public.to_bytes());

    CryptoKeypair { private_key, public_key }
}

fn main() {
    let keypair = generate_keypair();
    println!("Key pair: {:?}", keypair);
}
