use bitcoin::Network;
use bitcoin::secp256k1::{rand, Secp256k1, Keypair};

pub fn generate_wallet() {
    let elliptic_curve_fn = Secp256k1::new();
    let network = Network::Bitcoin;
    
    let (secret_key, public_key) = elliptic_curve_fn.generate_keypair(&mut rand::thread_rng());
    let key_pair = Keypair::from_secret_key(&elliptic_curve_fn, &secret_key);
}