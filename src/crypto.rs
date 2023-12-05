use openssl::rsa::Rsa;
use openssl::pkey::PKey;
use openssl::pkey::Private;

pub fn generate_keypair() -> (Vec<u8>, PKey<Private>) {
    let rsa = Rsa::generate(2048).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();

    let pub_key: Vec<u8> = pkey.public_key_to_pem().unwrap();
    (pub_key, pkey)
}
